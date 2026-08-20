use super::{MusicBackend, PlaybackState, Track, TrackId};
use crate::{
    app::{Background, CantusApp, Update},
    render::{
        lyrics::{self, LyricsRequest},
        text,
        track::{AudioFeatures, PALETTE_COLORS},
    },
};
use arrayvec::ArrayVec;
use image::{RgbaImage, imageops};
use isthmus::{Unorm8x4, glam::Vec3};
use palette::{Clamp, IntoColor, Lch, color_theory::Analogous};
use serde::Deserialize;
use std::{
    array,
    collections::HashMap,
    error::Error,
    ops::Range,
    time::{Duration, Instant},
};
use tracing::warn;
use ureq::Agent;

const RECCO_FEATURES_URL: &str = "https://api.reccobeats.com/v1/audio-features";
const RETRY_DELAY: Duration = Duration::from_secs(30);
pub const IMAGE_SIZE: u32 = 64;
pub type ArtState = Fetch<AlbumArt>;
#[derive(Clone)]
pub struct Enrichment {
    background: Background,
    pub(crate) http: Agent,
}

impl Enrichment {
    pub(crate) fn new(background: Background) -> Self {
        Self {
            background,
            http: Agent::config_builder()
                .user_agent(concat!("Cantus/", env!("CARGO_PKG_VERSION")))
                .timeout_global(Some(Duration::from_secs(15)))
                .build()
                .into(),
        }
    }

    fn submit(&self, work: impl FnOnce() -> Update<CantusApp> + Send + 'static) -> bool {
        self.background.submit(work)
    }

    pub(crate) fn request_lyrics(
        &self,
        track: &Track,
        music: MusicBackend,
        shaper: text::Shaper,
    ) -> bool {
        let request = LyricsRequest {
            uri: track.uri.clone(),
            track_id: track.id,
            name: track.name.clone(),
            artist: track.artist.clone(),
            album: track.album.clone(),
            duration_ms: track.duration_ms,
        };
        let http = self.http.clone();
        self.submit(move || {
            let uri = request.uri.clone();
            let state = fetch_lyrics(&request, &http, &music, &shaper);
            Box::new(move |app| {
                if let Some(track) =
                    app.playback.queue.iter_mut().find(|track| {
                        track.uri == uri && matches!(track.runtime.lyrics, Fetch::Fetching)
                    })
                {
                    track.runtime.lyrics = state;
                }
            })
        })
    }
}

#[derive(Clone)]
pub enum Fetch<T> {
    Missing(Instant),
    Fetching,
    Ready(T),
}

impl<T> Default for Fetch<T> {
    fn default() -> Self {
        Self::Missing(Instant::now())
    }
}

impl<T> Fetch<T> {
    pub fn retry() -> Self {
        Self::Missing(Instant::now() + RETRY_DELAY)
    }

    pub fn request(&mut self, now: Instant) -> bool {
        if !matches!(self, Self::Missing(retry_at) if *retry_at <= now) {
            return false;
        }
        *self = Self::Fetching;
        true
    }

    pub const fn ready(&self) -> Option<&T> {
        match self {
            Self::Ready(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct AlbumArt {
    pub pixels: Box<[u8]>,
    palette: [Unorm8x4; PALETTE_COLORS],
}

#[derive(Deserialize)]
struct ReccoResponse {
    #[serde(alias = "content")]
    items: Vec<ReccoFeatures>,
}

#[derive(Deserialize)]
struct ReccoFeatures {
    href: String,
    #[serde(flatten)]
    features: AudioFeatures,
}

impl Fetch<AlbumArt> {
    pub fn palette(&self) -> [Unorm8x4; PALETTE_COLORS] {
        self.ready()
            .map_or_else(|| [Unorm8x4::default(); PALETTE_COLORS], |art| art.palette)
    }
}

fn fetch_lyrics(
    request: &LyricsRequest,
    http: &Agent,
    music: &MusicBackend,
    shaper: &text::Shaper,
) -> Fetch<lyrics::Lyrics> {
    let result = lyrics::fetch(http, request).map_or_else(
        || {
            request
                .track_id
                .map_or_else(|| Ok(Vec::new()), |id| music.lyrics(id))
        },
        Ok,
    );
    match result {
        Ok(segments) => Fetch::Ready(
            lyrics::Lyrics::shape(segments, request.duration_ms as f32, shaper).unwrap_or_default(),
        ),
        Err(error) => {
            warn!(%error, track = request.name, "Failed to fetch lyrics");
            Fetch::retry()
        }
    }
}

fn fetch_art(http: &Agent, url: &str) -> ArtState {
    let result = (|| -> Result<_, Box<dyn Error + Send + Sync>> {
        let bytes = http.get(url).call()?.body_mut().read_to_vec()?;
        let image = image::load_from_memory(&bytes)?
            .resize_to_fill(IMAGE_SIZE, IMAGE_SIZE, imageops::FilterType::Lanczos3)
            .to_rgba8();
        Ok(AlbumArt {
            palette: image_palette(&image),
            pixels: image.into_raw().into_boxed_slice(),
        })
    })();
    match result {
        Ok(art) => Fetch::Ready(art),
        Err(error) => {
            warn!(%error, %url, "Failed to load image");
            Fetch::retry()
        }
    }
}

fn art_slots(playback: &mut PlaybackState) -> impl Iterator<Item = (&str, &mut ArtState)> {
    playback
        .queue
        .iter_mut()
        .filter_map(|track| track.image.as_deref().map(|url| (url, &mut track.runtime.art)))
        .chain(
            playback.playlists.iter_mut().filter_map(|playlist| {
                playlist.image_url.as_deref().map(|url| (url, &mut playlist.art))
            }),
        )
}

impl CantusApp {
    pub fn refresh_track_enrichment(&mut self) {
        self.refresh_enrichment(true);
    }

    pub fn refresh_art(&mut self) {
        self.refresh_enrichment(false);
    }

    fn refresh_enrichment(&mut self, include_audio: bool) {
        let now = Instant::now();
        let mut audio = if include_audio {
            self.playback
                .queue
                .iter_mut()
                .filter_map(|track| track.id.filter(|_| track.runtime.audio_features.request(now)))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        audio.sort_unstable();
        audio.dedup();

        if !audio.is_empty() {
            let http = self.enrichment.http.clone();
            self.enrichment.submit(move || {
                let features = resolve_audio_features(&http, &audio);
                Box::new(move |app| {
                    for track in &mut app.playback.queue {
                        let Some(features) = track.id.and_then(|id| features.get(&id)) else {
                            continue;
                        };
                        track.runtime.audio_features =
                            features.map_or_else(Fetch::default, Fetch::Ready);
                    }
                })
            });
        }

        let mut art = art_slots(&mut self.playback)
            .filter_map(|(url, state)| state.request(now).then(|| url.to_owned()))
            .collect::<Vec<_>>();
        art.sort_unstable();
        art.dedup();
        for url in art {
            let http = self.enrichment.http.clone();
            let requested = url.clone();
            if !self.enrichment.submit(move || {
                let state = fetch_art(&http, &url);
                Box::new(move |app| app.set_art_state(&url, &state))
            }) {
                self.set_art_state(&requested, &Fetch::Missing(now));
            }
        }
    }

    fn set_art_state(&mut self, url: &str, state: &ArtState) {
        for (slot_url, slot) in art_slots(&mut self.playback) {
            if slot_url == url {
                *slot = state.clone();
            }
        }
    }
}

fn resolve_audio_features(
    http: &Agent,
    track_ids: &[TrackId],
) -> HashMap<TrackId, Option<AudioFeatures>> {
    let mut output = track_ids.iter().map(|&id| (id, None)).collect::<HashMap<_, _>>();
    for batch in track_ids.chunks(40) {
        let ids = batch.iter().map(TrackId::as_str).collect::<Vec<_>>().join(",");
        let Ok(features) = http
            .get(RECCO_FEATURES_URL)
            .query("ids", &ids)
            .call()
            .and_then(|response| response.into_body().read_json::<ReccoResponse>())
            .inspect_err(|err| warn!("Failed to fetch ReccoBeats audio features: {err}"))
        else {
            continue;
        };
        output.extend(features.items.into_iter().filter_map(|item| {
            Some((
                item.href.rsplit('/').next()?.parse().ok()?,
                Some(item.features.normalized()),
            ))
        }));
    }
    output
}

fn complete_palette(colors: &mut ArrayVec<(Lch, f32), PALETTE_COLORS>) {
    colors.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut index = 1;
    while index < colors.len() {
        let (color, weight) = colors[index];
        if let Some(duplicate) = colors[..index]
            .iter()
            .position(|(other, _)| (color.hue - other.hue).into_degrees().abs() < 20.0)
        {
            colors[duplicate].1 += weight;
            colors.remove(index);
        } else {
            index += 1;
        }
    }

    let measured = colors.len();
    for index in 0..PALETTE_COLORS - measured {
        let (source, weight) = colors[index % measured];
        let (lower, upper) = source.analogous();
        let mut generated = match index {
            2 if measured == 1 => source.analogous_secondary().0,
            index if index % 2 == 0 => lower,
            _ => upper,
        };
        generated.chroma = generated.chroma.max(35.0);
        colors.push((generated, weight * 0.5));
    }
    colors.sort_by(|a, b| a.0.l.total_cmp(&b.0.l));
}

fn palette_color((color, weight): (Lch, f32), total: f32) -> Unorm8x4 {
    let rgb: palette::Srgb = color.into_color();
    let rgb = rgb.clamp();
    Unorm8x4::from_vec4(
        Vec3::new(rgb.red, rgb.green, rgb.blue).extend((weight / total).max(1.0 / 255.0)),
    )
}

const fn component(color: &palette::Lab, channel: usize) -> f32 {
    [color.l, color.a, color.b][channel]
}

fn dominant_colors(pixels: &mut [palette::Lab]) -> ArrayVec<(Lch, f32), PALETTE_COLORS> {
    let mut buckets = ArrayVec::<Range<usize>, PALETTE_COLORS>::new();
    buckets.push(0..pixels.len());

    while buckets.len() < PALETTE_COLORS {
        let Some((bucket_index, channel)) = buckets
            .iter()
            .enumerate()
            .filter(|(_, range)| range.len() > 1)
            .map(|(index, range)| {
                let mut min = [f32::INFINITY; 3];
                let mut max = [f32::NEG_INFINITY; 3];
                for color in &pixels[range.clone()] {
                    for channel in 0..3 {
                        min[channel] = min[channel].min(component(color, channel));
                        max[channel] = max[channel].max(component(color, channel));
                    }
                }
                let (channel, spread) = (0..3)
                    .map(|channel| (channel, max[channel] - min[channel]))
                    .max_by(|a, b| a.1.total_cmp(&b.1))
                    .unwrap();
                (index, channel, spread * range.len() as f32)
            })
            .max_by(|a, b| a.2.total_cmp(&b.2))
            .map(|(index, channel, _)| (index, channel))
        else {
            break;
        };

        let range = buckets.swap_remove(bucket_index);
        pixels[range.clone()]
            .sort_unstable_by(|a, b| component(a, channel).total_cmp(&component(b, channel)));
        let middle = range.start + range.len() / 2;
        buckets.push(range.start..middle);
        buckets.push(middle..range.end);
    }

    buckets
        .into_iter()
        .map(|range| {
            let weight = range.len() as f32;
            let sum = pixels[range].iter().fold([0.0; 3], |mut sum, color| {
                sum[0] += color.l;
                sum[1] += color.a;
                sum[2] += color.b;
                sum
            });
            (
                palette::Lab::new(sum[0] / weight, sum[1] / weight, sum[2] / weight).into_color(),
                weight,
            )
        })
        .collect()
}

fn image_palette(image: &RgbaImage) -> [Unorm8x4; PALETTE_COLORS] {
    let srgb_to_lab = |pixel: &image::Rgba<u8>| {
        palette::Srgb::new(
            f32::from(pixel[0]) / 255.0,
            f32::from(pixel[1]) / 255.0,
            f32::from(pixel[2]) / 255.0,
        )
        .into_color()
    };
    let mut pixels: Vec<palette::Lab> = image
        .pixels()
        .filter(|pixel| {
            let max = pixel[0].max(pixel[1]).max(pixel[2]);
            let min = pixel[0].min(pixel[1]).min(pixel[2]);
            pixel[3] >= 128 && max - min > 30
        })
        .map(srgb_to_lab)
        .collect();
    let use_harmony = !pixels.is_empty();
    if !use_harmony {
        pixels.extend(image.pixels().filter(|pixel| pixel[3] >= 128).map(srgb_to_lab));
    }
    if pixels.is_empty() {
        return [Unorm8x4::default(); PALETTE_COLORS];
    }
    let mut colors = dominant_colors(&mut pixels);
    if use_harmony {
        complete_palette(&mut colors);
    }
    let total = colors.iter().map(|(_, weight)| weight).sum();
    array::from_fn(|index| palette_color(colors[index % colors.len()], total))
}

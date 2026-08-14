use crate::{
    app::{
        CantusApp, Fetch, Update,
        music::{LyricSegment, MusicBackend, PlaybackState, TrackId},
    },
    render::{lyrics, text, track::PALETTE_COLORS},
};
use arrayvec::ArrayVec;
use image::{RgbaImage, imageops};
use isthmus::{Unorm8x4, glam::Vec3};
use palette::{Clamp, IntoColor, Lch, color_theory::Analogous};
use quick_xml::{Reader, XmlVersion, escape::unescape, events::Event};
use serde::Deserialize;
use std::{array, collections::HashMap, error::Error, mem, ops::Range, sync::Arc, time::Instant};
use tracing::warn;
use ureq::Agent;

const TTML_API: &str = "https://lyrics-api.binimum.org/";
pub const IMAGE_SIZE: u32 = 64;
pub type ArtState = Fetch<Arc<AlbumArt>>;

pub struct AlbumArt {
    pub pixels: Box<[u8]>,
    palette: [Unorm8x4; PALETTE_COLORS],
}

impl Fetch<Arc<AlbumArt>> {
    pub fn palette(&self) -> [Unorm8x4; PALETTE_COLORS] {
        self.ready()
            .map_or_else(|| [Unorm8x4::default(); PALETTE_COLORS], |art| art.palette)
    }
}

#[derive(Deserialize)]
struct TtmlResponse {
    results: Vec<TtmlResult>,
}

#[derive(Deserialize)]
struct TtmlResult {
    #[serde(rename = "lyricsUrl")]
    lyrics_url: String,
    timing_type: String,
}

pub(crate) fn fetch_lyrics(
    uri: String,
    http: &Agent,
    music: &MusicBackend,
    track_id: Option<TrackId>,
    shaper: &text::Shaper,
    name: &str,
    artist: &str,
    album: &str,
    duration_ms: u32,
) -> Update<CantusApp> {
    let ttml = http
        .get(TTML_API)
        .query("track", name)
        .query("artist", artist)
        .query("album", album)
        .query("duration", (duration_ms / 1000).to_string())
        .call()
        .and_then(|mut response| response.body_mut().read_json::<TtmlResponse>())
        .ok()
        .and_then(|response| {
            response
                .results
                .into_iter()
                .find(|result| result.timing_type == "word")
        })
        .and_then(|result| {
            let source = http
                .get(&result.lyrics_url)
                .call()
                .ok()?
                .body_mut()
                .read_to_string()
                .ok()?;
            lyrics::Lyrics::shape(parse_ttml(&source), shaper)
        });
    let state = ttml.map_or_else(
        || match track_id.map(|id| music.lyrics(id)) {
            Some(Ok(segments)) => {
                Fetch::Ready(lyrics::Lyrics::shape(segments, shaper).unwrap_or_default())
            }
            Some(Err(error)) => {
                warn!(%error, track = name, "Failed to fetch lyrics");
                Fetch::retry()
            }
            None => Fetch::Ready(lyrics::Lyrics::default()),
        },
        Fetch::Ready,
    );
    Box::new(move |app| {
        if let Some(track) = app
            .playback
            .queue
            .iter_mut()
            .find(|track| track.uri == uri && matches!(track.runtime.lyrics, Fetch::Fetching))
        {
            track.runtime.lyrics = state;
        }
    })
}

fn parse_time(time: &str) -> Option<f32> {
    time.strip_suffix('s')
        .unwrap_or(time)
        .split(':')
        .try_fold(0.0, |total, part| Some(total * 60.0 + part.parse::<f32>().ok()?))
        .map(|seconds| seconds * 1000.0)
}

fn parse_ttml(source: &str) -> Vec<LyricSegment> {
    let mut reader = Reader::from_str(source);
    let (mut segments, mut line_lane) = (Vec::new(), None);
    let mut line_start = 0;
    let mut line_time = None;
    let mut line_text = String::new();
    let mut primary_agent = None;
    let mut span_roles = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) if tag.local_name().as_ref() == b"p" => {
                span_roles.clear();
                line_text.clear();
                let agent = tag
                    .attributes()
                    .flatten()
                    .filter_map(|attr| {
                        let value = attr.normalized_value(XmlVersion::Implicit1_0).ok()?;
                        Some((attr.key.local_name().as_ref().to_vec(), value.into_owned()))
                    })
                    .fold((String::new(), None, None), |mut values, (key, value)| {
                        match key.as_slice() {
                            b"agent" => values.0 = value,
                            b"begin" => values.1 = parse_time(&value),
                            b"end" => values.2 = parse_time(&value),
                            _ => {}
                        }
                        values
                    });
                line_time = agent.1.zip(agent.2);
                let agent = agent.0;
                let lane = usize::from(primary_agent.as_ref().is_some_and(|primary| primary != &agent));
                primary_agent.get_or_insert(agent);
                line_lane = Some(lane);
                line_start = segments.len();
            }
            Ok(Event::Start(tag)) if line_lane.is_some() && tag.local_name().as_ref() == b"span" => {
                let (mut start, mut end, mut role) = (None, None, None);
                for attr in tag.attributes().flatten() {
                    let Ok(value) = attr.normalized_value(XmlVersion::Implicit1_0) else {
                        continue;
                    };
                    match attr.key.local_name().as_ref() {
                        b"begin" => start = parse_time(&value),
                        b"end" => end = parse_time(&value),
                        b"role" => role = Some(value.into_owned()),
                        _ => {}
                    }
                }
                span_roles.push(match role.as_deref() {
                    Some("x-bg") => (true, false),
                    Some("x-translation" | "x-roman") => (false, true),
                    _ => (false, false),
                });
                if !span_roles.iter().any(|&(_, ignored)| ignored)
                    && let Some(start_ms) = start
                {
                    segments.push(LyricSegment {
                        start_ms,
                        end_ms: end.unwrap_or(start_ms + 1_000.0),
                        text: String::new(),
                        lane: line_lane.unwrap()
                            ^ usize::from(span_roles.iter().any(|&(background, _)| background)),
                    });
                }
            }
            Ok(Event::Text(value))
                if line_lane.is_some() && !span_roles.iter().any(|&(_, ignored)| ignored) =>
            {
                let Ok(value) = value.decode() else {
                    return Vec::new();
                };
                let Ok(value) = unescape(&value) else {
                    return Vec::new();
                };
                line_text.push_str(&value);
                if segments.len() > line_start {
                    segments.last_mut().unwrap().text.push_str(&value);
                }
            }
            Ok(Event::End(tag)) if tag.local_name().as_ref() == b"span" => {
                span_roles.pop();
            }
            Ok(Event::End(tag)) if tag.local_name().as_ref() == b"p" => {
                if segments.len() == line_start
                    && let Some((start_ms, end_ms)) = line_time
                    && !line_text.trim().is_empty()
                {
                    segments.push(LyricSegment {
                        start_ms,
                        end_ms,
                        text: mem::take(&mut line_text),
                        lane: line_lane.unwrap_or_default(),
                    });
                }
                line_lane = None;
                span_roles.clear();
            }
            Ok(Event::Eof) => break,
            Err(_) => return Vec::new(),
            _ => {}
        }
    }
    segments
}

fn fetch_art(http: &Agent, url: &str) -> ArtState {
    let result = (|| -> Result<_, Box<dyn Error + Send + Sync>> {
        let bytes = http.get(url).call()?.body_mut().read_to_vec()?;
        let image = image::load_from_memory(&bytes)?
            .resize_to_fill(IMAGE_SIZE, IMAGE_SIZE, imageops::FilterType::Lanczos3)
            .to_rgba8();
        Ok(Arc::new(AlbumArt {
            palette: image_palette(&image),
            pixels: image.into_raw().into_boxed_slice(),
        }))
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
    pub fn refresh_art(&mut self) {
        let now = Instant::now();
        let shared = art_slots(&mut self.playback)
            .filter_map(|(url, state)| Some((url.to_owned(), Arc::clone(state.ready()?))))
            .collect::<HashMap<_, _>>();
        let mut download = Vec::new();
        for (url, state) in art_slots(&mut self.playback) {
            if !state.request(now) {
                continue;
            }
            if let Some(art) = shared.get(url) {
                *state = Fetch::Ready(Arc::clone(art));
            } else {
                download.push(url.to_owned());
            }
        }
        download.sort_unstable();
        download.dedup();
        for url in download {
            let http = self.background.http.clone();
            let requested = url.clone();
            if !self.background.submit(move || {
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

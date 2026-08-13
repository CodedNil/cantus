use crate::render::{
    shared::{FrameData, smoothstep},
    text,
};
use isthmus::glam::{Vec2, Vec4};

#[cfg(feature = "cpu")]
use {
    crate::{
        app::{
            Background, CantusApp, Fetch,
            spotify::{PlaybackState, Track, TrackId},
        },
        render::{
            cpu::{Frame, Passes},
            shared::PANEL_START,
            text::TextStyle,
            track::TrackPass,
        },
    },
    isthmus::glam::vec2,
    serde::Deserialize,
    std::time::{Duration, Instant},
    tracing::warn,
    ureq::{Agent, Error as HttpError},
};

pub const EXTENSION: f32 = 10.0;
#[cfg(feature = "cpu")]
pub(crate) const TEXT_GLYPHS: usize = 8_192;
#[cfg(feature = "cpu")]
const STYLE: TextStyle = TextStyle::new(15.0, 700.0);
#[cfg(feature = "cpu")]
const API: &str = "https://lrclib.net/api/get";
#[cfg(feature = "cpu")]
const PREFETCH_TRACKS: usize = 4;
#[cfg(feature = "cpu")]
const SPEED: f32 = 0.05;

#[isthmus::pass]
pub struct LyricsPass {
    lines: isthmus::Instances<Self>,
    background: Background,
}

#[cfg(feature = "cpu")]
struct Request {
    id: Option<TrackId>,
    name: String,
    artist: String,
    album: String,
    duration_ms: u32,
}

#[cfg(feature = "cpu")]
impl From<&Track> for Request {
    fn from(track: &Track) -> Self {
        Self {
            id: track.id,
            name: track.name.clone(),
            artist: track.artist().to_owned(),
            album: track.album.name.clone(),
            duration_ms: track.duration_ms,
        }
    }
}

#[cfg(feature = "cpu")]
impl Request {
    fn matches(&self, track: &Track) -> bool {
        self.id == track.id && self.name == track.name
    }
}

#[cfg(feature = "cpu")]
#[derive(Default)]
pub struct Lyrics {
    segments: Vec<TimedSegment>,
    ribbon: Option<text::ShapedLine>,
}

#[cfg(feature = "cpu")]
struct TimedSegment {
    start_ms: f32,
    text: String,
}

#[cfg(feature = "cpu")]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LrcResponse {
    synced_lyrics: Option<String>,
}

#[cfg(feature = "cpu")]
impl Lyrics {
    fn parse(source: &str) -> Self {
        let mut segments = source
            .lines()
            .filter_map(|line| {
                let (timestamp, text) = line.strip_prefix('[')?.split_once(']')?;
                let (minutes, seconds) = timestamp.split_once(':')?;
                let start_ms =
                    minutes.parse::<f32>().ok()? * 60_000.0 + seconds.parse::<f32>().ok()? * 1000.0;
                (!text.trim().is_empty()).then(|| TimedSegment {
                    start_ms,
                    text: text.trim().to_owned(),
                })
            })
            .collect::<Vec<_>>();
        segments.sort_by(|left, right| left.start_ms.total_cmp(&right.start_ms));
        Self {
            segments,
            ribbon: None,
        }
    }

    fn layout(&mut self, text: &text::Renderer) {
        if !self.segments.is_empty() {
            self.ribbon = Some(
                text.shape_positioned(
                    self.segments
                        .iter()
                        .map(|segment| (segment.text.as_str(), segment.start_ms * SPEED)),
                    STYLE,
                    TEXT_GLYPHS,
                ),
            );
        }
    }
}

#[isthmus::pass]
impl LyricsPass {
    pub fn new(passes: &Passes<'_>, text: &text::Renderer, background: Background) -> Self {
        let (placed_glyphs, glyphs, edges) = text.resources();
        Self {
            lines: passes.instances_with_capacity((placed_glyphs, glyphs, edges), 3),
            background,
        }
    }

    fn fetch(agent: &Agent, song: &Request) -> Fetch<Lyrics> {
        let result = agent
            .get(API)
            .query("track_name", &song.name)
            .query("artist_name", &song.artist)
            .query("album_name", &song.album)
            .query("duration", (song.duration_ms / 1000).to_string())
            .call()
            .and_then(|mut response| response.body_mut().read_json::<LrcResponse>());
        match result {
            Ok(response) => Fetch::Ready(
                response
                    .synced_lyrics
                    .as_deref()
                    .map_or_else(Lyrics::default, Lyrics::parse),
            ),
            Err(HttpError::StatusCode(404)) => Fetch::Ready(Lyrics::default()),
            Err(error) => {
                warn!(%error, track = %song.name, "Failed to fetch lyrics");
                Fetch::Missing(Instant::now() + Duration::from_secs(30))
            }
        }
    }

    pub fn update(
        &mut self,
        text: &mut text::Renderer,
        playback: &mut PlaybackState,
        track: &TrackPass,
        frame: &Frame<'_>,
    ) {
        let start = track
            .timeline_track
            .map_or(playback.position.index, |(index, _)| index)
            .min(playback.queue.len());
        let now = Instant::now();
        for track in playback
            .queue
            .iter_mut()
            .skip(start.saturating_sub(1))
            .take(PREFETCH_TRACKS)
        {
            if track.runtime.lyrics.request(now) {
                let song = Request::from(&*track);
                let agent = self.background.http.clone();
                if !self.background.submit(move || {
                    let lyrics = Self::fetch(&agent, &song);
                    move |app: &mut CantusApp| {
                        let start = app.playback.position.index.min(app.playback.queue.len());
                        let (history, upcoming) = app.playback.queue.split_at_mut(start);
                        if let Some(track) =
                            upcoming.iter_mut().chain(history.iter_mut()).find(|track| {
                                song.matches(track) && matches!(track.runtime.lyrics, Fetch::Fetching)
                            })
                        {
                            track.runtime.lyrics = lyrics;
                        }
                    }
                }) {
                    track.runtime.lyrics = Fetch::Missing(now);
                }
            }
        }

        let Some((index, progress_ms)) = track.timeline_track else {
            self.lines.clear();
            return;
        };
        let visible = index.saturating_sub(1)..(index + 2).min(playback.queue.len());
        for track in &mut playback.queue[visible.clone()] {
            if let Some(lyrics) = track.runtime.lyrics.ready_mut()
                && lyrics.ribbon.is_none()
            {
                lyrics.layout(text);
            }
        }
        let y = PANEL_START + frame.config.height + 10.0;
        self.lines.clear();
        let mut start_ms = -progress_ms
            - playback.queue[visible.start..index]
                .iter()
                .map(Track::queue_span_ms)
                .sum::<f32>();
        for item in visible {
            let track = &playback.queue[item];
            let line = track
                .runtime
                .lyrics
                .ready()
                .and_then(|lyrics| lyrics.ribbon.as_ref());
            if let Some(line) = line {
                let x = frame.shared.playhead_x + start_ms * SPEED;
                if x <= frame.shared.screen_size.x && x + line.width >= 0.0 {
                    self.lines
                        .push(text.place_centered(line, vec2(x + line.width * 0.5, y)));
                }
            }
            start_ms += track.queue_span_ms();
        }
    }

    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] line: text::Line,
    ) -> isthmus::Vertex<text::Varyings> {
        text::vertex(*line, vertex, frame)
    }

    #[gpu]
    pub fn fragment(
        text::Varyings { pixel }: text::Varyings,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] line: text::Line,
        #[gpu(resource)] placed_glyphs: &[text::PlacedGlyph],
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
    ) -> Vec4 {
        let edge_fade = smoothstep(0.0, 32.0, pixel.x)
            * smoothstep(frame.screen_size.x, frame.screen_size.x - 32.0, pixel.x);
        let alpha = text::line_alpha(*line, placed_glyphs, glyphs, edges, pixel) * edge_fade;
        let played = smoothstep(frame.playhead_x + 4.0, frame.playhead_x - 4.0, pixel.x);
        let color = line.color.to_vec3().lerp(line.color.to_vec3() * 0.42, played);
        (color * alpha).extend(alpha)
    }
}

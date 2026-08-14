use crate::render::{
    shared::{FrameData, smoothstep},
    text,
};
use isthmus::glam::{Vec2, Vec4};

#[cfg(feature = "cpu")]
use {
    crate::{
        app::{
            Background, Fetch,
            enrichment::Enrichment,
            spotify::{PlaybackState, Track},
        },
        render::{
            cpu::{Frame, Passes},
            shared::PANEL_START,
            text::TextStyle,
        },
    },
    isthmus::glam::vec2,
    std::time::Instant,
};

pub const EXTENSION: f32 = 10.0;
#[cfg(feature = "cpu")]
pub(crate) const TEXT_GLYPHS: usize = 2_048;
#[cfg(feature = "cpu")]
const STYLE: TextStyle = TextStyle::new(15.0, 700.0);
#[cfg(feature = "cpu")]
const PREFETCH_TRACKS: usize = 4;
#[cfg(feature = "cpu")]
const MIN_SPEED: f32 = 0.05;
#[cfg(feature = "cpu")]
const WORD_GAP: f32 = 3.0;
#[cfg(feature = "cpu")]
const MUSIC_GAP_MS: f32 = 5_000.0;
#[cfg(feature = "cpu")]
const LANE_OFFSET: f32 = 5.0;

#[isthmus::pass]
pub struct LyricsPass {
    lines: isthmus::Instances<Self>,
    background: Background,
}

#[cfg(feature = "cpu")]
pub(crate) struct TimedSegment {
    pub start_ms: f32,
    pub end_ms: f32,
    pub text: String,
    pub lane: usize,
}

#[cfg(feature = "cpu")]
#[derive(Default)]
pub(crate) struct Lyrics {
    lines: [text::ShapedLine; 2],
    speed: f32,
}

#[cfg(feature = "cpu")]
impl Lyrics {
    pub(crate) fn shape(mut segments: Vec<TimedSegment>, shaper: &text::Shaper) -> Option<Self> {
        segments.retain(|segment| !segment.text.trim().is_empty());
        segments.sort_by(|left, right| left.start_ms.total_cmp(&right.start_ms));
        if segments.is_empty() {
            return None;
        }

        let mut music = Vec::new();
        let mut vocal_end = segments[0].end_ms;
        for segment in &segments[1..] {
            if segment.start_ms - vocal_end >= MUSIC_GAP_MS {
                let middle = (vocal_end + segment.start_ms) * 0.5;
                music.push(TimedSegment {
                    start_ms: middle,
                    end_ms: middle + 1_000.0,
                    text: "♪".into(),
                    lane: 0,
                });
            }
            vocal_end = vocal_end.max(segment.end_ms);
        }
        segments.extend(music);
        segments.sort_by(|left, right| left.start_ms.total_cmp(&right.start_ms));

        let mut speed = MIN_SPEED;
        let mut previous: [Option<(f32, f32, bool)>; 2] = [None; 2];
        for segment in &segments {
            let lane = segment.lane;
            if let Some((start_ms, width, ended_word)) = previous[lane] {
                let gap =
                    f32::from(ended_word || segment.text.starts_with(char::is_whitespace)) * WORD_GAP;
                speed = speed.max((width + gap) / (segment.start_ms - start_ms).max(1.0));
            }
            previous[lane] = Some((
                segment.start_ms,
                shaper.width(segment.text.trim(), STYLE),
                segment.text.ends_with(char::is_whitespace),
            ));
        }
        let lines = [0, 1].map(|lane| {
            shaper.shape_positioned(
                segments
                    .iter()
                    .filter(move |segment| segment.lane == lane)
                    .map(|segment| (segment.text.trim(), segment.start_ms * speed)),
                STYLE,
                TEXT_GLYPHS,
            )
        });
        Some(Self { lines, speed })
    }
}

#[isthmus::pass]
impl LyricsPass {
    pub fn new(passes: &Passes<'_>, text: &text::Renderer, background: Background) -> Self {
        let (placed_glyphs, glyphs, edges) = text.resources();
        Self {
            lines: passes.instances_with_capacity((placed_glyphs, glyphs, edges), 6),
            background,
        }
    }

    pub fn update(
        &mut self,
        text: &mut text::Renderer,
        playback: &mut PlaybackState,
        frame: &Frame<'_>,
    ) {
        let start = playback
            .timeline
            .track_at_playhead(&playback.queue)
            .map_or(playback.timeline.index, |(index, _)| index)
            .min(playback.queue.len());
        let now = Instant::now();
        for track in playback
            .queue
            .iter_mut()
            .skip(start.saturating_sub(1))
            .take(PREFETCH_TRACKS)
        {
            if track.runtime.lyrics.request(now) {
                let Some(id) = track.id else {
                    track.runtime.lyrics = Fetch::Ready(Lyrics::default());
                    continue;
                };
                let (name, artist, album, duration_ms) = (
                    track.name.clone(),
                    track.artist().to_owned(),
                    track.album.name.clone(),
                    track.duration_ms,
                );
                let agent = self.background.http.clone();
                let shaper = text.shaper();
                if !self.background.submit(move || {
                    Enrichment::lyrics(id, &agent, &shaper, &name, &artist, &album, duration_ms)
                }) {
                    track.runtime.lyrics = Fetch::Missing(now);
                }
            }
        }

        let Some((index, progress_ms)) = playback.timeline.track_at_playhead(&playback.queue) else {
            self.lines.clear();
            return;
        };
        let visible = index.saturating_sub(1)..(index + 2).min(playback.queue.len());
        let y = PANEL_START + frame.config.height + 10.0;
        self.lines.clear();
        let speed = |track: &Track| {
            track
                .runtime
                .lyrics
                .ready()
                .map_or(MIN_SPEED, |lyrics| lyrics.speed.max(MIN_SPEED))
        };
        let mut x = frame.shared.playhead_x - progress_ms * speed(&playback.queue[index]);
        for track in &playback.queue[visible.start..index] {
            x -= track.queue_span_ms() * speed(track);
        }
        for item in visible {
            let track = &playback.queue[item];
            if let Some(lyrics) = track.runtime.lyrics.ready() {
                for (lane, line) in lyrics
                    .lines
                    .iter()
                    .enumerate()
                    .filter(|(_, line)| line.width > 0.0)
                {
                    if x <= frame.shared.screen_size.x && x + line.width >= 0.0 {
                        let color = if lane == 0 {
                            text::COLOR.extend(1.0)
                        } else {
                            Vec4::new(0.72, 0.86, 1.0, 1.0)
                        };
                        self.lines.push(
                            text.place_visible(
                                line,
                                vec2(x, y + lane as f32 * LANE_OFFSET),
                                0.0..frame.shared.screen_size.x,
                            )
                            .with_color(color),
                        );
                    }
                }
            }
            x += track.queue_span_ms() * speed(track);
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

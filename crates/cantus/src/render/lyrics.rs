use crate::render::{
    shared::{FrameData, smoothstep},
    text,
};
use isthmus::glam::{Vec2, Vec4};

#[cfg(feature = "cpu")]
use {
    crate::{
        app::music::{Enrichment, Fetch, LyricSegment, MusicBackend, PlaybackState},
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
pub(crate) const TEXT_GLYPHS: usize = 4_096;
#[cfg(feature = "cpu")]
const STYLE: TextStyle = TextStyle::new(15.0, 700.0);
#[cfg(feature = "cpu")]
const PREFETCH_TRACKS: usize = 4;
#[cfg(feature = "cpu")]
const SPEED: f32 = 0.06;
#[cfg(feature = "cpu")]
const MUSIC_GAP_MS: f32 = 5_000.0;
#[cfg(feature = "cpu")]
const LANE_OFFSET: f32 = 5.0;
#[cfg(feature = "cpu")]
const GAP: f32 = 4.0;

#[isthmus::pass]
pub struct LyricsPass {
    lines: isthmus::Instances<Self>,
    enrichment: Enrichment,
    music: MusicBackend,
}

#[cfg(feature = "cpu")]
#[derive(Default)]
pub(crate) struct Lyrics {
    lines: [text::ShapedLine; 2],
}

#[cfg(feature = "cpu")]
impl Lyrics {
    pub(crate) fn shape(mut segments: Vec<LyricSegment>, shaper: &text::Shaper) -> Option<Self> {
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
                music.push(LyricSegment {
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

        let mut positioned = [Vec::new(), Vec::new()];
        let mut previous_end = [0.0; 2];
        for segment in &segments {
            let lane = segment.lane.min(1);
            let text = segment.text.trim();
            let start = segment.start_ms.max(previous_end[lane]);
            previous_end[lane] = start + shaper.width(text, STYLE) / SPEED + GAP / SPEED;
            positioned[lane].push((text, start * SPEED));
        }
        let lines = [0, 1]
            .map(|lane| shaper.shape_positioned(positioned[lane].iter().copied(), STYLE, TEXT_GLYPHS));
        Some(Self { lines })
    }
}

#[isthmus::pass]
impl LyricsPass {
    pub fn new(
        passes: &Passes<'_>,
        text: &text::Renderer,
        enrichment: Enrichment,
        music: MusicBackend,
    ) -> Self {
        let (placed_glyphs, glyphs, edges) = text.resources();
        Self {
            lines: passes.instances_with_capacity((placed_glyphs, glyphs, edges), 6),
            enrichment,
            music,
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
            if track.runtime.lyrics.request(now)
                && !self
                    .enrichment
                    .request_lyrics(track, self.music.clone(), text.shaper())
            {
                track.runtime.lyrics = Fetch::Missing(now);
            }
        }

        let Some((index, progress_ms)) = playback.timeline.track_at_playhead(&playback.queue) else {
            self.lines.clear();
            return;
        };
        let visible = index.saturating_sub(1)..(index + 2).min(playback.queue.len());
        let y = PANEL_START + frame.config.height + 10.0;
        self.lines.clear();
        let mut x = frame.shared.playhead_x - progress_ms * SPEED;
        for track in &playback.queue[visible.start..index] {
            x -= track.queue_span_ms() * SPEED;
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
            x += track.queue_span_ms() * SPEED;
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
        let emphasis = smoothstep(110.0, 0.0, (pixel.x - frame.playhead_x).abs());
        let scale = 1.0 + emphasis * 0.24;
        let sample = Vec2::new(
            frame.playhead_x + (pixel.x - frame.playhead_x) / scale,
            line.origin.y + (pixel.y - line.origin.y) / scale,
        );
        let distance = text::line_distance(*line, placed_glyphs, glyphs, edges, sample);
        let fill = text::coverage(distance);
        let stroke = (text::coverage(distance + 0.65) - fill) * 0.5;
        let alpha = (fill + stroke) * edge_fade;
        let played = smoothstep(frame.playhead_x + 4.0, frame.playhead_x - 4.0, pixel.x);
        let color = line.color.to_vec3().lerp(line.color.to_vec3() * 0.42, played);
        (color * fill * edge_fade).extend(alpha)
    }
}

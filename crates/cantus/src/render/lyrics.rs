use crate::render::{TEXT_COLOR, TextFragment};
use isthmus::{glam::Vec4, text};

pub const EXTENSION: f32 = 10.0;
pub const TEXT_GLYPHS: usize = 4_096;

#[isthmus::paint]
mod host {
    use super::*;
    use crate::{
        app::music::{Enrichment, LyricSegment, Music, Track, TrackId},
        render::{BarLayout, PANEL_START, RenderContext},
    };
    use isthmus::{FloatExt, glam::vec2, text::TextStyle};
    use std::time::Instant;

    pub struct LyricsView {
        enrichment: Enrichment,
    }

    pub struct LyricsRequest {
        pub uri: String,
        pub track_id: Option<TrackId>,
        pub name: String,
        pub artist: String,
        pub album: String,
        pub duration_ms: u32,
    }

    impl LyricsRequest {
        pub async fn fetch(&self, http: &reqwest::Client) -> Option<Vec<LyricSegment>> {
            provider::fetch(http, self).await
        }
    }

    #[derive(Default)]
    pub struct Lyrics {
        lines: [text::ShapedLine; 2],
        timeline: Vec<(f32, f32)>,
        span: f32,
    }

    impl Lyrics {
        const STYLE: TextStyle = TextStyle::new(15.0, 700.0);
        const SILENCE_SPEED: f32 = 0.035;
        const SONG_GAP: f32 = 96.0;
        const MUSIC_GAP_MS: f32 = 5_000.0;
        const LINE_GAP: f32 = 14.0;

        pub(crate) fn shape(mut segments: Vec<LyricSegment>, duration_ms: f32, shaper: &text::Shaper) -> Option<Self> {
            segments.retain(|segment| !segment.text.trim().is_empty());
            segments.sort_by(|left, right| left.start_ms.total_cmp(&right.start_ms));
            if segments.is_empty() {
                return None;
            }

            let mut music = Vec::new();
            let mut vocal_end = segments[0].end_ms;
            for segment in &segments[1..] {
                if segment.start_ms - vocal_end >= Self::MUSIC_GAP_MS {
                    let middle = f32::midpoint(vocal_end, segment.start_ms);
                    music.push(LyricSegment {
                        start_ms: middle,
                        end_ms: middle + 1_000.0,
                        text: "♪".into(),
                        lane: 0,
                        line_end: true,
                    });
                }
                vocal_end = vocal_end.max(segment.end_ms);
            }
            segments.extend(music);
            segments.sort_by(|left, right| left.start_ms.total_cmp(&right.start_ms));

            let mut positioned = [Vec::new(), Vec::new()];
            let mut timeline = vec![(0.0, 0.0)];
            let mut cursors = [0.0f32; 2];
            let mut vocal_end = 0.0f32;
            for segment in &segments {
                let silence = (segment.start_ms - vocal_end).max(0.0);
                if silence > 0.0 {
                    let cursor = cursors[0].max(cursors[1]) + silence * Self::SILENCE_SPEED;
                    cursors = [cursor; 2];
                }
                let lane = segment.lane.min(1);
                let text = segment.text.trim_start();
                let width = shaper.width(text, Self::STYLE);
                let position = cursors[lane];
                positioned[lane].push((text, position));
                cursors[lane] += width + if segment.line_end { Self::LINE_GAP } else { 0.0 };
                vocal_end = vocal_end.max(segment.end_ms);
                if lane == 0 {
                    timeline.push((f32::midpoint(segment.start_ms, segment.end_ms), position + width * 0.5));
                }
            }
            let lines = [0, 1].map(|lane| shaper.shape_positioned(positioned[lane].iter().copied(), Self::STYLE, TEXT_GLYPHS));
            let position = cursors[0].max(cursors[1]) + (duration_ms - vocal_end).max(0.0) * Self::SILENCE_SPEED;
            timeline.push((duration_ms.max(vocal_end), position));
            timeline.sort_by(|left, right| left.0.total_cmp(&right.0));
            Some(Self {
                lines,
                timeline,
                span: position + Self::SONG_GAP,
            })
        }

        fn position(&self, time: f32) -> f32 {
            let upper = self.timeline.partition_point(|&(at, _)| at <= time);
            if upper == 0 {
                return self.timeline.first().map_or(0.0, |&(_, x)| x);
            }
            if upper == self.timeline.len() {
                return self.timeline.last().map_or(0.0, |&(_, x)| x);
            }
            let (t0, x0) = self.timeline[upper - 1];
            let (t1, x1) = self.timeline[upper];
            x0.lerp(x1, ((time - t0) / (t1 - t0).max(f32::EPSILON)).clamp(0.0, 1.0))
        }
    }

    mod provider {
        use super::{LyricSegment, LyricsRequest};
        use quick_xml::{
            Reader, XmlVersion,
            escape::unescape,
            events::{BytesStart, Event},
        };
        use reqwest::Client;
        use serde::Deserialize;
        use std::mem;

        const API: &str = "https://lyrics-api.binimum.org/";

        #[derive(Deserialize)]
        struct SearchResponse {
            results: Vec<SearchResult>,
        }

        #[derive(Deserialize)]
        struct SearchResult {
            #[serde(rename = "lyricsUrl")]
            url: String,
            timing_type: String,
        }

        pub(super) async fn fetch(http: &Client, query: &LyricsRequest) -> Option<Vec<LyricSegment>> {
            let result = http
                .get(API)
                .query(&[
                    ("track", query.name.clone()),
                    ("artist", query.artist.clone()),
                    ("album", query.album.clone()),
                    ("duration", (query.duration_ms / 1000).to_string()),
                ])
                .send()
                .await
                .ok()?
                .error_for_status()
                .ok()?
                .json::<SearchResponse>()
                .await
                .ok()?
                .results
                .into_iter()
                .find(|result| result.timing_type == "word")?;
            let source = http.get(result.url).send().await.ok()?.error_for_status().ok()?.text().await.ok()?;
            let segments = parse(&source);
            (!segments.is_empty()).then_some(segments)
        }

        fn time(value: &str) -> Option<f32> {
            value
                .strip_suffix('s')
                .unwrap_or(value)
                .split(':')
                .try_fold(0.0, |total, part| Some(total * 60.0 + part.parse::<f32>().ok()?))
                .map(|seconds| seconds * 1000.0)
        }

        fn attribute(tag: &BytesStart<'_>, name: &str) -> Option<String> {
            tag.attributes()
                .flatten()
                .find(|attr| attr.key.local_name().as_ref() == name)?
                .normalized_value(XmlVersion::Implicit1_0)
                .ok()
                .map(std::borrow::Cow::into_owned)
        }

        fn parse(source: &str) -> Vec<LyricSegment> {
            let mut reader = Reader::from_str(source);
            let (mut segments, mut line_lane) = (Vec::new(), None);
            let mut line_start = 0;
            let mut line_time = None;
            let mut line_text = String::new();
            let mut primary_agent = None;
            let mut span_roles = Vec::new();
            loop {
                match reader.read_event() {
                    Ok(Event::Start(tag)) if tag.local_name().as_ref() == "p" => {
                        span_roles.clear();
                        line_text.clear();
                        line_time = attribute(&tag, "begin").as_deref().and_then(time).zip(attribute(&tag, "end").as_deref().and_then(time));
                        let agent = attribute(&tag, "agent").unwrap_or_default();
                        let lane = usize::from(primary_agent.as_ref().is_some_and(|primary| primary != &agent));
                        primary_agent.get_or_insert(agent);
                        line_lane = Some(lane);
                        line_start = segments.len();
                    }
                    Ok(Event::Start(tag)) if line_lane.is_some() && tag.local_name().as_ref() == "span" => {
                        let start = attribute(&tag, "begin").as_deref().and_then(time);
                        let end = attribute(&tag, "end").as_deref().and_then(time);
                        span_roles.push(match attribute(&tag, "role").as_deref() {
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
                                lane: line_lane.unwrap() ^ usize::from(span_roles.iter().any(|&(background, _)| background)),
                                line_end: false,
                            });
                        }
                    }
                    Ok(Event::Text(value)) if line_lane.is_some() && !span_roles.iter().any(|&(_, ignored)| ignored) => {
                        let value = value.xml_content(XmlVersion::Implicit1_0);
                        let Ok(value) = unescape(&value) else {
                            return Vec::new();
                        };
                        line_text.push_str(&value);
                        if segments.len() > line_start {
                            let segment = &mut segments.last_mut().unwrap().text;
                            if value.chars().all(char::is_whitespace) {
                                if !segment.ends_with(char::is_whitespace) {
                                    segment.push(' ');
                                }
                            } else {
                                segment.push_str(&value);
                            }
                        }
                    }
                    Ok(Event::End(tag)) if tag.local_name().as_ref() == "span" => {
                        span_roles.pop();
                    }
                    Ok(Event::End(tag)) if tag.local_name().as_ref() == "p" => {
                        if segments.len() == line_start
                            && let Some((start_ms, end_ms)) = line_time
                            && !line_text.trim().is_empty()
                        {
                            segments.push(LyricSegment {
                                start_ms,
                                end_ms,
                                text: mem::take(&mut line_text),
                                lane: line_lane.unwrap_or_default(),
                                line_end: false,
                            });
                        }
                        if segments.len() > line_start {
                            segments.last_mut().unwrap().line_end = true;
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
    }

    impl LyricsView {
        pub const fn new(enrichment: Enrichment) -> Self {
            Self { enrichment }
        }

        pub fn show(&self, frame: &mut RenderContext, music: &mut Music, layout: BarLayout) {
            const PREFETCH_TRACKS: usize = 4;
            const LANE_OFFSET: f32 = 8.0;

            let playhead_track = music.timeline.track_at_playhead(&music.queue);
            let start = playhead_track.map_or(music.timeline.index, |(index, _)| index).min(music.queue.len());
            let now = Instant::now();
            for track in music.queue.iter_mut().skip(start.saturating_sub(1)).take(PREFETCH_TRACKS) {
                if track.runtime.lyrics.request(now) {
                    self.enrichment.request_lyrics(track, music.backend.clone(), frame.paint.text().shaper());
                }
            }

            let Some((index, progress_ms)) = playhead_track else {
                return;
            };
            let visible = index.saturating_sub(1)..(index + 2).min(music.queue.len());
            let y = PANEL_START + frame.config.height + 10.0;
            let span = |track: &Track| {
                track
                    .runtime
                    .lyrics
                    .ready()
                    .filter(|lyrics| lyrics.span > 0.0)
                    .map_or_else(|| track.queue_span_ms() * Lyrics::SILENCE_SPEED, |lyrics| lyrics.span)
            };
            let current = &music.queue[index];
            let progress = current
                .runtime
                .lyrics
                .ready()
                .map_or(progress_ms * Lyrics::SILENCE_SPEED, |lyrics| lyrics.position(progress_ms));
            let mut x = layout.playhead_x - progress;
            let playhead_x = layout.playhead_x;
            for track in &music.queue[visible.start..index] {
                x -= span(track);
            }
            for item in visible {
                let track = &music.queue[item];
                if let Some(lyrics) = track.runtime.lyrics.ready() {
                    for (lane, line) in lyrics.lines.iter().enumerate().filter(|(_, line)| line.width > 0.0) {
                        if x <= frame.paint.screen_size.x && x + line.width >= 0.0 {
                            let color = if lane == 0 { TEXT_COLOR.extend(1.0) } else { Vec4::new(0.72, 0.86, 1.0, 1.0) };
                            let screen_width = frame.paint.screen_size.x;
                            let placed = frame
                                .paint
                                .text()
                                .visible(line, vec2(x, y + lane as f32 * LANE_OFFSET), 0.0..screen_width)
                                .with_color(color);
                            let padding = placed.size * 0.2 + 1.0;
                            // GPU: Scrolling lyrics line.
                            frame.paint.paint_text(placed.expanded(padding), |text: TextFragment, playhead_x: f32, screen_width: f32| {
                                let edge_fade = text.pixel.x.smoothstep(0.0, 32.0) * text.pixel.x.smoothstep(screen_width, screen_width - 32.0);
                                let emphasis = (text.pixel.x - playhead_x).abs().smoothstep(110.0, 0.0);
                                let weight = (text.line.weight + emphasis * 0.15).min(1.0);
                                let distance = text.distance_scaled_with_weight(text.pixel, 1.0 + emphasis * 0.2, weight);
                                let fill = text::coverage(distance);
                                let outline = text::coverage(distance + 0.9) * 0.4;
                                let progress = text.pixel.x.smoothstep(playhead_x + 4.0, playhead_x - 4.0);
                                let fade = edge_fade * (1.0 - progress * 0.5);
                                (text.line.color.to_vec3() * fill * fade).extend((fill + outline * (1.0 - fill)) * fade)
                            });
                        }
                    }
                }
                x += span(track);
            }
        }
    }
}

use quick_xml::{
    Reader, XmlVersion,
    escape::unescape,
    events::{BytesStart, Event},
};
use serde::Deserialize;
use std::mem;
use ureq::Agent;

const TTML_API: &str = "https://lyrics-api.binimum.org/";

pub struct LyricSegment {
    pub start_ms: f32,
    pub end_ms: f32,
    pub text: String,
    pub lane: usize,
}

pub(super) struct LyricsRequest {
    pub uri: String,
    pub track_id: Option<super::TrackId>,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub duration_ms: u32,
}

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

pub(super) fn fetch(http: &Agent, query: &LyricsRequest) -> Option<Vec<LyricSegment>> {
    let result = http
        .get(TTML_API)
        .query("track", &query.name)
        .query("artist", &query.artist)
        .query("album", &query.album)
        .query("duration", (query.duration_ms / 1000).to_string())
        .call()
        .ok()?
        .body_mut()
        .read_json::<SearchResponse>()
        .ok()?
        .results
        .into_iter()
        .find(|result| result.timing_type == "word")?;
    let source = http
        .get(result.url)
        .call()
        .ok()?
        .body_mut()
        .read_to_string()
        .ok()?;
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

fn attribute(tag: &BytesStart<'_>, name: &[u8]) -> Option<String> {
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
            Ok(Event::Start(tag)) if tag.local_name().as_ref() == b"p" => {
                span_roles.clear();
                line_text.clear();
                line_time = attribute(&tag, b"begin")
                    .as_deref()
                    .and_then(time)
                    .zip(attribute(&tag, b"end").as_deref().and_then(time));
                let agent = attribute(&tag, b"agent").unwrap_or_default();
                let lane = usize::from(primary_agent.as_ref().is_some_and(|primary| primary != &agent));
                primary_agent.get_or_insert(agent);
                line_lane = Some(lane);
                line_start = segments.len();
            }
            Ok(Event::Start(tag)) if line_lane.is_some() && tag.local_name().as_ref() == b"span" => {
                let start = attribute(&tag, b"begin").as_deref().and_then(time);
                let end = attribute(&tag, b"end").as_deref().and_then(time);
                span_roles.push(match attribute(&tag, b"role").as_deref() {
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

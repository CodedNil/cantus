use crate::{PANEL_START, TRACK_SPACING_MS, art::ArtState};
use arrayvec::ArrayString;
use cantus_shared::AudioFeatures;
use glam::Vec2;
use serde::{Deserialize, Deserializer, de};
use std::{
    collections::HashSet,
    mem,
    sync::Arc,
    time::{Duration, Instant},
};

pub const NUM_SWATCHES: usize = 4;
const MAX_HISTORY_TRACKS: usize = 6;

pub type TrackId = ArrayString<22>;
pub type PlaylistId = ArrayString<22>;
pub type PlaylistTracks = Arc<HashSet<TrackId>>;

pub struct PlaybackState {
    pub playing: bool,
    pub progress: u32,
    pub volume: Option<u8>,
    pub queue: Vec<Track>,
    pub queue_index: usize,
    pub playlists: Vec<CondensedPlaylist>,
    pub last_interaction: Instant,
    pub last_progress_update: Instant,
}

impl Default for PlaybackState {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            playing: false,
            progress: 0,
            volume: None,
            queue: Vec::new(),
            queue_index: 0,
            playlists: Vec::new(),
            last_interaction: now,
            last_progress_update: now,
        }
    }
}

impl PlaybackState {
    pub const fn update_progress(&mut self, progress: u32, now: Instant) {
        self.progress = progress;
        self.last_progress_update = now;
    }

    pub fn defer_remote_updates(&mut self, duration: Duration) {
        self.last_interaction = Instant::now() + duration;
    }

    pub fn estimated_progress(&self) -> f32 {
        self.progress as f32
            + if self.playing {
                self.last_progress_update.elapsed().as_millis() as f32
            } else {
                0.0
            }
    }

    pub fn replace_queue(
        &mut self,
        new_queue: Vec<Track>,
        current_id: Option<TrackId>,
        context_changed: bool,
    ) {
        let mut old_queue = mem::take(&mut self.queue);
        let history_len = track_index(&old_queue, current_id, &new_queue[0].name)
            .filter(|_| !context_changed)
            .unwrap_or(0);
        let mut remaining = old_queue.split_off(history_len);
        let mut reconciled = old_queue.split_off(history_len.saturating_sub(MAX_HISTORY_TRACKS));
        self.queue_index = reconciled.len();

        for mut track in new_queue {
            if let Some(index) = remaining.iter().position(|old| old.id == track.id) {
                let old = remaining.swap_remove(index);
                track.art = old.art;
                track.runtime = old.runtime;
                track.audio_features = old.audio_features;
            } else if let Some(art) = remaining
                .iter()
                .chain(&old_queue)
                .chain(&reconciled)
                .find_map(|old| (old.album.image == track.album.image).then_some(&old.art))
                && let ArtState::Ready(art) = art
            {
                track.art = ArtState::Ready(Arc::clone(art));
            }
            reconciled.push(track);
        }
        self.queue = reconciled;
    }
}

#[derive(Deserialize)]
pub struct Track {
    pub id: Option<TrackId>,
    pub name: String,
    pub album: Album,
    #[serde(deserialize_with = "deserialize_first_artist", rename = "artists")]
    pub artist: Artist,
    pub duration_ms: u32,
    #[serde(skip)]
    pub art: ArtState,
    #[serde(skip)]
    pub runtime: TrackRuntime,
    #[serde(skip)]
    pub audio_features: Option<AudioFeatures>,
}

#[derive(Default)]
pub struct TrackRuntime {
    pub playlist_expansion: f32,
    pub detail_alpha: f32,
    pub primary_icon_alpha: f32,
    pub primary_playlist_count: u8,
    pub secondary_playlist_count: u8,
    pub start_ms: f32,
    pub start_x: f32,
    pub width: f32,
}

impl Track {
    pub fn queue_span_ms(&self) -> f32 {
        self.duration_ms as f32 + TRACK_SPACING_MS
    }

    pub fn is_current(&self) -> bool {
        self.runtime.start_ms <= 0.0 && self.runtime.start_ms + self.duration_ms as f32 >= 0.0
    }

    pub fn palette(&self) -> [u32; NUM_SWATCHES] {
        match &self.art {
            ArtState::Ready(art) => art.palette,
            _ => [0; NUM_SWATCHES],
        }
    }

    pub fn natural_x_range(&self, playhead_x: f32, px_per_ms: f32) -> (f32, f32) {
        let start = playhead_x + self.runtime.start_ms * px_per_ms;
        (start, start + self.duration_ms as f32 * px_per_ms)
    }

    pub fn contains(&self, point: Vec2, height: f32) -> bool {
        self.runtime
            .rect(height)
            .is_some_and(|rect| rect.contains(point))
    }
}

impl TrackRuntime {
    pub fn end_x(&self) -> f32 {
        self.start_x + self.width
    }

    pub fn rect(&self, height: f32) -> Option<Rect> {
        (self.width > 0.0 && self.end_x() > 0.0).then_some(Rect::pill(
            self.start_x,
            self.width,
            height,
        ))
    }
}

#[derive(Deserialize)]
pub struct Album {
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct Artist {
    pub name: String,
}

pub struct CondensedPlaylist {
    pub id: PlaylistId,
    pub name: String,
    pub image_url: Option<String>,
    pub tracks: PlaylistTracks,
    pub rating_index: Option<u8>,
    pub art: ArtState,
}

impl CondensedPlaylist {
    pub fn set_membership(&mut self, track_id: TrackId, add: bool) -> bool {
        let tracks = Arc::make_mut(&mut self.tracks);
        if add {
            tracks.insert(track_id)
        } else {
            tracks.remove(&track_id)
        }
    }
}

pub fn playlist_icons(
    track_id: TrackId,
    playlists: &[CondensedPlaylist],
    contains_track: bool,
) -> impl Iterator<Item = &CondensedPlaylist> {
    playlists.iter().filter(move |playlist| {
        playlist.rating_index.is_none() && playlist.tracks.contains(&track_id) == contains_track
    })
}

#[derive(Deserialize)]
struct Image {
    url: String,
    width: Option<u32>,
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Rect {
    pub const fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    pub const fn pill(x: f32, width: f32, height: f32) -> Self {
        Self::new(x, PANEL_START, x + width, PANEL_START + height)
    }

    pub fn from_center(center: Vec2, half_size: Vec2) -> Self {
        Self::new(
            center.x - half_size.x,
            center.y - half_size.y,
            center.x + half_size.x,
            center.y + half_size.y,
        )
    }

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.x0 && point.x <= self.x1 && point.y >= self.y0 && point.y <= self.y1
    }
}

pub fn deserialize_images<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Vec::<Image>::deserialize(deserializer)?
        .into_iter()
        .min_by_key(|image| image.width.unwrap_or(u32::MAX))
        .map(|image| image.url))
}

fn deserialize_first_artist<'de, D>(deserializer: D) -> Result<Artist, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<Artist>::deserialize(deserializer)?
        .into_iter()
        .next()
        .ok_or_else(|| de::Error::custom("artists array is empty"))
}

pub fn track_index(queue: &[Track], id: Option<TrackId>, name: &str) -> Option<usize> {
    id.and_then(|track_id| queue.iter().position(|track| track.id == Some(track_id)))
        .or_else(|| queue.iter().position(|track| track.name == name))
}

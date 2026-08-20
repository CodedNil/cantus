use crate::{
    app::{AppUpdater, config::Config},
    render::{lyrics::Lyrics, track::AudioFeatures},
};
use arrayvec::ArrayString;
use std::{collections::HashSet, error::Error, mem, sync::Arc, time::Instant};

mod enrichment;
mod spotify;

pub(crate) use crate::render::lyrics::LyricSegment;
pub use enrichment::{AlbumArt, ArtState, Enrichment, Fetch, IMAGE_SIZE};

pub type TrackId = ArrayString<22>;
pub type PlaylistId = ArrayString<22>;
pub type MusicResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
pub const TRACK_SPACING_MS: f32 = 4000.0;
pub(super) type PlaylistTracks = Arc<HashSet<TrackId>>;

#[derive(Default)]
pub struct PlaybackState {
    pub playing: bool,
    pub volume: Option<u8>,
    pub queue: Vec<Track>,
    pub playlists: Vec<CondensedPlaylist>,
    pub timeline: Timeline,
}

/// The observed and visually smoothed position of the playback queue.
pub struct Timeline {
    pub index: usize,
    pub position_ms: f32,
    pub rate: f32,
    pub observed_at: Instant,
    pub queue_start_ms: f32,
    pub movement: f32,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            index: 0,
            position_ms: 0.0,
            rate: 0.0,
            observed_at: Instant::now(),
            queue_start_ms: 0.0,
            movement: 0.0,
        }
    }
}

impl Timeline {
    pub fn track_at_playhead(&self, queue: &[Track]) -> Option<(usize, f32)> {
        let mut start_ms = self.queue_start_ms;
        queue.iter().enumerate().find_map(|(index, track)| {
            let current = (start_ms <= 0.0 && start_ms + track.duration_ms as f32 >= 0.0)
                .then_some((index, -start_ms));
            start_ms += track.queue_span_ms();
            current
        })
    }
}

impl PlaybackState {
    fn observe(&mut self, index: usize, position_ms: f32, rate: f32, observed_at: Instant) {
        self.timeline.index = index.min(self.queue.len().saturating_sub(1));
        self.timeline.position_ms = position_ms;
        self.timeline.rate = rate;
        self.timeline.observed_at = observed_at;
    }

    /// Replaces an authoritative queue snapshot without moving its rendered contents.
    pub fn replace_queue(
        &mut self,
        mut queue: Vec<Track>,
        index: usize,
        position_ms: f32,
        rate: f32,
        observed_at: Instant,
    ) {
        let old_index = self.timeline.index.min(self.queue.len().saturating_sub(1));
        let origin = self.queue.get(old_index).map(|track| {
            let progress = -self.timeline.queue_start_ms
                - self.queue[..old_index]
                    .iter()
                    .map(Track::queue_span_ms)
                    .sum::<f32>();
            (track.uri.clone(), progress)
        });

        let mut old = mem::take(&mut self.queue);
        for track in &mut queue {
            if let Some(index) = old.iter().position(|previous| previous.uri == track.uri) {
                track.runtime = old.remove(index).runtime;
            }
        }

        let index = index.min(queue.len().saturating_sub(1));
        let rebased = origin.and_then(|(uri, progress)| {
            queue
                .iter()
                .enumerate()
                .filter(|(_, track)| track.uri == uri)
                .min_by_key(|(candidate, _)| candidate.abs_diff(index))
                .map(|(index, _)| (index, progress))
        });
        let (origin, progress) = rebased.unwrap_or((index, position_ms));
        self.timeline.queue_start_ms =
            -progress - queue[..origin].iter().map(Track::queue_span_ms).sum::<f32>();
        self.queue = queue;
        self.observe(index, position_ms, rate, observed_at);
    }

    pub fn update_timeline(&mut self, drag_offset_ms: f32, dragging: bool, delta_time: f32) {
        if self.queue.is_empty() {
            self.timeline.queue_start_ms = 0.0;
            self.timeline.movement = 0.0;
            return;
        }
        let index = self.timeline.index.min(self.queue.len() - 1);
        let target = -(self.timeline.position_ms
            + self.timeline.observed_at.elapsed().as_millis() as f32 * self.timeline.rate)
            - self.queue[..index].iter().map(Track::queue_span_ms).sum::<f32>()
            + drag_offset_ms;
        let difference = target - self.timeline.queue_start_ms;
        let next = if !dragging && difference.abs() > 200.0 {
            self.timeline.queue_start_ms + difference * 3.5 * delta_time
        } else {
            target
        };
        let target_movement = (next - self.timeline.queue_start_ms) * delta_time;
        self.timeline.movement +=
            (target_movement - self.timeline.movement) * (delta_time * 10.0).min(1.0);
        self.timeline.queue_start_ms = next;
    }
}

pub struct Track {
    pub id: Option<TrackId>,
    pub uri: String,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub image: Option<String>,
    pub duration_ms: u32,
    pub runtime: TrackRuntime,
}

#[derive(Default)]
pub struct TrackRuntime {
    /// Album art, shared with other slots on the same URL and freed with the track.
    pub art: ArtState,
    pub playlist_expansion: f32,
    pub detail_alpha: f32,
    pub primary_icon_alpha: f32,
    pub audio_features: Fetch<AudioFeatures>,
    pub(crate) lyrics: Fetch<Lyrics>,
}

impl Track {
    pub fn queue_span_ms(&self) -> f32 {
        self.duration_ms as f32 + TRACK_SPACING_MS
    }
}

pub struct CondensedPlaylist {
    pub id: PlaylistId,
    pub(crate) name: String,
    pub image_url: Option<String>,
    pub art: ArtState,
    pub tracks: PlaylistTracks,
    pub rating_index: Option<u8>,
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

pub enum PlaybackCommand {
    SetPlaying(bool),
    SetVolume(u8),
    Seek(u32),
    Skip(i8),
    UpdateLibrary {
        track_id: TrackId,
        playlists: Vec<(PlaylistId, bool)>,
        liked: Option<bool>,
    },
}

pub trait MusicService: Send + Sync {
    fn command(&self, command: PlaybackCommand);

    /// Fetches timed lyrics for a Spotify track.
    ///
    /// # Errors
    ///
    /// Returns an error when the provider request or response fails.
    fn lyrics(&self, track_id: TrackId) -> MusicResult<Vec<LyricSegment>>;
}

#[derive(Clone)]
pub struct MusicBackend(Arc<dyn MusicService>);

impl MusicBackend {
    pub(crate) fn spotify(config: &Config, updater: &AppUpdater, http: ureq::Agent) -> Self {
        Self(Arc::new(spotify::SpotifyBackend::new(config, updater, http)))
    }

    pub fn command(&self, command: PlaybackCommand) {
        self.0.command(command);
    }

    /// Fetches timed lyrics from the active music service.
    ///
    /// # Errors
    ///
    /// Returns an error when the provider request or response fails.
    pub fn lyrics(&self, track_id: TrackId) -> MusicResult<Vec<LyricSegment>> {
        self.0.lyrics(track_id)
    }
}

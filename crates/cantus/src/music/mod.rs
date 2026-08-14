use crate::{
    app::{AppUpdater, Background, Fetch, TRACK_SPACING_MS, config::Config, enrichment::ArtState},
    render::{lyrics::Lyrics, track::AudioFeatures},
};
use arrayvec::ArrayString;
use std::{collections::HashSet, error::Error, sync::Arc, time::Instant};

mod spotify;

pub type TrackId = ArrayString<22>;
pub type PlaylistId = ArrayString<22>;
pub type MusicResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
type PlaylistTracks = Arc<HashSet<TrackId>>;

pub struct LyricSegment {
    pub start_ms: f32,
    pub end_ms: f32,
    pub text: String,
    pub lane: usize,
}

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
        let now = Instant::now();
        Self {
            index: 0,
            position_ms: 0.0,
            rate: 0.0,
            observed_at: now,
            queue_start_ms: 0.0,
            movement: 0.0,
        }
    }
}

impl Timeline {
    pub fn estimated_position(&self) -> f32 {
        self.position_ms + self.observed_at.elapsed().as_millis() as f32 * self.rate
    }

    pub const fn observe_server(
        &mut self,
        server_position: f32,
        server_index: usize,
        rate: f32,
        now: Instant,
    ) {
        self.index = server_index;
        self.position_ms = server_position;
        self.rate = rate;
        self.observed_at = now;
    }

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
    pub fn update_timeline(&mut self, drag_offset_ms: f32, dragging: bool, delta_time: f32) {
        if self.queue.is_empty() {
            self.timeline.queue_start_ms = 0.0;
            self.timeline.movement = 0.0;
            return;
        }
        let index = self.timeline.index.min(self.queue.len() - 1);
        let target = -self.timeline.estimated_position()
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
    Skip {
        forward: bool,
        count: usize,
    },
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
    pub fn spotify(config: &mut Config, updater: &AppUpdater, background: Background) -> Self {
        Self(Arc::new(spotify::SpotifyBackend::new(
            config, updater, background,
        )))
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

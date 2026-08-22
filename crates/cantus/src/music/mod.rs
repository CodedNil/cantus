use crate::{
    app::{AppUpdater, Background, config::Config},
    render::{lyrics::Lyrics, track::AudioFeatures},
};
use arrayvec::ArrayString;
use std::{collections::HashSet, error::Error, mem, sync::Arc, time::Instant};
use tracing::{info, warn};

mod enrichment;
mod spotify;

pub(crate) use crate::render::lyrics::LyricSegment;
pub use enrichment::{AlbumArt, ArtState, Enrichment, Fetch, IMAGE_SIZE};

pub type TrackId = ArrayString<22>;
pub type PlaylistId = ArrayString<22>;
pub type MusicResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
pub const TRACK_SPACING_MS: f32 = 4000.0;
pub const MAX_PLAYLIST_TARGETS: usize = 8;
pub(super) type PlaylistTracks = Arc<HashSet<TrackId>>;

pub struct PlaybackState {
    pub playing: bool,
    pub queue: Vec<Track>,
    pub playlists: Vec<CondensedPlaylist>,
    pub timeline: Timeline,
    pub last_toggle: Instant,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            playing: false,
            queue: Vec::new(),
            playlists: Vec::new(),
            timeline: Timeline::default(),
            last_toggle: Instant::now(),
        }
    }
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
            let current = (start_ms <= 0.0 && start_ms + track.duration_ms as f32 >= 0.0).then_some((index, -start_ms));
            start_ms += track.queue_span_ms();
            current
        })
    }
}

impl PlaybackState {
    pub fn mark_toggle(&mut self) {
        self.last_toggle = Instant::now();
    }

    fn observe(&mut self, index: usize, position_ms: f32, rate: f32, observed_at: Instant) {
        self.timeline.index = index.min(self.queue.len().saturating_sub(1));
        self.timeline.position_ms = position_ms;
        self.timeline.rate = rate;
        self.timeline.observed_at = observed_at;
    }

    /// Replaces an authoritative queue snapshot without moving its rendered contents.
    pub fn replace_queue(&mut self, mut queue: Vec<Track>, index: usize, position_ms: f32, rate: f32, observed_at: Instant) {
        let old_index = self.timeline.index.min(self.queue.len().saturating_sub(1));
        let origin = self.queue.get(old_index).map(|track| {
            let progress = -self.timeline.queue_start_ms - self.queue[..old_index].iter().map(Track::queue_span_ms).sum::<f32>();
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
        self.timeline.queue_start_ms = -progress - queue[..origin].iter().map(Track::queue_span_ms).sum::<f32>();
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
        let target = -(self.timeline.position_ms + self.timeline.observed_at.elapsed().as_millis() as f32 * self.timeline.rate)
            - self.queue[..index].iter().map(Track::queue_span_ms).sum::<f32>()
            + drag_offset_ms;
        let difference = target - self.timeline.queue_start_ms;
        let next = if !dragging && difference.abs() > 200.0 {
            self.timeline.queue_start_ms + difference * 3.5 * delta_time
        } else {
            target
        };
        let target_movement = (next - self.timeline.queue_start_ms) * delta_time;
        self.timeline.movement += (target_movement - self.timeline.movement) * (delta_time * 10.0).min(1.0);
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
    pub audio_features: Fetch<AudioFeatures>,
    pub(crate) lyrics: Fetch<Lyrics>,
    pub(crate) detail_alpha: f32,
    pub(crate) icon_alpha: f32,
    pub(crate) playlist_expansion: f32,
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
        if add { tracks.insert(track_id) } else { tracks.remove(&track_id) }
    }
}

pub fn playlist_icons(track_id: TrackId, playlists: &[CondensedPlaylist], contains_track: bool) -> impl Iterator<Item = &CondensedPlaylist> {
    playlists
        .iter()
        .filter(move |playlist| playlist.rating_index.is_none() && playlist.tracks.contains(&track_id) == contains_track)
}

pub enum PlaybackCommand {
    SetPlaying(bool),
    Seek(u32),
    Skip(i8),
    UpdateLibrary {
        track_id: TrackId,
        playlists: Vec<(PlaylistId, bool)>,
        liked: Option<bool>,
    },
}

#[derive(Clone)]
pub struct MusicBackend(Arc<spotify::SpotifyBackend>);

impl MusicBackend {
    pub(crate) fn spotify(config: &Config, updater: &AppUpdater, background: &Background) -> Self {
        Self(Arc::new(spotify::SpotifyBackend::new(config, updater, background)))
    }

    pub fn command(&self, command: PlaybackCommand) {
        self.0.command(command);
    }

    pub fn toggle_playing(&self, playing: bool) {
        let playing = !playing;
        info!("{} current track", if playing { "Playing" } else { "Pausing" });
        self.command(PlaybackCommand::SetPlaying(playing));
    }

    pub fn rate_track(&self, playlists: &mut [CondensedPlaylist], track_id: TrackId, rating: u8) {
        self.command(PlaybackCommand::UpdateLibrary {
            track_id,
            playlists: playlists
                .iter_mut()
                .filter_map(|playlist| {
                    let add = playlist.rating_index? == rating;
                    playlist.set_membership(track_id, add).then_some((playlist.id, add))
                })
                .collect(),
            liked: Some(rating >= 5),
        });
    }

    pub fn toggle_playlist(&self, playlists: &mut [CondensedPlaylist], track_id: TrackId, playlist_id: PlaylistId) {
        let Some(playlist) = playlists.iter_mut().find(|playlist| playlist.id == playlist_id) else {
            warn!(%playlist_id, %track_id, "Playlist not found for track");
            return;
        };
        let add = !playlist.tracks.contains(&track_id);
        playlist.set_membership(track_id, add);
        self.command(PlaybackCommand::UpdateLibrary {
            track_id,
            playlists: vec![(playlist_id, add)],
            liked: None,
        });
    }

    pub fn seek(&self, timeline: &Timeline, clicked_index: usize, clicked_duration_ms: u32, fraction: f32) {
        let skip_count = clicked_index.abs_diff(timeline.index);
        if skip_count == 0 {
            let milliseconds = if fraction < 0.05 { 0.0 } else { clicked_duration_ms as f32 * fraction }.round() as u32;
            self.command(PlaybackCommand::Seek(milliseconds));
        } else {
            let direction = if timeline.index < clicked_index { 1 } else { -1 };
            self.command(PlaybackCommand::Skip(direction * skip_count.min(10) as i8));
        }
    }

    /// Fetches timed lyrics from the active music service.
    ///
    /// # Errors
    ///
    /// Returns an error when the provider request or response fails.
    pub async fn lyrics(&self, track_id: TrackId) -> MusicResult<Vec<LyricSegment>> {
        self.0.lyrics(track_id).await
    }
}

use crate::{
    PANEL_START,
    spotify::{CondensedPlaylist, PlaylistId, SpotifyBackend, TrackId},
};
use isthmus::glam::Vec2;
use std::{
    mem,
    time::{Duration, Instant},
};
use tracing::{info, warn};

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
        let (min, max) = (center - half_size, center + half_size);
        Self::new(min.x, min.y, max.x, max.y)
    }

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.x0 && point.x <= self.x1 && point.y >= self.y0 && point.y <= self.y1
    }
}

pub struct InteractionState {
    mouse_pressure: f32, // 0 outside, 1 hovering, 2 held
    pub dragging: bool,
    drag_enabled: bool,
    pub press_origin: Vec2,
    pub pointer: Vec2,
    event: Option<PointerEvent>,
    scroll: i32,
    hover_claimed: bool,
    pulse: Option<Vec2>,
    pub regions: Vec<Rect>,
    /// Set when a rate/playlist-toggle action fires this frame, for particles to burst a spark.
    rate_burst: Option<Vec2>,
    spotify: SpotifyBackend,
}

#[derive(Clone, Copy)]
enum PointerEvent {
    Press,
    Release,
}

pub struct Response {
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

impl InteractionState {
    pub const fn new(spotify: SpotifyBackend) -> Self {
        Self {
            mouse_pressure: 0.0,
            dragging: false,
            drag_enabled: false,
            press_origin: Vec2::ZERO,
            pointer: Vec2::ZERO,
            event: None,
            scroll: 0,
            hover_claimed: false,
            pulse: None,
            regions: Vec::new(),
            rate_burst: None,
            spotify,
        }
    }

    pub fn surface(&mut self, rect: Rect) -> Response {
        self.regions.push(rect);
        let inside = rect.contains(self.pointer);
        let hovered = self.mouse_pressure > 0.0 && !self.hover_claimed && inside;
        self.hover_claimed |= hovered;
        let pressed = inside && matches!(self.event, Some(PointerEvent::Press));
        let clicked = inside
            && !self.dragging
            && self.press_origin.distance(self.pointer) < 2.0
            && matches!(self.event, Some(PointerEvent::Release));
        if pressed || clicked {
            self.event = None;
        }
        Response {
            hovered,
            pressed,
            clicked,
        }
    }

    pub fn scroll(&mut self, rect: Rect) -> i32 {
        self.regions.push(rect);
        if rect.contains(self.pointer) {
            mem::take(&mut self.scroll)
        } else {
            0
        }
    }

    pub fn contains(&self, rect: Rect) -> bool {
        self.mouse_pressure > 0.0 && rect.contains(self.pointer)
    }

    pub const fn mouse_pressure(&self) -> f32 {
        self.mouse_pressure
    }

    pub const fn down(&self) -> bool {
        self.mouse_pressure > 1.0
    }

    pub const fn released(&self) -> bool {
        matches!(self.event, Some(PointerEvent::Release))
    }

    pub const fn begin_frame(&mut self) {
        self.hover_claimed = false;
    }

    pub const fn end_frame(&mut self) -> Option<Vec2> {
        let pulse = self.pulse.take();
        self.event = None;
        self.scroll = 0;
        pulse
    }

    /// Takes this frame's rate/playlist-toggle spark-burst position, if one fired.
    pub const fn take_rate_burst(&mut self) -> Option<Vec2> {
        self.rate_burst.take()
    }

    pub const fn press(&mut self, position: Vec2) {
        self.mouse_pressure = 2.0;
        self.press_origin = position;
        self.event = Some(PointerEvent::Press);
        self.dragging = false;
    }

    pub fn release(&mut self) {
        if self.down() && !self.dragging && self.press_origin.distance(self.pointer) < 2.0 {
            self.pulse = Some(self.pointer);
        }
        self.event = self.down().then_some(PointerEvent::Release);
        self.mouse_pressure = 1.0;
    }

    pub fn motion(&mut self, position: Vec2) {
        self.pointer = position;
        if self.drag_enabled {
            self.dragging |= (position - self.press_origin).abs().max_element() >= 2.0;
        }
    }

    pub const fn cancel_drag(&mut self) {
        self.drag_enabled = false;
        self.dragging = false;
    }

    /// The pointer is over the surface but not held (entered, or a held press let go elsewhere).
    pub const fn hover(&mut self) {
        self.mouse_pressure = 1.0;
    }

    /// The pointer left the surface entirely.
    pub const fn leave(&mut self) {
        self.mouse_pressure = 0.0;
        self.cancel_drag();
    }

    /// Records this frame's raw OS scroll delta, for [`Self::scroll`] to consume.
    pub const fn scroll_input(&mut self, direction: i32) {
        self.scroll = direction;
    }

    /// Marks hover as already claimed this frame, so other surfaces won't take it.
    pub const fn claim_hover(&mut self) {
        self.hover_claimed = true;
    }

    /// Arms drag-tracking for the region that was just pressed.
    pub const fn enable_drag(&mut self) {
        self.drag_enabled = true;
    }

    pub fn toggle_playing(&self, playing: &mut bool) {
        *playing = !*playing;
        info!("{} current track", if *playing { "Playing" } else { "Pausing" });
        self.spotify.set_playing(*playing);
    }

    pub fn adjust_spotify_volume(&self, volume: &mut Option<u8>, direction: i32) {
        if let Some(volume) = volume {
            *volume = volume
                .saturating_add_signed(if direction < 0 { 5 } else { -5 })
                .min(100);
            info!("Setting volume to {volume}%");
            self.spotify.player_parameter("volume", "volume_percent", volume);
        }
    }

    /// Rates `track_id`, moving it into/out of whichever rating playlist matches `rating`.
    pub fn rate_track(&mut self, playlists: &mut [CondensedPlaylist], track_id: TrackId, rating: u8) {
        self.spotify.update_library(
            track_id,
            playlists
                .iter_mut()
                .filter_map(|playlist| {
                    let add = playlist.rating_index? == rating;
                    playlist
                        .set_membership(track_id, add)
                        .then_some((playlist.id, add))
                })
                .collect(),
            Some(rating >= 5),
        );
        self.rate_burst = Some(self.pointer);
    }

    /// Adds or removes `track_id` from `playlist_id`.
    pub fn toggle_playlist(
        &mut self,
        playlists: &mut [CondensedPlaylist],
        track_id: TrackId,
        playlist_id: PlaylistId,
    ) {
        let Some(playlist) = playlists.iter_mut().find(|playlist| playlist.id == playlist_id) else {
            warn!("Playlist {playlist_id} not found for track {track_id}");
            return;
        };
        let add = !playlist.tracks.contains(&track_id);
        playlist.set_membership(track_id, add);
        self.spotify
            .update_library(track_id, vec![(playlist_id, add)], None);
        self.rate_burst = Some(self.pointer);
    }

    pub fn seek(
        &self,
        queue_index: &mut usize,
        progress: &mut u32,
        last_progress_update: &mut Instant,
        last_interaction: &mut Instant,
        clicked_index: usize,
        clicked_duration_ms: u32,
        position: f32,
    ) {
        let skip_count = clicked_index.abs_diff(*queue_index);
        if skip_count == 0 {
            let milliseconds = if position < 0.05 {
                0.0
            } else {
                clicked_duration_ms as f32 * position
            }
            .round() as u32;
            *progress = milliseconds;
            *last_progress_update = Instant::now();
            self.spotify.player_parameter("seek", "position_ms", milliseconds);
        } else {
            let was_before = *queue_index < clicked_index;
            *queue_index = clicked_index;
            *progress = 0;
            *last_progress_update = Instant::now();
            self.spotify.skip(was_before, skip_count.min(10));
        }
        *last_interaction = Instant::now() + Duration::from_secs(2);
    }
}

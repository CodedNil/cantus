use crate::{
    CantusApp, PANEL_START,
    spotify::{PlaylistId, TrackId},
};
use glam::Vec2;
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

#[derive(Default)]
pub struct InteractionState {
    pub mouse_pressure: f32, // 0 outside, 1 hovering, 2 held
    pub dragging: bool,
    pub drag_enabled: bool,
    pub press_origin: Vec2,
    pub pointer: Vec2,
    event: Option<PointerEvent>,
    pub scroll: i32,
    pub hover_claimed: bool,
    pulse: Option<Vec2>,
    pub regions: Vec<Rect>,
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
        if clicked {
            self.pulse = Some(self.pointer);
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

    pub const fn down(&self) -> bool {
        self.mouse_pressure > 1.0
    }

    pub const fn released(&self) -> bool {
        matches!(self.event, Some(PointerEvent::Release))
    }

    pub const fn begin_frame(&mut self, pointer: Vec2) {
        self.pointer = pointer;
        self.hover_claimed = false;
    }

    pub const fn end_frame(&mut self) -> Option<Vec2> {
        let pulse = self.pulse.take();
        self.event = None;
        self.scroll = 0;
        pulse
    }

    pub const fn press(&mut self, position: Vec2) {
        self.mouse_pressure = 2.0;
        self.press_origin = position;
        self.event = Some(PointerEvent::Press);
        self.dragging = false;
    }

    pub fn release(&mut self) {
        self.event = self.down().then_some(PointerEvent::Release);
        self.mouse_pressure = 1.0;
    }

    pub fn motion(&mut self, position: Vec2) {
        if self.drag_enabled {
            self.dragging |= (position - self.press_origin).abs().max_element() >= 2.0;
        }
    }

    pub const fn cancel_drag(&mut self) {
        self.drag_enabled = false;
        self.dragging = false;
    }
}

#[derive(Clone, Copy)]
pub enum TrackAction {
    Rate(TrackId, u8),
    TogglePlaylist(TrackId, PlaylistId),
    Seek(TrackId, f32),
}

impl CantusApp {
    pub fn handle_track_action(&mut self, action: TrackAction) {
        match action {
            TrackAction::Rate(track_id, rating) => self.spotify.update_library(
                track_id,
                self.playback
                    .playlists
                    .iter_mut()
                    .filter_map(|playlist| {
                        let add = playlist.rating_index? == rating;
                        playlist
                            .set_membership(track_id, add)
                            .then_some((playlist.id, add))
                    })
                    .collect(),
                Some(rating >= 5),
            ),
            TrackAction::TogglePlaylist(track_id, playlist_id) => {
                let Some(playlist) = self
                    .playback
                    .playlists
                    .iter_mut()
                    .find(|playlist| playlist.id == playlist_id)
                else {
                    warn!("Playlist {playlist_id} not found for track {track_id}");
                    return;
                };
                let add = !playlist.tracks.contains(&track_id);
                playlist.set_membership(track_id, add);
                self.spotify
                    .update_library(track_id, vec![(playlist_id, add)], None);
            }
            TrackAction::Seek(track_id, position) => {
                let state = &mut self.playback;
                let queue_index = state.queue_index;
                let Some(position_in_queue) =
                    state.queue.iter().position(|track| track.id == Some(track_id))
                else {
                    warn!("Track not found in queue");
                    return;
                };
                let skip_count = position_in_queue.abs_diff(queue_index);
                if skip_count == 0 {
                    let milliseconds = if position < 0.05 {
                        0.0
                    } else {
                        state.queue[position_in_queue].duration_ms as f32 * position
                    }
                    .round() as u32;
                    state.update_progress(milliseconds, Instant::now());
                    self.spotify.player_parameter("seek", "position_ms", milliseconds);
                } else {
                    state.queue_index = position_in_queue;
                    state.update_progress(0, Instant::now());
                    self.spotify
                        .skip(queue_index < position_in_queue, skip_count.min(10));
                }
                state.last_interaction = Instant::now() + Duration::from_secs(2);
            }
        }
    }

    pub fn toggle_playing(&mut self) {
        self.render.last_toggle_time = self.render.uniforms.time;
        self.playback.playing = !self.playback.playing;
        info!(
            "{} current track",
            if self.playback.playing {
                "Playing"
            } else {
                "Pausing"
            }
        );
        self.spotify.set_playing(self.playback.playing);
    }

    pub fn adjust_playback_volume(&mut self, direction: i32) {
        if let Some(volume) = &mut self.playback.volume {
            *volume = volume
                .saturating_add_signed(if direction < 0 { 5 } else { -5 })
                .min(100);
            info!("Setting volume to {volume}%");
            self.spotify.player_parameter("volume", "volume_percent", volume);
        }
    }
}

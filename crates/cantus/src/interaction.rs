use crate::{
    app::music::{CondensedPlaylist, MusicBackend, PlaybackCommand, PlaylistId, Timeline, TrackId},
    render::PANEL_START,
};
use isthmus::glam::Vec2;
use std::mem;
use tracing::{info, warn};

#[derive(Copy, Clone)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pointer {
    Outside,
    Hovering,
    Held,
}

pub struct InteractionState {
    state: Pointer,
    pub dragging: bool,
    drag_enabled: bool,
    pub press_origin: Vec2,
    pub pointer: Vec2,
    event: Option<PointerEvent>,
    scroll: i32,
    hover_claimed: bool,
    pulse: Option<Vec2>,
    regions: Vec<Rect>,
    /// Position of a rate/playlist-toggle spark burst for this frame.
    rate_burst: Option<Vec2>,
    music: MusicBackend,
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

impl InteractionState {
    pub const fn new(music: MusicBackend) -> Self {
        Self {
            state: Pointer::Outside,
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
            music,
        }
    }

    pub fn surface(&mut self, rect: Rect) -> Response {
        self.regions.push(rect);
        let inside = rect.contains(self.pointer);
        let hovered = self.state != Pointer::Outside && !self.hover_claimed && inside;
        self.hover_claimed |= hovered;
        let pressed = inside && matches!(self.event, Some(PointerEvent::Press));
        let clicked = inside && !self.dragging && self.press_origin.distance(self.pointer) < 2.0 && matches!(self.event, Some(PointerEvent::Release));
        if pressed || clicked {
            self.event = None;
        }
        Response { hovered, pressed, clicked }
    }

    pub fn scroll(&mut self, rect: Rect) -> i32 {
        self.regions.push(rect);
        if rect.contains(self.pointer) { mem::take(&mut self.scroll) } else { 0 }
    }

    pub fn contains(&self, rect: Rect) -> bool {
        self.state != Pointer::Outside && rect.contains(self.pointer)
    }

    pub const fn pressure(&self) -> f32 {
        match self.state {
            Pointer::Outside => 0.0,
            Pointer::Hovering => 1.0,
            Pointer::Held => 2.0,
        }
    }

    pub const fn down(&self) -> bool {
        matches!(self.state, Pointer::Held)
    }

    pub fn input_region(&mut self, rect: Rect) {
        self.regions.push(rect);
    }

    pub fn take_regions(&mut self) -> impl Iterator<Item = Rect> + '_ {
        self.regions.drain(..)
    }

    pub const fn released(&self) -> bool {
        matches!(self.event, Some(PointerEvent::Release))
    }

    pub const fn begin_frame(&mut self) {
        self.hover_claimed = false;
    }

    pub const fn end_frame(&mut self) -> Option<Vec2> {
        let pulse = self.pulse.take();
        if !self.down() {
            self.cancel_drag();
        }
        self.event = None;
        self.scroll = 0;
        pulse
    }

    pub const fn take_rate_burst(&mut self) -> Option<Vec2> {
        self.rate_burst.take()
    }

    pub const fn press(&mut self, position: Vec2) {
        self.state = Pointer::Held;
        self.press_origin = position;
        self.event = Some(PointerEvent::Press);
        self.dragging = false;
    }

    pub fn release(&mut self) {
        if self.down() && !self.dragging && self.press_origin.distance(self.pointer) < 2.0 {
            self.pulse = Some(self.pointer);
        }
        self.event = self.down().then_some(PointerEvent::Release);
        self.state = Pointer::Hovering;
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

    pub const fn hover(&mut self) {
        self.state = Pointer::Hovering;
    }

    pub const fn leave(&mut self) {
        self.state = Pointer::Outside;
        self.cancel_drag();
    }

    pub const fn scroll_input(&mut self, direction: i32) {
        self.scroll = direction;
    }

    pub const fn claim_hover(&mut self) {
        self.hover_claimed = true;
    }

    pub const fn enable_drag(&mut self) {
        self.drag_enabled = true;
    }

    pub fn toggle_playing(&self, playing: bool) {
        let playing = !playing;
        info!("{} current track", if playing { "Playing" } else { "Pausing" });
        self.music.command(PlaybackCommand::SetPlaying(playing));
    }

    pub fn rate_track(&mut self, playlists: &mut [CondensedPlaylist], track_id: TrackId, rating: u8) {
        self.music.command(PlaybackCommand::UpdateLibrary {
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
        self.rate_burst = Some(self.pointer);
    }

    pub fn toggle_playlist(&mut self, playlists: &mut [CondensedPlaylist], track_id: TrackId, playlist_id: PlaylistId) {
        let Some(playlist) = playlists.iter_mut().find(|playlist| playlist.id == playlist_id) else {
            warn!("Playlist {playlist_id} not found for track {track_id}");
            return;
        };
        let add = !playlist.tracks.contains(&track_id);
        playlist.set_membership(track_id, add);
        self.music.command(PlaybackCommand::UpdateLibrary {
            track_id,
            playlists: vec![(playlist_id, add)],
            liked: None,
        });
        self.rate_burst = Some(self.pointer);
    }

    pub fn seek(&self, timeline: &Timeline, clicked_index: usize, clicked_duration_ms: u32, fraction: f32) {
        let skip_count = clicked_index.abs_diff(timeline.index);
        if skip_count == 0 {
            let milliseconds = if fraction < 0.05 { 0.0 } else { clicked_duration_ms as f32 * fraction }.round() as u32;
            self.music.command(PlaybackCommand::Seek(milliseconds));
        } else {
            let direction = if timeline.index < clicked_index { 1 } else { -1 };
            self.music.command(PlaybackCommand::Skip(direction * skip_count.min(10) as i8));
        }
    }
}

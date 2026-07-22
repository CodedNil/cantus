use crate::{
    CantusApp, PANEL_START, Rect,
    render::status::{PowerAction, Status},
    spotify::{CondensedPlaylist, PlaylistId, Track, TrackId, playlist_icons},
};
use cantus_shared::{
    ICON_WIDTH, PillIconRow, RIPPLE_COUNT, RipplePulse, pill_icon_primary_center_y, pill_icon_rows,
};
use glam::{Vec2, vec2};
use std::time::{Duration, Instant};
use tracing::{info, warn};

enum IconAction {
    Rate(u8),
    TogglePlaylist(PlaylistId),
}

const POWER_HOLD_DURATION: Duration = Duration::from_millis(1_500);
const POWER_PULSE_INTERVAL: Duration = Duration::from_millis(420);
const VOLUME_SCROLL_RANGE: f32 = 100.0;

#[derive(Clone, Copy)]
struct PowerHold {
    action: PowerAction,
    started: Instant,
    last_pulse: Instant,
}

#[derive(Default)]
pub struct InteractionState {
    pub mouse_pressure: f32, // 0 not hovered - 1 hovered - 2 mouse down
    pub hovered_track: Option<usize>,
    pub dragging: bool,
    drag_enabled: bool,
    pub press_origin: Vec2,
    power_hold: Option<PowerHold>,
    ripple_cursor: usize,
}

impl CantusApp {
    pub fn left_click(&mut self) {
        let mouse_pos = self.render.uniforms.mouse_pos;
        let drag_enabled = self
            .interaction
            .hovered_track
            .and_then(|index| self.playback.queue.get(index))
            .is_some_and(|track| track.contains(mouse_pos, self.config.height));
        let power_action =
            Status::power_action_at(mouse_pos, &self.render.status, self.config.height);
        let now = Instant::now();
        self.interaction.mouse_pressure = 2.0;
        self.interaction.press_origin = mouse_pos;
        self.interaction.drag_enabled = drag_enabled;
        self.interaction.dragging = false;
        self.interaction.power_hold = power_action.map(|action| {
            let center =
                Status::power_action_center(action, &self.render.status, self.config.height);
            self.pulse(center, 1.0);
            PowerHold {
                action,
                started: now,
                last_pulse: now,
            }
        });
    }

    pub fn left_click_released(&mut self) {
        let held_power_action = self.interaction.power_hold.take().is_some();
        let same_spot = self
            .interaction
            .press_origin
            .distance(self.render.uniforms.mouse_pos)
            < 2.0;
        if !held_power_action
            && same_spot
            && !self.interaction.dragging
            && self.interaction.mouse_pressure > 1.0
        {
            self.handle_click();
        }
        if self.interaction.dragging
            && let Some(track) = self.playback.queue.iter().find(|track| track.is_current())
            && let Some(track_id) = track.id
        {
            let timeline = self.timeline();
            let (start, end) = track.natural_x_range(timeline.playhead_x, timeline.px_per_ms);
            let position = (timeline.playhead_x.max(track.runtime.start_x) - start) / (end - start);
            self.pulse_at_playhead();
            self.skip_to_track(track_id, position);
        }
        self.cancel_drag();
        self.interaction.mouse_pressure = 1.0;
    }

    /// Handle click events.
    fn handle_click(&mut self) {
        let mouse_pos = self.render.uniforms.mouse_pos;
        let timeline = self.timeline();
        if self
            .weather
            .navigate_calendar(mouse_pos, &self.render.status, self.config.height)
        {
            self.pulse(mouse_pos, 1.0);
            return;
        }
        if self.overlay_contains(mouse_pos) {
            self.pulse(mouse_pos, 1.0);
            return;
        }
        let icon_click = |track: &Track| self.icon_at(track, &self.playback.playlists);

        if let Some((track_id, action)) = self
            .interaction
            .hovered_track
            .into_iter()
            .chain(0..self.playback.queue.len())
            .filter_map(|index| self.playback.queue.get(index))
            .find_map(icon_click)
        {
            self.emit_click_particles(mouse_pos);

            match action {
                IconAction::Rate(rating) => self.update_star_rating(track_id, rating),
                IconAction::TogglePlaylist(playlist_id) => {
                    self.toggle_playlist_membership(track_id, playlist_id);
                }
            }
        } else if self.playhead_rect().contains(mouse_pos) {
            // Play/pause
            self.pulse_at_playhead();
            self.render.last_toggle_playing = Instant::now();
            self.toggle_playing(!self.playback.playing);
        } else if let Some((track_id, (track_range_a, track_range_b))) =
            self.playback.queue.iter().rev().find_map(|track| {
                let range = track.natural_x_range(timeline.playhead_x, timeline.px_per_ms);
                track
                    .contains(mouse_pos, self.config.height)
                    .then_some((track.id, range))
            })
        {
            // Seek track
            self.pulse(mouse_pos, 1.0);

            // If click is near the very left, reset to the start of the song, else seek to clicked position
            let position = if mouse_pos.x < self.config.history_width + 40.0 {
                0.0
            } else {
                (mouse_pos.x - track_range_a) / (track_range_b - track_range_a)
            };
            if let Some(track_id) = track_id {
                self.skip_to_track(track_id, position);
            }
        }
    }

    fn pulse(&mut self, pos: Vec2, strength: f32) {
        let cursor = self.interaction.ripple_cursor;
        self.render.uniforms.ripples[cursor] = RipplePulse {
            origin: pos,
            animation: vec2(self.render.start_time.elapsed().as_secs_f32(), strength),
        };
        self.interaction.ripple_cursor = (cursor + 1) % RIPPLE_COUNT;
    }

    fn pulse_at_playhead(&mut self) {
        let center = vec2(
            self.timeline().playhead_x,
            PANEL_START + self.config.height * 0.5,
        );
        self.pulse(center, 1.0);
    }

    /// Drag across the progress bar to seek.
    pub fn handle_mouse_drag(&mut self) {
        let interaction = &mut self.interaction;
        if !interaction.drag_enabled {
            return;
        }
        let delta = (self.render.uniforms.mouse_pos - interaction.press_origin).abs();
        interaction.dragging |= delta.x >= 2.0 || delta.y >= 2.0;
    }

    /// Handle scrolling events to adjust volume. `direction` must be -1 or 1.
    pub fn handle_scroll(&mut self, direction: i32) {
        if Status::audio_at(
            self.render.uniforms.mouse_pos,
            &self.render.status,
            self.config.height,
        ) {
            self.status.adjust_volume(direction);
            return;
        }
        let near_playhead = (self.render.uniforms.mouse_pos.x - self.timeline().playhead_x).abs()
            <= VOLUME_SCROLL_RANGE;
        if !near_playhead {
            return;
        }
        if let Some(volume) = &mut self.playback.volume {
            *volume = volume
                .saturating_add_signed(if direction < 0 { 5 } else { -5 })
                .min(100);
            info!("Setting volume to {volume}%");
            self.spotify
                .player_parameter("volume", "volume_percent", volume);
        }
    }

    pub const fn cancel_drag(&mut self) {
        self.interaction.drag_enabled = false;
        self.interaction.dragging = false;
    }

    pub fn power_hold_scene(&mut self) -> (Option<PowerAction>, f32) {
        let Some(mut hold) = self.interaction.power_hold else {
            return (None, 0.0);
        };
        let still_over_action = Status::power_action_at(
            self.render.uniforms.mouse_pos,
            &self.render.status,
            self.config.height,
        ) == Some(hold.action);
        if self.interaction.mouse_pressure <= 1.0 || !still_over_action {
            self.interaction.power_hold = None;
            return (None, 0.0);
        }

        let now = Instant::now();
        let progress =
            now.duration_since(hold.started).as_secs_f32() / POWER_HOLD_DURATION.as_secs_f32();
        let pulse_interval = POWER_PULSE_INTERVAL.mul_f32(1.0 - progress.min(1.0) * 0.65);
        if now.duration_since(hold.last_pulse) >= pulse_interval {
            let center =
                Status::power_action_center(hold.action, &self.render.status, self.config.height);
            self.pulse(center, 1.0 + progress.min(1.0).powi(2) * 2.0);
            hold.last_pulse = now;
            self.interaction.power_hold = Some(hold);
        }

        if progress >= 1.0 {
            self.interaction.power_hold = None;
            Status::run_power_action(hold.action);
            return (Some(hold.action), 1.0);
        }
        (Some(hold.action), progress)
    }

    fn icon_layout(&self, track: &Track) -> Option<(TrackId, usize, PillIconRow, PillIconRow)> {
        let track_id = track.id?;
        let stars = usize::from(self.config.ratings_enabled) * 5;
        let (primary_row, secondary_row) = pill_icon_rows(
            track.runtime.start_x + track.runtime.width * 0.5,
            pill_icon_primary_center_y(PANEL_START, self.config.height),
            (stars + track.runtime.primary_playlist_count as usize) as f32,
            f32::from(track.runtime.secondary_playlist_count),
            track.runtime.playlist_expansion,
        );
        Some((track_id, stars, primary_row, secondary_row))
    }

    pub fn icon_row_rects(&self, track: &Track) -> [Option<Rect>; 2] {
        let Some((_, _, primary_row, secondary_row)) = self.icon_layout(track) else {
            return [None, None];
        };
        [
            (primary_row, track.runtime.primary_icon_alpha),
            (secondary_row, secondary_row.expansion),
        ]
        .map(|(row, alpha)| {
            (row.count > 0.0 && alpha > 0.0)
                .then(|| Rect::from_center(row.center, row.half_size(9.0 + ICON_WIDTH / 3.0)))
        })
    }

    fn icon_at(
        &self,
        track: &Track,
        playlists: &[CondensedPlaylist],
    ) -> Option<(TrackId, IconAction)> {
        let mouse_pos = self.render.uniforms.mouse_pos;
        let (track_id, stars, primary_row, secondary_row) = self.icon_layout(track)?;

        if track.runtime.primary_icon_alpha > 0.0
            && let Some((index, right_half)) = primary_row.hit(mouse_pos)
        {
            return if index < stars {
                Some((
                    track_id,
                    IconAction::Rate(index as u8 * 2 + u8::from(right_half)),
                ))
            } else {
                playlist_icons(track_id, playlists, true)
                    .nth(index - stars)
                    .map(|playlist| (track_id, IconAction::TogglePlaylist(playlist.id)))
            };
        }

        secondary_row
            .hit(mouse_pos)
            .and_then(|(index, _)| playlist_icons(track_id, playlists, false).nth(index))
            .map(|playlist| (track_id, IconAction::TogglePlaylist(playlist.id)))
    }

    /// Skip to the specified track in the queue.
    fn skip_to_track(&mut self, track_id: TrackId, position: f32) {
        let state = &mut self.playback;
        let queue_index = state.queue_index;
        let Some(position_in_queue) = state.queue.iter().position(|t| t.id == Some(track_id))
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
            self.spotify
                .player_parameter("seek", "position_ms", milliseconds);
        } else {
            state.queue_index = position_in_queue;
            state.update_progress(0, Instant::now());
            self.spotify
                .skip(queue_index < position_in_queue, skip_count.min(10));
        }
        // Ignore remote playback updates briefly so they don't fight the local seek.
        state.last_interaction = Instant::now() + Duration::from_secs(2);
    }

    /// Update Spotify rating playlists for the given track.
    fn update_star_rating(&mut self, track_id: TrackId, rating_slot: u8) {
        let changes = self
            .playback
            .playlists
            .iter_mut()
            .filter_map(|playlist| {
                let add = playlist.rating_index? == rating_slot;
                playlist
                    .set_membership(track_id, add)
                    .then_some((playlist.id, add))
            })
            .collect::<Vec<_>>();

        self.spotify
            .update_library(track_id, changes, Some(rating_slot >= 5));
    }

    fn toggle_playlist_membership(&mut self, track_id: TrackId, playlist_id: PlaylistId) {
        let Some(playlist) = self
            .playback
            .playlists
            .iter_mut()
            .find(|playlist| playlist.id == playlist_id)
        else {
            warn!(
                "Playlist {playlist_id} not found while toggling membership for track {track_id}"
            );
            return;
        };
        let add = !playlist.tracks.contains(&track_id);
        playlist.set_membership(track_id, add);
        self.spotify
            .update_library(track_id, vec![(playlist_id, add)], None);
    }

    /// Set Spotify playing or paused.
    fn toggle_playing(&mut self, play: bool) {
        info!("{} current track", if play { "Playing" } else { "Pausing" });
        self.playback.playing = play;
        self.spotify.set_playing(play);
    }
}

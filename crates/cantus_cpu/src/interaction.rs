use crate::{
    CantusApp, PANEL_START,
    model::{CondensedPlaylist, PlaylistId, Rect, Track, TrackId, playlist_icons},
    status::Status,
    weather,
};
use cantus_shared::{ICON_WIDTH, PillIconRow, pill_icon_primary_center_y, pill_icon_rows};
use glam::{Vec2, vec2};
use std::time::{Duration, Instant};
use tracing::{info, warn};

enum IconAction {
    Rate(u8),
    TogglePlaylist(PlaylistId),
}

fn row_rect(row: PillIconRow) -> Rect {
    Rect::from_center(row.center, row.half_size(9.0 + ICON_WIDTH / 3.0))
}

#[derive(Default)]
pub struct InteractionState {
    pub mouse_pressure: f32, // 0 not hovered - 1 hovered - 2 mouse down
    pub hovered_track: Option<usize>,
    pub dragging: bool,
    pub drag_origin: Option<Vec2>,
    pub drag_track: Option<(Option<TrackId>, f32)>,
}

impl CantusApp {
    pub fn left_click(&mut self) {
        let mouse_pos = self.render.uniforms.mouse_pos;
        let drag_origin = self
            .playback
            .queue
            .iter()
            .any(|track| {
                track
                    .runtime
                    .rect(self.config.height)
                    .is_some_and(|rect| rect.contains(mouse_pos))
            })
            .then_some(mouse_pos);
        let interaction = &mut self.interaction;
        interaction.mouse_pressure = 2.0;
        interaction.drag_origin = drag_origin;
        interaction.drag_track = None;
        interaction.dragging = false;
    }

    pub fn left_click_released(&mut self) {
        if !self.interaction.dragging && self.interaction.mouse_pressure > 1.0 {
            self.handle_click();
        }
        if let Some((track_id, position)) = self.interaction.drag_track.take() {
            // Get the x position of the playhead, run an expansion animation there
            self.pulse_at_playhead();
            if let Some(track_id) = track_id {
                self.skip_to_track(track_id, position);
            }
        }
        self.cancel_drag();
        self.interaction.mouse_pressure = 1.0;
    }

    pub const fn right_click(&mut self) {
        self.cancel_drag();
        self.interaction.mouse_pressure = 1.0;
    }

    /// Handle click events.
    fn handle_click(&mut self) {
        let mouse_pos = self.render.uniforms.mouse_pos;
        let timeline = self.timeline();
        if Status::run_power_action(mouse_pos, self.render.status) {
            return;
        }
        if weather::rect(self.render.status, self.config.height).contains(mouse_pos) {
            self.pulse_at(mouse_pos);
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
                let rect = track.runtime.rect(self.config.height)?;
                let range = track.natural_x_range(timeline.playhead_x, timeline.px_per_ms);
                rect.contains(mouse_pos).then_some((track.id, range))
            })
        {
            // Seek track
            self.pulse_at(mouse_pos);

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

    fn pulse_at(&mut self, pos: Vec2) {
        self.render.uniforms.expansion_xy = pos;
        self.render.uniforms.expansion_time = self.render.start_time.elapsed().as_secs_f32();
    }

    fn pulse_at_playhead(&mut self) {
        self.pulse_at(vec2(
            self.timeline().playhead_x,
            PANEL_START + self.config.height * 0.5,
        ));
    }

    /// Drag across the progress bar to seek.
    pub fn handle_mouse_drag(&mut self) {
        let interaction = &mut self.interaction;
        let Some(origin) = interaction.drag_origin else {
            return;
        };
        let delta = (self.render.uniforms.mouse_pos - origin).abs();
        interaction.dragging |= delta.x >= 2.0 || delta.y >= 2.0;
    }

    /// Handle scrolling events to adjust volume.
    pub fn handle_scroll(&mut self, delta: i32) {
        let scroll_direction = delta.signum();
        if scroll_direction == 0 {
            return;
        }
        if let Some(volume) = &mut self.playback.volume {
            *volume = volume
                .saturating_add_signed(if scroll_direction < 0 { 5 } else { -5 })
                .min(100);
            info!("Setting volume to {volume}%");
            self.spotify
                .player_parameter("volume", "volume_percent", volume);
        }
    }

    pub const fn cancel_drag(&mut self) {
        let interaction = &mut self.interaction;
        interaction.drag_track = None;
        interaction.drag_origin = None;
        interaction.dragging = false;
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
        .map(|(row, alpha)| (row.count > 0.0 && alpha > 0.0).then(|| row_rect(row)))
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
        state.defer_remote_updates(Duration::from_secs(2));
    }

    /// Update Spotify rating playlists for the given track.
    fn update_star_rating(&mut self, track_id: TrackId, rating_slot: u8) {
        self.playback
            .defer_remote_updates(Duration::from_millis(500));
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
        self.playback
            .defer_remote_updates(Duration::from_millis(500));

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

use crate::{
    CantusApp, PANEL_START,
    model::{AppUpdater, CondensedPlaylist, PlaylistId, Rect, Track, TrackId, playlist_icons},
    spotify,
};
use cantus_shared::{
    BACKPLATE_RADIUS, ICON_WIDTH, PillIconRow, pill_icon_primary_center_y, pill_icon_rows,
};
use glam::{Vec2, vec2};
use std::{
    sync::Arc,
    thread::spawn,
    time::{Duration, Instant},
};
use tracing::{error, info, warn};
use ureq::http::Method;

enum IconAction {
    Rate(u8),
    TogglePlaylist(PlaylistId),
}

fn row_rect(row: PillIconRow) -> Rect {
    let half_size = row.half_size(BACKPLATE_RADIUS + ICON_WIDTH / 3.0);
    Rect::new(
        row.center.x - half_size.x,
        row.center.y - half_size.y,
        row.center.x + half_size.x,
        row.center.y + half_size.y,
    )
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
    pub const fn left_click(&mut self) {
        let interaction = &mut self.interaction;
        interaction.mouse_pressure = 2.0;
        interaction.drag_origin = Some(self.render.uniforms.mouse_pos);
        interaction.drag_track = None;
        interaction.dragging = false;
    }

    pub fn left_click_released(&mut self) {
        if !self.interaction.dragging && self.interaction.drag_origin.is_some() {
            self.handle_click();
        }
        if let Some((track_id, position)) = self.interaction.drag_track.take() {
            // Get the x position of the playhead, run an expansion animation there
            self.pulse_at_playhead();
            if let Some(track_id) = track_id {
                let client = Arc::clone(&self.spotify.client);
                skip_to_track(track_id, position, &self.spotify.updater, client);
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
        let playlists = &self.playback.playlists;
        let icon_click = |track: &Track| self.icon_at(track, playlists);

        if let Some((track_id, action)) = self
            .interaction
            .hovered_track
            .and_then(|index| self.playback.queue.get(index))
            .and_then(icon_click)
            .or_else(|| {
                self.playback
                    .queue
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| Some(*index) != self.interaction.hovered_track)
                    .find_map(|(_, track)| icon_click(track))
            })
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
                let range =
                    track.natural_x_range(self.config.playhead_x(), self.config.px_per_ms());
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
                let client = Arc::clone(&self.spotify.client);
                skip_to_track(track_id, position, &self.spotify.updater, client);
            }
        }
    }

    fn pulse_at(&mut self, pos: Vec2) {
        self.render.uniforms.expansion_xy = pos;
        self.render.uniforms.expansion_time = self.render.start_time.elapsed().as_secs_f32();
    }

    fn pulse_at_playhead(&mut self) {
        self.pulse_at(vec2(
            self.config.playhead_x(),
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
            *volume = if scroll_direction < 0 {
                volume.saturating_add(5).min(100)
            } else {
                volume.saturating_sub(5)
            };
            let volume = *volume;
            info!("Setting volume to {volume}%");
            self.spotify.client.put_in_background(
                format!("me/player/volume?volume_percent={volume}"),
                "set volume",
            );
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
            (primary_row.count > 0.0 && track.runtime.primary_icon_alpha > 0.0)
                .then(|| row_rect(primary_row)),
            (secondary_row.count > 0.0 && secondary_row.expansion > 0.0)
                .then(|| row_rect(secondary_row)),
        ]
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
}

/// Skip to the specified track in the queue.
fn skip_to_track(
    track_id: TrackId,
    position: f32,
    updater: &AppUpdater,
    client: Arc<spotify::SpotifyClient>,
) {
    updater.send(move |app| {
        let state = &mut app.playback;
        let queue_index = state.queue_index;
        let Some(position_in_queue) = state.queue.iter().position(|t| t.id == Some(track_id))
        else {
            error!("Track not found in queue");
            return;
        };
        let song_ms = state.queue[position_in_queue].duration_ms;
        if queue_index == position_in_queue {
            let milliseconds = if position < 0.05 {
                0.0
            } else {
                song_ms as f32 * position
            };
            state.update_progress(milliseconds.round() as u32, Instant::now());
            state.defer_remote_updates(Duration::from_secs(2));
            client.put_in_background(
                format!("me/player/seek?position_ms={}", milliseconds.round()),
                "seek track",
            );
            return;
        }

        state.queue_index = position_in_queue;
        state.update_progress(0, Instant::now());
        state.defer_remote_updates(Duration::from_secs(2));
        spawn(move || {
            let path = if queue_index < position_in_queue {
                "me/player/next"
            } else {
                "me/player/previous"
            };
            for _ in 0..position_in_queue.abs_diff(queue_index).min(10) {
                if let Err(err) = client.api_request(Method::POST, path, None) {
                    error!("Failed to skip track: {err}");
                }
            }
        });
    });
}

fn set_playlist_membership(
    client: &spotify::SpotifyClient,
    playlist_id: PlaylistId,
    track_id: TrackId,
    add: bool,
) {
    let track_uri = format!("spotify:track:{track_id}");
    let path = format!("playlists/{playlist_id}/items");
    let (method, body) = if add {
        (Method::POST, format!(r#"{{"uris": ["{track_uri}"]}}"#))
    } else {
        (
            Method::DELETE,
            format!(r#"{{"items": [{{"uri": "{track_uri}"}}]}}"#),
        )
    };
    if let Err(err) = client.api_request(method, &path, Some(&body)) {
        error!("Failed to update playlist {playlist_id} for track {track_id}: {err}");
    }
}

/// Update Spotify rating playlists for the given track.
impl CantusApp {
    fn update_star_rating(&mut self, track_id: TrackId, rating_slot: u8) {
        self.playback
            .defer_remote_updates(Duration::from_millis(500));
        let changes = self
            .playback
            .playlists
            .iter_mut()
            .filter_map(|playlist| {
                let add = playlist.rating_index? == rating_slot;
                let tracks = Arc::make_mut(&mut playlist.tracks);
                let changed = if add {
                    tracks.insert(track_id)
                } else {
                    tracks.remove(&track_id)
                };
                changed.then_some((playlist.id, add))
            })
            .collect::<Vec<_>>();

        let client = Arc::clone(&self.spotify.client);
        spawn(move || {
            for (playlist_id, add) in changes {
                set_playlist_membership(&client, playlist_id, track_id, add);
            }

            let track_uri = format!("spotify:track:{track_id}");
            let should_like = rating_slot >= 5;
            if let Some([liked]) = client.api_json::<[bool; 1]>(
                &format!("me/library/contains/?uris={track_uri}"),
                "liked state",
            ) && liked != should_like
            {
                let method = if should_like {
                    Method::PUT
                } else {
                    Method::DELETE
                };
                if let Err(err) =
                    client.api_request(method, &format!("me/library/?uris={track_uri}"), None)
                {
                    error!("Failed to update liked state for track {track_id}: {err}");
                }
            }
        });
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
        let tracks = Arc::make_mut(&mut playlist.tracks);
        let add = !tracks.remove(&track_id);
        if add {
            tracks.insert(track_id);
        }
        self.playback
            .defer_remote_updates(Duration::from_millis(500));

        let client = Arc::clone(&self.spotify.client);
        spawn(move || {
            set_playlist_membership(&client, playlist_id, track_id, add);
        });
    }

    /// Set Spotify playing or paused.
    fn toggle_playing(&mut self, play: bool) {
        info!("{} current track", if play { "Playing" } else { "Pausing" });
        self.playback.playing = play;

        let action = if play { "play" } else { "pause" };
        self.spotify.client.put_in_background(
            format!("me/player/{action}"),
            if play {
                "start playback"
            } else {
                "pause playback"
            },
        );
    }
}

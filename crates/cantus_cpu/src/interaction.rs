use crate::{
    AppUpdater, CantusApp, CondensedPlaylist, PANEL_START, PlaylistId, Track, TrackId,
    render::{IconInstance, Rect, lerpf32},
    spotify,
};
use cantus_shared::ICON_SPACING;
use glam::{Vec2, vec2};
use smallvec::SmallVec;
use std::{
    cmp::Ordering,
    thread::spawn,
    time::{Duration, Instant},
};
use std::{f32::consts::PI, sync::Arc};
use tracing::{error, info, warn};

pub struct IconHitbox {
    pub rect: Rect,
    pub action: IconAction,
}

#[derive(Clone, Copy)]
pub enum IconAction {
    Rate(u8),
    TogglePlaylist(PlaylistId),
}

#[derive(Default)]
pub struct InteractionState {
    pub mouse_pressure: f32, // 0 not hovered - 1 hovered - 2 mouse down

    pub mouse_down: bool,
    pub dragging: bool,
    pub drag_origin: Option<Vec2>,
    pub drag_track: Option<(Option<TrackId>, f32)>,
}

impl CantusApp {
    pub const fn left_click(&mut self) {
        let interaction = &mut self.interaction;
        interaction.mouse_down = true;
        interaction.mouse_pressure = 2.0;
        interaction.drag_origin = Some(self.global_uniforms.mouse_pos);
        interaction.drag_track = None;
        interaction.dragging = false;
    }

    pub fn left_click_released(&mut self) {
        if !self.interaction.dragging && self.interaction.mouse_down {
            self.handle_click();
        }
        let interaction = &mut self.interaction;
        if let Some((track_id, position)) = interaction.drag_track.take() {
            // Get the x position of the playhead, run an expansion animation there
            self.global_uniforms.expansion_xy = vec2(
                self.config.playhead_x(),
                PANEL_START + self.config.height * 0.5,
            );
            self.global_uniforms.expansion_time = self.start_time.elapsed().as_secs_f32();
            if let Some(track_id) = track_id {
                let client = Arc::clone(&self.spotify.client);
                skip_to_track(track_id, position, false, &self.updater, client);
            }
        }
        interaction.drag_origin = None;
        interaction.dragging = false;
        interaction.mouse_down = false;
        interaction.mouse_pressure = 1.0;
    }

    pub const fn right_click(&mut self) {
        self.cancel_drag();
        self.interaction.mouse_down = false;
    }

    /// Handle click events.
    fn handle_click(&mut self) {
        let mouse_pos = self.global_uniforms.mouse_pos;
        let playing = self.playback_state.playing;

        // Click on rating/playlist icons
        if let Some((track_id, rect, action)) = self.playback_state.queue.iter().find_map(|track| {
            let hitbox = track
                .runtime
                .icon_hitboxes
                .iter()
                .find(|hitbox| hitbox.rect.contains(mouse_pos))?;
            Some((track.id?, hitbox.rect, hitbox.action))
        }) {
            // Spawn particles
            let time = self.start_time.elapsed().as_secs_f32();
            for particle in self
                .particles
                .iter_mut()
                .filter(|particle| time > particle.end_time)
                .take(20)
            {
                particle.spawn_pos = vec2(mouse_pos.x, mouse_pos.y);
                let angle = fastrand::f32() * 2.0 * PI;
                let speed = 30.0 + (fastrand::f32() * 20.0);
                particle.spawn_vel = vec2(angle.cos() * speed, angle.sin() * speed);
                let duration = lerpf32(fastrand::f32(), 0.5, 1.5);
                particle.color =
                    u32::from_le_bytes([255, 215, 50, (duration * 100.0).min(255.0) as u8]);
                particle.end_time = time + duration;
            }

            match action {
                IconAction::Rate(index) if self.config.ratings_enabled => {
                    let center_x = (rect.x0 + rect.x1) * 0.5;
                    let rating_slot = index * 2 + u8::from(mouse_pos.x >= center_x);
                    self.update_star_rating(track_id, rating_slot);
                }
                IconAction::TogglePlaylist(playlist_id) => {
                    self.toggle_playlist_membership(track_id, playlist_id);
                }
                IconAction::Rate(_) => {}
            }
        } else if Rect::new(
            self.config.playhead_x() - self.config.height * 0.25,
            PANEL_START,
            self.config.playhead_x() + self.config.height * 0.25,
            PANEL_START + self.config.height,
        )
        .contains(mouse_pos)
        {
            // Play/pause
            self.global_uniforms.expansion_xy = vec2(
                self.config.playhead_x(),
                PANEL_START + self.config.height * 0.5,
            );
            self.global_uniforms.expansion_time = self.start_time.elapsed().as_secs_f32();
            self.last_toggle_playing = Instant::now();
            self.toggle_playing(!playing);
        } else if let Some((track_id, (track_range_a, track_range_b))) =
            self.playback_state.queue.iter().rev().find_map(|track| {
                let rect = track.runtime.rect(self.config.height)?;
                let range =
                    track.natural_x_range(self.config.playhead_x(), self.config.px_per_ms());
                rect.contains(mouse_pos).then_some((track.id, range))
            })
        {
            // Seek track
            self.global_uniforms.expansion_xy = mouse_pos;
            self.global_uniforms.expansion_time = self.start_time.elapsed().as_secs_f32();

            // If click is near the very left, reset to the start of the song, else seek to clicked position
            let position = if mouse_pos.x < self.config.history_width + 40.0 {
                0.0
            } else {
                (mouse_pos.x - track_range_a) / (track_range_b - track_range_a)
            };
            if let Some(track_id) = track_id {
                let client = Arc::clone(&self.spotify.client);
                skip_to_track(track_id, position, false, &self.updater, client);
            }
        }
    }

    /// Drag across the progress bar to seek.
    pub fn handle_mouse_drag(&mut self) {
        let interaction = &mut self.interaction;
        if let Some(origin_pos) = interaction.drag_origin {
            let delta_x = self.global_uniforms.mouse_pos.x - origin_pos.x;
            let delta_y = self.global_uniforms.mouse_pos.y - origin_pos.y;
            if !interaction.dragging && (delta_x.abs() >= 2.0 || delta_y.abs() >= 2.0) {
                interaction.dragging = true;
            }
        }
    }

    /// Handle scrolling events to adjust volume.
    pub fn handle_scroll(&mut self, delta: i32) {
        let scroll_direction = delta.signum();
        if scroll_direction == 0 {
            return;
        }
        if let Some(volume) = &mut self.playback_state.volume {
            *volume = if scroll_direction < 0 {
                volume.saturating_add(5).min(100)
            } else {
                volume.saturating_sub(5)
            };
            let volume = *volume;
            let client = Arc::clone(&self.spotify.client);
            spawn(move || {
                set_volume(volume, &client);
            });
        }
    }

    pub const fn cancel_drag(&mut self) {
        let interaction = &mut self.interaction;
        interaction.drag_track = None;
        interaction.drag_origin = None;
        interaction.dragging = false;
    }
}

enum IconEntry<'a> {
    Star {
        index: u8,
    },
    Playlist {
        playlist: &'a CondensedPlaylist,
        contained: bool,
    },
}

impl IconEntry<'_> {
    const fn is_secondary(&self) -> bool {
        matches!(
            self,
            Self::Playlist {
                contained: false,
                ..
            }
        )
    }
}

impl CantusApp {
    /// Star ratings and favourite playlists
    pub fn draw_playlist_buttons(
        &mut self,
        track: &mut Track,
        hovered: bool,
        secondary_expansion: f32,
        playlists: &[CondensedPlaylist],
    ) -> Option<(f32, f32)> {
        let width = track.runtime.width;
        let pos_x = track.runtime.start_x;
        let track_id = track.id?;
        let mut entries = SmallVec::<[IconEntry; 10]>::new();
        let track_rating_index = if self.config.ratings_enabled {
            let rating = playlists
                .iter()
                .find_map(|p| {
                    p.rating_index
                        .filter(|_| p.tracks.contains(&track_id))
                        .map(|rating| rating + 1)
                })
                .unwrap_or(0);
            entries.extend((0..5).map(|index| IconEntry::Star { index }));
            rating
        } else {
            0
        };

        entries.extend(
            playlists
                .iter()
                .filter(|p| p.rating_index.is_none() && p.tracks.contains(&track_id))
                .map(|playlist| IconEntry::Playlist {
                    playlist,
                    contained: true,
                }),
        );
        if secondary_expansion > 0.0 {
            entries.extend(
                playlists
                    .iter()
                    .filter(|p| p.rating_index.is_none() && !p.tracks.contains(&track_id))
                    .map(|playlist| IconEntry::Playlist {
                        playlist,
                        contained: false,
                    }),
            );
        }

        // Fade out and fit based on size
        let mouse_pos = self.global_uniforms.mouse_pos;

        if entries.is_empty() {
            return None;
        }
        let primary_count = entries.iter().filter(|entry| !entry.is_secondary()).count();
        let secondary_count = entries.len() - primary_count;

        let needed_width = ICON_SPACING * primary_count as f32 * 0.7;

        let width_fade = if primary_count > 0 {
            ((width - needed_width) / (needed_width * 0.5)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let fade_alpha = if hovered { 1.0 } else { width_fade };
        let center_x = pos_x + width * 0.5;
        let center_y = PANEL_START + self.config.height * 0.975;

        let mut hover_rating_index = None;
        let mut icon_data = SmallVec::<[(IconEntry, bool, Vec2); 10]>::new();
        let mut primary_index = 0;
        let mut secondary_index = 0;

        for entry in entries {
            let secondary = entry.is_secondary();
            let (row_index, row_count, origin_y) = if secondary {
                let index = secondary_index;
                secondary_index += 1;
                (
                    index,
                    secondary_count,
                    center_y + ICON_SPACING * secondary_expansion,
                )
            } else {
                let index = primary_index;
                primary_index += 1;
                (index, primary_count, center_y)
            };
            let row_center = (row_count.saturating_sub(1)) as f32 * 0.5;
            let row_expansion = if secondary { secondary_expansion } else { 1.0 };
            let origin_x =
                center_x + (row_index as f32 - row_center) * ICON_SPACING * row_expansion;
            let half_size = ICON_SPACING * 0.6; // Add slight hitbox padding
            let rect = Rect::new(
                origin_x - half_size,
                origin_y - half_size,
                origin_x + half_size,
                origin_y + half_size,
            );
            let is_hovered = rect.contains(mouse_pos) && self.interaction.mouse_pressure > 0.0;

            match &entry {
                IconEntry::Star { index } => {
                    if is_hovered {
                        hover_rating_index = Some(
                            index * 2 + 1 + u8::from(mouse_pos.x >= (rect.x0 + rect.x1) * 0.5),
                        );
                    }
                    track.runtime.icon_hitboxes.push(IconHitbox {
                        rect,
                        action: IconAction::Rate(*index),
                    });
                }
                IconEntry::Playlist { playlist, .. } => {
                    track.runtime.icon_hitboxes.push(IconHitbox {
                        rect,
                        action: IconAction::TogglePlaylist(playlist.id),
                    });
                }
            }
            icon_data.push((entry, is_hovered, vec2(origin_x, origin_y)));
        }

        // Sort by distance to mouse for overlap rendering
        icon_data.sort_by(|(_, _, p1), (_, _, p2)| {
            let mouse = vec2(mouse_pos.x, mouse_pos.y);
            let d1 = p1.distance_squared(mouse);
            let d2 = p2.distance_squared(mouse);
            d2.partial_cmp(&d1).unwrap_or(Ordering::Equal)
        });

        let display_rating = hover_rating_index.unwrap_or(track_rating_index);
        let full_stars = display_rating / 2;
        let has_half = display_rating % 2 == 1;

        for (entry, is_hovered, origin) in icon_data {
            let secondary = entry.is_secondary();
            let icon_alpha = fade_alpha * if secondary { secondary_expansion } else { 1.0 };
            let instance = IconInstance {
                pos: origin,
                data: (((icon_alpha * 65535.0) as u32) << 16)
                    | (match entry {
                        IconEntry::Star { index } => {
                            (if index < full_stars {
                                1.0
                            } else if index == full_stars && has_half {
                                0.75
                            } else {
                                0.51
                            } * 65535.0) as u32
                        }
                        IconEntry::Playlist { contained, .. } => {
                            if !contained && !is_hovered {
                                (65535.0 * 0.2) as u32
                            } else {
                                0
                            }
                        }
                    }),
                image_index: match entry {
                    IconEntry::Playlist { playlist, .. } => self.get_image_index(&playlist.art),
                    IconEntry::Star { .. } => 0,
                },
            };
            self.icon_pills.push(instance);
        }

        Some((primary_count as f32, secondary_count as f32))
    }
}

/// Skip to the specified track in the queue.
fn skip_to_track(
    track_id: TrackId,
    position: f32,
    always_seek: bool,
    updater: &AppUpdater,
    client: Arc<spotify::SpotifyClient>,
) {
    updater.send(move |app| {
        let state = &mut app.playback_state;
        let queue_index = state.queue_index;
        let Some(position_in_queue) = state.queue.iter().position(|t| t.id == Some(track_id))
        else {
            error!("Track not found in queue");
            return;
        };
        let song_ms = state.queue[position_in_queue].duration_ms;
        if queue_index != position_in_queue {
            state.queue_index = position_in_queue;
            state.progress = 0;
            state.last_progress_update = Instant::now();
            state.last_interaction = Instant::now() + Duration::from_secs(2);
            let skip_client = Arc::clone(&client);
            spawn(move || {
                let forward = queue_index < position_in_queue;
                let skips = position_in_queue.abs_diff(queue_index);
                for _ in 0..skips.min(10) {
                    let path = if forward {
                        "me/player/next"
                    } else {
                        "me/player/previous"
                    };
                    if let Err(err) = skip_client.api_post(path) {
                        error!("Failed to skip track: {err}");
                    }
                }
            });
        }
        if queue_index == position_in_queue || always_seek {
            let milliseconds = if position < 0.05 {
                0.0
            } else {
                song_ms as f32 * position
            };
            state.progress = milliseconds.round() as u32;
            state.last_progress_update = Instant::now();
            state.last_interaction = Instant::now() + Duration::from_secs(2);
            spawn(move || {
                if let Err(err) = client.api_put(&format!(
                    "me/player/seek?position_ms={}",
                    milliseconds.round()
                )) {
                    error!("Failed to seek track: {err}");
                }
            });
        }
    });
}

fn set_playlist_membership(
    client: &spotify::SpotifyClient,
    playlist_id: PlaylistId,
    playlist_name: &str,
    track_id: TrackId,
    track_uri: &str,
    add: bool,
) {
    let (action, error_action, preposition) = if add {
        ("Adding", "add", "to")
    } else {
        ("Removing", "remove", "from")
    };
    info!("{action} track {track_id} {preposition} playlist {playlist_name}");
    let path = format!("playlists/{playlist_id}/items");
    let result = if add {
        client.api_post_payload(&path, &format!(r#"{{"uris": ["{track_uri}"]}}"#))
    } else {
        client.api_delete_payload(
            &path,
            &format!(r#"{{"items": [{{"uri": "{track_uri}"}}]}}"#),
        )
    };
    if let Err(err) = result {
        error!(
            "Failed to {error_action} track {track_id} {preposition} playlist {playlist_name}: {err}"
        );
    }
}

/// Update Spotify rating playlists for the given track.
impl CantusApp {
    fn update_star_rating(&mut self, track_id: TrackId, rating_slot: u8) {
        if !self.config.ratings_enabled {
            return;
        }

        self.playback_state.last_interaction = Instant::now() + Duration::from_millis(500);
        let changes = self
            .playback_state
            .playlists
            .iter_mut()
            .filter_map(|playlist| {
                let add = playlist.rating_index == Some(rating_slot);
                let changed = playlist.rating_index.is_some()
                    && if add {
                        playlist.tracks.insert(track_id)
                    } else {
                        playlist.tracks.remove(&track_id)
                    };
                changed.then(|| (playlist.id, playlist.name.clone(), add))
            })
            .collect::<Vec<_>>();

        let client = Arc::clone(&self.spotify.client);
        spawn(move || {
            let track_uri = format!("spotify:track:{track_id}");
            for (playlist_id, playlist_name, add) in changes {
                set_playlist_membership(
                    &client,
                    playlist_id,
                    &playlist_name,
                    track_id,
                    &track_uri,
                    add,
                );
            }

            let library_path = format!("me/library/?uris={track_uri}");
            let should_like = rating_slot >= 5;
            match client.api_get(&format!("me/library/contains/?uris={track_uri}")) {
                Ok(liked) if (liked == "[true]") != should_like => {
                    let (action, error_action) = if should_like {
                        ("Adding", "add")
                    } else {
                        ("Removing", "remove")
                    };
                    info!(
                        "{action} track {track_id} {} liked songs",
                        if should_like { "to" } else { "from" }
                    );
                    let result = if should_like {
                        client.api_put(&library_path)
                    } else {
                        client.api_delete(&library_path)
                    };
                    if let Err(err) = result {
                        error!(
                            "Failed to {} track {track_id} in liked songs: {err}",
                            error_action
                        );
                    }
                }
                Err(err) => error!("Failed to check if track {track_id} is already liked: {err}"),
                _ => {}
            }
        });
    }

    fn toggle_playlist_membership(&mut self, track_id: TrackId, playlist_id: PlaylistId) {
        let Some(playlist) = self
            .playback_state
            .playlists
            .iter_mut()
            .find(|playlist| playlist.id == playlist_id)
        else {
            warn!(
                "Playlist {playlist_id} not found while toggling membership for track {track_id}"
            );
            return;
        };
        let playlist_name = playlist.name.clone();
        let add = !playlist.tracks.contains(&track_id);
        if add {
            playlist.tracks.insert(track_id);
        } else {
            playlist.tracks.remove(&track_id);
        }
        self.playback_state.last_interaction = Instant::now() + Duration::from_millis(500);

        let client = Arc::clone(&self.spotify.client);
        spawn(move || {
            let track_uri = format!("spotify:track:{track_id}");
            set_playlist_membership(
                &client,
                playlist_id,
                &playlist_name,
                track_id,
                &track_uri,
                add,
            );
        });
    }

    /// Set Spotify playing or paused.
    fn toggle_playing(&mut self, play: bool) {
        info!("{} current track", if play { "Playing" } else { "Pausing" });
        self.playback_state.playing = play;

        let client = Arc::clone(&self.spotify.client);
        spawn(move || {
            // https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback
            // https://developer.spotify.com/documentation/web-api/reference/#/operations/pause-a-users-playback
            if play {
                if let Err(err) = client.api_put("me/player/play") {
                    error!("Failed to play playback: {err}");
                }
            } else if let Err(err) = client.api_put("me/player/pause") {
                error!("Failed to pause playback: {err}");
            }
        });
    }
}

/// Set the volume of the current playback device.
fn set_volume(volume_percent: u8, client: &spotify::SpotifyClient) {
    info!("Setting volume to {}%", volume_percent);

    // https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback
    if let Err(err) = client.api_put(&format!("me/player/volume?volume_percent={volume_percent}")) {
        error!("Failed to set volume: {err}");
    }
}

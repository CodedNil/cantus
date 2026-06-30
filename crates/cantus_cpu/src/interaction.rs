use crate::{
    CantusApp, CondensedPlaylist, PANEL_START, PlaylistId, TrackId, queue_playback_update,
    render::{IconInstance, Rect, TrackRender, lerpf32},
};
use glam::{Vec2, Vec4, vec2, vec4};
use itertools::Itertools;
#[cfg(feature = "spotify")]
use std::sync::Arc;
use std::{
    collections::HashMap,
    thread::spawn,
    time::{Duration, Instant},
};
use tracing::{error, info, warn};

pub struct IconHitbox {
    pub rect: Rect,
    pub track_id: TrackId,
    pub playlist_id: Option<PlaylistId>,
    pub rating_index: Option<u8>,
}

pub struct InteractionState {
    pub mouse_pressure: f32, // 0 not hovered - 1 hovered - 2 mouse down

    pub track_hitboxes: Vec<(Option<TrackId>, Rect, (f32, f32))>,
    pub icon_hitboxes: Vec<IconHitbox>,

    pub mouse_down: bool,
    pub dragging: bool,
    pub drag_origin: Option<Vec2>,
    pub drag_track: Option<(Option<TrackId>, f32)>,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self {
            mouse_pressure: 0.0,
            track_hitboxes: Vec::new(),
            icon_hitboxes: Vec::new(),
            mouse_down: false,
            dragging: false,
            drag_origin: None,
            drag_track: None,
        }
    }
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
                #[cfg(feature = "spotify")]
                let client = Arc::clone(&self.spotify.client);
                skip_to_track(
                    track_id,
                    position,
                    false,
                    #[cfg(feature = "spotify")]
                    client,
                );
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
        let interaction = &mut self.interaction;
        if let Some(hitbox) = interaction
            .icon_hitboxes
            .iter()
            .find(|h| h.rect.contains(mouse_pos))
        {
            // Spawn particles
            let time = self.start_time.elapsed().as_secs_f32();
            for particle in self
                .particles
                .iter_mut()
                .filter(|p| time > p.end_time)
                .take(20)
            {
                particle.spawn_pos = vec2(mouse_pos.x, mouse_pos.y);
                let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                let speed = 30.0 + (fastrand::f32() * 20.0);
                particle.spawn_vel = vec2(angle.cos() * speed, angle.sin() * speed);
                let duration = lerpf32(fastrand::f32(), 0.5, 1.5);
                particle.color =
                    u32::from_le_bytes([255, 215, 50, (duration * 100.0).min(255.0) as u8]);
                particle.end_time = time + duration;
            }

            let track_id = hitbox.track_id;
            if self.config.ratings_enabled
                && let Some(index) = hitbox.rating_index
            {
                let center_x = (hitbox.rect.x0 + hitbox.rect.x1) * 0.5;
                let rating_slot = index * 2 + u8::from(mouse_pos.x >= center_x);
                self.update_star_rating(track_id, rating_slot);
            } else if let Some(playlist_id) = hitbox.playlist_id {
                self.toggle_playlist_membership(track_id, playlist_id);
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
        } else if let Some((track_id, _, (track_range_a, track_range_b))) = interaction
            .track_hitboxes
            .iter()
            .rev()
            .find(|(_, track_rect, _)| track_rect.contains(mouse_pos))
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
            if let Some(track_id) = *track_id {
                #[cfg(feature = "spotify")]
                let client = Arc::clone(&self.spotify.client);
                skip_to_track(
                    track_id,
                    position,
                    false,
                    #[cfg(feature = "spotify")]
                    client,
                );
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
            #[cfg(feature = "spotify")]
            let client = Arc::clone(&self.spotify.client);
            spawn(move || {
                set_volume(
                    volume,
                    #[cfg(feature = "spotify")]
                    &client,
                );
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

impl CantusApp {
    /// Star ratings and favourite playlists
    pub fn draw_playlist_buttons(
        &mut self,
        track_render: &TrackRender,
        hovered: bool,
        secondary_expansion: f32,
        playlists: &HashMap<PlaylistId, CondensedPlaylist>,
    ) -> Option<Vec4> {
        let track = track_render.track;
        let width = track_render.width;
        let pos_x = track_render.start_x;
        let track_id = track.id?;
        let (track_rating_index, mut icon_entries) = if self.config.ratings_enabled {
            let index = playlists
                .values()
                .find(|p| p.rating_index.is_some() && p.tracks.contains(&track_id))
                .and_then(|p| p.rating_index.map(|r| r + 1))
                .unwrap_or(0);
            (
                index,
                (0..5).map(|index| IconEntry::Star { index }).collect_vec(),
            )
        } else {
            (0, Vec::new())
        };

        // Add playlists that are contained in the favourited playlists
        icon_entries.extend(
            playlists
                .values()
                .filter(|p| p.rating_index.is_none())
                .filter_map(|p| {
                    let contained = p.tracks.contains(&track_id);
                    (contained || secondary_expansion > 0.0).then_some((p, contained))
                })
                .sorted_by(|(a, ac), (b, bc)| bc.cmp(ac).then_with(|| a.name.cmp(&b.name)))
                .map(|(playlist, contained)| IconEntry::Playlist {
                    playlist,
                    contained,
                }),
        );

        // Fade out and fit based on size
        let icon_size = 20.0;
        let mouse_pos = self.global_uniforms.mouse_pos;

        let num_icons = icon_entries.len();
        if num_icons == 0 {
            return None;
        }

        let primary_count = icon_entries
            .iter()
            .filter(|entry| {
                !matches!(
                    entry,
                    IconEntry::Playlist {
                        contained: false,
                        ..
                    }
                )
            })
            .count();
        let secondary_count = num_icons - primary_count;
        let needed_width = icon_size * primary_count as f32 * 0.7;

        let width_fade = if primary_count > 0 {
            ((width - needed_width) / (needed_width * 0.5)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let fade_alpha = if hovered { 1.0 } else { width_fade };
        let background_fade = width_fade.max(secondary_expansion);
        let center_x = pos_x + width * 0.5;
        let center_y = PANEL_START + self.config.height * 0.975;
        let secondary_y = center_y + icon_size;

        let mut hover_rating_index = None;
        let mut icon_data = Vec::with_capacity(num_icons);
        let mut primary_index = 0;
        let mut secondary_index = 0;

        for entry in icon_entries {
            let secondary = matches!(
                &entry,
                IconEntry::Playlist {
                    contained: false,
                    ..
                }
            );
            let (row_index, row_count, origin_y) = if secondary {
                let index = secondary_index;
                secondary_index += 1;
                (
                    index,
                    secondary_count,
                    center_y + (secondary_y - center_y) * secondary_expansion,
                )
            } else {
                let index = primary_index;
                primary_index += 1;
                (index, primary_count, center_y)
            };
            let row_center = (row_count.saturating_sub(1)) as f32 * 0.5;
            let row_expansion = if secondary { secondary_expansion } else { 1.0 };
            let origin_x = center_x + (row_index as f32 - row_center) * icon_size * row_expansion;
            let half_size = icon_size * 0.6; // Add slight hitbox padding
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
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect,
                        track_id,
                        playlist_id: None,
                        rating_index: Some(*index),
                    });
                }
                IconEntry::Playlist { playlist, .. } => {
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect,
                        track_id,
                        playlist_id: Some(playlist.id),
                        rating_index: None,
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
            d2.partial_cmp(&d1).unwrap_or(std::cmp::Ordering::Equal)
        });

        let display_rating = hover_rating_index.unwrap_or(track_rating_index);
        let full_stars = display_rating / 2;
        let has_half = display_rating % 2 == 1;

        for (entry, is_hovered, origin) in icon_data {
            let secondary = matches!(
                &entry,
                IconEntry::Playlist {
                    contained: false,
                    ..
                }
            );
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
                    IconEntry::Playlist {
                        playlist:
                            CondensedPlaylist {
                                image_url: Some(url),
                                ..
                            },
                        ..
                    } => self.get_image_index(url),
                    _ => 0,
                },
            };
            self.icon_pills.push(instance);
        }

        let primary_half_span = primary_count.saturating_sub(1) as f32 * icon_size * 0.5;
        let secondary_half_span = secondary_count.saturating_sub(1) as f32 * icon_size * 0.5;
        Some(vec4(
            if primary_count > 0 {
                primary_half_span + 1.0
            } else {
                0.0
            },
            secondary_half_span,
            if secondary_count > 0 {
                secondary_expansion
            } else {
                0.0
            },
            background_fade * 0.5,
        ))
    }
}

/// Skip to the specified track in the queue.
fn skip_to_track(
    track_id: TrackId,
    position: f32,
    always_seek: bool,
    #[cfg(feature = "spotify")] client: Arc<crate::spotify::SpotifyClient>,
) {
    queue_playback_update(move |state| {
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
            #[cfg(feature = "spotify")]
            let skip_client = Arc::clone(&client);
            #[cfg(feature = "spotify")]
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
            #[cfg(feature = "spotify")]
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

/// Update Spotify rating playlists for the given track.
impl CantusApp {
    fn update_star_rating(&mut self, track_id: TrackId, rating_slot: u8) {
        if !self.config.ratings_enabled {
            return;
        }

        #[cfg(feature = "spotify")]
        let mut playlists_to_remove_from = Vec::new();
        #[cfg(feature = "spotify")]
        let mut playlists_to_add_to = Vec::new();

        // Remove tracks from existing playlists, add to target playlist if not present
        {
            let state = &mut self.playback_state;
            state.last_interaction = Instant::now() + Duration::from_millis(500);
            state.playlists.values_mut().for_each(|playlist| {
                if playlist.rating_index.is_some()
                    && playlist.rating_index != Some(rating_slot)
                    && playlist.tracks.remove(&track_id)
                {
                    #[cfg(feature = "spotify")]
                    playlists_to_remove_from.push((playlist.id, playlist.name.clone()));
                }
                if playlist.rating_index == Some(rating_slot) && playlist.tracks.insert(track_id) {
                    #[cfg(feature = "spotify")]
                    playlists_to_add_to.push((playlist.id, playlist.name.clone()));
                }
            });
        }

        #[cfg(feature = "spotify")]
        let client = Arc::clone(&self.spotify.client);
        #[cfg(feature = "spotify")]
        spawn(move || {
            let track_uri = format!("spotify:track:{track_id}");
            for (playlist_id, playlist_name) in playlists_to_remove_from {
                info!("Removing track {track_id} from rating playlist {playlist_name}");
                // https://developer.spotify.com/documentation/web-api/reference/remove-items-playlist
                if let Err(err) = client.api_delete_payload(
                    &format!("playlists/{playlist_id}/items"),
                    &format!(r#"{{"items": [{{"uri": "{track_uri}"}}]}}"#),
                ) {
                    error!(
                        "Failed to remove track {track_id} from rating playlist {playlist_name}: {err}"
                    );
                }
            }
            for (playlist_id, playlist_name) in playlists_to_add_to {
                info!("Adding track {track_id} to rating playlist {playlist_name}");
                // https://developer.spotify.com/documentation/web-api/reference/add-items-to-playlist
                if let Err(err) = client.api_post_payload(
                    &format!("playlists/{playlist_id}/items"),
                    &format!(r#"{{"uris": ["{track_uri}"]}}"#),
                ) {
                    error!(
                        "Failed to add track {track_id} to rating playlist {playlist_name}: {err}"
                    );
                }
            }

            // Add the track the liked songs if its rated above 3 stars
            // https://developer.spotify.com/documentation/web-api/reference/check-library-contains
            match client.api_get(&format!("me/library/contains/?uris={track_uri}")) {
                Ok(already_liked) => match (already_liked == "[true]", rating_slot >= 5) {
                    (true, false) => {
                        info!("Removing track {track_id} from liked songs");
                        // https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user
                        if let Err(err) =
                            client.api_delete(&format!("me/library/?uris={track_uri}"))
                        {
                            error!("Failed to remove track {track_id} from liked songs: {err}");
                        }
                    }
                    (false, true) => {
                        info!("Adding track {track_id} to liked songs");
                        // https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user
                        if let Err(err) = client.api_put(&format!("me/library/?uris={track_uri}")) {
                            error!("Failed to add track {track_id} to liked songs: {err}");
                        }
                    }
                    _ => {}
                },
                Err(err) => {
                    error!("Failed to check if track {track_id} is already liked: {err}");
                }
            }
        });
    }

    fn toggle_playlist_membership(&mut self, track_id: TrackId, playlist_id: PlaylistId) {
        let (playlist_name, contained) = {
            let Some(playlist) = self.playback_state.playlists.get(&playlist_id) else {
                warn!(
                    "Playlist {playlist_id} not found while toggling membership for track {track_id}"
                );
                return;
            };
            (playlist.name.clone(), playlist.tracks.contains(&track_id))
        };

        info!(
            "{} track {track_id} {} playlist {playlist_name}",
            if contained { "Removing" } else { "Adding" },
            if contained { "from" } else { "to" }
        );

        {
            let state = &mut self.playback_state;
            let playlist_tracks = &mut state.playlists.get_mut(&playlist_id).unwrap().tracks;
            if contained {
                playlist_tracks.remove(&track_id);
            } else {
                playlist_tracks.insert(track_id);
            }
            state.last_interaction = Instant::now() + Duration::from_millis(500);
        }

        #[cfg(feature = "spotify")]
        let client = Arc::clone(&self.spotify.client);
        #[cfg(feature = "spotify")]
        spawn(move || {
            let track_uri = format!("spotify:track:{track_id}");
            let result = if contained {
                // https://developer.spotify.com/documentation/web-api/reference/remove-items-playlist
                client.api_delete_payload(
                    &format!("playlists/{playlist_id}/items"),
                    &format!(r#"{{"tracks": [ {{"uri": "{track_uri}"}} ]}}"#),
                )
            } else {
                // https://developer.spotify.com/documentation/web-api/reference/add-items-to-playlist
                client.api_post_payload(
                    &format!("playlists/{playlist_id}/items"),
                    &format!(r#"{{"uris": ["{track_uri}"]}}"#),
                )
            };
            if let Err(err) = result {
                error!(
                    "Failed to {} track {track_id} {} playlist {playlist_name}: {err}",
                    if contained { "remove" } else { "add" },
                    if contained { "from" } else { "to" }
                );
            }
        });
    }

    /// Set Spotify playing or paused.
    fn toggle_playing(&mut self, play: bool) {
        info!("{} current track", if play { "Playing" } else { "Pausing" });
        self.playback_state.playing = play;

        #[cfg(feature = "spotify")]
        let client = Arc::clone(&self.spotify.client);
        #[cfg(feature = "spotify")]
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
fn set_volume(
    volume_percent: u8,
    #[cfg(feature = "spotify")] client: &crate::spotify::SpotifyClient,
) {
    info!("Setting volume to {}%", volume_percent);

    #[cfg(feature = "spotify")]
    // https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback
    if let Err(err) = client.api_put(&format!("me/player/volume?volume_percent={volume_percent}")) {
        error!("Failed to set volume: {err}");
    }
}

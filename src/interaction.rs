use crate::{
    CantusApp, CondensedPlaylist, PANEL_START, PLAYBACK_STATE, PlaylistId, Track, TrackId,
    config::CONFIG,
    render::{IconInstance, Point, Rect, lerpf32},
    update_playback_state,
};
use itertools::Itertools;
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
    pub mouse_position: Point,
    pub mouse_pressure: f32, // 0 not hovered - 1 hovered - 2 mouse down

    pub last_hitbox_hash: u64,
    pub play_hitbox: Rect,
    pub track_hitboxes: Vec<(TrackId, Rect, (f32, f32))>,
    pub icon_hitboxes: Vec<IconHitbox>,

    pub mouse_down: bool,
    pub dragging: bool,
    pub drag_origin: Option<Point>,
    pub drag_track: Option<(TrackId, f32)>,

    // Playhead
    pub last_expansion: (Instant, Point),
    pub last_toggle_playing: Instant,
    pub playing: bool,
    pub playhead_bar: f32,
    pub playhead_play: f32,
    pub playhead_pause: f32,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self {
            mouse_position: Point::default(),
            mouse_pressure: 0.0,
            last_hitbox_hash: 0,
            play_hitbox: Rect::default(),
            track_hitboxes: Vec::new(),
            icon_hitboxes: Vec::new(),
            mouse_down: false,
            dragging: false,
            drag_origin: None,
            drag_track: None,
            last_expansion: (
                Instant::now().checked_sub(Duration::from_secs(5)).unwrap(),
                Point::default(),
            ),
            last_toggle_playing: Instant::now(),
            playing: false,
            playhead_bar: 0.0,
            playhead_play: 0.0,
            playhead_pause: 0.0,
        }
    }
}

impl CantusApp {
    pub fn left_click(&mut self) {
        let interaction = &mut self.interaction;
        interaction.mouse_down = true;
        interaction.mouse_pressure = 2.0;
        interaction.drag_origin = Some(interaction.mouse_position);
        interaction.drag_track = None;
        interaction.dragging = false;
        PLAYBACK_STATE.write().interaction = false;
    }

    pub fn left_click_released(&mut self) {
        if !self.interaction.dragging && self.interaction.mouse_down {
            self.handle_click();
        }
        let interaction = &mut self.interaction;
        if let Some((track_id, position)) = interaction.drag_track.take() {
            // Get the x position of the playhead, run an expansion animation there
            interaction.last_expansion = (
                Instant::now(),
                Point::new(CONFIG.playhead_x(), PANEL_START + CONFIG.height * 0.5),
            );
            spawn(move || {
                skip_to_track(&track_id, position, false);
            });
        }
        interaction.drag_origin = None;
        interaction.dragging = false;
        interaction.mouse_down = false;
        interaction.mouse_pressure = 1.0;
        PLAYBACK_STATE.write().interaction = false;
    }

    pub fn right_click(&mut self) {
        self.cancel_drag();
        self.interaction.mouse_down = false;
    }

    /// Handle click events.
    fn handle_click(&mut self) {
        let mouse_pos = self.interaction.mouse_position;
        let (playing, interaction) = {
            let state = PLAYBACK_STATE.read();
            (state.playing, state.interaction)
        };
        if interaction {
            return;
        }
        PLAYBACK_STATE.write().interaction = true;

        // Click on rating/playlist icons
        let interaction = &mut self.interaction;
        if let Some(hitbox) = interaction
            .icon_hitboxes
            .iter()
            .find(|h| h.rect.contains(mouse_pos))
        {
            // Spawn particles
            let time = self.start_time.elapsed().as_secs_f32();
            let mut emit_count = 20;
            for particle in &mut self.particles {
                if emit_count > 0 && time > particle.end_time {
                    particle.spawn_pos = [mouse_pos.x, mouse_pos.y];
                    let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                    let speed = 30.0 + (fastrand::f32() * 20.0);
                    particle.spawn_vel = [angle.cos() * speed, angle.sin() * speed];
                    let duration = lerpf32(fastrand::f32(), 0.5, 1.5);
                    particle.color =
                        u32::from_le_bytes([255, 215, 50, (duration * 100.0).min(255.0) as u8]);
                    particle.end_time = time + duration;
                    emit_count -= 1;
                }
            }

            let track_id = hitbox.track_id;
            if CONFIG.ratings_enabled
                && let Some(index) = hitbox.rating_index
            {
                let center_x = (hitbox.rect.x0 + hitbox.rect.x1) * 0.5;
                let rating_slot = index * 2 + u8::from(mouse_pos.x >= center_x);
                spawn(move || {
                    update_star_rating(&track_id, rating_slot);
                });
            } else if let Some(playlist_id) = hitbox.playlist_id {
                spawn(move || {
                    toggle_playlist_membership(&track_id, &playlist_id);
                });
            }
        } else if interaction.play_hitbox.contains(mouse_pos) {
            // Play/pause
            interaction.last_expansion = (
                Instant::now(),
                Point::new(CONFIG.playhead_x(), PANEL_START + CONFIG.height * 0.5),
            );
            interaction.last_toggle_playing = Instant::now();
            spawn(move || {
                toggle_playing(!playing);
            });
        } else if let Some((track_id, _, (track_range_a, track_range_b))) = interaction
            .track_hitboxes
            .iter()
            .rev()
            .find(|(_, track_rect, _)| track_rect.contains(mouse_pos))
        {
            // Seek track
            interaction.last_expansion = (Instant::now(), mouse_pos);

            // If click is near the very left, reset to the start of the song, else seek to clicked position
            let position = if mouse_pos.x < CONFIG.history_width + 40.0 {
                0.0
            } else {
                (mouse_pos.x - track_range_a) / (track_range_b - track_range_a)
            };
            let track_id = *track_id;
            spawn(move || {
                skip_to_track(&track_id, position, false);
            });
        }
        PLAYBACK_STATE.write().interaction = false;
    }

    /// Drag across the progress bar to seek.
    pub fn handle_mouse_drag(&mut self) {
        let interaction = &mut self.interaction;
        if let Some(origin_pos) = interaction.drag_origin {
            let delta_x = interaction.mouse_position.x - origin_pos.x;
            let delta_y = interaction.mouse_position.y - origin_pos.y;
            if !interaction.dragging && (delta_x.abs() >= 2.0 || delta_y.abs() >= 2.0) {
                interaction.dragging = true;
                PLAYBACK_STATE.write().interaction = true;
            }
        }
    }

    /// Handle scrolling events to adjust volume.
    pub fn handle_scroll(delta: i32) {
        let scroll_direction = delta.signum();
        if scroll_direction == 0 {
            return;
        }
        update_playback_state(|state| {
            if let Some(volume) = &mut state.volume {
                *volume = if scroll_direction < 0 {
                    volume.saturating_add(5).min(100)
                } else {
                    volume.saturating_sub(5)
                };
                let volume = *volume;
                spawn(move || {
                    set_volume(volume);
                });
            }
        });
    }

    pub fn cancel_drag(&mut self) {
        let interaction = &mut self.interaction;
        interaction.drag_track = None;
        interaction.drag_origin = None;
        interaction.dragging = false;
        PLAYBACK_STATE.write().interaction = false;
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
        track: &Track,
        hovered: bool,
        playlists: &HashMap<PlaylistId, CondensedPlaylist>,
        width: f32,
        pos_x: f32,
    ) {
        let (track_rating_index, mut icon_entries) = if CONFIG.ratings_enabled {
            let index = playlists
                .values()
                .find(|p| p.rating_index.is_some() && p.tracks.contains(&track.id))
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
                    let contained = p.tracks.contains(&track.id);
                    (contained || hovered).then_some((p, contained))
                })
                .sorted_by(|(a, ac), (b, bc)| bc.cmp(ac).then_with(|| a.name.cmp(&b.name)))
                .map(|(playlist, contained)| IconEntry::Playlist {
                    playlist,
                    contained,
                }),
        );

        // Fade out and fit based on size
        let icon_size = 20.0;
        let mouse_pos = self.interaction.mouse_position;

        if width < icon_size * icon_entries.len() as f32 {
            // Strip out all playlists that arent contained
            icon_entries.retain(|entry| {
                if let IconEntry::Playlist { contained, .. } = entry {
                    *contained
                } else {
                    true
                }
            });
        }

        let num_icons = icon_entries.len();
        let needed_width = icon_size * num_icons as f32;
        if num_icons == 0 {
            return;
        }

        let fade_alpha = if hovered {
            1.0
        } else {
            ((width - needed_width) / (needed_width * 0.25)).clamp(0.0, 1.0)
        };
        let center_x = pos_x + width * 0.5;
        let center_y = PANEL_START + CONFIG.height * 0.975;

        // Count only the standard icons for spacing
        let half_icons = icon_entries
            .iter()
            .filter(|entry| {
                if let IconEntry::Playlist { contained, .. } = entry {
                    *contained
                } else {
                    true
                }
            })
            .count() as f32
            / 2.0;

        let mut hover_rating_index = None;
        let mut icon_data = Vec::with_capacity(num_icons);

        for (i, entry) in icon_entries.into_iter().enumerate() {
            let origin_x = center_x + (i as f32 - half_icons) * icon_size;
            let half_size = icon_size * 0.6; // Add slight hitbox padding
            let rect = Rect::new(
                origin_x - half_size,
                center_y - half_size,
                origin_x + half_size,
                center_y + half_size,
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
                        track_id: track.id,
                        playlist_id: None,
                        rating_index: Some(*index),
                    });
                }
                IconEntry::Playlist { playlist, .. } => {
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect,
                        track_id: track.id,
                        playlist_id: Some(playlist.id),
                        rating_index: None,
                    });
                }
            }
            icon_data.push((entry, is_hovered, origin_x));
        }

        // Sort by distance to mouse for overlap rendering
        icon_data.sort_by(|(_, _, x1), (_, _, x2)| {
            let d1 = (x1 - mouse_pos.x).powi(2);
            let d2 = (x2 - mouse_pos.x).powi(2);
            d2.partial_cmp(&d1).unwrap_or(std::cmp::Ordering::Equal)
        });

        let display_rating = hover_rating_index.unwrap_or(track_rating_index);
        let full_stars = display_rating / 2;
        let has_half = display_rating % 2 == 1;

        for (entry, is_hovered, origin_x) in icon_data {
            let instance = IconInstance {
                pos: [origin_x, center_y],
                data: (((fade_alpha * 65535.0) as u32) << 16)
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
                        IconEntry::Playlist {
                            playlist: _playlist,
                            contained,
                        } => {
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
                        contained: _contained,
                    } => self.get_image_index(url),
                    _ => 0,
                },
            };
            self.icon_pills.push(instance);
        }
    }
}

/// Skip to the specified track in the queue.
fn skip_to_track(track_id: &TrackId, position: f32, always_seek: bool) {
    let (queue_index, position_in_queue, ms_lookup) = {
        let state = PLAYBACK_STATE.read();
        let queue_index = state.queue_index;
        let Some(position_in_queue) = state.queue.iter().position(|t| &t.id == track_id) else {
            error!("Track not found in queue");
            return;
        };
        let ms_lookup = state
            .queue
            .iter()
            .map(|playlist| playlist.duration_ms)
            .collect::<Vec<_>>();
        drop(state);
        (queue_index, position_in_queue, ms_lookup)
    };
    // Skip or rewind to the track
    if queue_index != position_in_queue {
        update_playback_state(|state| {
            state.queue_index = position_in_queue;
            state.progress = 0;
            state.last_progress_update = Instant::now();
            state.last_interaction = Instant::now() + Duration::from_millis(2000);
        });
        let forward = queue_index < position_in_queue;
        let skips = if forward {
            position_in_queue - queue_index
        } else {
            queue_index - position_in_queue
        };
        info!(
            "{} to track {track_id}, {skips} skips",
            if forward { "Skipping" } else { "Rewinding" }
        );
        #[cfg(feature = "spotify")]
        for _ in 0..skips.min(10) {
            let result = if forward {
                // https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-next-track
                crate::spotify::SPOTIFY_CLIENT.api_post("me/player/next")
            } else {
                // https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-previous-track
                crate::spotify::SPOTIFY_CLIENT.api_post("me/player/previous")
            };
            if let Err(err) = result {
                error!("Failed to skip to track: {err}");
            }
        }
    }
    // Seek to the position
    if queue_index == position_in_queue || always_seek {
        let song_ms = ms_lookup[position_in_queue];
        let milliseconds = if position < 0.05 {
            0.0
        } else {
            song_ms as f32 * position
        };
        info!(
            "Seeking track {track_id} to {}%",
            (milliseconds / song_ms as f32 * 100.0).round()
        );
        update_playback_state(|state| {
            state.progress = milliseconds.round() as u32;
            state.last_progress_update = Instant::now();
            state.last_interaction = Instant::now() + Duration::from_millis(2000);
        });

        #[cfg(feature = "spotify")]
        {
            // https://developer.spotify.com/documentation/web-api/reference/#/operations/seek-to-position-in-currently-playing-track
            if let Err(err) = crate::spotify::SPOTIFY_CLIENT.api_put(&format!(
                "me/player/seek?position_ms={}",
                milliseconds.round()
            )) {
                error!("Failed to seek track: {err}");
            }
        }
    }
}

/// Update Spotify rating playlists for the given track.
fn update_star_rating(track_id: &TrackId, rating_slot: u8) {
    if !CONFIG.ratings_enabled {
        return;
    }

    #[cfg(feature = "spotify")]
    let mut playlists_to_remove_from = Vec::new();
    #[cfg(feature = "spotify")]
    let mut playlists_to_add_to = Vec::new();

    // Remove tracks from existing playlists, add to target playlist if not present
    update_playback_state(|state| {
        state.last_interaction = Instant::now() + Duration::from_millis(500);
        state.playlists.values_mut().for_each(|playlist| {
            if playlist.rating_index.is_some()
                && playlist.rating_index != Some(rating_slot)
                && playlist.tracks.remove(track_id)
            {
                #[cfg(feature = "spotify")]
                playlists_to_remove_from.push((playlist.id, playlist.name.clone()));
            }
            if playlist.rating_index == Some(rating_slot) && playlist.tracks.insert(*track_id) {
                #[cfg(feature = "spotify")]
                playlists_to_add_to.push((playlist.id, playlist.name.clone()));
            }
        });
    });

    #[cfg(feature = "spotify")]
    {
        // Make the changes
        for (playlist_id, playlist_name) in playlists_to_remove_from {
            info!("Removing track {track_id} from rating playlist {playlist_name}");
            let track_uri = format!("spotify:track:{track_id}");
            // https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist
            if let Err(err) = crate::spotify::SPOTIFY_CLIENT.api_delete_payload(
                &format!("playlists/{playlist_id}/tracks"),
                &format!(r#"{{"tracks": [ {{"uri": "{track_uri}"}} ]}}"#),
            ) {
                error!(
                    "Failed to remove track {track_id} from rating playlist {playlist_name}: {err}"
                );
            }
        }
        for (playlist_id, playlist_name) in playlists_to_add_to {
            info!("Adding track {track_id} to rating playlist {playlist_name}");
            let track_uri = format!("spotify:track:{track_id}");
            // https://developer.spotify.com/documentation/web-api/reference/#/operations/add-tracks-to-playlist)
            if let Err(err) = crate::spotify::SPOTIFY_CLIENT.api_post_payload(
                &format!("playlists/{playlist_id}/tracks"),
                &format!(r#"{{"uris": ["{track_uri}"]}}"#),
            ) {
                error!("Failed to add track {track_id} to rating playlist {playlist_name}: {err}");
            }
        }

        // Add the track the liked songs if its rated above 3 stars
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks
        match crate::spotify::SPOTIFY_CLIENT.api_get(&format!("me/tracks/contains/?ids={track_id}"))
        {
            Ok(already_liked) => match (already_liked == "[true]", rating_slot >= 5) {
                (true, false) => {
                    info!("Removing track {track_id} from liked songs");
                    // https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user
                    if let Err(err) = crate::spotify::SPOTIFY_CLIENT
                        .api_delete(&format!("me/tracks/?ids={track_id}"))
                    {
                        error!("Failed to remove track {track_id} from liked songs: {err}");
                    }
                }
                (false, true) => {
                    info!("Adding track {track_id} to liked songs");
                    // https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user
                    if let Err(err) = crate::spotify::SPOTIFY_CLIENT
                        .api_put(&format!("me/tracks/?ids={track_id}"))
                    {
                        error!("Failed to add track {track_id} to liked songs: {err}");
                    }
                }
                _ => {}
            },
            Err(err) => {
                error!("Failed to check if track {track_id} is already liked: {err}");
            }
        }
    }
}

/// Toggle Spotify playlist membership for the given track.
fn toggle_playlist_membership(track_id: &TrackId, playlist_id: &PlaylistId) {
    let Some((playlist_id, playlist_name, contained)) = PLAYBACK_STATE
        .read()
        .playlists
        .iter()
        .find(|(_, p)| &p.id == playlist_id)
        .map(|(key, playlist)| {
            (
                *key,
                playlist.name.clone(),
                playlist.tracks.contains(track_id),
            )
        })
    else {
        warn!("Playlist {playlist_id} not found while toggling membership for track {track_id}");
        return;
    };

    info!(
        "{} track {track_id} {} playlist {playlist_name}",
        if contained { "Removing" } else { "Adding" },
        if contained { "from" } else { "to" }
    );

    update_playback_state(|state| {
        let playlist_tracks = &mut state.playlists.get_mut(&playlist_id).unwrap().tracks;
        if contained {
            playlist_tracks.remove(track_id);
        } else {
            playlist_tracks.insert(*track_id);
        }
        state.last_interaction = Instant::now() + Duration::from_millis(500);
    });

    #[cfg(feature = "spotify")]
    {
        let track_uri = format!("spotify:track:{track_id}");
        let result = if contained {
            crate::spotify::SPOTIFY_CLIENT.api_delete_payload(
                &format!("playlists/{playlist_id}/tracks"),
                &format!(r#"{{"tracks": [ {{"uri": "{track_uri}"}} ]}}"#),
            )
        } else {
            crate::spotify::SPOTIFY_CLIENT.api_post_payload(
                &format!("playlists/{playlist_id}/tracks"),
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
    }
}

/// Set Spotify playing or paused.
fn toggle_playing(play: bool) {
    info!("{} current track", if play { "Playing" } else { "Pausing" });
    update_playback_state(|state| {
        state.playing = play;
    });

    #[cfg(feature = "spotify")]
    {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/pause-a-users-playback
        if play {
            if let Err(err) = crate::spotify::SPOTIFY_CLIENT.api_put("me/player/play") {
                error!("Failed to play playback: {err}");
            }
        } else if let Err(err) = crate::spotify::SPOTIFY_CLIENT.api_put("me/player/pause") {
            error!("Failed to pause playback: {err}");
        }
    }
}

/// Set the volume of the current playback device.
fn set_volume(volume_percent: u8) {
    info!("Setting volume to {}%", volume_percent);

    #[cfg(feature = "spotify")]
    {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback
        if let Err(err) = crate::spotify::SPOTIFY_CLIENT
            .api_put(&format!("me/player/volume?volume_percent={volume_percent}"))
        {
            error!("Failed to set volume: {err}");
        }
    }
}

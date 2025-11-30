use crate::{
    CantusApp,
    config::CONFIG,
    rspotify::{PlaylistId, Track, TrackId},
    spotify::{
        CondensedPlaylist, IMAGES_CACHE, PLAYBACK_STATE, RATING_PLAYLISTS, SPOTIFY_CLIENT,
        update_playback_state,
    },
};
use itertools::Itertools;
use std::time::Duration;
use std::{cmp::Ordering, collections::HashMap, sync::LazyLock, thread::spawn, time::Instant};
use tracing::{error, info, warn};
use vello::{
    kurbo::{Affine, BezPath, Point, Rect, RoundedRect, Shape, Stroke},
    peniko::{Color, Fill, ImageBrush},
};

pub struct IconHitbox {
    pub rect: Rect,
    pub track_id: TrackId,
    pub playlist_id: Option<PlaylistId>,
    pub rating_index: Option<u8>,
}

pub struct InteractionState {
    pub last_event: InteractionEvent,
    pub last_click: Option<(Instant, TrackId, Point)>,
    pub mouse_position: Point,
    #[cfg(feature = "wayland")]
    pub last_hitbox_update: Instant,
    pub play_hitbox: Rect,
    pub track_hitboxes: Vec<(TrackId, Rect, (f64, f64))>,
    pub icon_hitboxes: Vec<IconHitbox>,
    pub mouse_down: bool,
    pub drag_origin: Option<Point>,
    pub drag_track: Option<(TrackId, f64)>,
    pub dragging: bool,
    pub drag_delta_pixels: f64,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self {
            last_event: InteractionEvent::Pause(
                Instant::now().checked_sub(Duration::from_secs(5)).unwrap(),
            ),
            last_click: None,
            mouse_position: Point::default(),
            #[cfg(feature = "wayland")]
            last_hitbox_update: Instant::now(),
            play_hitbox: Rect::default(),
            track_hitboxes: Vec::new(),
            icon_hitboxes: Vec::new(),
            mouse_down: false,
            drag_origin: None,
            drag_track: None,
            dragging: false,
            drag_delta_pixels: 0.0,
        }
    }
}

impl InteractionState {
    pub fn left_click(&mut self) {
        self.mouse_down = true;
        self.drag_origin = Some(self.mouse_position);
        self.drag_track = None;
        self.dragging = false;
        self.drag_delta_pixels = 0.0;
        PLAYBACK_STATE.write().interaction = false;
    }

    pub fn left_click_released(&mut self, scale_factor: f64) {
        if !self.dragging && self.mouse_down {
            self.handle_click(scale_factor);
        }
        if let Some((track_id, position)) = self.drag_track.take() {
            spawn(move || {
                skip_to_track(&track_id, position, false);
            });
        }
        self.drag_origin = None;
        self.dragging = false;
        self.drag_delta_pixels = 0.0;
        self.mouse_down = false;
        PLAYBACK_STATE.write().interaction = false;
    }

    pub fn right_click(&mut self) {
        self.cancel_drag();
        self.mouse_down = false;
    }

    /// Handle click events.
    fn handle_click(&mut self, scale_factor: f64) {
        let mouse_pos = self.mouse_position;
        let (playing, interaction) = {
            let state = PLAYBACK_STATE.read();
            (state.playing, state.interaction)
        };
        if interaction {
            return;
        }
        PLAYBACK_STATE.write().interaction = true;

        // Click on rating/playlist icons
        if let Some(hitbox) = self
            .icon_hitboxes
            .iter()
            .find(|h| h.rect.contains(mouse_pos))
        {
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
        } else if self.play_hitbox.contains(mouse_pos) {
            // Play/pause
            if let Some((track_id, track_rect, _)) = self
                .track_hitboxes
                .iter()
                .find(|(_, track_rect, _)| track_rect.contains(mouse_pos))
            {
                self.last_click = Some((
                    Instant::now(),
                    *track_id,
                    Point::new(mouse_pos.x - track_rect.x0, mouse_pos.y - track_rect.y0),
                ));
            }
            if playing {
                self.last_event = InteractionEvent::Pause(Instant::now());
            } else {
                self.last_event = InteractionEvent::Play(Instant::now());
            }
            spawn(move || {
                toggle_playing(!playing);
            });
        } else if let Some((track_id, track_rect, (track_range_a, track_range_b))) = self
            .track_hitboxes
            .iter()
            .rev()
            .find(|(_, track_rect, _)| track_rect.contains(mouse_pos))
        {
            // Seek track
            self.last_click = Some((
                Instant::now(),
                *track_id,
                Point::new(mouse_pos.x - track_rect.x0, mouse_pos.y - track_rect.y0),
            ));

            // If click is near the very left, reset to the start of the song, else seek to clicked position
            let position = if mouse_pos.x < (CONFIG.history_width + 20.0) * scale_factor {
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
        if let Some(origin_pos) = self.drag_origin {
            let delta_x = self.mouse_position.x - origin_pos.x;
            let delta_y = self.mouse_position.y - origin_pos.y;
            if !self.dragging && (delta_x.abs() >= 2.0 || delta_y.abs() >= 2.0) {
                self.dragging = true;
                PLAYBACK_STATE.write().interaction = true;
            }
            self.drag_delta_pixels = if self.dragging { delta_x } else { 0.0 };
        } else {
            self.drag_delta_pixels = 0.0;
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
        self.drag_track = None;
        self.drag_origin = None;
        self.dragging = false;
        self.drag_delta_pixels = 0.0;
        PLAYBACK_STATE.write().interaction = false;
    }
}

#[derive(PartialEq, Eq)]
pub enum InteractionEvent {
    Pause(Instant),
    Play(Instant),
    PauseHover(Instant),
    PlayHover(Instant),
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

/// Star images
static STAR_SVG: LazyLock<BezPath> =
    LazyLock::new(|| BezPath::from_svg(include_str!("../assets/star.path")).unwrap());
static STAR_SVG_HALF: LazyLock<BezPath> =
    LazyLock::new(|| BezPath::from_svg(include_str!("../assets/star-half.path")).unwrap());

impl CantusApp {
    /// Star ratings and favourite playlists
    pub fn draw_playlist_buttons(
        &mut self,
        track: &Track,
        hovered: bool,
        playlists: &HashMap<PlaylistId, CondensedPlaylist>,
        width: f64,
        height: f64,
        pos_x: f64,
    ) {
        let track_rating_index = if CONFIG.ratings_enabled {
            playlists
                .iter()
                .find(|(_, playlist)| {
                    playlist.rating_index.is_some() && playlist.tracks.contains(&track.id)
                })
                .map_or(0, |(_, playlist)| {
                    playlist.rating_index.map_or(0, |rating| rating + 1)
                })
        } else {
            0
        };

        let mut icon_entries = if CONFIG.ratings_enabled {
            (0..5)
                .map(|index| IconEntry::Star { index })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        // Add playlists that are contained in the favourited playlists
        icon_entries.extend(
            playlists
                .values()
                .filter_map(|playlist| {
                    if CONFIG.ratings_enabled && RATING_PLAYLISTS.contains(&playlist.name.as_str())
                    {
                        return None;
                    }
                    let contained = playlist.tracks.contains(&track.id);
                    if !contained && !hovered {
                        return None;
                    }
                    Some((playlist, contained))
                })
                .sorted_by(|(a, ac), (b, bc)| match bc.cmp(ac) {
                    Ordering::Equal => a.name.cmp(&b.name),
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                })
                .map(|(playlist, contained)| IconEntry::Playlist {
                    playlist,
                    contained,
                }),
        );

        // Fade out when there's not enough space
        let icon_size = 14.0 * self.scale_factor;
        let icon_spacing = 2.0 * self.scale_factor;
        let mouse_pos = self.interaction.mouse_position;

        let needed_width = icon_size * icon_entries.len() as f64;
        if width < needed_width {
            // Strip out all playlists that arent contained
            icon_entries.retain(|entry| {
                if let IconEntry::Playlist { contained, .. } = entry {
                    *contained
                } else {
                    true
                }
            });
        }
        // Try fitting again
        let num_icons = icon_entries.len();
        if num_icons == 0 {
            return;
        }
        let needed_width = icon_size * num_icons as f64;
        if width < needed_width {
            return;
        }

        let fade_alpha = if hovered {
            1.0
        } else {
            ((width - needed_width) / (needed_width * 0.25)).clamp(0.0, 1.0) as f32
        };

        let center_x = pos_x + width * 0.5;
        let center_y = height * 0.975;

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
            .count() as f64
            / 2.0;

        // Track hovers, and add hitboxes
        let mut hover_rating_index = None;
        let mut icon_entry_extras = Vec::new();
        for (i, entry) in icon_entries.iter().enumerate() {
            let icon_origin_x = center_x + (i as f64 - half_icons) * (icon_size + icon_spacing);
            let half_icon_size = (icon_size + icon_spacing) * 0.5; // Include some spacing so the hitboxes don't have gaps
            let button_rect = Rect::new(
                icon_origin_x - half_icon_size,
                center_y - half_icon_size,
                icon_origin_x + half_icon_size,
                center_y + half_icon_size,
            );
            let hovered = button_rect.contains(mouse_pos);
            icon_entry_extras.push((hovered, icon_origin_x));
            match entry {
                IconEntry::Star { index } => {
                    if hovered {
                        let rect_center_x = (button_rect.x0 + button_rect.x1) * 0.5;
                        hover_rating_index =
                            Some(*index * 2 + 1 + u8::from(mouse_pos.x >= rect_center_x));
                    }
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect: button_rect,
                        track_id: track.id,
                        playlist_id: None,
                        rating_index: Some(*index),
                    });
                }
                IconEntry::Playlist {
                    playlist,
                    contained: _,
                } => {
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect: button_rect,
                        track_id: track.id,
                        playlist_id: Some(playlist.id),
                        rating_index: None,
                    });
                }
            }
        }

        // Render out the icons
        let display_rating_index = hover_rating_index.unwrap_or(track_rating_index);
        let display_full_stars = display_rating_index / 2;
        let display_has_half = display_rating_index % 2 == 1;
        for (entry, (hovered, icon_origin_x)) in
            icon_entries.into_iter().zip(icon_entry_extras.into_iter())
        {
            let icon_size = icon_size * if hovered { 1.6 } else { 1.0 };
            let half_icon_size = icon_size * 0.5;
            let icon_transform =
                Affine::translate((icon_origin_x - half_icon_size, center_y - half_icon_size));

            match entry {
                IconEntry::Star { index } => {
                    let fill_transform =
                        icon_transform * Affine::scale(icon_size / STAR_SVG.bounding_box().width());

                    // Shadow outline
                    self.scene.stroke(
                        &Stroke::new(2.0 * self.scale_factor),
                        fill_transform,
                        Color::from_rgb8(0, 0, 0).with_alpha(fade_alpha),
                        None,
                        &*STAR_SVG,
                    );

                    self.scene.fill(
                        Fill::EvenOdd,
                        fill_transform,
                        if index < display_full_stars {
                            Color::from_rgb8(220, 180, 0)
                        } else {
                            Color::from_rgb8(85, 85, 85)
                        }
                        .with_alpha(fade_alpha),
                        None,
                        &*STAR_SVG,
                    );
                    if index == display_full_stars && display_has_half {
                        self.scene.fill(
                            Fill::EvenOdd,
                            fill_transform,
                            Color::from_rgb8(220, 180, 0).with_alpha(fade_alpha),
                            None,
                            &*STAR_SVG_HALF,
                        );
                    }
                }
                IconEntry::Playlist {
                    playlist,
                    contained,
                } => {
                    let Some(playlist_image_ref) = IMAGES_CACHE.get(&playlist.image_url) else {
                        continue;
                    };
                    let Some(playlist_image) = playlist_image_ref.as_ref() else {
                        continue;
                    };

                    // Shadow outline
                    self.scene.stroke(
                        &Stroke::new(1.0 * self.scale_factor),
                        icon_transform,
                        Color::from_rgb8(0, 0, 0).with_alpha(fade_alpha),
                        None,
                        &RoundedRect::new(0.0, 0.0, icon_size, icon_size, 6.0),
                    );

                    self.scene.push_clip_layer(
                        icon_transform,
                        &RoundedRect::new(0.0, 0.0, icon_size, icon_size, 6.0),
                    );
                    let zoom_pixels = 12.0;
                    let image_size = f64::from(playlist_image.image.width);
                    self.scene.fill(
                        Fill::EvenOdd,
                        icon_transform
                            * Affine::translate((-zoom_pixels, -zoom_pixels))
                            * Affine::scale((icon_size + zoom_pixels * 2.0) / image_size),
                        ImageBrush {
                            image: &playlist_image.image,
                            sampler: playlist_image.sampler.with_alpha(fade_alpha),
                        },
                        None,
                        &Rect::new(0.0, 0.0, image_size, image_size),
                    );
                    if !contained && !hovered {
                        self.scene.fill(
                            Fill::EvenOdd,
                            icon_transform,
                            Color::from_rgb8(60, 60, 60).with_alpha(0.7 * fade_alpha),
                            None,
                            &Rect::new(0.0, 0.0, icon_size, icon_size),
                        );
                    }
                    self.scene.pop_layer();
                }
            }
        }
    }
}

/// Skip to the specified track in the queue.
fn skip_to_track(track_id: &TrackId, position: f64, always_seek: bool) {
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
        for _ in 0..skips.min(10) {
            let result = if forward {
                // https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-next-track
                SPOTIFY_CLIENT.api_post("me/player/next")
            } else {
                // https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-previous-track
                SPOTIFY_CLIENT.api_post("me/player/previous")
            };
            update_playback_state(|state| {
                state.queue_index = position_in_queue;
                state.progress = 0;
                state.last_progress_update = Instant::now();
                state.last_interaction = Instant::now() + Duration::from_millis(2000);
            });
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
            f64::from(song_ms) * position
        };
        info!(
            "Seeking track {track_id} to {}%",
            (milliseconds / f64::from(song_ms) * 100.0).round()
        );
        update_playback_state(|state| {
            state.progress = milliseconds.round() as u32;
            state.last_progress_update = Instant::now();
            state.last_interaction = Instant::now() + Duration::from_millis(2000);
        });
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/seek-to-position-in-currently-playing-track
        if let Err(err) = SPOTIFY_CLIENT.api_put(&format!(
            "me/player/seek?position_ms={}",
            milliseconds.round()
        )) {
            error!("Failed to seek track: {err}");
        }
    }
}

/// Update Spotify rating playlists for the given track.
fn update_star_rating(track_id: &TrackId, rating_slot: u8) {
    if !CONFIG.ratings_enabled {
        return;
    }
    let Some(rating_name) = RATING_PLAYLISTS.get(rating_slot as usize) else {
        return;
    };

    let mut playlists_to_remove_from = Vec::new();
    let mut playlists_to_add_to = Vec::new();
    update_playback_state(|state| {
        state.last_interaction = Instant::now() + Duration::from_millis(500);

        // Remove tracks from existing playlists
        for playlist in state.playlists.values_mut().filter(|playlist| {
            RATING_PLAYLISTS.contains(&playlist.name.as_str())
                && playlist.name != *rating_name
                && playlist.tracks.contains(track_id)
        }) {
            playlist.tracks.remove(track_id);
            playlists_to_remove_from.push((playlist.id, playlist.name.clone()));
        }

        // Add the track to the target playlist if it's not already there
        for playlist in state
            .playlists
            .values_mut()
            .filter(|playlist| playlist.name == *rating_name && !playlist.tracks.contains(track_id))
        {
            playlist.tracks.insert(*track_id);
            playlists_to_add_to.push((playlist.id, playlist.name.clone()));
        }
    });

    // Make the changes
    for (playlist_id, playlist_name) in playlists_to_remove_from {
        info!("Removing track {track_id} from rating playlist {playlist_name}");
        let track_uri = format!("spotify:track:{track_id}");
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist
        if let Err(err) = SPOTIFY_CLIENT.api_delete_payload(
            &format!("playlists/{playlist_id}/tracks"),
            &format!(r#"{{"tracks": [ {{"uri": "{track_uri}"}} ]}}"#),
        ) {
            error!("Failed to remove track {track_id} from rating playlist {playlist_name}: {err}");
        }
    }
    for (playlist_id, playlist_name) in playlists_to_add_to {
        info!("Adding track {track_id} to rating playlist {playlist_name}");
        let track_uri = format!("spotify:track:{track_id}");
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/add-tracks-to-playlist)
        if let Err(err) = SPOTIFY_CLIENT.api_post_payload(
            &format!("playlists/{playlist_id}/tracks"),
            &format!(r#"{{"uris": ["{track_uri}"]}}"#),
        ) {
            error!("Failed to add track {track_id} to rating playlist {playlist_name}: {err}");
        }
    }

    // Add the track the liked songs if its rated above 3 stars
    // https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks
    match SPOTIFY_CLIENT.api_get(&format!("me/tracks/contains/?ids={track_id}")) {
        Ok(already_liked) => match (already_liked == "[true]", rating_slot >= 5) {
            (true, false) => {
                info!("Removing track {track_id} from liked songs");
                // https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user
                if let Err(err) = SPOTIFY_CLIENT.api_delete(&format!("me/tracks/?ids={track_id}")) {
                    error!("Failed to remove track {track_id} from liked songs: {err}");
                }
            }
            (false, true) => {
                info!("Adding track {track_id} to liked songs");
                // https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user
                if let Err(err) = SPOTIFY_CLIENT.api_put(&format!("me/tracks/?ids={track_id}")) {
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

    let track_uri = format!("spotify:track:{track_id}");
    let result = if contained {
        SPOTIFY_CLIENT.api_delete_payload(
            &format!("playlists/{playlist_id}/tracks"),
            &format!(r#"{{"tracks": [ {{"uri": "{track_uri}"}} ]}}"#),
        )
    } else {
        SPOTIFY_CLIENT.api_post_payload(
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

/// Toggle Spotify playlist membership for the given track.
fn toggle_playing(play: bool) {
    info!("{} current track", if play { "Playing" } else { "Pausing" });
    update_playback_state(|state| {
        state.last_interaction = Instant::now() + Duration::from_millis(2000);
        state.playing = play;
    });
    // https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback
    // https://developer.spotify.com/documentation/web-api/reference/#/operations/pause-a-users-playback
    if play {
        if let Err(err) = SPOTIFY_CLIENT.api_put("me/player/play") {
            error!("Failed to play playback: {err}");
        }
    } else if let Err(err) = SPOTIFY_CLIENT.api_put("me/player/pause") {
        error!("Failed to pause playback: {err}");
    }
}

/// Set the volume of the current playback device.
fn set_volume(volume_percent: u8) {
    info!("Setting volume to {}%", volume_percent);
    update_playback_state(|state| {
        state.last_interaction = Instant::now() + Duration::from_millis(500);
    });
    // https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback
    if let Err(err) =
        SPOTIFY_CLIENT.api_put(&format!("me/player/volume?volume_percent={volume_percent}"))
    {
        error!("Failed to set volume: {err}");
    }
}

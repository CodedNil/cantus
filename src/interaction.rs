use crate::{
    CantusApp, HISTORY_WIDTH,
    spotify::{
        IMAGES_CACHE, PLAYBACK_STATE, Playlist, RATING_PLAYLISTS, SPOTIFY_CLIENT, Track,
        update_playback_state,
    },
};
use chrono::TimeDelta;
use itertools::Itertools;
use rspotify::{
    model::{PlayableId, PlaylistId, TrackId},
    prelude::OAuthClient,
};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::{cmp::Ordering, collections::HashMap, sync::LazyLock, time::Instant};
use tracing::{error, info, warn};
use vello::{
    kurbo::{Affine, BezPath, Point, Rect, RoundedRect, Shape, Stroke},
    peniko::{Color, Fill, ImageBrush},
};

static SPOTIFY_INTERACTION_GUARD: AtomicBool = AtomicBool::new(false);
struct SpotifyInteractionToken;
fn try_acquire_spotify_guard() -> Option<SpotifyInteractionToken> {
    SPOTIFY_INTERACTION_GUARD
        .compare_exchange(
            false,
            true,
            AtomicOrdering::Acquire,
            AtomicOrdering::Relaxed,
        )
        .ok()
        .map(|_| SpotifyInteractionToken)
}
impl Drop for SpotifyInteractionToken {
    fn drop(&mut self) {
        SPOTIFY_INTERACTION_GUARD.store(false, AtomicOrdering::Release);
    }
}

pub struct IconHitbox {
    pub rect: Rect,
    pub track_id: TrackId<'static>,
    pub playlist_id: Option<PlaylistId<'static>>,
    pub rating_index: Option<usize>,
}

pub struct InteractionState {
    pub last_event: InteractionEvent,
    pub last_click: Option<(Instant, TrackId<'static>, Point)>,
    pub mouse_position: Point,
    #[cfg(feature = "wayland")]
    pub last_hitbox_update: Instant,
    pub play_hitbox: Rect,
    pub track_hitboxes: HashMap<TrackId<'static>, (Rect, (f64, f64))>,
    pub icon_hitboxes: Vec<IconHitbox>,
    pub mouse_down: bool,
    pub drag_origin: Option<Point>,
    pub drag_track: Option<(TrackId<'static>, f64)>,
    pub dragging: bool,
    pub drag_delta_pixels: f64,
    spotify_guard: Option<SpotifyInteractionToken>,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self {
            last_event: InteractionEvent::None,
            last_click: None,
            mouse_position: Point::default(),
            #[cfg(feature = "wayland")]
            last_hitbox_update: Instant::now(),
            play_hitbox: Rect::default(),
            track_hitboxes: HashMap::new(),
            icon_hitboxes: Vec::new(),
            mouse_down: false,
            drag_origin: None,
            drag_track: None,
            dragging: false,
            drag_delta_pixels: 0.0,
            spotify_guard: None,
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
        self.spotify_guard = None;
    }

    pub fn left_click_released(&mut self, scale_factor: f64) {
        if !self.dragging && self.mouse_down {
            self.handle_click(scale_factor);
        }
        if let Some((track_id, position)) = self.drag_track.take() {
            let track_id = track_id.clone();
            tokio::spawn(async move {
                skip_to_track(track_id, position, false).await;
            });
        }
        self.drag_origin = None;
        self.dragging = false;
        self.drag_delta_pixels = 0.0;
        self.spotify_guard = None;
        self.mouse_down = false;
    }

    pub fn right_click(&mut self) {
        self.cancel_drag();
        self.mouse_down = false;
    }

    /// Handle click events.
    fn handle_click(&mut self, scale_factor: f64) -> bool {
        let mouse_pos = self.mouse_position;

        // Click on rating/playlist icons
        if let Some(hitbox) = self
            .icon_hitboxes
            .iter()
            .find(|h| h.rect.contains(mouse_pos))
        {
            let track_id = hitbox.track_id.clone();
            if let Some(index) = hitbox.rating_index {
                let center_x = (hitbox.rect.x0 + hitbox.rect.x1) * 0.5;
                let rating_slot = index * 2 + 1 + usize::from(mouse_pos.x >= center_x);
                tokio::spawn(async move {
                    update_star_rating(track_id, rating_slot).await;
                });
            } else if let Some(playlist_id) = hitbox.playlist_id.clone() {
                tokio::spawn(async move {
                    toggle_playlist_membership(track_id, playlist_id).await;
                });
            }
            return true;
        }

        // Play/pause
        if self.play_hitbox.contains(mouse_pos) {
            let playing = PLAYBACK_STATE.read().playing;
            // Add a click event
            if let Some((track_id, (track_rect, _))) = self
                .track_hitboxes
                .iter()
                .find(|(_, (track_rect, _))| track_rect.contains(mouse_pos))
            {
                self.last_click = Some((
                    Instant::now(),
                    track_id.clone(),
                    Point::new(mouse_pos.x - track_rect.x0, mouse_pos.y - track_rect.y0),
                ));
            }
            if playing {
                self.last_event = InteractionEvent::Pause(Instant::now());
            } else {
                self.last_event = InteractionEvent::Play(Instant::now());
            }
            tokio::spawn(async move {
                toggle_playing(!playing).await;
            });
            return true;
        }

        // Seek track
        if let Some((track_id, (track_rect, (track_range_a, track_range_b)))) = self
            .track_hitboxes
            .iter()
            .find(|(_, (track_rect, _))| track_rect.contains(mouse_pos))
        {
            self.last_click = Some((
                Instant::now(),
                track_id.clone(),
                Point::new(mouse_pos.x - track_rect.x0, mouse_pos.y - track_rect.y0),
            ));

            let track_id = track_id.clone();
            // If click is near the very left, reset to the start of the song, else seek to clicked position
            let position = if mouse_pos.x < (HISTORY_WIDTH + 20.0) * scale_factor {
                0.0
            } else {
                (mouse_pos.x - track_range_a) / (track_range_b - track_range_a)
            };
            tokio::spawn(async move {
                skip_to_track(track_id, position, false).await;
            });
            return true;
        }
        false
    }

    /// Drag across the progress bar to seek.
    pub fn handle_mouse_drag(&mut self) {
        if let Some(origin_pos) = self.drag_origin {
            let delta_x = self.mouse_position.x - origin_pos.x;
            let delta_y = self.mouse_position.y - origin_pos.y;
            if !self.dragging && (delta_x.abs() >= 2.0 || delta_y.abs() >= 2.0) {
                self.dragging = true;
                self.ensure_spotify_guard();
            }
            self.drag_delta_pixels = if self.dragging { delta_x } else { 0.0 };
        } else {
            self.drag_delta_pixels = 0.0;
        }
    }

    /// Handle scrolling events to adjust volume.
    pub fn handle_scroll(delta: i32) {
        if delta == 0 {
            return;
        }
        update_playback_state(|state| {
            if let Some(volume) = &mut state.volume {
                *volume = if delta < 0 {
                    volume.saturating_add(5).min(100)
                } else {
                    volume.saturating_sub(5)
                };
            }
        });
        let current_volume = PLAYBACK_STATE.read().volume;
        if let Some(volume) = current_volume {
            tokio::spawn(async move {
                set_volume(volume).await;
            });
        }
    }

    pub fn cancel_drag(&mut self) {
        self.drag_track = None;
        self.drag_origin = None;
        self.dragging = false;
        self.drag_delta_pixels = 0.0;
        self.spotify_guard = None;
    }

    fn ensure_spotify_guard(&mut self) {
        if self.spotify_guard.is_none() {
            self.spotify_guard = try_acquire_spotify_guard();
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum InteractionEvent {
    None,
    Pause(Instant),
    Play(Instant),
    PauseHover(Instant),
    PlayHover(Instant),
}

enum IconEntry<'a> {
    Star {
        index: usize,
    },
    Playlist {
        playlist: &'a Playlist,
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
        playlists: &HashMap<&str, &Playlist>,
        width: f64,
        height: f64,
        pos_x: f64,
    ) {
        let track_rating_index = RATING_PLAYLISTS
            .iter()
            .position(|&rating_key| {
                playlists
                    .get(rating_key)
                    .is_some_and(|playlist| playlist.tracks.contains(&track.id))
            })
            .unwrap_or(0);

        let mut icon_entries = (0..5)
            .map(|index| IconEntry::Star { index })
            .collect::<Vec<_>>();
        // Add playlists that are contained in the favourited playlists
        icon_entries.extend(
            playlists
                .iter()
                .filter_map(|(&key, &playlist)| {
                    if RATING_PLAYLISTS.contains(&key) {
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
        let half_icons = num_icons as f64 / 2.0;

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
            match entry {
                IconEntry::Star { index } => {
                    if hovered {
                        let rect_center_x = (button_rect.x0 + button_rect.x1) * 0.5;
                        hover_rating_index =
                            Some(*index * 2 + 1 + usize::from(mouse_pos.x >= rect_center_x));
                    }
                    icon_entry_extras.push((hovered, icon_origin_x));
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect: button_rect,
                        track_id: track.id.clone(),
                        playlist_id: None,
                        rating_index: Some(*index),
                    });
                }
                IconEntry::Playlist {
                    playlist,
                    contained: _,
                } => {
                    icon_entry_extras.push((hovered, icon_origin_x));
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect: button_rect,
                        track_id: track.id.clone(),
                        playlist_id: Some(playlist.id.clone()),
                        rating_index: None,
                    });
                }
            }
        }

        // Render out the icons
        let display_rating_index = hover_rating_index.unwrap_or(track_rating_index);
        let display_full_stars = display_rating_index / 2;
        let display_has_half = display_rating_index % 2 == 1;
        for (i, entry) in icon_entries.into_iter().enumerate() {
            let (hovered, icon_origin_x) = icon_entry_extras[i];

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
                    let Some(playlist_image) = IMAGES_CACHE.get(&playlist.image_url) else {
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
                    let image_size = f64::from(playlist_image.width);
                    self.scene.fill(
                        Fill::EvenOdd,
                        icon_transform
                            * Affine::translate((-zoom_pixels, -zoom_pixels))
                            * Affine::scale((icon_size + zoom_pixels * 2.0) / image_size),
                        &ImageBrush::new(playlist_image.clone()).with_alpha(fade_alpha),
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
async fn skip_to_track(track_id: TrackId<'static>, position: f64, always_seek: bool) {
    let Some(_guard) = try_acquire_spotify_guard() else {
        return;
    };

    let (queue_index, position_in_queue, ms_lookup) = {
        let state = PLAYBACK_STATE.read();
        let queue_index = state.queue_index;
        let Some(position_in_queue) = state.queue.iter().position(|t| t.id == track_id) else {
            error!("Track not found in queue");
            return;
        };
        let ms_lookup = state
            .queue
            .iter()
            .map(|playlist| playlist.milliseconds)
            .collect::<Vec<_>>();
        drop(state);
        (queue_index, position_in_queue, ms_lookup)
    };
    // Skip or rewind to the track
    if queue_index != position_in_queue {
        update_playback_state(|state| {
            state.queue_index = position_in_queue;
            state.progress = 0;
            state.last_updated = Instant::now();
            state.last_interaction = Instant::now();
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
        let client = SPOTIFY_CLIENT.get().unwrap();
        for _ in 0..skips.min(10) {
            let result = if forward {
                client.next_track(None).await
            } else {
                client.previous_track(None).await
            };
            update_playback_state(|state| {
                state.queue_index = position_in_queue;
                state.progress = 0;
                state.last_updated = Instant::now();
                state.last_interaction = Instant::now();
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
            state.last_updated = Instant::now();
            state.last_interaction = Instant::now();
        });
        if let Err(err) = SPOTIFY_CLIENT
            .get()
            .unwrap()
            .seek_track(TimeDelta::milliseconds(milliseconds as i64), None)
            .await
        {
            error!("Failed to seek track: {err}");
        }
    }
}

/// Update Spotify rating playlists for the given track.
async fn update_star_rating(track_id: TrackId<'static>, rating_slot: usize) {
    let Some(_guard) = try_acquire_spotify_guard() else {
        return;
    };
    let Some(rating_name) = RATING_PLAYLISTS.get(rating_slot) else {
        return;
    };

    let mut playlists_to_remove_from = Vec::new();
    let mut playlists_to_add_to = Vec::new();
    {
        let mut write_state = PLAYBACK_STATE.write();
        write_state.last_interaction = Instant::now();

        // Remove tracks from existing playlists
        for playlist in write_state.playlists.iter_mut().filter(|playlist| {
            RATING_PLAYLISTS.contains(&playlist.name.as_str())
                && playlist.name != *rating_name
                && playlist.tracks.contains(&track_id)
        }) {
            playlist.tracks.remove(&track_id);
            playlists_to_remove_from.push((playlist.id.clone(), playlist.name.clone()));
        }

        // Add the track to the target playlist if it's not already there
        for playlist in write_state.playlists.iter_mut().filter(|playlist| {
            playlist.name == *rating_name && !playlist.tracks.contains(&track_id)
        }) {
            info!(
                "Adding track {track_id} to rating playlist {}",
                playlist.name
            );
            playlist.tracks.insert(track_id.clone());
            playlists_to_add_to.push((playlist.id.clone(), playlist.name.clone()));
        }
    }

    // Make the changes
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();
    for (playlist_id, playlist_name) in playlists_to_remove_from {
        info!("Removing track {track_id} from rating playlist {playlist_name}");
        if let Err(err) = spotify_client
            .playlist_remove_all_occurrences_of_items(
                playlist_id,
                [PlayableId::Track(track_id.clone())],
                None,
            )
            .await
        {
            error!("Failed to remove track {track_id} from rating playlist {playlist_name}: {err}");
        }
    }
    for (playlist_id, playlist_name) in playlists_to_add_to {
        info!("Adding track {track_id} to rating playlist {playlist_name}");
        if let Err(err) = spotify_client
            .playlist_add_items(playlist_id, [PlayableId::Track(track_id.clone())], None)
            .await
        {
            error!("Failed to add track {track_id} to rating playlist {playlist_name}: {err}");
        }
    }

    // Add the track the liked songs if its rated above 3 stars
    let should_be_liked = rating_slot >= 6;
    match spotify_client
        .current_user_saved_tracks_contains([track_id.clone()])
        .await
    {
        Ok(already_liked) => match (already_liked[0], should_be_liked) {
            (true, false) => {
                info!("Removing track {track_id} from liked songs");
                if let Err(err) = spotify_client
                    .current_user_saved_tracks_delete([track_id.clone()])
                    .await
                {
                    error!("Failed to remove track {track_id} from liked songs: {err}");
                }
            }
            (false, true) => {
                info!("Adding track {track_id} to liked songs");
                if let Err(err) = spotify_client
                    .current_user_saved_tracks_add([track_id.clone()])
                    .await
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

/// Toggle Spotify playlist membership for the given track.
async fn toggle_playlist_membership(track_id: TrackId<'static>, playlist_id: PlaylistId<'static>) {
    let Some(_guard) = try_acquire_spotify_guard() else {
        return;
    };
    let Some((playlist_idx, (playlist_id, playlist_name, contained))) = PLAYBACK_STATE
        .read()
        .playlists
        .iter()
        .enumerate()
        .find(|(_, p)| p.id == playlist_id)
        .map(|(idx, playlist)| {
            (
                idx,
                (
                    playlist.id.clone(),
                    playlist.name.clone(),
                    playlist.tracks.contains(&track_id),
                ),
            )
        })
    else {
        warn!("Playlist {playlist_id} not found while toggling membership for track {track_id}");
        return;
    };

    let track_playable = PlayableId::Track(track_id.clone());

    info!(
        "{} track {track_id} {} playlist {playlist_name}",
        if contained { "Removing" } else { "Adding" },
        if contained { "from" } else { "to" }
    );

    update_playback_state(|state| {
        let playlist_tracks = &mut state.playlists[playlist_idx].tracks;
        if contained {
            playlist_tracks.remove(&track_id);
        } else {
            playlist_tracks.insert(track_id.clone());
        }
        state.last_interaction = Instant::now();
    });

    let spotify_client = SPOTIFY_CLIENT.get().unwrap();
    let result = if contained {
        spotify_client.playlist_remove_all_occurrences_of_items(
            playlist_id.clone(),
            [track_playable],
            None,
        )
    } else {
        spotify_client.playlist_add_items(playlist_id.clone(), [track_playable], None)
    };
    if let Err(err) = result.await {
        error!(
            "Failed to {} track {track_id} {} playlist {playlist_name}: {err}",
            if contained { "remove" } else { "add" },
            if contained { "from" } else { "to" }
        );
    }
}

/// Toggle Spotify playlist membership for the given track.
async fn toggle_playing(play: bool) {
    let Some(_guard) = try_acquire_spotify_guard() else {
        return;
    };

    info!("{} current track", if play { "Playing" } else { "Pausing" });
    update_playback_state(|state| {
        state.last_interaction = Instant::now();
    });
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();
    if play {
        if let Err(err) = spotify_client.resume_playback(None, None).await {
            error!("Failed to play playback: {err}");
        }
    } else if let Err(err) = spotify_client.pause_playback(None).await {
        error!("Failed to pause playback: {err}");
    }
}

/// Set the volume of the current playback device.
async fn set_volume(volume: u8) {
    let Some(_guard) = try_acquire_spotify_guard() else {
        return;
    };

    info!("Setting volume to {}%", volume);
    update_playback_state(|state| {
        state.last_interaction = Instant::now();
    });
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();
    if let Err(err) = spotify_client.volume(volume, None).await {
        error!("Failed to set volume: {err}");
    }
}

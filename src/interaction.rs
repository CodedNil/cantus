use crate::{
    CantusApp,
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
    Scene,
    kurbo::{Affine, Point, Rect, RoundedRect},
    peniko::{BlendMode, Color, Compose, Fill, ImageBrush, Mix},
};
use vello_svg::usvg;

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

#[derive(Clone)]
pub struct IconHitbox {
    pub rect: Rect,
    pub track_id: TrackId<'static>,
    pub playlist_id: Option<PlaylistId<'static>>,
    pub rating_index: Option<usize>,
}

pub struct InteractionState {
    pub pointer_position: (f64, f64),
    #[cfg(feature = "layer-shell")]
    pub last_hitbox_update: Instant,
    pub track_hitboxes: HashMap<TrackId<'static>, Rect>,
    pub icon_hitboxes: Vec<IconHitbox>,
    pub drag_origin: Option<(f64, f64)>,
    pub dragging: bool,
    pub drag_delta_pixels: f64,
    spotify_guard: Option<SpotifyInteractionToken>,
}

impl InteractionState {
    pub fn new() -> Self {
        Self {
            pointer_position: (0.0, 0.0),
            #[cfg(feature = "layer-shell")]
            last_hitbox_update: Instant::now(),
            track_hitboxes: HashMap::new(),
            icon_hitboxes: Vec::new(),
            drag_origin: None,
            dragging: false,
            drag_delta_pixels: 0.0,
            spotify_guard: None,
        }
    }

    pub fn start_drag(&mut self) {
        self.drag_origin = Some(self.pointer_position);
        self.dragging = false;
        self.drag_delta_pixels = 0.0;
        self.spotify_guard = None;
    }

    pub fn end_drag(&mut self) {
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
static STAR_IMAGES: LazyLock<(f64, [Scene; 4])> = LazyLock::new(|| {
    let full_svg = include_str!("../assets/star.svg");
    let half_svg = include_str!("../assets/star-half.svg");
    (
        f64::from(
            usvg::Tree::from_str(full_svg, &usvg::Options::default())
                .unwrap()
                .size()
                .width(),
        ),
        [
            vello_svg::render(&full_svg.replace("fill=\"none\"", "fill=\"#555555\"")).unwrap(),
            vello_svg::render(&full_svg.replace("fill=\"none\"", "fill=\"#000000\"")).unwrap(),
            vello_svg::render(&full_svg.replace("fill=\"none\"", "fill=\"#dcb400\"")).unwrap(),
            vello_svg::render(&half_svg.replace("fill=\"none\"", "fill=\"#dcb400\"")).unwrap(),
        ],
    )
});

impl CantusApp {
    /// Handle pointer click events.
    pub fn handle_pointer_click(&self) -> bool {
        let point = Point::new(
            self.interaction.pointer_position.0,
            self.interaction.pointer_position.1,
        );
        if let Some(hitbox) = self
            .interaction
            .icon_hitboxes
            .iter()
            .find(|h| h.rect.contains(point))
        {
            let track_id = hitbox.track_id.clone();
            if let Some(index) = hitbox.rating_index {
                let center_x = (hitbox.rect.x0 + hitbox.rect.x1) * 0.5;
                let rating_slot = index * 2 + 1 + usize::from(point.x >= center_x);
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
        if let Some((id, rect)) = self
            .interaction
            .track_hitboxes
            .iter()
            .find(|(_, rect)| rect.contains(point))
        {
            let (id, rect) = (id.clone(), *rect);
            tokio::spawn(async move {
                skip_to_track(id, point, rect).await;
            });
            return true;
        }
        false
    }

    /// Drag across the progress bar to seek.
    pub fn handle_pointer_drag_motion(&mut self) {
        if let Some((origin_x, origin_y)) = self.interaction.drag_origin {
            let delta_x = self.interaction.pointer_position.0 - origin_x;
            let delta_y = self.interaction.pointer_position.1 - origin_y;
            if !self.interaction.dragging && (delta_x.abs() >= 2.0 || delta_y.abs() >= 2.0) {
                self.interaction.dragging = true;
                self.interaction.ensure_spotify_guard();
            }
            self.interaction.drag_delta_pixels = if self.interaction.dragging {
                delta_x
            } else {
                0.0
            };
        } else {
            self.interaction.drag_delta_pixels = 0.0;
        }
    }

    /// Star ratings and favourite playlists
    pub fn draw_playlist_buttons(
        &mut self,
        track: &Track,
        is_current: bool,
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

        let icon_size = 14.0 * self.scale_factor;
        let star_size_border = 1.0 * self.scale_factor;
        let icon_spacing = 2.0 * self.scale_factor;
        let icon_total_size = icon_size + star_size_border * 2.0;
        let pointer_point = Point::new(
            self.interaction.pointer_position.0,
            self.interaction.pointer_position.1,
        );

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
                    if !contained && !is_current {
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
        let num_icons = icon_entries.len();
        // Fade out when there's not enough space
        let needed_width = icon_size * num_icons as f64;
        if width < needed_width {
            return;
        }
        let fade_alpha = ((width - needed_width) / (needed_width * 0.25)).clamp(0.0, 1.0) as f32;

        let inv_scale = 1.0 / self.scale_factor;
        let base_y = height * 0.8;
        let icon_center_y = base_y - star_size_border + icon_total_size * 0.5;
        let center_x = pos_x + width * 0.5;
        let half_icons = num_icons as f64 / 2.0;
        let spacing = icon_size + icon_spacing;
        let button_rect = |icon_origin_x: f64| {
            Rect::new(
                (icon_origin_x - star_size_border) * inv_scale,
                (base_y - star_size_border) * inv_scale,
                (icon_origin_x - star_size_border + icon_total_size) * inv_scale,
                (base_y - star_size_border + icon_total_size) * inv_scale,
            )
        };

        let mut hover_rating_index = None;
        for (i, entry) in icon_entries.iter().enumerate() {
            let icon_origin_x = center_x + (i as f64 - half_icons) * spacing;
            let button_rect = button_rect(icon_origin_x);
            if hover_rating_index.is_none()
                && button_rect.contains(pointer_point)
                && let IconEntry::Star { index } = entry
            {
                let rect_center_x = (button_rect.x0 + button_rect.x1) * 0.5;
                hover_rating_index =
                    Some(*index * 2 + 1 + usize::from(pointer_point.x >= rect_center_x));
            }
        }

        let display_rating_index = hover_rating_index.unwrap_or(track_rating_index);
        let display_full_stars = display_rating_index / 2;
        let display_has_half = display_rating_index % 2 == 1;

        let star_border_scale = (icon_size + star_size_border * 2.0) / STAR_IMAGES.0;
        let star_fill_scale = icon_size / STAR_IMAGES.0;
        for (i, entry) in icon_entries.into_iter().enumerate() {
            let icon_origin_x = center_x + (i as f64 - half_icons) * spacing;
            let base_transform = Affine::translate((icon_origin_x, base_y));
            let button_rect = button_rect(icon_origin_x);
            let is_hovered = button_rect.contains(pointer_point);
            let icon_center_x = icon_origin_x - star_size_border + icon_total_size * 0.5;
            let hover_transform = if is_hovered {
                Affine::translate((icon_center_x, icon_center_y))
                    * Affine::scale(1.2)
                    * Affine::translate((-icon_center_x, -icon_center_y))
            } else {
                Affine::IDENTITY
            };
            let combined_transform = hover_transform * base_transform;

            match entry {
                IconEntry::Star { index } => {
                    if fade_alpha < 1.0 {
                        self.scene.push_layer(
                            BlendMode::new(Mix::Normal, Compose::SrcOver),
                            fade_alpha,
                            Affine::IDENTITY,
                            &Rect::new(-1e9, -1e9, 1e9, 1e9),
                        );
                    }
                    self.scene.append(
                        &STAR_IMAGES.1[1],
                        Some(
                            combined_transform
                                * Affine::translate((-star_size_border, -star_size_border))
                                * Affine::scale(star_border_scale),
                        ),
                    );

                    let fill_transform = combined_transform * Affine::scale(star_fill_scale);
                    if index < display_full_stars {
                        self.scene.append(&STAR_IMAGES.1[2], Some(fill_transform));
                    } else {
                        self.scene.append(&STAR_IMAGES.1[0], Some(fill_transform));
                    }
                    if index == display_full_stars && display_has_half {
                        self.scene.append(&STAR_IMAGES.1[3], Some(fill_transform));
                    }
                    if fade_alpha < 1.0 {
                        self.scene.pop_layer();
                    }
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect: button_rect,
                        track_id: track.id.clone(),
                        playlist_id: None,
                        rating_index: Some(index),
                    });
                }
                IconEntry::Playlist {
                    playlist,
                    contained,
                } => {
                    let Some(playlist_image) = IMAGES_CACHE.get(&playlist.image_url) else {
                        continue;
                    };
                    let icon_transform = combined_transform
                        * Affine::translate((-star_size_border, -star_size_border));
                    let playlist_icon_size = icon_total_size;
                    self.scene.push_clip_layer(
                        icon_transform,
                        &RoundedRect::new(0.0, 0.0, playlist_icon_size, playlist_icon_size, 10.0),
                    );
                    let zoom_pixels = 16.0;
                    let image_size = f64::from(playlist_image.width);
                    self.scene.fill(
                        Fill::NonZero,
                        icon_transform
                            * Affine::translate((-zoom_pixels, -zoom_pixels))
                            * Affine::scale((playlist_icon_size + zoom_pixels * 2.0) / image_size),
                        &ImageBrush::new(playlist_image.clone()).with_alpha(fade_alpha),
                        None,
                        &Rect::new(0.0, 0.0, image_size, image_size),
                    );
                    if !contained {
                        self.scene.fill(
                            Fill::NonZero,
                            icon_transform,
                            Color::from_rgb8(60, 60, 60).with_alpha(0.7 * fade_alpha),
                            None,
                            &Rect::new(0.0, 0.0, playlist_icon_size, playlist_icon_size),
                        );
                    }
                    self.scene.pop_layer();
                    self.interaction.icon_hitboxes.push(IconHitbox {
                        rect: button_rect,
                        track_id: track.id.clone(),
                        playlist_id: Some(playlist.id.clone()),
                        rating_index: None,
                    });
                }
            }
        }
    }
}

/// Skip to the specified track in the queue.
async fn skip_to_track(track_id: TrackId<'static>, point: Point, rect: Rect) {
    let Some(_guard) = try_acquire_spotify_guard() else {
        return;
    };

    let (queue_index, position_in_queue, ms_lookup) = {
        let playback_state = PLAYBACK_STATE.read();
        let queue_index = playback_state.queue_index;
        let Some(position_in_queue) = playback_state.queue.iter().position(|t| t.id == track_id)
        else {
            error!("Track not found in queue");
            return;
        };
        let ms_lookup = playback_state
            .queue
            .iter()
            .map(|playlist| playlist.milliseconds)
            .collect::<Vec<_>>();
        drop(playback_state);
        (queue_index, position_in_queue, ms_lookup)
    };
    if queue_index.cmp(&position_in_queue) == Ordering::Equal {
        let position = (point.x - rect.x0) / rect.width();
        let song_ms = ms_lookup[position_in_queue];
        // If click is near the very left, reset to the start of the song, else seek to clicked position
        let milliseconds = if point.x < 20.0 || position < 0.05 {
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
        });
        if let Err(err) = SPOTIFY_CLIENT
            .get()
            .unwrap()
            .seek_track(TimeDelta::milliseconds(milliseconds as i64), None)
            .await
        {
            error!("Failed to seek track: {err}");
        }
    } else {
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
            if let Err(err) = result {
                error!("Failed to skip to track: {err}");
            }
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
    let filtered_playlists = PLAYBACK_STATE
        .read()
        .playlists
        .iter()
        .enumerate()
        .filter(|(_, p)| RATING_PLAYLISTS.contains(&p.name.as_str()))
        .map(|(index, p)| (index, p.clone()))
        .collect::<Vec<_>>();
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();

    let mut target_playlist: Option<(usize, Playlist)> = None;
    let track_playable = PlayableId::Track(track_id.clone());
    for (playlist_idx, playlist) in filtered_playlists {
        if playlist.name == *rating_name {
            target_playlist = Some((playlist_idx, playlist));
            continue;
        }
        if !playlist.tracks.contains(&track_id) {
            continue;
        }
        info!(
            "Removing track {track_id} from rating playlist {}",
            playlist.name
        );
        update_playback_state(|state| {
            state.playlists[playlist_idx].tracks.remove(&track_id);
        });
        if let Err(err) = spotify_client
            .playlist_remove_all_occurrences_of_items(
                playlist.id.clone(),
                [track_playable.clone()],
                None,
            )
            .await
        {
            error!(
                "Failed to remove track {track_id} from rating playlist {}: {err}",
                playlist.name
            );
        }
    }

    // Add the track to the target playlist if it's not already there
    if let Some((target_playlist_idx, target_playlist)) = target_playlist
        && !target_playlist.tracks.contains(&track_id)
    {
        info!(
            "Adding track {track_id} to rating playlist {}",
            target_playlist.name
        );
        update_playback_state(|state| {
            state.playlists[target_playlist_idx]
                .tracks
                .insert(track_id.clone());
        });
        if let Err(err) = spotify_client
            .playlist_add_items(target_playlist.id.clone(), [track_playable], None)
            .await
        {
            error!(
                "Failed to add track {track_id} to rating playlist {}: {err}",
                target_playlist.name
            );
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
    let Some((playlist_idx, playlist)) = PLAYBACK_STATE
        .read()
        .playlists
        .iter()
        .enumerate()
        .find(|(_, p)| p.id == playlist_id)
        .map(|(idx, playlist)| (idx, playlist.clone()))
    else {
        warn!("Playlist {playlist_id} not found while toggling membership for track {track_id}");
        return;
    };

    let spotify_client = SPOTIFY_CLIENT.get().unwrap();
    let playlist_id = playlist.id.clone();
    let playlist_name = playlist.name.clone();
    let contained = playlist.tracks.contains(&track_id);
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
    });

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

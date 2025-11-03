use crate::{
    CantusLayer,
    spotify::{
        IMAGES_CACHE, PLAYBACK_STATE, Playlist, RATING_PLAYLISTS, SPOTIFY_CLIENT, Track,
        update_state_from_spotify,
    },
};
use chrono::TimeDelta;
use itertools::Itertools;
use rspotify::{
    model::{PlaylistId, TrackId},
    prelude::OAuthClient,
};
use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::{LazyLock, atomic::AtomicBool},
    time::{Duration, Instant},
};
use tracing::{error, info, warn};
use vello::{
    Scene,
    kurbo::{Affine, Point, Rect, RoundedRect},
    peniko::{Color, Fill, ImageBrush},
};
use vello_svg::usvg;
use wayland_client::QueueHandle;

static SPOTIFY_INTERACTION_ACTIVE: AtomicBool = AtomicBool::new(false);

struct SpotifyInteractionGuard;
impl SpotifyInteractionGuard {
    fn try_acquire() -> Option<Self> {
        SPOTIFY_INTERACTION_ACTIVE
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_ok()
            .then(|| Self)
    }
}
impl Drop for SpotifyInteractionGuard {
    fn drop(&mut self) {
        SPOTIFY_INTERACTION_ACTIVE.store(false, std::sync::atomic::Ordering::Release);
    }
}

#[derive(Clone, Debug)]
pub struct IconHitbox {
    pub rect: Rect,
    pub track_id: TrackId<'static>,
    pub playlist_id: Option<PlaylistId<'static>>,
    pub rating_index: Option<usize>,
}

enum IconEntry {
    Star { index: usize },
    Playlist { playlist: Playlist, contained: bool },
}

/// Star images
static STAR_IMAGES: LazyLock<[Scene; 4]> = LazyLock::new(|| {
    let full_svg = include_str!("../assets/star.svg");
    let half_svg = include_str!("../assets/star-half.svg");
    let options = usvg::Options::default();

    let full_gray = full_svg.replace("fill=\"none\"", "fill=\"#555555\"");
    let full_border = full_svg.replace("fill=\"none\"", "fill=\"#000000\"");
    let full_yellow = full_svg.replace("fill=\"none\"", "fill=\"#dcb400\"");
    let half_yellow = half_svg.replace("fill=\"none\"", "fill=\"#dcb400\"");

    [
        vello_svg::render_tree(&usvg::Tree::from_data(full_gray.as_bytes(), &options).unwrap()),
        vello_svg::render_tree(&usvg::Tree::from_data(full_border.as_bytes(), &options).unwrap()),
        vello_svg::render_tree(&usvg::Tree::from_data(full_yellow.as_bytes(), &options).unwrap()),
        vello_svg::render_tree(&usvg::Tree::from_data(half_yellow.as_bytes(), &options).unwrap()),
    ]
});
static STAR_IMAGE_SIZE: LazyLock<f64> = LazyLock::new(|| {
    f64::from(
        usvg::Tree::from_data(
            include_bytes!("../assets/star-half.svg"),
            &usvg::Options::default(),
        )
        .unwrap()
        .size()
        .width(),
    )
});

impl CantusLayer {
    /// Handle pointer click events.
    pub fn handle_pointer_click(&self) -> bool {
        let point = Point::new(self.pointer_position.0, self.pointer_position.1);
        if let Some(hitbox) = self
            .icon_hitboxes
            .iter()
            .find(|hitbox| hitbox.rect.contains(point))
        {
            if let Some(index) = hitbox.rating_index {
                let center_x = (hitbox.rect.x0 + hitbox.rect.x1) * 0.5;
                let left_half = point.x < center_x;
                let half_label = if left_half { "left" } else { "right" };
                let rating_preview = index as f64 + if left_half { 0.5 } else { 1.0 };
                println!(
                    "Clicked button for track {:?}, playlist {:?}, rating index {:?}, star_half {}, rating {:.1}",
                    hitbox.track_id,
                    hitbox.playlist_id,
                    hitbox.rating_index,
                    half_label,
                    rating_preview,
                );
            } else {
                println!(
                    "Clicked button for track {:?}, playlist {:?}, rating index {:?}",
                    hitbox.track_id, hitbox.playlist_id, hitbox.rating_index
                );
            }
            return true;
        }
        if let Some((id, rect)) = self
            .track_hitboxes
            .iter()
            .find(|(_, rect)| rect.contains(point))
        {
            let id = id.clone();
            let rect = *rect;
            tokio::spawn(async move {
                skip_to_track(id, point, rect).await;
            });
            return true;
        }
        false
    }

    /// Update the input region for the surface.
    pub fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        if self.last_hitbox_update.elapsed() <= Duration::from_millis(500) {
            return;
        }

        let (Some(wl_surface), Some(compositor)) = (&self.wl_surface, &self.compositor) else {
            return;
        };

        let region = compositor.create_region(qhandle, ());
        for rect in self.track_hitboxes.values() {
            region.add(
                rect.x0.round() as i32,
                rect.y0.round() as i32,
                (rect.x1 - rect.x0).round() as i32,
                (rect.y1 - rect.y0).round() as i32,
            );
        }
        for hitbox in &self.icon_hitboxes {
            let rect = &hitbox.rect;
            region.add(
                rect.x0.round() as i32,
                rect.y0.round() as i32,
                (rect.x1 - rect.x0).round() as i32,
                (rect.y1 - rect.y0).round() as i32,
            );
        }

        wl_surface.set_input_region(Some(&region));
        wl_surface.commit();
        self.last_hitbox_update = Instant::now();
    }

    /// Star ratings and favourite playlists
    pub fn draw_playlist_buttons(
        &mut self,
        track: &Track,
        is_current: bool,
        playlists: &HashMap<String, Playlist>,
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
        let pointer_point = Point::new(self.pointer_position.0, self.pointer_position.1);

        let mut icon_entries: Vec<IconEntry> =
            (0..5).map(|index| IconEntry::Star { index }).collect();
        icon_entries.extend(
            playlists
                .iter()
                .filter_map(|(key, playlist)| {
                    let is_rating = RATING_PLAYLISTS.contains(&key.as_str());
                    let contained = playlist.tracks.contains(&track.id);
                    let should_include = !is_rating && (is_current || contained);
                    should_include.then(|| (playlist.clone(), contained))
                })
                .sorted_by(|(a, _), (b, _)| a.name.cmp(&b.name))
                .map(|(playlist, contained)| IconEntry::Playlist {
                    playlist,
                    contained,
                }),
        );
        let num_icons = icon_entries.len();
        if width < icon_size * num_icons as f64 {
            return;
        }

        let mut hover_rating_index: Option<usize> = None;
        let inv_scale = 1.0 / self.scale_factor;
        let base_y = height * 0.8;
        let icon_center_y = base_y - star_size_border + icon_total_size * 0.5;
        let center_x = pos_x + width * 0.5;
        let half_icons = num_icons as f64 / 2.0;

        for (i, entry) in icon_entries.iter().enumerate() {
            let offset = (i as f64 - half_icons) * (icon_size + icon_spacing);
            let icon_origin_x = center_x + offset;
            let button_rect = Rect::new(
                (icon_origin_x - star_size_border) * inv_scale,
                (base_y - star_size_border) * inv_scale,
                (icon_origin_x - star_size_border + icon_total_size) * inv_scale,
                (base_y - star_size_border + icon_total_size) * inv_scale,
            );
            if button_rect.contains(pointer_point)
                && let IconEntry::Star { index } = entry
            {
                let rect_center_x = (button_rect.x0 + button_rect.x1) * 0.5;
                let left_half = pointer_point.x < rect_center_x;
                hover_rating_index = Some(*index * 2 + if left_half { 1 } else { 2 });
                break;
            }
        }

        let display_rating_index = hover_rating_index.unwrap_or(track_rating_index);
        let display_full_stars = display_rating_index / 2;
        let display_has_half = display_rating_index % 2 == 1;

        let star_border_scale = (icon_size + star_size_border * 2.0) / *STAR_IMAGE_SIZE;
        let star_fill_scale = icon_size / *STAR_IMAGE_SIZE;
        for (i, entry) in icon_entries.into_iter().enumerate() {
            let offset = (i as f64 - half_icons) * (icon_size + icon_spacing);
            let icon_origin_x = center_x + offset;
            let base_transform = Affine::translate((icon_origin_x, base_y));
            let button_rect = Rect::new(
                (icon_origin_x - star_size_border) * inv_scale,
                (base_y - star_size_border) * inv_scale,
                (icon_origin_x - star_size_border + icon_total_size) * inv_scale,
                (base_y - star_size_border + icon_total_size) * inv_scale,
            );
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
                    self.scene.append(
                        &STAR_IMAGES[1],
                        Some(
                            combined_transform
                                * Affine::translate((-star_size_border, -star_size_border))
                                * Affine::scale(star_border_scale),
                        ),
                    );

                    let fill_transform = combined_transform * Affine::scale(star_fill_scale);
                    if index < display_full_stars {
                        self.scene.append(&STAR_IMAGES[2], Some(fill_transform));
                    } else {
                        self.scene.append(&STAR_IMAGES[0], Some(fill_transform));
                    }
                    if index == display_full_stars && display_has_half {
                        self.scene.append(&STAR_IMAGES[3], Some(fill_transform));
                    }
                    self.icon_hitboxes.push(IconHitbox {
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
                    if let Some(playlist_image) = IMAGES_CACHE.get(&playlist.image_url) {
                        let icon_transform = combined_transform
                            * Affine::translate((-star_size_border, -star_size_border));
                        let playlist_icon_size = icon_total_size;
                        self.scene.push_clip_layer(
                            icon_transform,
                            &RoundedRect::new(
                                0.0,
                                0.0,
                                playlist_icon_size,
                                playlist_icon_size,
                                10.0,
                            ),
                        );
                        let zoom_pixels = 16.0;
                        let image_size = f64::from(playlist_image.width);
                        self.scene.fill(
                            Fill::NonZero,
                            icon_transform
                                * Affine::translate((-zoom_pixels, -zoom_pixels))
                                * Affine::scale(
                                    (playlist_icon_size + zoom_pixels * 2.0) / image_size,
                                ),
                            &ImageBrush::new(playlist_image.clone()),
                            None,
                            &Rect::new(0.0, 0.0, image_size, image_size),
                        );
                        if !contained {
                            self.scene.fill(
                                Fill::NonZero,
                                icon_transform,
                                Color::from_rgba8(60, 60, 60, 180),
                                None,
                                &Rect::new(0.0, 0.0, playlist_icon_size, playlist_icon_size),
                            );
                        }
                        self.scene.pop_layer();
                        self.icon_hitboxes.push(IconHitbox {
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
}

/// Skip to the specified track in the queue.
pub async fn skip_to_track(track_id: TrackId<'static>, point: Point, rect: Rect) {
    let Some(_interaction_guard) = SpotifyInteractionGuard::try_acquire() else {
        warn!("Spotify interaction already in progress; skip_to_track returning early");
        return;
    };

    let playback_state = PLAYBACK_STATE.lock().clone();
    let queue_index = playback_state.queue_index;
    let Some(position_in_queue) = playback_state.queue.iter().position(|t| t.id == track_id) else {
        error!("Track not found in queue");
        return;
    };
    match queue_index.cmp(&position_in_queue) {
        Ordering::Equal => {
            let position = (point.x - rect.x0) / rect.width();
            let song_ms = playback_state.queue[position_in_queue].milliseconds;
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
            if let Err(err) = SPOTIFY_CLIENT
                .get()
                .unwrap()
                .seek_track(TimeDelta::milliseconds(milliseconds as i64), None)
                .await
            {
                error!("Failed to seek track: {err}");
            }
        }
        Ordering::Greater => {
            let position_difference = queue_index - position_in_queue;
            info!("Rewinding to track {track_id}, {position_difference} skips");
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().previous_track(None).await {
                    error!("Failed to skip to track: {err}");
                }
            }
        }
        Ordering::Less => {
            let position_difference = position_in_queue - queue_index;
            info!("Skipping to track {track_id}, {position_difference} skips");
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().next_track(None).await {
                    error!("Failed to skip to track: {err}");
                }
            }
        }
    }

    update_state_from_spotify(false).await;
}

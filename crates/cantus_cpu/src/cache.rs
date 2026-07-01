use crate::{AlbumId, CantusApp, NUM_SWATCHES, PlaybackState, pipelines::IMAGE_SIZE, render};
use image::RgbaImage;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Default)]
pub struct AppCaches {
    pub images: HashMap<String, Arc<RgbaImage>>,
    pub album_palettes: HashMap<AlbumId, [u32; NUM_SWATCHES]>,
}

#[derive(Default)]
pub struct ImageCacheState {
    pub in_flight: HashSet<String>,
    pub failed: HashSet<String>,
    pub dirty: bool,
}

impl ImageCacheState {
    pub const fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn apply_decoded_image(
        &mut self,
        caches: &mut AppCaches,
        playback: &PlaybackState,
        url: String,
        image: image::DynamicImage,
    ) {
        let image = if image.width() != IMAGE_SIZE || image.height() != IMAGE_SIZE {
            image.resize_to_fill(
                IMAGE_SIZE,
                IMAGE_SIZE,
                image::imageops::FilterType::Lanczos3,
            )
        } else {
            image
        };
        self.finish(&url);
        if Self::required_image_urls(playback).contains(url.as_str()) {
            caches.images.insert(url, Arc::new(image.to_rgba8()));
            render::update_color_palettes(caches, &playback.queue);
        }
    }

    pub fn fail(&mut self, url: String) {
        self.in_flight.remove(&url);
        self.failed.insert(url);
    }

    pub fn sync(
        &mut self,
        caches: &mut AppCaches,
        playback: &PlaybackState,
        mut download: impl FnMut(String),
    ) {
        let required: HashSet<String> = Self::required_image_urls(playback)
            .into_iter()
            .map(str::to_owned)
            .collect();
        let album_ids: HashSet<_> = playback
            .queue
            .iter()
            .filter_map(|track| track.album.id)
            .collect();
        caches.images.retain(|url, _| required.contains(url));
        caches.album_palettes.retain(|id, _| album_ids.contains(id));
        self.in_flight.retain(|url| required.contains(url));
        self.failed.retain(|url| required.contains(url));

        for url in required {
            if !caches.images.contains_key(&url)
                && !self.failed.contains(&url)
                && self.in_flight.insert(url.clone())
            {
                download(url);
            }
        }
    }

    fn finish(&mut self, url: &str) {
        self.in_flight.remove(url);
        self.failed.remove(url);
    }

    fn required_image_urls(playback: &PlaybackState) -> HashSet<&str> {
        playback
            .queue
            .iter()
            .filter_map(|track| track.album.image.as_deref())
            .chain(
                playback
                    .playlists
                    .values()
                    .filter_map(|playlist| playlist.image_url.as_deref()),
            )
            .collect()
    }
}

pub fn cache_decoded_image(app: &mut CantusApp, url: String, image: image::DynamicImage) {
    app.image_cache
        .apply_decoded_image(&mut app.caches, &app.playback_state, url, image);
}

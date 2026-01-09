use crate::spotify::{
    ALBUM_DATA_CACHE, ARTIST_DATA_CACHE, AlbumData, IMAGES_CACHE, PLAYBACK_STATE,
};
use auto_palette::Palette;
use image::RgbaImage;
use itertools::Itertools;

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

/// Downloads and caches an image from the given URL.
pub fn update_color_palettes() {
    let state = PLAYBACK_STATE.read();
    for track in &state.queue {
        if ALBUM_DATA_CACHE.contains_key(&track.album.id) {
            continue;
        }
        let Some(image_ref) = IMAGES_CACHE.get(&track.album.image) else {
            continue;
        };
        let Some(album_image) = image_ref.as_ref() else {
            continue;
        };
        let Some(artist_image_url_ref) = ARTIST_DATA_CACHE
            .get(&track.artist.id)
            .map(|entry| entry.value().clone())
        else {
            continue;
        };
        ALBUM_DATA_CACHE.insert(track.album.id, None);

        let width = album_image.width();
        let height = album_image.height();

        let get_swatches = |img_data| {
            let palette: Palette<f64> = Palette::builder()
                .algorithm(auto_palette::Algorithm::SLIC)
                .filter(ChromaFilter { threshold: 30 })
                .build(&img_data)
                .unwrap();
            palette
                .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                .or_else(|_| palette.find_swatches(NUM_SWATCHES))
                .unwrap()
        };

        let mut swatches = get_swatches(
            auto_palette::ImageData::new(width, height, album_image.as_ref()).unwrap(),
        );
        if swatches.len() < NUM_SWATCHES
            && let Some(artist_image_url) = artist_image_url_ref.as_ref()
        {
            let Some(artist_image_ref) = IMAGES_CACHE.get(artist_image_url) else {
                ALBUM_DATA_CACHE.remove(&track.album.id);
                continue;
            };
            let Some(artist_image) = artist_image_ref.as_ref() else {
                ALBUM_DATA_CACHE.remove(&track.album.id);
                continue;
            };
            let artist_new_width = (width as f32 * 0.1).round() as u32;
            let mut new_img = RgbaImage::new(width + artist_new_width, height);
            image::imageops::overlay(&mut new_img, album_image.as_ref(), 0, 0);
            let artist_img_resized = image::imageops::resize(
                artist_image.as_ref(),
                artist_new_width,
                height,
                image::imageops::FilterType::Nearest,
            );
            image::imageops::overlay(&mut new_img, &artist_img_resized, i64::from(width), 0);

            swatches = get_swatches(
                auto_palette::ImageData::new(new_img.width(), new_img.height(), &new_img).unwrap(),
            );
        }

        let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
        let primary_colors = swatches
            .iter()
            .map(|s| {
                let rgb = s.color().to_rgb();
                let ratio = ((s.ratio() / total_ratio_sum) as f32 * 255.0).round() as u8;
                [rgb.r, rgb.g, rgb.b, ratio]
            })
            .sorted_by(|a, b| b[3].cmp(&a[3]))
            .collect::<Vec<_>>();

        ALBUM_DATA_CACHE.insert(track.album.id, Some(AlbumData { primary_colors }));
    }
    drop(state);
}

/// A filter that filters chroma values.
#[derive(Debug)]
struct ChromaFilter {
    threshold: u8,
}
impl auto_palette::Filter for ChromaFilter {
    fn test(&self, pixel: &auto_palette::Rgba) -> bool {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max - min) > self.threshold
    }
}

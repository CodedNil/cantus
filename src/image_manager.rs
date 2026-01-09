use crate::spotify::IMAGES_CACHE;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use wgpu::{
    Device, Extent3d, Origin3d, Queue, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};

const MAX_TEXTURE_LAYERS: u32 = 48;
const IMAGE_SIZE: u32 = 64;

pub struct ImageManager {
    queue: Arc<Queue>,
    texture_array: Texture,
    last_set: HashSet<String>,
    pub url_to_index: HashMap<String, i32>,
}

impl ImageManager {
    pub fn new(device: &Arc<Device>, queue: Arc<Queue>) -> Self {
        let texture_array = device.create_texture(&TextureDescriptor {
            label: Some("Image Manager Texture Array"),
            size: Extent3d {
                width: IMAGE_SIZE,
                height: IMAGE_SIZE,
                depth_or_array_layers: MAX_TEXTURE_LAYERS,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Self {
            queue,
            texture_array,
            last_set: HashSet::new(),
            url_to_index: HashMap::new(),
        }
    }

    /// Prepare indices based on requested URLs. Returns true if the texture array was rebuilt.
    pub fn update(&mut self, requested_urls: &HashSet<String>) -> bool {
        // Filter requested_urls to only those that are actually in the IMAGES_CACHE
        let available_urls: HashSet<String> = requested_urls
            .iter()
            .filter(|url| IMAGES_CACHE.contains_key(*url))
            .cloned()
            .collect();

        if self.last_set == available_urls {
            return false;
        }

        if available_urls.is_empty() {
            self.last_set.clear();
            return false;
        }

        let mut sorted_urls: Vec<_> = available_urls.iter().cloned().collect();
        sorted_urls.sort();

        let mut next_url_to_index = HashMap::new();
        let mut images_data = Vec::new();

        for url in &sorted_urls {
            if let Some(img_ref) = IMAGES_CACHE.get(url)
                && let Some(image) = img_ref.as_ref()
            {
                next_url_to_index.insert(url.clone(), images_data.len() as i32);
                images_data.push(image.clone());
            }
        }

        if images_data.is_empty() {
            // Don't update last_set if we didn't actually load anything,
            // to allow trying again next frame.
            return false;
        }

        for (i, img) in images_data
            .iter()
            .take(MAX_TEXTURE_LAYERS as usize)
            .enumerate()
        {
            self.queue.write_texture(
                TexelCopyTextureInfo {
                    texture: &self.texture_array,
                    mip_level: 0,
                    origin: Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                    aspect: TextureAspect::All,
                },
                img.as_raw(),
                TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * IMAGE_SIZE),
                    rows_per_image: Some(IMAGE_SIZE),
                },
                Extent3d {
                    width: IMAGE_SIZE,
                    height: IMAGE_SIZE,
                    depth_or_array_layers: 1,
                },
            );
        }

        self.url_to_index = next_url_to_index;
        self.last_set = available_urls;
        true
    }

    pub fn create_view(&self) -> TextureView {
        self.texture_array.create_view(&TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        })
    }
}

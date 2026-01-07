use crate::spotify::IMAGES_CACHE;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use wgpu::{
    Device, Extent3d, Origin3d, Queue, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};

pub struct ImageManager {
    device: Arc<Device>,
    queue: Arc<Queue>,
    texture_array: Option<Texture>,
    last_set: HashSet<String>,
    pub url_to_index: HashMap<String, i32>,
}

impl ImageManager {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self {
            device,
            queue,
            texture_array: None,
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

        let mut sorted_urls: Vec<_> = available_urls.iter().cloned().collect();
        sorted_urls.sort();

        let mut next_url_to_index = HashMap::new();
        let mut images_data = Vec::new();

        for url in &sorted_urls {
            if let Some(img_ref) = IMAGES_CACHE.get(url)
                && let Some(img_brush) = img_ref.as_ref()
            {
                next_url_to_index.insert(url.clone(), images_data.len() as i32);
                images_data.push(img_brush.image.clone());
            }
        }

        if images_data.is_empty() {
            // Don't update last_set if we didn't actually load anything,
            // to allow trying again next frame.
            return false;
        }

        let width = images_data[0].width;
        let height = images_data[0].height;
        let layers = images_data.len() as u32;

        let texture = self.device.create_texture(&TextureDescriptor {
            label: Some("Image Manager Texture Array"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        for (i, img) in images_data.iter().enumerate() {
            self.queue.write_texture(
                TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                    aspect: TextureAspect::All,
                },
                img.data.as_ref(),
                TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * width),
                    rows_per_image: Some(height),
                },
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }

        self.texture_array = Some(texture);
        self.url_to_index = next_url_to_index;
        self.last_set = available_urls;
        true
    }

    pub fn create_view(&self) -> Option<TextureView> {
        self.texture_array.as_ref().map(|t| {
            t.create_view(&TextureViewDescriptor {
                dimension: Some(TextureViewDimension::D2Array),
                ..Default::default()
            })
        })
    }
}

use super::context::{Context, Inner};
use crate::data::ImageHandle;
use ash::vk;
use std::{
    cell::Cell,
    rc::{Rc, Weak as RcWeak},
    sync::{Arc, Weak},
    vec::Vec,
};

const IMAGE_BLOCK_BYTES: u64 = 4 * 1024 * 1024;
const CACHE_LAYERS: usize = 64;

#[derive(Default)]
pub(super) struct ImageAllocator {
    blocks: Vec<ImageMemoryBlock>,
}

struct ImageMemoryBlock {
    memory: vk::DeviceMemory,
    kind: u32,
    size: u64,
    cursor: u64,
}

impl ImageAllocator {
    pub(super) fn allocate(&mut self, device: &ash::Device, kind: u32, requirements: vk::MemoryRequirements) -> Result<(vk::DeviceMemory, u64), vk::Result> {
        if let Some((block, offset)) = self.blocks.iter_mut().find_map(|block| {
            let offset = block.cursor.next_multiple_of(requirements.alignment);
            (block.kind == kind && offset + requirements.size <= block.size).then_some((block, offset))
        }) {
            block.cursor = offset + requirements.size;
            return Ok((block.memory, offset));
        }
        let size = IMAGE_BLOCK_BYTES.max(requirements.size);
        let memory = unsafe { device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(size).memory_type_index(kind), None) }?;
        self.blocks.push(ImageMemoryBlock {
            memory,
            kind,
            size,
            cursor: requirements.size,
        });
        Ok((memory, 0))
    }

    pub(super) fn destroy(&mut self, device: &ash::Device) {
        for block in self.blocks.drain(..) {
            unsafe { device.free_memory(block.memory, None) };
        }
    }
}

pub(super) struct Image {
    inner: Rc<Inner>,
    pub(super) raw: vk::Image,
    pub(super) view: vk::ImageView,
    pub(super) layers: u32,
    pub(super) initialized: Cell<bool>,
}

pub(super) struct ImageUpload {
    pub(super) image: RcWeak<Image>,
    pub(super) origin: [u32; 3],
    pub(super) size: [u32; 2],
    pub(super) pixels: Arc<[u8]>,
}
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            self.inner.device.destroy_image_view(self.view, None);
            self.inner.device.destroy_image(self.raw, None);
        }
    }
}
#[derive(Clone, Copy)]
struct Extent3d {
    width: u32,
    height: u32,
    depth_or_array_layers: u32,
}
struct SampledTexture {
    context: Context,
    pub(crate) image: Rc<Image>,
    size: Extent3d,
}

pub(super) struct ImageCache {
    textures: Vec<CachedTexture>,
    frame: u64,
    capacity: u32,
}

impl ImageCache {
    pub(crate) fn new(capacity: u32) -> Self {
        Self {
            textures: Vec::with_capacity(capacity as usize),
            frame: 0,
            capacity,
        }
    }

    pub(crate) fn image(&mut self, context: &Context, size: [u32; 2], pixels: &Arc<[u8]>) -> ImageHandle {
        let source = Arc::downgrade(pixels);
        for texture in &mut self.textures {
            if texture.size == size
                && let Some(layer) = texture
                    .layers
                    .iter_mut()
                    .position(|layer| layer.source.as_ref().is_some_and(|cached| cached.ptr_eq(&source)))
            {
                texture.layers[layer].used = self.frame;
                return ImageHandle::new(texture.index, layer as u32);
            }
        }
        for texture in &mut self.textures {
            if texture.size == size
                && let Some(layer) = texture.layers.iter_mut().position(|layer| layer.used != self.frame)
                && texture.texture.write([0, 0, layer as u32], size, Arc::clone(pixels))
            {
                texture.layers[layer] = CachedLayer {
                    source: Some(source),
                    used: self.frame,
                };
                return ImageHandle::new(texture.index, layer as u32);
            }
        }
        assert!(self.textures.len() < self.capacity as usize, "image descriptor capacity exceeded");
        let index = self.textures.len() as u32;
        let texture = SampledTexture::new(
            context,
            Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: CACHE_LAYERS as u32,
            },
        );
        assert!(texture.write([0, 0, 0], size, Arc::clone(pixels)), "validated image upload was rejected");
        let mut layers = Vec::with_capacity(CACHE_LAYERS);
        layers.push(CachedLayer {
            source: Some(source),
            used: self.frame,
        });
        layers.resize_with(CACHE_LAYERS, CachedLayer::default);
        self.textures.push(CachedTexture { texture, size, index, layers });
        ImageHandle::new(index, 0)
    }

    pub(crate) const fn begin_frame(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    pub(crate) fn images(&self) -> impl Iterator<Item = &Rc<Image>> {
        self.textures.iter().map(|texture| &texture.texture.image)
    }
}

struct CachedTexture {
    texture: SampledTexture,
    size: [u32; 2],
    index: u32,
    layers: Vec<CachedLayer>,
}

#[derive(Default)]
struct CachedLayer {
    source: Option<Weak<[u8]>>,
    used: u64,
}
impl SampledTexture {
    fn new(context: &Context, size: Extent3d) -> Self {
        let inner = context.0.clone();
        let vk_format = vk::Format::R8G8B8A8_UNORM;
        let raw = unsafe {
            inner.device.create_image(
                &vk::ImageCreateInfo::default()
                    .image_type(vk::ImageType::TYPE_2D)
                    .format(vk_format)
                    .extent(vk::Extent3D {
                        width: size.width,
                        height: size.height,
                        depth: 1,
                    })
                    .mip_levels(1)
                    .array_layers(size.depth_or_array_layers)
                    .samples(vk::SampleCountFlags::TYPE_1)
                    .tiling(vk::ImageTiling::OPTIMAL)
                    .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None,
            )
        }
        .expect("create Vulkan image");
        let requirements = unsafe { inner.device.get_image_memory_requirements(raw) };
        let kind = context
            .memory_type(requirements.memory_type_bits, vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .expect("image memory type");
        let (memory, offset) = context.allocate_image_memory(kind, requirements).expect("allocate image memory");
        unsafe {
            inner.device.bind_image_memory(raw, memory, offset).expect("bind image memory");
        }
        let range = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            level_count: 1,
            layer_count: size.depth_or_array_layers,
            ..Default::default()
        };
        let view = unsafe {
            inner.device.create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(raw)
                    .view_type(vk::ImageViewType::TYPE_2D_ARRAY)
                    .format(vk_format)
                    .subresource_range(range),
                None,
            )
        }
        .expect("create image view");
        let image = Rc::new(Image {
            inner,
            raw,
            view,
            layers: size.depth_or_array_layers,
            initialized: Cell::new(false),
        });
        Self {
            context: context.clone(),
            image,
            size,
        }
    }
    fn write(&self, [x, y, layer]: [u32; 3], [width, height]: [u32; 2], data: impl Into<Arc<[u8]>>) -> bool {
        let data = data.into();
        if width == 0 || height == 0 {
            return false;
        }
        if x + width > self.size.width || y + height > self.size.height || layer >= self.size.depth_or_array_layers {
            return false;
        }
        if data.len() != (width * height * 4) as usize {
            return false;
        }
        self.context.queue_image(ImageUpload {
            image: Rc::downgrade(&self.image),
            origin: [x, y, layer],
            size: [width, height],
            pixels: data,
        });
        true
    }
}

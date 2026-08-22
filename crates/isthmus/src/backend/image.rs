use super::context::{Context, Inner, transition_image};
use crate::ImageHandle;
use ash::vk;
use core::{error::Error, fmt};
use std::{
    borrow::ToOwned,
    cell::{Cell, Ref, RefCell},
    rc::Rc,
    string::String,
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};

const IMAGE_BLOCK_BYTES: u64 = 4 * 1024 * 1024;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureWriteError {
    EmptyRegion,
    OutOfBounds,
    InvalidDataLength { expected: usize, actual: usize },
}
impl fmt::Display for TextureWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for TextureWriteError {}

pub struct Image {
    inner: Arc<Inner>,
    pub raw: vk::Image,
    pub view: vk::ImageView,
    pub layers: u32,
}

pub struct ImageUpload {
    pub image: Weak<Image>,
    pub origin: [u32; 3],
    pub size: [u32; 2],
    pub pixels: Arc<[u8]>,
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
pub struct SampledTexture {
    context: Context,
    pub(crate) image: Arc<Image>,
    size: Extent3d,
    bytes: u32,
}

#[derive(Clone)]
pub struct TextureHeap {
    images: Rc<RefCell<Vec<Arc<Image>>>>,
    frame: Rc<Cell<u64>>,
    capacity: u32,
}

impl TextureHeap {
    pub(crate) fn new(capacity: u32) -> Self {
        Self {
            images: Rc::new(RefCell::new(Vec::with_capacity(capacity as usize))),
            frame: Rc::new(Cell::new(0)),
            capacity,
        }
    }

    pub(crate) fn insert(&self, image: Arc<Image>) -> u32 {
        let mut images = self.images.borrow_mut();
        assert!(images.len() < self.capacity as usize, "texture heap is full");
        let index = images.len() as u32;
        images.push(image);
        index
    }

    pub(crate) fn images(&self) -> Ref<'_, [Arc<Image>]> {
        Ref::map(self.images.borrow(), Vec::as_slice)
    }

    pub(crate) fn begin_frame(&self) {
        self.frame.set(self.frame.get().wrapping_add(1));
    }
}

pub struct ImageArray {
    texture: SampledTexture,
    index: u32,
    layers: Vec<Layer>,
    frame: Rc<Cell<u64>>,
}

#[derive(Clone)]
enum Layer {
    Available,
    Permanent,
    Cached { key: String, used: u64 },
}

impl ImageArray {
    pub(crate) fn new(context: &Context, heap: &TextureHeap, size: [u32; 2], layers: usize) -> Self {
        let texture = SampledTexture::new(
            context,
            Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: layers as u32,
            },
        );
        let index = heap.insert(texture.image.clone());
        Self {
            texture,
            index,
            layers: vec![Layer::Available; layers],
            frame: Rc::clone(&heap.frame),
        }
    }

    pub fn cache(&mut self, key: &str, size: [u32; 2], pixels: &Arc<[u8]>) -> Option<ImageHandle> {
        let frame = self.frame.get();
        let existing = self.layers.iter().position(|layer| matches!(layer, Layer::Cached { key: cached, .. } if cached == key));
        if let Some(layer) = existing {
            if let Layer::Cached { used, .. } = &mut self.layers[layer] {
                *used = frame;
            }
            return Some(ImageHandle::new(self.index, layer as u32));
        }
        let layer = self.available(frame)?;
        self.texture.write([0, 0, layer as u32], size, Arc::clone(pixels)).ok()?;
        self.layers[layer] = Layer::Cached { key: key.to_owned(), used: frame };
        Some(ImageHandle::new(self.index, layer as u32))
    }

    /// # Errors
    /// Returns an error when the array is full or the pixel data is invalid.
    pub fn insert(&mut self, size: [u32; 2], data: Vec<u8>) -> Result<ImageHandle, TextureWriteError> {
        let frame = self.frame.get();
        let layer = self.available(frame).ok_or(TextureWriteError::OutOfBounds)?;
        self.texture.write([0, 0, layer as u32], size, data)?;
        self.layers[layer] = Layer::Permanent;
        Ok(ImageHandle::new(self.index, layer as u32))
    }

    fn available(&self, frame: u64) -> Option<usize> {
        self.layers.iter().position(|layer| match layer {
            Layer::Available => true,
            Layer::Cached { used, .. } => *used != frame,
            Layer::Permanent => false,
        })
    }
}
impl SampledTexture {
    fn new(context: &Context, size: Extent3d) -> Self {
        let inner = context.inner();
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
        let image = Arc::new(Image {
            inner,
            raw,
            view,
            layers: size.depth_or_array_layers,
        });
        let result = Self {
            context: context.clone(),
            image,
            size,
            bytes: 4,
        };
        result.transition(vk::ImageLayout::UNDEFINED, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        result
    }
    fn transition(&self, old: vk::ImageLayout, new: vk::ImageLayout) {
        let image = self.image.raw;
        let layers = self.size.depth_or_array_layers;
        self.context
            .immediate(|command| transition_image(&self.context.0.device, command, image, old, new, layers))
            .expect("transition Vulkan image");
    }
    fn write(&self, [x, y, layer]: [u32; 3], [width, height]: [u32; 2], data: impl Into<Arc<[u8]>>) -> Result<(), TextureWriteError> {
        let data = data.into();
        if width == 0 || height == 0 {
            return Err(TextureWriteError::EmptyRegion);
        }
        if x + width > self.size.width || y + height > self.size.height || layer >= self.size.depth_or_array_layers {
            return Err(TextureWriteError::OutOfBounds);
        }
        let expected = (width * height * self.bytes) as usize;
        if data.len() != expected {
            return Err(TextureWriteError::InvalidDataLength { expected, actual: data.len() });
        }
        self.context.queue_image(ImageUpload {
            image: Arc::downgrade(&self.image),
            origin: [x, y, layer],
            size: [width, height],
            pixels: data,
        });
        Ok(())
    }
}

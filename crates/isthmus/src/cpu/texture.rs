use super::{
    buffer::Buffer,
    context::{Context, Inner, SetupError, transition_image},
};
use crate::contract::{Texture2D, Texture2DArray};
use ash::vk;
use core::{error::Error, fmt, marker::PhantomData};
use std::sync::Arc;

pub trait SampledTextureDimension: private::Sealed {
    const VIEW: vk::ImageViewType;
}
mod private {
    pub trait Sealed {}
}
impl private::Sealed for Texture2D {}
impl SampledTextureDimension for Texture2D {
    const VIEW: vk::ImageViewType = vk::ImageViewType::TYPE_2D;
}
impl private::Sealed for Texture2DArray {}
impl SampledTextureDimension for Texture2DArray {
    const VIEW: vk::ImageViewType = vk::ImageViewType::TYPE_2D_ARRAY;
}

#[derive(Clone, Copy)]
pub enum FilterableFloatFormat {
    R8Unorm,
    Rgba8Unorm,
}
impl FilterableFloatFormat {
    const fn vk(self) -> vk::Format {
        match self {
            Self::R8Unorm => vk::Format::R8_UNORM,
            Self::Rgba8Unorm => vk::Format::R8G8B8A8_UNORM,
        }
    }
    const fn bytes(self) -> u32 {
        match self {
            Self::R8Unorm => 1,
            Self::Rgba8Unorm => 4,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureWriteError {
    EmptyRegion,
    OutOfBounds,
    InvalidDataLength { expected: usize, actual: usize },
    Vulkan(vk::Result),
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
    memory: vk::DeviceMemory,
    pub view: vk::ImageView,
    pub format: vk::Format,
}
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            self.inner.device.destroy_image_view(self.view, None);
            self.inner.device.destroy_image(self.raw, None);
            self.inner.device.free_memory(self.memory, None);
        }
    }
}
pub struct TextureView<D> {
    pub(crate) image: Arc<Image>,
    marker: PhantomData<D>,
}
impl<D> Clone for TextureView<D> {
    fn clone(&self) -> Self {
        Self {
            image: self.image.clone(),
            marker: PhantomData,
        }
    }
}
impl<D> TextureView<D> {
    pub fn raw(&self) -> vk::ImageView {
        self.image.view
    }
}
#[derive(Clone, Copy)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}
pub struct SampledTexture<D> {
    context: Context,
    image: Arc<Image>,
    view: TextureView<D>,
    size: Extent3d,
    bytes: u32,
}
impl<D: SampledTextureDimension> SampledTexture<D> {
    pub(crate) fn new(context: &Context, _label: &str, size: Extent3d, format: FilterableFloatFormat) -> Self {
        let inner = context.inner();
        let vk_format = format.vk();
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
        let memory = unsafe {
            inner
                .device
                .allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(kind), None)
        }
        .expect("allocate image memory");
        unsafe {
            inner.device.bind_image_memory(raw, memory, 0).expect("bind image memory");
        }
        let range = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            level_count: 1,
            layer_count: size.depth_or_array_layers,
            ..Default::default()
        };
        let view = unsafe {
            inner.device.create_image_view(
                &vk::ImageViewCreateInfo::default().image(raw).view_type(D::VIEW).format(vk_format).subresource_range(range),
                None,
            )
        }
        .expect("create image view");
        let image = Arc::new(Image {
            inner,
            raw,
            memory,
            view,
            format: vk_format,
        });
        let typed = TextureView {
            image: image.clone(),
            marker: PhantomData,
        };
        let result = Self {
            context: context.clone(),
            image,
            view: typed,
            size,
            bytes: format.bytes(),
        };
        result.transition(vk::ImageLayout::UNDEFINED, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        result
    }
    pub const fn view(&self) -> &TextureView<D> {
        &self.view
    }
    fn transition(&self, old: vk::ImageLayout, new: vk::ImageLayout) {
        let image = self.image.raw;
        let layers = self.size.depth_or_array_layers;
        self.context
            .immediate(|command| unsafe { transition_image(&self.context.0.device, command, image, old, new, layers) })
            .expect("transition Vulkan image");
    }
    /// Uploads one tightly packed region into a texture-array layer.
    ///
    /// # Errors
    /// Returns an error for empty or out-of-bounds regions, mismatched byte
    /// lengths, or a Vulkan staging/copy failure.
    pub fn write(&self, [x, y, layer]: [u32; 3], [width, height]: [u32; 2], data: &[u8]) -> Result<(), TextureWriteError> {
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
        let staging = Buffer::new(&self.context, data.len(), vk::BufferUsageFlags::TRANSFER_SRC).map_err(|e| match e {
            SetupError::Vulkan(v) => TextureWriteError::Vulkan(v),
            _ => TextureWriteError::OutOfBounds,
        })?;
        staging.write(data);
        let image = self.image.raw;
        self.context
            .immediate(|command| unsafe {
                transition_image(
                    &self.context.0.device,
                    command,
                    image,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    self.size.depth_or_array_layers,
                );
                self.context.0.device.cmd_copy_buffer_to_image(
                    command,
                    staging.raw(),
                    image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[vk::BufferImageCopy::default()
                        .image_subresource(vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            mip_level: 0,
                            base_array_layer: layer,
                            layer_count: 1,
                        })
                        .image_offset(vk::Offset3D { x: x as i32, y: y as i32, z: 0 })
                        .image_extent(vk::Extent3D { width, height, depth: 1 })],
                );
                transition_image(
                    &self.context.0.device,
                    command,
                    image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    self.size.depth_or_array_layers,
                );
            })
            .map_err(|e| match e {
                SetupError::Vulkan(v) => TextureWriteError::Vulkan(v),
                _ => TextureWriteError::OutOfBounds,
            })?;
        Ok(())
    }
}

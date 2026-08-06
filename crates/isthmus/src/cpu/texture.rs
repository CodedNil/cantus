use crate::{Texture2D, Texture2DArray, TextureView};
use core::{error::Error, fmt};
use wgpu::{
    Device, Extent3d, Origin3d, Queue, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};

mod dimension {
    pub trait Sealed {}
}

pub trait SampledTextureDimension: dimension::Sealed {
    const VIEW: Option<TextureViewDimension>;
}

#[derive(Clone, Copy)]
pub enum FilterableFloatFormat {
    R8Unorm,
    Rgba8Unorm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureWriteError {
    EmptyRegion,
    OutOfBounds,
    InvalidDataLength { expected: usize, actual: usize },
}

impl fmt::Display for TextureWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRegion => f.write_str("texture write region is empty"),
            Self::OutOfBounds => f.write_str("texture write region is out of bounds"),
            Self::InvalidDataLength { expected, actual } => {
                write!(f, "texture write needs {expected} bytes, received {actual}")
            }
        }
    }
}

impl Error for TextureWriteError {}

/// A writable texture with a typed sampled view.
pub struct SampledTexture<D> {
    queue: Queue,
    texture: Texture,
    view: TextureView<D>,
    size: Extent3d,
    bytes_per_texel: u32,
}

impl FilterableFloatFormat {
    const fn bytes_per_texel(self) -> u32 {
        match self {
            Self::R8Unorm => 1,
            Self::Rgba8Unorm => 4,
        }
    }
}

impl From<FilterableFloatFormat> for TextureFormat {
    fn from(format: FilterableFloatFormat) -> Self {
        match format {
            FilterableFloatFormat::R8Unorm => Self::R8Unorm,
            FilterableFloatFormat::Rgba8Unorm => Self::Rgba8Unorm,
        }
    }
}

impl dimension::Sealed for Texture2D {}
impl SampledTextureDimension for Texture2D {
    const VIEW: Option<TextureViewDimension> = Some(TextureViewDimension::D2);
}

impl dimension::Sealed for Texture2DArray {}
impl SampledTextureDimension for Texture2DArray {
    const VIEW: Option<TextureViewDimension> = Some(TextureViewDimension::D2Array);
}

impl<D: SampledTextureDimension> SampledTexture<D> {
    pub(crate) fn new(
        device: &Device,
        queue: &Queue,
        label: &str,
        size: Extent3d,
        format: FilterableFloatFormat,
    ) -> Self {
        let bytes_per_texel = format.bytes_per_texel();
        let format = format.into();
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let raw = texture.create_view(&TextureViewDescriptor {
            dimension: D::VIEW,
            ..Default::default()
        });
        Self {
            queue: queue.clone(),
            texture,
            view: TextureView::new(raw),
            size,
            bytes_per_texel,
        }
    }

    pub const fn view(&self) -> &TextureView<D> {
        &self.view
    }

    /// Writes one texture region.
    ///
    /// # Errors
    /// Returns an error if the region or data length is invalid.
    pub fn write(
        &self,
        [x, y, layer]: [u32; 3],
        [width, height]: [u32; 2],
        data: &[u8],
    ) -> Result<(), TextureWriteError> {
        if width == 0 || height == 0 {
            return Err(TextureWriteError::EmptyRegion);
        }
        if x.checked_add(width).is_none_or(|end| end > self.size.width)
            || y.checked_add(height).is_none_or(|end| end > self.size.height)
            || layer >= self.size.depth_or_array_layers
        {
            return Err(TextureWriteError::OutOfBounds);
        }
        let expected = width
            .checked_mul(height)
            .and_then(|texels| texels.checked_mul(self.bytes_per_texel))
            .and_then(|bytes| usize::try_from(bytes).ok())
            .ok_or(TextureWriteError::OutOfBounds)?;
        if data.len() != expected {
            return Err(TextureWriteError::InvalidDataLength {
                expected,
                actual: data.len(),
            });
        }

        self.queue.write_texture(
            TexelCopyTextureInfo {
                origin: Origin3d { x, y, z: layer },
                ..self.texture.as_image_copy()
            },
            data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * self.bytes_per_texel),
                rows_per_image: Some(height),
            },
            Extent3d {
                width,
                height,
                ..Default::default()
            },
        );
        Ok(())
    }
}

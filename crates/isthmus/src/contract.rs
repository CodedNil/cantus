use crate::ShaderData;
use spirv_std::image::{Image2dArray, SampledImage};

#[doc(hidden)]
pub type Images = spirv_std::RuntimeArray<SampledImage<Image2dArray>>;

#[repr(C)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct FrameInfo {
    pub screen_size: glam::Vec2,
    pub mouse_pos: glam::Vec2,
    pub mouse_pressure: f32,
    pub time: f32,
}

pub trait FrameGlobals: ShaderData {
    fn frame_info(&self) -> &FrameInfo;
    fn frame_info_mut(&mut self) -> &mut FrameInfo;
}

impl ShaderData for FrameInfo {}

/// Samples an image handle whose descriptor index was created by Isthmus.
///
/// The unchecked descriptor-array access is confined here: `ImageHandle` is
/// opaque and host code can only construct one through Isthmus image storage.
#[doc(hidden)]
pub fn sample_image(heap: &Images, handle: crate::ImageHandle, uv: glam::Vec2) -> Option<glam::Vec4> {
    handle.parts().map(|(image, layer)| unsafe { heap.index(image as usize) }.sample(uv.extend(layer as f32)))
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Quad {
    pub center: glam::Vec2,
    pub size: glam::Vec2,
    pub axis: glam::Vec2,
}

impl ShaderData for Quad {}

#[doc(hidden)]
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct DrawRecord {
    pub quad: Quad,
    pub payload: u32,
}

impl ShaderData for DrawRecord {}

#[cfg(not(target_arch = "spirv"))]
#[doc(hidden)]
pub trait IntoQuad {
    fn into_quad(self) -> Quad;
}

#[cfg(not(target_arch = "spirv"))]
impl IntoQuad for Quad {
    fn into_quad(self) -> Quad {
        self
    }
}

impl Quad {
    pub const fn new(center: glam::Vec2, size: glam::Vec2, axis: glam::Vec2) -> Self {
        Self { center, size, axis }
    }

    pub fn from_min_max(min: glam::Vec2, max: glam::Vec2) -> Self {
        Self::new((min + max) * 0.5, max - min, glam::Vec2::X)
    }

    pub fn sample(self, vertex: u32, screen_size: glam::Vec2) -> RasterSample {
        let local = (quad_coord(vertex) - 0.5) * self.size;
        let pixel = self.center + self.axis * local.x + self.axis.perp() * local.y;
        RasterSample {
            position: pixel_to_ndc(pixel, screen_size),
            pixel,
        }
    }

    #[doc(hidden)]
    pub fn local(self, pixel: glam::Vec2) -> glam::Vec2 {
        let offset = pixel - self.center;
        glam::vec2(offset.dot(self.axis), offset.dot(self.axis.perp()))
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Fragment {
    pub pixel: glam::Vec2,
    pub local: glam::Vec2,
    pub uv: glam::Vec2,
    pub size: glam::Vec2,
}

pub struct RasterSample {
    pub position: glam::Vec4,
    pub pixel: glam::Vec2,
}

/// Coordinates for a two-triangle unit quad from its vertex index.
const fn quad_coord(vertex: u32) -> glam::Vec2 {
    glam::vec2((vertex & 1) as f32, (vertex >> 1) as f32)
}

/// Converts a top-left-origin pixel position to clip space.
///
/// Vulkan's positive-height viewport maps -1 NDC Y to the top edge, preserving
/// the renderer's logical top-left coordinate system.
fn pixel_to_ndc(pixel: glam::Vec2, screen_size: glam::Vec2) -> glam::Vec4 {
    let ndc = pixel / screen_size * 2.0 - 1.0;
    glam::vec4(ndc.x, ndc.y, 0.0, 1.0)
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurfaceHandle(pub(crate) usize);

#[cfg(not(target_arch = "spirv"))]
impl SurfaceHandle {
    pub const PRIMARY: Self = Self(0);

    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

/// Describes the host-visible interface and fixed state of one shader.
#[cfg(not(target_arch = "spirv"))]
pub trait ShaderSpec {
    type Shared: ShaderData;
    type Instance: ShaderData;
    const NAME: &'static str;
}

#[cfg(not(target_arch = "spirv"))]
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PipelineKey(usize);

#[cfg(not(target_arch = "spirv"))]
impl PipelineKey {
    #[doc(hidden)]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

#[cfg(not(target_arch = "spirv"))]
#[doc(hidden)]
pub trait PipelineIdentity {
    const KEY: PipelineKey;
}

/// Removes the crate name from a Rust module path.
#[cfg(not(target_arch = "spirv"))]
#[must_use]
pub const fn shader_module_name(path: &str) -> &str {
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i] != b':' {
        i += 1;
    }
    while i < bytes.len() && bytes[i] == b':' {
        i += 1;
    }
    path.split_at(i).1
}

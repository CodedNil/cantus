#[cfg(not(target_arch = "spirv"))]
use crate::backend::renderer::ShaderModule;
use crate::{ShaderData, data::ImageHandle};
use spirv_std::image::{Image2dArray, SampledImage};
#[cfg(not(target_arch = "spirv"))]
use std::sync::{
    OnceLock,
    atomic::{AtomicUsize, Ordering},
};

pub type ImageHeap = spirv_std::RuntimeArray<SampledImage<Image2dArray>>;

#[doc(hidden)]
pub struct ShaderImage<'a> {
    heap: &'a ImageHeap,
    handle: ImageHandle,
}

impl<'a> ShaderImage<'a> {
    pub const fn new(heap: &'a ImageHeap, handle: ImageHandle) -> Self {
        Self { heap, handle }
    }

    pub fn sample(&self, uv: glam::Vec2) -> glam::Vec4 {
        let (image, layer) = self.handle.parts();
        unsafe { self.heap.index(image as usize) }.sample(uv.extend(layer as f32))
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct FrameInfo {
    pub screen_size: glam::Vec2,
    pub mouse_pos: glam::Vec2,
    pub mouse_pressure: f32,
    pub time: f32,
}

#[cfg(not(target_arch = "spirv"))]
pub struct Program<Shared> {
    pub(crate) shader: ShaderModule,
    pub(crate) frame_info_mut: fn(&mut Shared) -> &mut FrameInfo,
}

#[cfg(not(target_arch = "spirv"))]
impl<Shared> Program<Shared> {
    pub const fn new(shader: ShaderModule, frame_info_mut: fn(&mut Shared) -> &mut FrameInfo) -> Self {
        Self { shader, frame_info_mut }
    }
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone, Copy)]
pub struct PaintPipeline {
    pub(crate) token: &'static PipelineToken,
    pub(crate) entry: &'static str,
}

#[cfg(not(target_arch = "spirv"))]
impl PaintPipeline {
    pub const fn new(token: &'static PipelineToken, entry: &'static str) -> Self {
        Self { token, entry }
    }
}

#[cfg(not(target_arch = "spirv"))]
static NEXT_PIPELINE: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(target_arch = "spirv"))]
#[derive(Default)]
pub struct PipelineToken(OnceLock<usize>);

#[cfg(not(target_arch = "spirv"))]
impl PipelineToken {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    pub(crate) fn index(&self) -> usize {
        *self.0.get_or_init(|| NEXT_PIPELINE.fetch_add(1, Ordering::Relaxed))
    }
}

unsafe impl ShaderData for FrameInfo {}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Quad {
    pub center: glam::Vec2,
    pub size: glam::Vec2,
    pub axis: glam::Vec2,
}

unsafe impl ShaderData for Quad {}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct DrawRecord {
    pub quad: Quad,
    pub payload: u32,
}

unsafe impl ShaderData for DrawRecord {}

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

    pub fn local(self, pixel: glam::Vec2) -> glam::Vec2 {
        let offset = pixel - self.center;
        glam::vec2(offset.dot(self.axis), offset.dot(self.axis.perp()))
    }

    #[must_use]
    pub fn expanded(mut self, amount: f32) -> Self {
        let expansion = glam::Vec2::splat(amount);
        self.size += expansion * 2.0;
        self
    }
}

#[derive(Clone, Copy)]
pub struct Fragment {
    pub pixel: glam::Vec2,
    pub local: glam::Vec2,
    pub uv: glam::Vec2,
}

impl Fragment {
    pub fn new(pixel: glam::Vec2, local: glam::Vec2, size: glam::Vec2) -> Self {
        Self {
            pixel,
            local,
            uv: local / size + 0.5,
        }
    }
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
pub struct SurfaceHandle {
    index: u32,
    generation: u32,
}

#[cfg(not(target_arch = "spirv"))]
impl SurfaceHandle {
    pub(crate) const fn new(index: usize, generation: u32) -> Self {
        Self { index: index as u32, generation }
    }

    pub(crate) const fn index(self) -> usize {
        self.index as usize
    }

    pub(crate) const fn generation(self) -> u32 {
        self.generation
    }
}

/// Describes the host-visible interface and fixed state of one shader.
#[cfg(not(target_arch = "spirv"))]
pub trait ShaderSpec {
    type Shared: ShaderData;
    type Instance: ShaderData;
    const PIPELINE: PaintPipeline;
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

use glam::{UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};

/// Four normalized channels stored in one 32-bit word.
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Unorm8x4(u32);

/// Compact descriptor/layer pair into the shader-visible image heap.
#[repr(transparent)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct ImageHandle(i32);

impl ImageHandle {
    #[cfg(not(target_arch = "spirv"))]
    pub(crate) const fn new(image: u32, layer: u32) -> Self {
        Self(((image & 0xff) << 23 | (layer & 0x7f_ffff)) as i32)
    }

    pub(crate) const fn parts(self) -> (u32, u32) {
        let value = self.0 as u32;
        (value >> 23, value & 0x7f_ffff)
    }
}

impl Unorm8x4 {
    pub fn from_vec4(value: Vec4) -> Self {
        let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
        Self(channel(value.x) | channel(value.y) << 8 | channel(value.z) << 16 | channel(value.w) << 24)
    }

    pub fn from_vec3(value: Vec3) -> Self {
        Self::from_vec4(value.extend(1.0))
    }

    pub fn to_vec4(self) -> Vec4 {
        #[cfg(target_arch = "spirv")]
        {
            spirv_std::float::u8x4_to_vec4_unorm(self.0)
        }
        #[cfg(not(target_arch = "spirv"))]
        {
            Vec4::new((self.0 & 255) as f32, ((self.0 >> 8) & 255) as f32, ((self.0 >> 16) & 255) as f32, (self.0 >> 24) as f32) / 255.0
        }
    }

    pub fn to_vec3(self) -> Vec3 {
        self.to_vec4().truncate()
    }
}

/// Plain data whose Rust representation is also its scalar-layout SPIR-V ABI.
///
/// # Safety
/// Implementations must have identical Rust and scalar-layout SPIR-V representations,
/// four-byte alignment or less, and a size divisible by four.
#[cfg(target_arch = "spirv")]
pub unsafe trait ShaderData: Copy {}
/// Plain data whose Rust representation is also its scalar-layout SPIR-V ABI.
///
/// # Safety
/// Implementations must have identical Rust and scalar-layout SPIR-V representations,
/// four-byte alignment or less, and a size divisible by four.
#[cfg(not(target_arch = "spirv"))]
pub unsafe trait ShaderData: Copy + bytemuck::Pod {}

unsafe impl ShaderData for u32 {}
unsafe impl ShaderData for i32 {}
unsafe impl ShaderData for f32 {}
unsafe impl ShaderData for () {}
unsafe impl ShaderData for Unorm8x4 {}
unsafe impl ShaderData for ImageHandle {}
unsafe impl ShaderData for Vec2 {}
unsafe impl ShaderData for Vec3 {}
unsafe impl ShaderData for Vec4 {}
unsafe impl ShaderData for UVec2 {}
unsafe impl ShaderData for UVec3 {}
unsafe impl ShaderData for UVec4 {}
#[cfg(target_arch = "spirv")]
unsafe impl<T: ShaderData, const N: usize> ShaderData for [T; N] {}
#[cfg(not(target_arch = "spirv"))]
unsafe impl<T: ShaderData, const N: usize> ShaderData for [T; N] where [T; N]: bytemuck::Pod {}

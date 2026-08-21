use glam::{UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};

/// Four normalized channels stored in one 32-bit word.
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Unorm8x4(u32);

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
#[cfg(target_arch = "spirv")]
pub trait ShaderData: Copy {}
/// Plain data whose Rust representation is also its scalar-layout SPIR-V ABI.
#[cfg(not(target_arch = "spirv"))]
pub trait ShaderData: Copy + bytemuck::Pod {}

impl ShaderData for () {}
impl ShaderData for u32 {}
impl ShaderData for i32 {}
impl ShaderData for f32 {}
impl ShaderData for Unorm8x4 {}
impl ShaderData for Vec2 {}
impl ShaderData for Vec3 {}
impl ShaderData for Vec4 {}
impl ShaderData for UVec2 {}
impl ShaderData for UVec3 {}
impl ShaderData for UVec4 {}
#[cfg(target_arch = "spirv")]
impl<T: ShaderData, const N: usize> ShaderData for [T; N] {}
#[cfg(not(target_arch = "spirv"))]
impl<T: ShaderData, const N: usize> ShaderData for [T; N] where [T; N]: bytemuck::Pod {}

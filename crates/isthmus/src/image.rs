#[cfg(not(target_arch = "spirv"))]
use std::sync::Arc;

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone)]
pub struct Image {
    size: [u32; 2],
    pixels: Arc<[u8]>,
}

#[cfg(target_arch = "spirv")]
pub struct Image;

#[cfg(not(target_arch = "spirv"))]
impl Image {
    /// # Panics
    /// Panics when either dimension is zero or `pixels` does not contain exactly four bytes per pixel.
    pub fn rgba8(size: [u32; 2], pixels: impl Into<Arc<[u8]>>) -> Self {
        assert!(size[0] > 0 && size[1] > 0, "image dimensions must be non-zero");
        let pixels = pixels.into();
        assert_eq!(pixels.len(), size[0] as usize * size[1] as usize * 4, "RGBA8 image data has the wrong length");
        Self { size, pixels }
    }

    pub fn sample(&self, uv: glam::Vec2) -> glam::Vec4 {
        let size = glam::UVec2::from_array(self.size);
        let position = uv.clamp(glam::Vec2::ZERO, glam::Vec2::ONE) * size.as_vec2() - 0.5;
        let lower = position.floor().max(glam::Vec2::ZERO).as_uvec2().min(size - 1);
        let upper = (lower + 1).min(size - 1);
        let fraction = position.fract().max(glam::Vec2::ZERO);
        let pixel = |point: glam::UVec2| {
            let offset = (point.y * size.x + point.x) as usize * 4;
            let rgba = &self.pixels[offset..offset + 4];
            glam::Vec4::new(f32::from(rgba[0]), f32::from(rgba[1]), f32::from(rgba[2]), f32::from(rgba[3])) / 255.0
        };
        pixel(lower)
            .lerp(pixel(glam::uvec2(upper.x, lower.y)), fraction.x)
            .lerp(pixel(glam::uvec2(lower.x, upper.y)).lerp(pixel(upper), fraction.x), fraction.y)
    }

    pub(crate) const fn size(&self) -> [u32; 2] {
        self.size
    }

    pub(crate) const fn pixels(&self) -> &Arc<[u8]> {
        &self.pixels
    }
}

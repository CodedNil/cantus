use glam::{UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};

/// Four normalized channels stored in one 32-bit word.
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
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
            Vec4::new(
                (self.0 & 255) as f32,
                ((self.0 >> 8) & 255) as f32,
                ((self.0 >> 16) & 255) as f32,
                (self.0 >> 24) as f32,
            ) / 255.0
        }
    }

    pub fn to_vec3(self) -> Vec3 {
        self.to_vec4().truncate()
    }
}

/// A value with a generated WGSL storage-buffer layout.
pub trait BufferData: Copy {
    const BUFFER_ALIGN: usize;
    const BUFFER_SIZE: usize;
    const BUFFER_STRIDE: usize = align_to(Self::BUFFER_SIZE, Self::BUFFER_ALIGN);
    const BUFFER_WORDS: usize = Self::BUFFER_STRIDE.div_ceil(4);

    fn write_at(&self, words: &mut [u32], byte: usize);

    fn write(&self, words: &mut [u32], index: usize) {
        self.write_at(words, index * Self::BUFFER_STRIDE);
    }
}

#[doc(hidden)]
pub const fn align_to(value: usize, alignment: usize) -> usize {
    value.div_ceil(alignment) * alignment
}

impl BufferData for () {
    const BUFFER_ALIGN: usize = 1;
    const BUFFER_SIZE: usize = 0;

    fn write_at(&self, _words: &mut [u32], _byte: usize) {}
}

macro_rules! scalar {
    ($ty:ty, $encode:expr) => {
        impl BufferData for $ty {
            const BUFFER_ALIGN: usize = 4;
            const BUFFER_SIZE: usize = 4;

            fn write_at(&self, words: &mut [u32], byte: usize) {
                words[byte / 4] = $encode(*self);
            }
        }
    };
}

scalar!(u32, |value| value);
scalar!(i32, |value| value as u32);
scalar!(f32, f32::to_bits);

impl BufferData for Unorm8x4 {
    const BUFFER_ALIGN: usize = 4;
    const BUFFER_SIZE: usize = 4;

    fn write_at(&self, words: &mut [u32], byte: usize) {
        words[byte / 4] = self.0;
    }
}

macro_rules! vector {
    ($ty:ty, $align:literal, $size:literal, $($field:ident: $offset:literal),+) => {
        impl BufferData for $ty {
            const BUFFER_ALIGN: usize = $align;
            const BUFFER_SIZE: usize = $size;

            fn write_at(&self, words: &mut [u32], byte: usize) {
                $(self.$field.write_at(words, byte + $offset);)+
            }
        }
    };
}

vector!(Vec2, 8, 8, x: 0, y: 4);
vector!(Vec3, 16, 12, x: 0, y: 4, z: 8);
vector!(Vec4, 16, 16, x: 0, y: 4, z: 8, w: 12);
vector!(UVec2, 8, 8, x: 0, y: 4);
vector!(UVec3, 16, 12, x: 0, y: 4, z: 8);
vector!(UVec4, 16, 16, x: 0, y: 4, z: 8, w: 12);

impl<T: BufferData, const N: usize> BufferData for [T; N] {
    const BUFFER_ALIGN: usize = {
        assert!(N > 0, "BufferData arrays cannot be empty");
        T::BUFFER_ALIGN
    };
    const BUFFER_SIZE: usize = T::BUFFER_STRIDE * N;

    fn write_at(&self, words: &mut [u32], byte: usize) {
        for (index, value) in self.iter().enumerate() {
            value.write_at(words, byte + index * T::BUFFER_STRIDE);
        }
    }
}

#[cfg(all(test, feature = "cpu"))]
mod tests {
    use super::*;
    use std::vec;

    #[crate::data]
    #[derive(Default)]
    struct Layout {
        vector: Vec4,
        pair: Vec2,
        scalar: f32,
        tail: u32,
    }

    #[crate::data]
    #[derive(Default)]
    struct NestedLine {
        vectors: [Vec2; 3],
        scalars: [u32; 4],
    }

    #[crate::data]
    #[derive(Default)]
    struct NestedText {
        lines: [NestedLine; 2],
        glyphs: [Vec2; 2],
        count: u32,
    }

    #[test]
    fn storage_layout_matches_wgsl() {
        assert_eq!(Layout::BUFFER_ALIGN, 16);
        assert_eq!(Layout::BUFFER_SIZE, 32);
        let value = Layout {
            vector: Vec4::new(2.0, 3.0, 4.0, 5.0),
            pair: Vec2::new(6.0, 7.0),
            scalar: 1.0,
            tail: 8,
        };
        let mut words = vec![0; Layout::BUFFER_WORDS];
        value.write(&mut words, 0);
        assert_eq!(words[0], 2.0f32.to_bits());
        assert_eq!(words[4], 6.0f32.to_bits());
        assert_eq!(words[6], 1.0f32.to_bits());
        assert_eq!(words[7], 8);

        assert_eq!(Vec3::BUFFER_SIZE, 12);
        assert_eq!(Vec3::BUFFER_STRIDE, 16);
        assert_eq!(<[Vec3; 2]>::BUFFER_SIZE, 32);
        assert_eq!(<[[f32; 3]; 2]>::BUFFER_SIZE, 24);
        assert_eq!(NestedLine::BUFFER_ALIGN, 8);
        assert_eq!(NestedLine::BUFFER_SIZE, 40);
        assert_eq!(NestedText::BUFFER_ALIGN, 8);
        assert_eq!(NestedText::BUFFER_SIZE, 104);
    }

    #[test]
    fn normalized_color_uses_one_word() {
        let color = Unorm8x4::from_vec4(Vec4::new(1.0, 0.5, 0.0, 1.0));
        let mut words = [0];
        color.write(&mut words, 0);
        assert_eq!(words[0], 0xff00_80ff);
        assert!(
            (color.to_vec4() - Vec4::new(1.0, 128.0 / 255.0, 0.0, 1.0))
                .abs()
                .max_element()
                < 1e-6
        );
    }
}

use crate::render::cpu::Passes;
use isthmus::{FilterableFloatFormat, SampledTexture, Texture2DArray, TextureView, wgpu::Extent3d};

/// Frame-local texture-array allocator shared by artwork and icon passes; keys remain owned by the atlas.
pub struct TextureAtlas {
    texture: SampledTexture<Texture2DArray>,
    keys: Vec<String>,
    used: Vec<bool>,
}

impl TextureAtlas {
    pub fn new(passes: &Passes<'_>, label: &str, size: [u32; 2], layers: usize) -> Self {
        Self {
            texture: passes.sampled_texture::<Texture2DArray>(
                label,
                Extent3d {
                    width: size[0],
                    height: size[1],
                    depth_or_array_layers: layers as u32,
                },
                FilterableFloatFormat::Rgba8Unorm,
            ),
            keys: vec![String::new(); layers],
            used: vec![false; layers],
        }
    }

    pub const fn view(&self) -> &TextureView<Texture2DArray> {
        self.texture.view()
    }

    pub fn begin_frame(&mut self) {
        self.used.fill(false);
    }

    pub fn index_of(&mut self, key: &str, size: [u32; 2], pixels: &[u8]) -> Option<u32> {
        if let Some(index) = self.keys.iter().position(|value| value == key) {
            self.used[index] = true;
            return Some(index as u32);
        }
        let index = self.used.iter().position(|used| !used)?;
        self.texture.write([0, 0, index as u32], size, pixels).ok()?;
        self.used[index] = true;
        key.clone_into(&mut self.keys[index]);
        Some(index as u32)
    }

    pub fn write(&self, layer: u32, size: [u32; 2], pixels: &[u8]) -> bool {
        self.texture.write([0, 0, layer], size, pixels).is_ok()
    }
}

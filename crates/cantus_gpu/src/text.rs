use crate::common::{pixel_to_ndc, quad_coord, smoothstep, unpack4x8unorm};
use cantus_shared::{GlobalUniforms, GlyphInstance, MAX_GLYPH_INSTANCES};
use spirv_std::{
    Sampler,
    arch::kill,
    glam::{Vec2, Vec4},
    image::Image2d,
    spirv,
};

/// Width of the fade-out band at the edges of the clip rect, in logical pixels.
const FADE_BAND: f32 = 8.0;

#[spirv(vertex)]
pub fn vs_text(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] glyphs: &[GlyphInstance;
         MAX_GLYPH_INSTANCES],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_uv: &mut Vec2,
    #[spirv(location = 1, flat)] out_color: &mut Vec4,
    #[spirv(location = 2)] out_clip_local: &mut Vec4,
) {
    let glyph = glyphs[i_idx as usize];
    let unit = quad_coord(v_idx);
    let pixel_pos = glyph.pos + unit * glyph.size;

    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_uv = glyph.atlas_min + unit * (glyph.atlas_max - glyph.atlas_min);
    *out_color = unpack4x8unorm(glyph.color);

    // Pass clip rect in local pixel space so the fragment shader can compute fade.
    // All values are positive when the pixel is inside the clip rect.
    *out_clip_local = Vec4::new(
        pixel_pos.x - glyph.clip_min.x,
        pixel_pos.y - glyph.clip_min.y,
        glyph.clip_max.x - pixel_pos.x,
        glyph.clip_max.y - pixel_pos.y,
    );
}

#[spirv(fragment)]
pub fn fs_text(
    #[spirv(location = 0)] uv: Vec2,
    #[spirv(location = 1, flat)] color: Vec4,
    #[spirv(location = 2)] clip_local: Vec4,
    #[spirv(descriptor_set = 0, binding = 2)] atlas: &Image2d,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    // Horizontal boundaries are set to +/- MAX when that side is not clipped,
    // so only a genuinely overflowing edge receives a fade.
    let clip_fade_left = smoothstep(0.0, FADE_BAND, clip_local.x);
    let clip_fade_right = smoothstep(0.0, FADE_BAND, clip_local.z);
    let clip_fade = clip_fade_left * clip_fade_right;

    if clip_fade <= 0.0 {
        kill();
    }

    let alpha = atlas.sample(*sampler, uv).x * color.w * clip_fade;
    if alpha <= 0.0 {
        kill();
    }

    *out_color = (color.truncate() * alpha).extend(alpha);
}

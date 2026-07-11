use crate::common::{pixel_to_ndc, quad_coord, smoothstep, unpack4x8unorm};
use cantus_shared::{GlobalUniforms, GlyphInstance, MAX_GLYPH_INSTANCES};
use spirv_std::{
    Sampler,
    arch::kill,
    glam::{Vec2, Vec4},
    image::Image2d,
    spirv,
};

/// Width of the fade-out band at the right clip edge, in logical pixels.
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
    #[spirv(location = 1, flat)] out_glyph_idx: &mut u32,
) {
    let glyph = glyphs[i_idx as usize];
    let unit = quad_coord(v_idx);
    let pixel_pos = glyph.pos + unit * glyph.size;

    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_uv = glyph.atlas_min + unit * (glyph.atlas_max - glyph.atlas_min);
    *out_glyph_idx = i_idx;
}

#[spirv(fragment)]
pub fn fs_text(
    #[spirv(location = 0)] uv: Vec2,
    #[spirv(location = 1, flat)] glyph_idx: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] glyphs: &[GlyphInstance;
         MAX_GLYPH_INSTANCES],
    #[spirv(descriptor_set = 0, binding = 2)] atlas: &Image2d,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let glyph = glyphs[glyph_idx as usize];
    let color = unpack4x8unorm(glyph.color);
    let unit_x = (uv.x - glyph.atlas_min.x) / (glyph.atlas_max.x - glyph.atlas_min.x);
    let pixel_x = glyph.pos.x + unit_x * glyph.size.x;
    let clip_fade = smoothstep(0.0, FADE_BAND, glyph.clip_right - pixel_x);

    if clip_fade <= 0.0 {
        kill();
    }

    let alpha = atlas.sample(*sampler, uv).x * color.w * clip_fade;
    if alpha <= 0.0 {
        kill();
    }

    *out_color = (color.truncate() * alpha).extend(alpha);
}

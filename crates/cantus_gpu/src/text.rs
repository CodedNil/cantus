use crate::common::{pixel_to_ndc, quad_coord};
use cantus_shared::{GlobalUniforms, GlyphInstance, MAX_GLYPH_INSTANCES, smoothstep};
use spirv_std::{
    Sampler,
    arch::kill,
    glam::{Vec2, Vec4},
    image::Image2d,
    spirv,
};

#[spirv(vertex)]
pub fn vs_text(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] glyphs: &[GlyphInstance;
         MAX_GLYPH_INSTANCES],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_uv: &mut Vec2,
    #[spirv(location = 1)] out_fade: &mut Vec2,
) {
    let glyph = glyphs[i_idx as usize];
    let unit = quad_coord(v_idx);
    let pixel_pos = glyph.pos + unit * glyph.size;

    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_uv = glyph.atlas_min + unit * (glyph.atlas_max - glyph.atlas_min);
    *out_fade = Vec2::new(glyph.clip_right - pixel_pos.x, glyph.alpha);
}

#[spirv(fragment)]
pub fn fs_text(
    #[spirv(location = 0)] uv: Vec2,
    #[spirv(location = 1)] fade: Vec2,
    #[spirv(descriptor_set = 0, binding = 2)] atlas: &Image2d,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let alpha = atlas.sample(*sampler, uv).x * fade.y * smoothstep(0.0, 8.0, fade.x);
    if alpha <= 0.0 {
        kill();
    }

    *out_color = Vec4::new(0.94 * alpha, 0.94 * alpha, 0.94 * alpha, alpha);
}

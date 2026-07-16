use crate::{pixel_to_ndc, quad_coord};
use cantus_shared::{
    GLYPH_ATLAS_SIZE, GlobalUniforms, GlyphInstance, MAX_GLYPH_INSTANCES, smoothstep, unpack_u16x2,
};
use spirv_std::{
    Sampler,
    arch::kill,
    glam::{Vec2, Vec3, Vec4},
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
    #[spirv(location = 1)] out_style: &mut Vec3,
) {
    let glyph = glyphs[i_idx as usize];
    let unit = quad_coord(v_idx);
    let pixel_pos = glyph.pos + unit * glyph.size;
    let atlas_min = unpack_u16x2(glyph.atlas[0]);
    let atlas_max = unpack_u16x2(glyph.atlas[1]);

    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_uv = (atlas_min + unit * (atlas_max - atlas_min)) / GLYPH_ATLAS_SIZE as f32;
    *out_style = Vec3::new(glyph.clip_right - pixel_pos.x, glyph.alpha, glyph.weight);
}

#[spirv(fragment)]
pub fn fs_text(
    #[spirv(location = 0)] uv: Vec2,
    #[spirv(location = 1)] style: Vec3,
    #[spirv(descriptor_set = 0, binding = 2)] atlas: &Image2d,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let coverage = atlas.sample(*sampler, uv).x;
    let coverage = ((coverage - style.z) / (1.0 - style.z)).clamp(0.0, 1.0);
    let alpha = coverage * style.y * smoothstep(0.0, 8.0, style.x);
    if alpha <= 0.0 {
        kill();
    }

    *out_color = Vec4::new(0.94 * alpha, 0.94 * alpha, 0.94 * alpha, alpha);
}

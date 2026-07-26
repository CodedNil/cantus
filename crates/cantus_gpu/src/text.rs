use crate::{pixel_to_ndc, quad_coord, unpack_u16x2};
use cantus_shared::{GLYPH_ATLAS_SIZE, GlobalUniforms, GlyphInstance, MAX_GLYPH_INSTANCES, smoothstep};
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    glam::{Vec2, Vec3, Vec4, vec2},
    image::Image2d,
    spirv,
};

#[spirv(vertex)]
pub fn vs_text(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] glyphs: &[GlyphInstance;
         MAX_GLYPH_INSTANCES],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_uv: &mut Vec2,
    #[spirv(location = 1)] out_style: &mut Vec2,
) {
    let glyph = glyphs[i_idx as usize];
    let unit = quad_coord(v_idx);
    let pixel_pos = glyph.pos + unit * glyph.size;
    let atlas_min = unpack_u16x2(glyph.atlas[0]);
    let atlas_max = unpack_u16x2(glyph.atlas[1]);

    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_uv = (atlas_min + unit * (atlas_max - atlas_min)) / GLYPH_ATLAS_SIZE as f32;
    *out_style = Vec2::new(glyph.clip_right - pixel_pos.x, glyph.alpha);
}

#[spirv(fragment)]
pub fn fs_text(
    #[spirv(location = 0)] uv: Vec2,
    #[spirv(location = 1)] style: Vec2,
    #[spirv(descriptor_set = 0, binding = 2)] atlas: &Image2d,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let offset = uv.fwidth() * 0.25;
    let coverage = (atlas.sample(*sampler, uv - offset).x
        + atlas.sample(*sampler, uv + offset).x
        + atlas.sample(*sampler, uv + vec2(offset.x, -offset.y)).x
        + atlas.sample(*sampler, uv + vec2(-offset.x, offset.y)).x)
        * 0.25;
    // A negative value marks a shadow; values over 2 mark red calendar text.
    let red = style.y > 1.0;
    let opacity = if red { style.y - 2.0 } else { style.y.abs() };
    let alpha = coverage * opacity * smoothstep(0.0, 8.0, style.x);
    if alpha <= 0.0 {
        kill();
    }
    let color = if style.y < 0.0 {
        Vec3::ZERO
    } else if red {
        Vec3::new(1.0, 0.68, 0.68)
    } else {
        Vec3::splat(0.94)
    };
    *out_color = (color * alpha).extend(alpha);
}

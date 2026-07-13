use crate::common::{pixel_to_ndc, quad_coord, sd_rounded_triangle};
use cantus_shared::{GlobalUniforms, PlayheadUniforms, smoothstep};
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

fn paired_vertical_segments(point: Vec2, center: Vec2, size: Vec2, radius: f32) -> f32 {
    let local = (point - center).abs();
    let offset = vec2(local.x, (local.y - size.x).abs() - size.y);
    offset.max(Vec2::ZERO).length() - radius
}

#[spirv(vertex)]
pub fn vs_playhead(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_world_pos: &mut Vec2,
) {
    let height = global.bar_height.y;
    let uv = quad_coord(v_idx);
    let world_pos = vec2(
        global.playhead_x + (uv.x * 2.0 - 1.0) * height * 0.4,
        global.bar_height.x - 5.0 + uv.y * (height + 10.0),
    );

    *out_pos = pixel_to_ndc(world_pos, global.screen_size);
    *out_world_pos = world_pos;
}

#[spirv(fragment)]
pub fn fs_playhead(
    #[spirv(location = 0)] world_pos: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(uniform, descriptor_set = 0, binding = 1)] state: &PlayheadUniforms,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let height = global.bar_height.y;
    let center = vec2(global.playhead_x, global.bar_height.x + height * 0.5);
    let line_thickness = 4.5;

    let bar_len = height * (0.5 - 0.375 * state.bar_split);
    let dist_bar = paired_vertical_segments(
        world_pos,
        center,
        vec2((height - bar_len) * 0.5, bar_len * 0.5),
        line_thickness,
    );

    let icon_alpha = state.icon_presence.clamp(0.0, 1.0);
    let pause_point = (world_pos - center).abs();
    let pause_offset = vec2(
        (pause_point.x - 4.0 * state.bar_split.clamp(0.0, 1.0)).abs(),
        (pause_point.y - height * 0.1).max(0.0),
    );
    let dist_pause = pause_offset.length() - 3.5;

    let play_scale = height * 0.18 * (1.0 + state.icon_morph * (1.0 - icon_alpha));
    let dist_play = sd_rounded_triangle((world_pos - center).perp(), play_scale, play_scale * 0.5);

    let dist_icon = dist_pause + (dist_play - dist_pause) * state.icon_morph.clamp(0.0, 1.0);

    let mask_bar = 1.0 - smoothstep(-0.8, 0.2, dist_bar);
    let mask_icon = (1.0 - smoothstep(-0.8, 0.2, dist_icon)) * icon_alpha;
    let main_mask = mask_icon + mask_bar * (1.0 - mask_icon);

    let shadow =
        Vec2::ONE - (vec2(dist_bar, dist_icon) / line_thickness).clamp(Vec2::ZERO, Vec2::ONE);
    let shadow = shadow.powf(2.0) * vec2(0.4, 0.4 * icon_alpha);
    let shadow_mask = shadow.x.max(shadow.y);

    if main_mask <= 0.0 && shadow_mask <= 0.0 {
        kill();
    }
    let fill = vec3(1.0, 0.878, 0.824);
    let border = Vec3::splat(0.15);
    let bar_rgb = fill.lerp(border, smoothstep(-2.5, -1.0, dist_bar));
    let icon_rgb = fill.lerp(border, smoothstep(-2.5, -1.0, dist_icon));
    let rgb = icon_rgb * mask_icon + bar_rgb * mask_bar * (1.0 - mask_icon);
    *out_color = rgb.extend(main_mask.max(shadow_mask));
}

use crate::common::{
    pixel_to_ndc, quad_coord, sd_rounded_triangle, sd_vertical_segment, smoothstep,
};
use cantus_shared::{GlobalUniforms, PlayheadUniforms};
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[spirv(vertex)]
pub fn vs_playhead(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_world_pos: &mut Vec2,
) {
    let x_coord = global.playhead_x;
    let start_y = global.bar_height.x;
    let height = global.bar_height.y;

    let h_width = height * 0.4;
    let uv = quad_coord(v_idx);
    let world_pos = vec2(
        x_coord + (uv.x * 2.0 - 1.0) * h_width,
        start_y - 5.0 + uv.y * (height + 10.0),
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
    let x_coord = global.playhead_x;
    let start_y = global.bar_height.x;
    let height = global.bar_height.y;
    let mid_y = start_y + height * 0.5;
    let line_thickness = 4.5;
    let icon_thickness = 3.5;

    let bar_len = height * (0.5 - 0.375 * state.bar_split);
    let bar_center_offset = (height - bar_len) * 0.5;
    let dist_bar = sd_vertical_segment(
        world_pos,
        x_coord,
        mid_y - bar_center_offset,
        bar_len * 0.5,
        line_thickness,
    )
    .min(sd_vertical_segment(
        world_pos,
        x_coord,
        mid_y + bar_center_offset,
        bar_len * 0.5,
        line_thickness,
    ));

    let icon_alpha = state.icon_presence.clamp(0.0, 1.0);
    let pause_gap = 4.0 * state.bar_split.clamp(0.0, 1.0);
    let pause_half_height = height * 0.1;
    let dist_pause = sd_vertical_segment(
        world_pos,
        x_coord - pause_gap,
        mid_y,
        pause_half_height,
        icon_thickness,
    )
    .min(sd_vertical_segment(
        world_pos,
        x_coord + pause_gap,
        mid_y,
        pause_half_height,
        icon_thickness,
    ));

    let play_local = world_pos - vec2(x_coord, mid_y);
    let play_rotated = vec2(-play_local.y, play_local.x);
    let play_scale = height * 0.18 * state.icon_scale.max(0.0);
    let dist_play = sd_rounded_triangle(play_rotated, play_scale, play_scale * 0.5);

    let dist_icon = if icon_alpha > 0.0 {
        dist_pause + (dist_play - dist_pause) * state.icon_morph.clamp(0.0, 1.0)
    } else {
        1e6
    };

    let mask_bar = 1.0 - smoothstep(-0.8, 0.2, dist_bar);
    let mask_icon = (1.0 - smoothstep(-0.8, 0.2, dist_icon)) * icon_alpha;
    let main_mask = mask_icon + mask_bar * (1.0 - mask_icon);

    let shadow =
        Vec2::ONE - (vec2(dist_bar, dist_icon) / line_thickness).clamp(Vec2::ZERO, Vec2::ONE);
    let shadow = shadow.powf(2.0) * vec2(0.4, 0.4 * icon_alpha);
    let shadow_mask = shadow.x.max(shadow.y);

    if main_mask > 0.0 || shadow_mask > 0.0 {
        let fill = vec3(1.0, 0.878, 0.824);
        let border = Vec3::splat(0.15);
        let bar_rgb = fill.lerp(border, smoothstep(-2.5, -1.0, dist_bar));
        let icon_rgb = fill.lerp(border, smoothstep(-2.5, -1.0, dist_icon));
        let rgb = icon_rgb * mask_icon + bar_rgb * mask_bar * (1.0 - mask_icon);
        *out_color = rgb.extend(main_mask.max(shadow_mask));
    } else {
        kill();
    }
}

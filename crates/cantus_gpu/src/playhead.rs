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

    let bar_len = height * (0.5 + (0.125 - 0.5) * state.bar_lerp);
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

    let (dist_pause, pause_active) = if state.pause_lerp >= 0.001 {
        let pause_gap = 4.0 * smoothstep(0.0, 0.5, state.pause_lerp);
        let pause_half_height = height * 0.1;
        let dist = sd_vertical_segment(
            world_pos,
            x_coord - pause_gap,
            mid_y,
            pause_half_height,
            line_thickness,
        )
        .min(sd_vertical_segment(
            world_pos,
            x_coord + pause_gap,
            mid_y,
            pause_half_height,
            line_thickness,
        ));
        (dist, 1.0 - smoothstep(0.5, 1.0, state.pause_lerp))
    } else {
        (1e6, 0.0)
    };

    let (dist_play, play_active) = if state.play_lerp >= 0.001 {
        let p_local = world_pos - vec2(x_coord, mid_y);
        let p_rotated = vec2(-p_local.y, p_local.x);
        let play_growth = (state.play_lerp * 2.0).min(1.0);
        let play_scale = height
            * (0.01 + (0.18 - 0.01) * play_growth)
            * (1.0 + smoothstep(0.5, 1.0, state.play_lerp));
        (
            sd_rounded_triangle(p_rotated, play_scale, play_scale * 0.5),
            1.0 - smoothstep(0.5, 1.0, state.play_lerp),
        )
    } else {
        (1e6, 0.0)
    };

    let icon_alpha = (pause_active + play_active).clamp(0.0, 1.0);
    let icon_mix = pause_active / (pause_active + play_active + 1e-6);
    let dist_icon = dist_play + (dist_pause - dist_play) * icon_mix;

    let mask_bar = 1.0 - smoothstep(-0.8, 0.2, dist_bar);
    let mask_icon = (1.0 - smoothstep(-0.8, 0.2, dist_icon)) * icon_alpha;
    let main_mask = (mask_bar + mask_icon).clamp(0.0, 1.0);

    let shadow_bar = 1.0 - (dist_bar / 4.5).clamp(0.0, 1.0);
    let shadow_bar = shadow_bar * shadow_bar * 0.4;
    let shadow_icon = 1.0 - (dist_icon / 4.5).clamp(0.0, 1.0);
    let shadow_icon = shadow_icon * shadow_icon * 0.4 * icon_alpha;
    let shadow_mask = shadow_bar.max(shadow_icon);

    if main_mask > 0.0 || shadow_mask > 0.0 {
        let normalized_y = 1.0 - ((world_pos.y - start_y) / height).clamp(0.0, 1.0);
        let color_state = if normalized_y <= state.volume {
            vec3(1.0, 0.878, 0.824)
        } else {
            Vec3::splat(0.5)
        };
        let border_mask = smoothstep(-2.5, -1.0, dist_bar.min(dist_icon));
        let final_rgb = color_state.lerp(Vec3::splat(0.15), border_mask);
        *out_color = (final_rgb * main_mask).extend(main_mask.max(shadow_mask));
    } else {
        kill();
    }
}

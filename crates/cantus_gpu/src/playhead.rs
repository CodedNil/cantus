use crate::common::{mix_f32, mix_vec3, sign_no_nan, smoothstep, step};
use cantus_shared::{GlobalUniforms, PlayheadUniforms};
use spirv_std::{
    glam::{Vec2, Vec4, vec2, vec3, vec4},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn sd_segment(p: Vec2, a: Vec2, b: Vec2, radius: f32) -> f32 {
    let ba = b - a;
    let pa = p - a;
    let h = (pa.dot(ba) / ba.dot(ba)).clamp(0.0, 1.0);
    (pa - h * ba).length() - radius
}

fn sd_rounded_triangle(p: Vec2, side_len: f32, radius: f32) -> f32 {
    let k = 3.0f32.sqrt();
    let mut p_sym = p;
    p_sym.x = p_sym.x.abs();
    let h = (p_sym.x + k * p_sym.y).max(0.0);
    p_sym -= 0.5 * vec2(h, h * k);
    p_sym -= vec2(
        p_sym.x.clamp(
            -0.5 * (side_len - radius) * k,
            0.5 * (side_len - radius) * k,
        ),
        -0.5 * (side_len - radius),
    );
    p_sym.length() * sign_no_nan(-p_sym.y) - radius
}

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
    let uv = vec2((v_idx % 2) as f32, (v_idx / 2) as f32);
    let world_pos = vec2(
        x_coord + (uv.x * 2.0 - 1.0) * h_width,
        start_y - 5.0 + uv.y * (height + 10.0),
    );

    let ndc = (world_pos / global.screen_size * 2.0 - 1.0) * vec2(1.0, -1.0);
    *out_pos = vec4(ndc.x, ndc.y, 0.0, 1.0);
    *out_world_pos = world_pos;
}

#[spirv(fragment)]
pub fn fs_playhead(
    #[spirv(location = 0)] world_pos: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(uniform, descriptor_set = 0, binding = 1)] state: &PlayheadUniforms,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pixel_pos = world_pos;
    let scale = global.scale_factor;
    let x_coord = global.playhead_x;
    let start_y = global.bar_height.x;
    let height = global.bar_height.y;
    let mid_y = start_y + height * 0.5;
    let line_thickness = 3.5 * scale;

    let bar_len = height * mix_f32(0.5, 0.125, state.bar_lerp);
    let dist_bar = sd_segment(
        pixel_pos,
        vec2(x_coord, start_y),
        vec2(x_coord, start_y + bar_len),
        line_thickness,
    )
    .min(sd_segment(
        pixel_pos,
        vec2(x_coord, start_y + height - bar_len),
        vec2(x_coord, start_y + height),
        line_thickness,
    ));

    let pause_gap = mix_f32(0.0, 4.0 * scale, smoothstep(0.0, 0.5, state.pause_lerp));
    let dist_pause = sd_segment(
        pixel_pos,
        vec2(x_coord - pause_gap, mid_y - height * 0.1),
        vec2(x_coord - pause_gap, mid_y + height * 0.1),
        line_thickness,
    )
    .min(sd_segment(
        pixel_pos,
        vec2(x_coord + pause_gap, mid_y - height * 0.1),
        vec2(x_coord + pause_gap, mid_y + height * 0.1),
        line_thickness,
    ));
    let pause_active =
        step(0.001, state.pause_lerp) * (1.0 - smoothstep(0.5, 1.0, state.pause_lerp));

    let p_local = pixel_pos - vec2(x_coord, mid_y);
    let p_rotated = vec2(-p_local.y, p_local.x);
    let play_scale = mix_f32(
        0.01 * height,
        height * 0.18,
        (state.play_lerp * 2.0).min(1.0),
    ) * mix_f32(1.0, 2.0, smoothstep(0.5, 1.0, state.play_lerp));
    let dist_play = sd_rounded_triangle(p_rotated, play_scale, play_scale * 0.5);
    let play_active = step(0.001, state.play_lerp) * (1.0 - smoothstep(0.5, 1.0, state.play_lerp));

    let icon_alpha = (pause_active + play_active).clamp(0.0, 1.0);
    let dist_icon = mix_f32(
        dist_play,
        dist_pause,
        pause_active / (pause_active + play_active + 1e-6),
    );

    let mask_bar = 1.0 - smoothstep(-0.8, 0.2, dist_bar);
    let mask_icon = (1.0 - smoothstep(-0.8, 0.2, dist_icon)) * icon_alpha;
    let main_mask = (mask_bar + mask_icon).clamp(0.0, 1.0);

    let shadow_bar = (1.0 - (dist_bar / (4.5 * scale)).clamp(0.0, 1.0)).powf(2.0) * 0.4;
    let shadow_icon =
        (1.0 - (dist_icon / (4.5 * scale)).clamp(0.0, 1.0)).powf(2.0) * 0.4 * icon_alpha;
    let shadow_mask = shadow_bar.max(shadow_icon);

    if main_mask > 0.0 || shadow_mask > 0.0 {
        let normalized_y = 1.0 - ((pixel_pos.y - start_y) / height).clamp(0.0, 1.0);
        let color_state = mix_vec3(
            vec3(0.5, 0.5, 0.5),
            vec3(1.0, 0.878, 0.824),
            if normalized_y <= state.volume {
                1.0
            } else {
                0.0
            },
        );
        let border_mask = smoothstep(-2.5, -1.0, dist_bar.min(dist_icon));
        let final_rgb = mix_vec3(color_state, vec3(0.15, 0.15, 0.15), border_mask);
        *out_color =
            mix_vec3(vec3(0.0, 0.0, 0.0), final_rgb, main_mask).extend(main_mask.max(shadow_mask));
    } else {
        *out_color = Vec4::ZERO;
    }
}

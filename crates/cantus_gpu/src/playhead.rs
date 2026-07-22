use crate::{pixel_to_ndc, quad_coord, sd_rounded_triangle};
use cantus_shared::{GlobalUniforms, PlayheadUniforms, smoothstep};
use spirv_std::{
    arch::kill,
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[spirv(vertex)]
pub fn vs_playhead(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
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
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] state: &PlayheadUniforms,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let height = global.bar_height.y;
    let center = vec2(global.playhead_x, global.bar_height.x + height * 0.5);
    let pause = (world_pos - center).abs();

    // Two vertical capsule segments mirrored around the center, splitting apart as the bar does.
    let bar_len = height * (0.5 - 0.375 * state.bar_split);
    let segments = vec2(
        pause.x,
        (pause.y - (height - bar_len) * 0.5).abs() - bar_len * 0.5,
    );
    let dist_bar = segments.max(Vec2::ZERO).length() - 4.5;

    let icon_alpha = state.icon_presence.saturate();
    let dist_pause = vec2(
        (pause.x - 4.0 * state.bar_split.saturate()).abs(),
        (pause.y - height * 0.1).max(0.0),
    )
    .length()
        - 3.5;
    let play_scale = height * 0.18 * (1.0 + state.icon_morph * (1.0 - icon_alpha));
    let dist_play = sd_rounded_triangle((world_pos - center).perp(), play_scale, play_scale * 0.5);
    let dist_icon = dist_pause + (dist_play - dist_pause) * state.icon_morph.saturate();
    let bar_mask = 1.0 - smoothstep(-0.8, 0.2, dist_bar);
    let icon_mask = (1.0 - smoothstep(-0.8, 0.2, dist_icon)) * icon_alpha;
    let alpha = icon_mask.max(bar_mask);
    if alpha <= 0.0 {
        kill();
    }
    let edge = smoothstep(-2.5, -1.0, dist_bar.min(dist_icon));
    *out_color = vec3(1.0, 0.878, 0.824)
        .lerp(Vec3::splat(0.15), edge)
        .extend(alpha);
}

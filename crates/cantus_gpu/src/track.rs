use crate::{
    pill_coverage, pill_fragment, pill_interaction, pill_sheen, pixel_to_ndc, quad_coord,
    sd_capsule_box, sd_star, smooth_union, unpack3x8unorm,
};
use cantus_shared::{
    GlobalUniforms, ICON_WIDTH, MAX_PILL_PLAYLIST_ICONS, TrackPill, smoothstep,
};
use core::f32::consts::TAU;
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    image::Image2dArray,
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn noise(point: Vec2, seed: f32) -> f32 {
    ((point.dot(vec2(127.1, 311.7)) + seed).sin() * 43_758.547).fract()
}

fn icon_local(pixel_pos: Vec2, center: Vec2, global: &GlobalUniforms) -> (Vec2, Vec2, f32, f32) {
    let mouse_distance = center.distance(global.mouse_pos);
    let proximity = smoothstep(
        30.0,
        8.0,
        mouse_distance / global.mouse_pressure.clamp(0.001, 1.0),
    );
    let pixel_radius = ICON_WIDTH * 0.5 * (1.05 + 0.63 * proximity);
    let x_push = (center.x - global.mouse_pos.x) * proximity * 0.5;
    let local = pixel_pos - (center + vec2(x_push, 0.0));
    let local = Vec2::from_angle(-x_push * 0.01).rotate(local);
    (
        local / (pixel_radius * 2.0) + 0.5,
        local,
        pixel_radius,
        mouse_distance,
    )
}

fn near_icon(pixel_pos: Vec2, center: Vec2) -> bool {
    (pixel_pos - center)
        .abs()
        .cmplt(Vec2::splat(ICON_WIDTH * 1.8))
        .all()
}

fn over_icon(base: Vec4, color: Vec3, dist: f32, alpha: f32, pill_alpha: f32) -> Vec4 {
    let mask = (0.5 - dist).clamp(0.0, 1.0);
    let shadow = 1.0 - smoothstep(0.0, 6.0, dist);
    let bevel = 1.0 - smoothstep(0.0, -5.0, dist);
    let opacity = alpha * pill_alpha;
    let overlay = ((color + bevel * bevel * 0.045) * mask * opacity)
        .extend(mask.max(shadow * shadow * 0.2) * opacity);
    base * (1.0 - overlay.w) + overlay
}

#[spirv(vertex)]
pub fn vs_track(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[TrackPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel_pos: &mut Vec2,
    #[spirv(location = 1, flat)] out_pill_idx: &mut u32,
) {
    let pill = pills[i_idx as usize];
    let margin = 48.0;
    let pill_size = vec2(pill.width, global.bar_height.y);
    let pill_origin = vec2(pill.x, global.bar_height.x);

    let (primary_row, secondary_row) = pill.icon_rows(global.bar_height.x, global.bar_height.y);
    let icon_row_radius = ICON_WIDTH * 0.9;
    let icon_half_size = (primary_row.half_size(icon_row_radius) * pill.primary_alpha)
        .max(secondary_row.half_size(icon_row_radius) * pill.secondary_expansion);
    let render_min = vec2(
        (pill_origin.x - margin).min(primary_row.center.x - icon_half_size.x),
        pill_origin.y - margin,
    );
    let render_max = vec2(
        (pill_origin.x + pill_size.x + margin).max(primary_row.center.x + icon_half_size.x),
        (pill_origin.y + pill_size.y + margin)
            .max(secondary_row.backplate_center().y + icon_half_size.y),
    );

    let pixel_pos = render_min + quad_coord(v_idx) * (render_max - render_min);
    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_pixel_pos = pixel_pos;
    *out_pill_idx = i_idx;
}

#[spirv(fragment)]
pub fn fs_track(
    #[spirv(location = 0)] pixel_pos: Vec2,
    #[spirv(location = 1, flat)] pill_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[TrackPill],
    #[spirv(descriptor_set = 0, binding = 2)] images: &Image2dArray,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = pills[pill_idx as usize];
    let interaction = pill_interaction(pixel_pos, global);
    let bulge = interaction.bulge();
    let (local_pixel, pill_size, body_dist) =
        pill_fragment(pixel_pos, global, pill.x, pill.width, bulge);
    let local_uv = local_pixel / pill_size;
    let local_centered = local_uv - 0.5;
    let stretched_uv_y = local_centered.y * (pill_size.y / (pill_size.y + bulge)) + 0.5;

    let (primary_row, secondary_row) = pill.icon_rows(global.bar_height.x, global.bar_height.y);

    let backplate_radius = 9.0 + interaction.mouse.length() * ICON_WIDTH * 0.25;
    let mut dist = smooth_union(
        body_dist,
        sd_capsule_box(
            pixel_pos - (primary_row.backplate_center() - vec2(0.0, 2.0)),
            primary_row.half_span(),
            backplate_radius,
        ),
        10.0,
        pill.primary_alpha,
    );
    dist = smooth_union(
        dist,
        sd_capsule_box(
            pixel_pos - secondary_row.backplate_center(),
            secondary_row.half_span(),
            backplate_radius + 1.5,
        ),
        ICON_WIDTH * 0.5,
        pill.secondary_expansion,
    );
    let (mask, shadow) = pill_coverage(dist);

    let seed = (pill.colors[0] % 1000) as f32 * 0.013;
    let audio = pill.audio_features.decode();
    let loudness = ((audio.loudness + 60.0) / 60.0).clamp(0.0, 1.0);
    let beat = (global.time * audio.tempo_hz() * TAU).sin() * 0.5 + 0.5;
    let beat = beat * beat * audio.danceability;
    let flow_time = global.time
        * (0.08 + audio.energy * 0.2 + audio.tempo_normalized() * 0.12 - audio.acousticness * 0.05)
        + seed;
    let uv = interaction.refract(local_pixel, pill_size, dist);
    let scale = 4.0 - audio.instrumentalness * 1.4;
    let warp = vec2(
        (uv.y * scale + flow_time).sin(),
        (uv.x * (scale + 2.0) - flow_time * 0.8).cos(),
    ) * (0.025 + audio.turbulence() * 0.08 + beat * 0.025);
    let uv = uv + warp;
    let blend_x = (uv.x * scale + flow_time * 0.35).sin() * 0.5 + 0.5;
    let blend_y = (uv.y * (scale + 1.0) - flow_time * 0.25).cos() * 0.5 + 0.5;
    let mut color = unpack3x8unorm(pill.colors[0])
        .lerp(unpack3x8unorm(pill.colors[1]), blend_x)
        .lerp(
            unpack3x8unorm(pill.colors[2]).lerp(unpack3x8unorm(pill.colors[3]), blend_y),
            (blend_x + blend_y) * 0.25,
        );

    let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
    let played = smoothstep(
        global.playhead_x + 3.0,
        global.playhead_x - 3.0,
        pixel_pos.x,
    );
    color = Vec3::splat(luma)
        .lerp(color, 1.25 + audio.valence * 0.65)
        .clamp(Vec3::splat(0.035), Vec3::splat(0.92))
        * (0.52 / luma.max(0.001)).min(1.0)
        * (0.88 + loudness * 0.12 + beat * (0.04 + audio.energy * 0.09))
        * (0.84 + smoothstep(0.45, 1.0, stretched_uv_y) * 0.1)
        * (1.0 - 0.4 * played);

    let glitter_uv = local_pixel / (9.0 - audio.acousticness * 3.0)
        + vec2(global.time * 0.025, -global.time * 0.04);
    let glitter_seed = noise(glitter_uv.floor(), seed);
    let glitter = smoothstep(0.45, 0.9, audio.acousticness)
        * smoothstep(0.94, 1.0, glitter_seed)
        * (1.0 - smoothstep(0.08, 0.35, (glitter_uv.fract() - 0.5).length()))
        * ((global.time * (1.5 + glitter_seed) + glitter_seed * TAU).sin() * 0.5 + 0.5);
    color = color.lerp(Vec3::ONE, glitter * 0.7);

    let image_left = pill_size.x - pill_size.y;
    if pill.image_index >= 0 && local_pixel.x >= image_left {
        let uv_img = vec2((local_pixel.x - image_left) / pill_size.y, stretched_uv_y);
        let tex = images.sample(*sampler, uv_img.extend(pill.image_index as f32));
        let image_dist = sd_capsule_box((uv_img - 0.5) * pill_size.y, 0.0, pill_size.y * 0.5);
        let img_mask =
            (1.0 - smoothstep(-4.0, 0.0, image_dist)) * (1.0 - smoothstep(-0.5, 0.5, body_dist));
        color = color.lerp(tex.truncate(), img_mask * tex.w);
    }

    color += color.lerp(Vec3::ONE, 0.32) * pill_sheen(stretched_uv_y, dist, interaction);
    color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);

    let mut output = (color * mask * pill.alpha).extend(mask.max(shadow) * pill.alpha);

    if pill.rating >= 0 && pill.primary_alpha > 0.0 {
        let rating = pill.rating as f32;
        for index in 0..5 {
            let fill = ((rating - index as f32 * 2.0) * 0.5).clamp(0.0, 1.0);
            let center = primary_row.icon_center(index as f32);
            if !near_icon(pixel_pos, center) {
                continue;
            }
            let (local_uv, local_pixel, pixel_radius, _) = icon_local(pixel_pos, center, global);
            let dist =
                sd_star(local_pixel, pixel_radius * 0.5, pixel_radius * 0.32) - pixel_radius * 0.1;
            let split_line = local_uv.x - fill;
            let selection_mask = (split_line / split_line.fwidth() + 0.5).clamp(0.0, 1.0);
            let color = vec3(1.0, 0.85, 0.2).lerp(vec3(0.33, 0.33, 0.33), selection_mask);
            output = over_icon(output, color, dist, pill.primary_alpha, pill.alpha);
        }
    }

    let primary_playlists = pill.primary_playlist_count as usize;
    let playlist_count =
        (primary_playlists + pill.secondary_playlist_count as usize).min(MAX_PILL_PLAYLIST_ICONS);
    for index in 0..playlist_count {
        let image_index = pill.playlist_images[index];
        if image_index < 0 {
            continue;
        }

        let primary_icon = index < primary_playlists;
        let (row, icon_index, alpha) = if primary_icon {
            (
                primary_row,
                index as f32 + pill.star_count(),
                pill.primary_alpha,
            )
        } else {
            (
                secondary_row,
                (index - primary_playlists) as f32,
                pill.secondary_expansion,
            )
        };
        if alpha <= 0.0 {
            continue;
        }
        let center = row.icon_center(icon_index);
        if !near_icon(pixel_pos, center) {
            continue;
        }
        let (local_uv, local_pixel, pixel_radius, mouse_distance) =
            icon_local(pixel_pos, center, global);
        let desaturation = if primary_icon
            || (global.mouse_pressure > 0.0 && mouse_distance <= ICON_WIDTH * 0.5)
        {
            0.0
        } else {
            0.2
        };
        let dist = sd_capsule_box(local_pixel, 0.0, pixel_radius * 0.6);
        let tex = images.sample(*sampler, local_uv.extend(image_index as f32));
        output = over_icon(
            output,
            tex.truncate().lerp(Vec3::splat(0.24), desaturation),
            dist,
            alpha,
            pill.alpha,
        );
    }

    if output.w <= 0.0 {
        kill();
    }
    *out_color = output;
}

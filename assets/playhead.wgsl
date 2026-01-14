struct GlobalUniforms {
    screen_size: vec2<f32>,
    layer_metrics: vec2<f32>, // [start_y, height]
    mouse_pos: vec2<f32>,
    playhead_x: f32,
    time: f32,
    expansion_xy: vec2<f32>,
    expansion_time: f32,
    scale_factor: f32,
};

struct Particle {
    spawn_vel: vec2<f32>,
    spawn_y: f32,
    spawn_time: f32,
    duration: f32,
    color: u32,
};

struct PlayheadState {
    volume: f32,
    bar_visibility: f32,
    play_animation: f32,
    pause_animation: f32,
};

@group(0) @binding(0) var<uniform> global: GlobalUniforms;
@group(0) @binding(1) var<uniform> state: PlayheadState;
@group(0) @binding(2) var<storage, read> particles: array<Particle>;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) local_uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32) -> VertexOutput {
    let pos_data = array<vec2<f32>, 4>(vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0));
    let uv_data = array<vec2<f32>, 4>(vec2(0.0, 1.0), vec2(1.0, 1.0), vec2(0.0, 0.0), vec2(1.0, 0.0));

    var out: VertexOutput;
    out.clip_pos = vec4(pos_data[v_idx], 0.0, 1.0);
    out.local_uv = uv_data[v_idx];
    return out;
}

fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, radius: f32) -> f32 {
    let ba = b - a;
    let pa = p - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - h * ba) - radius;
}

fn sd_rounded_triangle(p: vec2<f32>, side_len: f32, radius: f32) -> f32 {
    let k = sqrt(3.0);
    var p_sym = p;
    p_sym.x = abs(p_sym.x);
    let h = max(p_sym.x + k * p_sym.y, 0.0);
    p_sym -= 0.5 * vec2(h, h * k);
    p_sym -= vec2(clamp(p_sym.x, -0.5 * (side_len - radius) * k, 0.5 * (side_len - radius) * k), -0.5 * (side_len - radius));
    return length(p_sym) * sign(-p_sym.y) - radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_pos = in.local_uv * global.screen_size;
    var out_rgba = vec4(0.0);

    // --- Particles ---
    for (var i = 0u; i < 64u; i++) {
        let p = particles[i];
        let dt = global.time - p.spawn_time;
        if (dt < 0.0 || dt > p.duration) { continue; }

        let p_life = 1.0 - (dt / p.duration);
        let fade = p_life * smoothstep(0.0, 0.05, dt);
        let pos = vec2<f32>(global.playhead_x, p.spawn_y) + p.spawn_vel * dt + vec2(0.0, 150.0 * dt * dt);
        let dir = normalize(p.spawn_vel + vec2(0.0, 300.0 * dt));
        let len = mix(8.0, 12.0, p_life);
        let thickness = mix(2.5, 4.0, p_life);

        let pa = (pixel_pos - pos) + (dir * len * 0.5);
        let ba = dir * len;
        let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
        let dist = length(pa - ba * h);

        if (dist < thickness) {
            let intensity = 1.0 - (dist / thickness);
            let alpha = fade * intensity * intensity;
            let color = mix(unpack4x8unorm(p.color).rgb, vec3(1.0), intensity * 0.5);
            out_rgba += vec4(color * alpha * 1.5, alpha);
        }
    }

    // --- Playhead Geometry ---
    let scale = global.scale_factor;
    let x_coord = global.playhead_x;
    let start_y = global.layer_metrics.x;
    let height = global.layer_metrics.y;
    let mid_y = start_y + height * 0.5;
    let line_thickness = 3.5 * scale;

    let bar_len = height * mix(0.5, 0.125, state.bar_visibility);
    let dist_bar = min(
        sd_segment(pixel_pos, vec2(x_coord, start_y), vec2(x_coord, start_y + bar_len), line_thickness),
        sd_segment(pixel_pos, vec2(x_coord, start_y + height - bar_len), vec2(x_coord, start_y + height), line_thickness)
    );

    let pause_gap = mix(0.0, 4.0 * scale, smoothstep(0.0, 0.5, state.pause_animation));
    let dist_pause = min(
        sd_segment(pixel_pos, vec2(x_coord - pause_gap, mid_y - height * 0.1), vec2(x_coord - pause_gap, mid_y + height * 0.1), line_thickness),
        sd_segment(pixel_pos, vec2(x_coord + pause_gap, mid_y - height * 0.1), vec2(x_coord + pause_gap, mid_y + height * 0.1), line_thickness)
    );
    let pause_active = step(0.001, state.pause_animation) * (1.0 - smoothstep(0.5, 1.0, state.pause_animation));

    let p_local = pixel_pos - vec2(x_coord, mid_y);
    let p_rotated = vec2(-p_local.y, p_local.x);
    let play_scale = mix(0.01 * height, height * 0.18, min(state.play_animation * 2.0, 1.0)) *
                     mix(1.0, 2.0, smoothstep(0.5, 1.0, state.play_animation));
    let dist_play = sd_rounded_triangle(p_rotated, play_scale, play_scale * 0.5);
    let play_active = step(0.001, state.play_animation) * (1.0 - smoothstep(0.5, 1.0, state.play_animation));

    let icon_alpha = clamp(pause_active + play_active, 0.0, 1.0);
    let dist_icon = mix(dist_play, dist_pause, pause_active / (pause_active + play_active + 1e-6));

    let mask_bar = 1.0 - smoothstep(-0.8, 0.2, dist_bar);
    let mask_icon = (1.0 - smoothstep(-0.8, 0.2, dist_icon)) * icon_alpha;
    let main_mask = clamp(mask_bar + mask_icon, 0.0, 1.0);

    let shadow_bar = pow(1.0 - clamp(dist_bar / (4.5 * scale), 0.0, 1.0), 2.0) * 0.4;
    let shadow_icon = pow(1.0 - clamp(dist_icon / (4.5 * scale), 0.0, 1.0), 2.0) * 0.4 * icon_alpha;
    let shadow_mask = max(shadow_bar, shadow_icon);

    if (main_mask > 0.0 || shadow_mask > 0.0) {
        let normalized_y = 1.0 - clamp((pixel_pos.y - start_y) / height, 0.0, 1.0);
        let color_state = mix(vec3(0.5), vec3(1.0, 0.878, 0.824), f32(normalized_y <= state.volume));
        let border_mask = smoothstep(-2.5, -1.0, min(dist_bar, dist_icon));
        let final_rgb = mix(color_state, vec3(0.15), border_mask);

        let playhead_rgba = vec4(mix(vec3(0.0), final_rgb, main_mask), max(main_mask, shadow_mask));
        out_rgba = vec4(playhead_rgba.rgb + out_rgba.rgb * (1.0 - playhead_rgba.a), max(playhead_rgba.a, out_rgba.a));
    }

    if (out_rgba.a <= 0.0) { discard; }
    return out_rgba;
}

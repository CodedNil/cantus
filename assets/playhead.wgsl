struct Uniform {
    screen_size: vec2<f32>,
    time: f32,
    scale_factor: f32,
    mouse_pos: vec2<f32>,
};

struct PlayheadInfo {
    origin_x: f32,
    panel_start: f32,
    height: f32,
    volume: f32,
    bar_lerp: f32,
    play_lerp: f32,
    pause_lerp: f32,
    _padding: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniform;
@group(0) @binding(1) var<uniform> playhead: PlayheadInfo;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    let pos = array<vec2<f32>, 4>(vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0));
    let uv = array<vec2<f32>, 4>(vec2(0.0, 1.0), vec2(1.0, 1.0), vec2(0.0, 0.0), vec2(1.0, 0.0));
    return VertexOutput(vec4(pos[vi], 0.0, 1.0), uv[vi]);
}

fn sd_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let ba = b - a;
    let pa = p - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - h * ba) - r;
}

fn sd_rounded_triangle(p: vec2<f32>, r: f32, corner_radius: f32) -> f32 {
    let k = sqrt(3.0);
    var p_mod = p;
    p_mod.x = abs(p_mod.x);
    // Symmetry split
    let h = max(p_mod.x + k * p_mod.y, 0.0);
    p_mod -= 0.5 * vec2(h, h * k);
    // Clamp to segment and subtract radius
    p_mod -= vec2(clamp(p_mod.x, -0.5 * (r - corner_radius) * k, 0.5 * (r - corner_radius) * k), -0.5 * (r - corner_radius));
    return length(p_mod) * sign(-p_mod.y) - corner_radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_pos = in.uv * uniforms.screen_size;
    let scale = uniforms.scale_factor;
    let line_x = playhead.origin_x;
    let height = playhead.height;
    let top = playhead.panel_start;
    let mid_y = top + height * 0.5;

    let thickness = 3.5 * scale;
    // Distance to the sdf, for the main bar and the icon
    var d = 1e6;
    var d_icon = 1e6;
    var icon_alpha = 0.0;

    // --- Main Bar ---
    let bar_h = height * mix(0.5, 0.125, playhead.bar_lerp);
    d = min(
        sd_segment(pixel_pos, vec2(line_x, playhead.panel_start), vec2(line_x, playhead.panel_start + bar_h), thickness),
        sd_segment(pixel_pos, vec2(line_x, playhead.panel_start + playhead.height - bar_h), vec2(line_x, playhead.panel_start + playhead.height), thickness)
    );

    // --- Pause Icon ---
    let p_off = mix(0.0, 4.0 * scale, smoothstep(0.0, 0.5, playhead.pause_lerp));
    let d_pause = min(
        sd_segment(pixel_pos, vec2(line_x - p_off, mid_y - height * 0.1), vec2(line_x - p_off, mid_y + height * 0.1), thickness),
        sd_segment(pixel_pos, vec2(line_x + p_off, mid_y - height * 0.1), vec2(line_x + p_off, mid_y + height * 0.1), thickness)
    );
    let pause_alpha = step(0.001, playhead.pause_lerp) * (1.0 - smoothstep(0.5, 1.0, playhead.pause_lerp));

    // --- Play Icon ---
    let p_tri = pixel_pos - vec2(line_x, mid_y);
    let p_play = vec2(-p_tri.y, p_tri.x);
    let play_size = mix(0.01 * height, height * 0.18, min(playhead.play_lerp * 2.0, 1.0)) * mix(1.0, 2.0, smoothstep(0.5, 1.0, playhead.play_lerp));
    let d_play = sd_rounded_triangle(p_play, play_size, play_size * 0.5);
    let play_alpha = step(0.001, playhead.play_lerp) * (1.0 - smoothstep(0.5, 1.0, playhead.play_lerp));

    // Unified Icon logic
    d_icon = mix(d_play, d_pause, pause_alpha / (pause_alpha + play_alpha + 1e-6));
    icon_alpha = clamp(pause_alpha + play_alpha, 0.0, 1.0);

    // Final Antialiased Masks
    let mask1 = 1.0 - smoothstep(-0.8, 0.2, d);
    let mask2 = (1.0 - smoothstep(-0.8, 0.2, d_icon)) * icon_alpha;
    let combined_mask = clamp(mask1 + mask2, 0.0, 1.0);

    // --- Drop Shadow Logic ---
    let s_bar = pow(1.0 - clamp(d / (4.5 * scale), 0.0, 1.0), 2.0) * 0.4;
    let s_icon = pow(1.0 - clamp(d_icon / (4.5 * scale), 0.0, 1.0), 2.0) * 0.4 * icon_alpha;
    let shadow_mask = max(s_bar, s_icon);

    if (combined_mask <= 0.0 && shadow_mask <= 0.0) { discard; }

    // Coloring
    let rel_y = 1.0 - clamp((pixel_pos.y - top) / height, 0.0, 1.0);
    let is_vol = f32(rel_y <= playhead.volume);
    let color = mix(vec3(0.5), vec3(1.0, 0.878, 0.824), is_vol);

    let dist_combined = min(d, d_icon);
    let border = smoothstep(-2.5, -1.0, dist_combined);
    let final_rgb = mix(color, vec3(0.15), border);

    let final_color = mix(vec3(0.0), final_rgb, combined_mask);
    let final_alpha = max(combined_mask, shadow_mask);

    return vec4(final_color, final_alpha);
}

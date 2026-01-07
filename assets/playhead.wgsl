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
    let pos = array<vec2<f32>, 4>(vec2(-1., -1.), vec2(1., -1.), vec2(-1., 1.), vec2(1., 1.));
    let uv = array<vec2<f32>, 4>(vec2(0., 1.), vec2(1., 1.), vec2(0., 0.), vec2(1., 0.));
    return VertexOutput(vec4(pos[vi], 0., 1.), uv[vi]);
}

fn sd_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return length(max(q, vec2(0.))) + min(max(q.x, q.y), 0.) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_pos = in.uv * uniforms.screen_size;
    let scale = uniforms.scale_factor;
    let line_x = playhead.origin_x;
    let h = playhead.height;
    let top = playhead.panel_start;
    let mid_y = top + h * 0.5;

    let thickness = 3.5 * scale;
    var d = 1e6;

    // --- The Main Line Bar ---
    // At bar_lerp = 0, we want a solid rectangle from top to bottom.
    // At bar_lerp = 1, we want two smaller bars at top and bottom.
    if playhead.bar_lerp == 0.0 {
        let d_bar = sd_rounded_rect(pixel_pos - vec2(line_x, top + h * 0.5), vec2(thickness, h * 0.5), thickness);
        d = min(d, d_bar);
    } else {
        let gap = (h * 0.3) * playhead.bar_lerp;
        let bar_h = (h - gap) * 0.5;

        let d_top = sd_rounded_rect(pixel_pos - vec2(line_x, top + bar_h * 0.5), vec2(thickness, bar_h * 0.5), thickness);
        let d_bot = sd_rounded_rect(pixel_pos - vec2(line_x, top + h - bar_h * 0.5), vec2(thickness, bar_h * 0.5), thickness);
        d = min(d_top, d_bot);
    }

    // --- Pause Bars ---
    let p_val = playhead.pause_lerp;
    if p_val > 0.0 {
        let p_alpha = select(smoothstep(0.0, 0.5, p_val), 1.0 - smoothstep(0.5, 1.0, p_val), p_val > 0.5);
        let p_off = mix(0.0, 9.0 * scale, smoothstep(0.0, 0.5, p_val));
        let d_p1 = sd_rounded_rect(pixel_pos - vec2(line_x - p_off, mid_y), vec2(thickness, h * 0.18), thickness);
        let d_p2 = sd_rounded_rect(pixel_pos - vec2(line_x + p_off, mid_y), vec2(thickness, h * 0.18), thickness);
        // Combine with main dist using a "soft" alpha-like blend via distance manipulation is tricky,
        // simpler to just use the min distance and handle alpha in the final mask.
        d = min(d, min(d_p1, d_p2) + (1.0 - p_alpha) * 100.0);
    }

    // --- Play Box ---
    let i_val = playhead.play_lerp;
    if i_val > 0.0 {
        let i_alpha = select(smoothstep(0.0, 0.3, i_val), 1.0 - smoothstep(0.7, 1.0, i_val), i_val > 0.5);
        let i_size = h * 0.25 * (smoothstep(0.0, 0.5, i_val) + max(i_val - 0.5, 0.0) * 2.0);
        let d_i = sd_rounded_rect(pixel_pos - vec2(line_x, mid_y), vec2(i_size * 0.5), 2.0 * scale);
        d = min(d, d_i + (1.0 - i_alpha) * 100.0);
    }

    // Final Antialiased Mask
    let mask = 1.0 - smoothstep(-0.8, 0.2, d);
    if mask <= 0.0 { discard; }

    // Coloring
    let rel_y = 1.0 - clamp((pixel_pos.y - top) / h, 0.0, 1.0);
    let is_vol = f32(rel_y <= playhead.volume);
    let color = mix(vec3(0.5), vec3(1.0, 0.878, 0.824), is_vol);

    // Dark outline ("sticker" effect from icons.wgsl)
    let border = smoothstep(-2.5, -1.0, d);
    let final_rgb = mix(color, vec3(0.15), border);

    return vec4(final_rgb * mask, mask);
}

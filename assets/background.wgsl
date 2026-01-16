struct GlobalUniforms {
    screen_size: vec2<f32>,
    bar_height: vec2<f32>, // [start_y, height]
    mouse_pos: vec2<f32>,
    mouse_pressure: f32,
    playhead_x: f32,
    expansion_xy: vec2<f32>,
    expansion_time: f32,
    time: f32,
    scale_factor: f32,
};

struct BackgroundPill {
    rect: vec2<f32>, // [x_position, width]
    colors: array<u32, 4>,
    alpha: f32,
    image_index: i32,
};

@group(0) @binding(0) var<uniform> global: GlobalUniforms;
@group(0) @binding(1) var<storage, read> pills: array<BackgroundPill>;
@group(0) @binding(2) var t_images: texture_2d_array<f32>;
@group(0) @binding(3) var s_images: sampler;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) local_uv: vec2<f32>,
    @location(1) world_uv: vec2<f32>,
    @location(2) @interpolate(flat) pill_idx: u32,
    @location(3) pixel_pos: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    let pill = pills[i_idx];
    let margin = 16.0;
    let unit_coord = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));
    let pill_size = vec2(pill.rect.y, global.bar_height.y);

    // Expand vertex bounds to accommodate shadows/glows
    let local_pixel = unit_coord * (pill_size + 2.0 * margin) - margin;
    let pixel_pos = vec2(pill.rect.x, global.bar_height.x) + local_pixel;

    var out: VertexOutput;
    let ndc = (pixel_pos / global.screen_size) * 2.0 - 1.0;
    out.clip_pos = vec4(ndc.x, -ndc.y, 0.0, 1.0);
    out.local_uv = local_pixel / pill_size;
    out.world_uv = local_pixel / global.screen_size.y;
    out.pill_idx = i_idx;
    out.pixel_pos = pixel_pos;
    return out;
}

/// 4th-order squircle distance function
fn sd_squircle(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return pow(pow(max(q.x, 0.0), 4.0) + pow(max(q.y, 0.0), 4.0), 0.25) - r + min(max(q.x, q.y), 0.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pill = pills[in.pill_idx];
    let pill_size = vec2(pill.rect.y, global.bar_height.y);
    let rounding = 22.0 * global.scale_factor;

    // --- Interaction Logic ---
    let anim_t = (global.time - global.expansion_time) * 1.2;
    let ripple_active = step(0.0, anim_t) * step(anim_t, 1.0);

    // Geometry Ripple (Expand wave from expansion_xy)
    let ripple_vec = in.pixel_pos - global.expansion_xy;
    let wave_dist = abs(length(ripple_vec) - anim_t * 600.0);
    let wave_prof = (0.5 + 0.5 * cos(clamp(wave_dist / 80.0, 0.0, 1.0) * 3.14159)) * step(wave_dist, 80.0);
    let ripple_str = pow(1.0 - anim_t, 2.0) * wave_prof * 0.5 * ripple_active;

    // Cursor Influence
    let mouse_vec = in.pixel_pos - global.mouse_pos;
    let mouse_d = length(mouse_vec);
    let mouse_inf = pow(smoothstep(120.0 * global.scale_factor, 0.0, mouse_d), 2.0) * global.mouse_pressure;
    let mouse_pull = normalize(mouse_vec + 0.001) * mouse_inf * 15.0 * global.scale_factor;

    // --- Masking & Depth ---
    let bulge = ripple_str * 22.0 * global.scale_factor + mouse_inf * 8.0;
    let stretched_uv_y = (in.local_uv.y - 0.5) * (pill_size.y / (pill_size.y + bulge)) + 0.5;
    let dist = sd_squircle((in.local_uv - 0.5) * pill_size, (pill_size + vec2(0.0, bulge)) * 0.5, rounding);
    let mask = clamp(0.5 - dist, 0.0, 1.0); // Simple AA mask
    let shadow = (1.0 - smoothstep(0.0, 16.0, dist)) * 0.2;

    if (mask <= 0.0 && shadow <= 0.0) { discard; }

    // --- Background Synthesis ---
    let seed = f32(pill.colors[0] % 1000u) * 0.137 + f32(in.pill_idx) * 2.4;
    let t = (global.time * 0.15) + seed;

    // Lens Refraction: combines edge curvature warp, ripple wave, and mouse pull
    let lens_warp = pow(clamp(1.0 + min(dist, 0.0) / 120.0, 0.0, 1.0), 2.0) * 0.6;
    let uv = in.world_uv - (in.local_uv - 0.5) * lens_warp - normalize(ripple_vec + 0.001) * ripple_str - mouse_pull * 0.002;

    // Procedural Flow Mixing
    let p = uv * 0.2 * vec2(1.0, global.screen_size.y / global.screen_size.x);
    let s1 = sin(p.x * 6.0 + t + sin(p.y * 4.0 + t * 0.5));
    let s2 = sin(p.y * 5.0 - t + sin(p.x * 3.0 + t * 0.8));
    let mix_val = clamp((s1 * 0.5 + s2 * 0.3 + sin(length(p) * 4.0 + s1 + t) * 0.2) * 0.5 + 0.5, 0.0, 1.0);

    // Color Palette Unpacking
    let c0 = unpack4x8unorm(pill.colors[0]).rgb;
    let c1 = unpack4x8unorm(pill.colors[1]).rgb;
    let c2 = unpack4x8unorm(pill.colors[2]).rgb;
    let c3 = unpack4x8unorm(pill.colors[3]).rgb;

    // Vibrancy Post-Processing
    var bg = mix(mix(c0, c1, mix_val), mix(c3, c2, s2 * 0.5 + 0.5), mix_val);
    bg = mix(bg, (c0 + c1 + c2 + c3) * 0.25, 0.1); // Base color blend

    let luma = dot(bg, vec3(0.2126, 0.7152, 0.0722));
    bg = mix(vec3(luma), bg, mix(3.2, 1.6, smoothstep(0.1, 0.4, luma))); // Saturation boost
    bg = clamp(bg, vec3(0.06), vec3(0.85)) * min(1.0, 0.52 / max(luma, 0.001)); // Luma cap for UI readability
    bg = mix(bg, bg * 0.45, smoothstep(global.playhead_x + 1.2, global.playhead_x - 1.2, in.pixel_pos.x));

    // --- Layering & FX ---
    var color = bg;

    // Cover art
    let img_x = pill_size.x - pill_size.y;
    let local_x = in.local_uv.x * pill_size.x;
    let uv_img = vec2((local_x - img_x) / pill_size.y, stretched_uv_y);
    let tex = textureSample(t_images, s_images, uv_img, max(0, pill.image_index));
    let img_mask = (1.0 - smoothstep(-0.5, 0.5, sd_squircle((uv_img - 0.5) * pill_size.y, vec2(pill_size.y * 0.5), rounding)))
                 * step(0.0, f32(pill.image_index)) * step(img_x, local_x);
    color = mix(color, tex.rgb, img_mask * tex.a);

    // Glass sheen, rim light, and mouse-reactive highlight
    let sheen = smoothstep(0.1, 0.0, stretched_uv_y) * mask * 0.15;
    let rim = (1.0 - smoothstep(0.0, -6.0, dist)) * 0.1;
    let mouse_sheen = smoothstep(30.0, 0.0, abs(mouse_d - 15.0)) * mouse_inf * 0.2;
    color += mix(color, vec3(1.0), 0.3) * (sheen + rim + mouse_sheen);

    // Corner glints
    let stretched_local_uv = vec2(in.local_uv.x, stretched_uv_y);
    let glint = smoothstep(60.0, 0.0, length(stretched_local_uv * pill_size - 20.0)) * 0.1
              + smoothstep(60.0, 0.0, length(stretched_local_uv * pill_size - (pill_size - 20.0))) * 0.05;
    color += glint * mask;

    // Expansion flash
    color = mix(color, color * 1.5 + 0.1, (1.0 - anim_t) * smoothstep(80.0, 0.0, wave_dist) * ripple_active * 0.5);

    // Composition
    return vec4(color * mask * pill.alpha, max(mask, shadow) * pill.alpha);
}

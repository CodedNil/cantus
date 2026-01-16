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

struct IconInstance {
    pos: vec2<f32>,    // Center coordinate in pixels
    data: u32,
    image_index: i32,
};

@group(0) @binding(0) var<uniform> global: GlobalUniforms;
@group(0) @binding(1) var<storage, read> icons: array<IconInstance>;
@group(0) @binding(2) var t_images: texture_2d_array<f32>;
@group(0) @binding(3) var s_images: sampler;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) local_uv: vec2<f32>,
    @location(1) @interpolate(flat) icon_id: u32,
    @location(2) @interpolate(flat) pixel_radius: f32,
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    let icon = icons[i_idx];
    let unit_coord = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));

    // Proximity-based interactive growth
    let dist = distance(icon.pos, global.mouse_pos) / global.scale_factor / min(global.mouse_pressure, 1.0);
    let proximity = smoothstep(30.0, 8.0, dist); // 1.0 when touching, 0.0 when far

    let growth = 1.0 + (0.6 * proximity);
    let pixel_radius = 9.0 * global.scale_factor * growth;

    // Smoothly push left/right based on x difference
    let x_push = (icon.pos.x - global.mouse_pos.x) * proximity * 0.5;
    let offset_pos = icon.pos + vec2(x_push, 0.0);

    // Rotation based on x difference
    let angle = x_push * 0.03;
    let rotation = (unit_coord - 0.5) * (pixel_radius * 2.0);
    let rotated_pos = vec2(
        rotation.x * cos(angle) - rotation.y * sin(angle),
        rotation.x * sin(angle) + rotation.y * cos(angle)
    );

    // Final output
    let screen_pixel = offset_pos + rotated_pos;
    let ndc_pos = (screen_pixel / global.screen_size) * 2.0 - 1.0;

    var out: VertexOutput;
    out.clip_pos = vec4(ndc_pos.x, -ndc_pos.y, 0.0, 1.0);
    out.local_uv = unit_coord;
    out.icon_id = i_idx;
    out.pixel_radius = pixel_radius;
    return out;
}

fn sd_squircle(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + radius;
    let n = 4.0;
    return pow(pow(max(q.x, 0.0), n) + pow(max(q.y, 0.0), n), 1.0/n) - radius + min(max(q.x, q.y), 0.0);
}

fn sd_star(p: vec2<f32>, radius: f32, indent: f32) -> f32 {
    let k1 = vec2(0.80901699, -0.58778525);
    let k2 = vec2(-k1.x, k1.y);
    var p_sym = vec2(p.x, -p.y);
    p_sym.x = abs(p_sym.x);
    p_sym -= 2.0 * max(dot(k1, p_sym), 0.0) * k1;
    p_sym -= 2.0 * max(dot(k2, p_sym), 0.0) * k2;
    p_sym.x = abs(p_sym.x);
    p_sym.y -= radius;
    let ba = indent * vec2(-k1.y, k1.x) - vec2(0.0, radius);
    let h = clamp(dot(p_sym, ba) / dot(ba, ba), 0.0, 1.0);
    return length(p_sym - ba * h) * sign(p_sym.y * ba.x - p_sym.x * ba.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let icon = icons[in.icon_id];
    let local_pixel = (in.local_uv - 0.5) * (in.pixel_radius * 2.0);

    var out_color: vec3<f32>;
    var dist_to_shape: f32;

    let data = unpack2x16unorm(icon.data);
    let param = data.x;
    let alpha = data.y;

    if param >= 0.5 {
        // Render Favorite Star
        dist_to_shape = sd_star(local_pixel, in.pixel_radius * 0.5, in.pixel_radius * 0.32) - in.pixel_radius * 0.1 * global.scale_factor;
        // Horizontal split effect for toggle animation
        let star_fullness = (param - 0.5) * 2.0 ; // Star fullness is 0.5-1.0
        let split_line = in.local_uv.x - star_fullness;
        let selection_mask = clamp(split_line / fwidth(split_line) + 0.5, 0.0, 1.0);
        out_color = mix(vec3(1.0, 0.85, 0.2), vec3(0.33), selection_mask);
    } else {
        // Render Playlist Squircle
        dist_to_shape = sd_squircle(local_pixel, vec2(in.pixel_radius * 0.6), 6.0 * global.scale_factor);
        let tex_sample = textureSample(t_images, s_images, in.local_uv, icon.image_index).rgb;
        let icon_saturation = select(0.0, 0.7, param > 0.0);
        out_color = mix(tex_sample, vec3(0.24), icon_saturation);
    }

    // Masking & Shadow
    let mask = clamp(0.5 - dist_to_shape, 0.0, 1.0);
    let shadow = pow(1.0 - smoothstep(0.0, 6.0, dist_to_shape), 2.0) * 0.2;

    if (mask <= 0.0 && shadow <= 0.0) { discard; }

    // A subtle rim light around the edge
    let highlighting = pow((1.0 - smoothstep(0.0, -5.0, dist_to_shape)), 4.0) * 0.08;
    out_color += highlighting * mask;

    return vec4(out_color * mask * alpha, max(mask, shadow) * alpha);
}

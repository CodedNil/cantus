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

struct IconInstance {
    pos: vec2<f32>,    // Center coordinate in pixels
    alpha: f32,
    style_variant: f32, // 1.0 for Star (fav), 0.0 for Squircle (playlist)
    activity_param: f32,   // Generic interpolator for animations/states
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

    // Proximity-based interactive growth
    let distance_to_mouse = distance(icon.pos, global.mouse_pos) / global.scale_factor;
    let growth_factor = 1.0 + 0.6 * (1.0 - smoothstep(8.0, 24.0, distance_to_mouse));

    let pixel_radius = 11.0 * global.scale_factor * growth_factor;
    let unit_coord = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));

    // Scale quad centered on icon.pos
    let screen_pixel = icon.pos + (unit_coord - 0.5) * (pixel_radius * 2.0);
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

    if icon.style_variant > 0.5 {
        // Render Favorite Star
        dist_to_shape = sd_star(local_pixel, in.pixel_radius * 0.5, in.pixel_radius * 0.32) - in.pixel_radius * 0.1 * global.scale_factor;
        // Horizontal split effect for toggle animation
        let split_line = (in.local_uv.x - icon.activity_param);
        let selection_mask = clamp(split_line / fwidth(split_line) + 0.5, 0.0, 1.0);
        out_color = mix(vec3(1.0, 0.85, 0.2), vec3(0.33), selection_mask);
    } else {
        // Render Playlist Squircle
        dist_to_shape = sd_squircle(local_pixel, vec2(in.pixel_radius * 0.6), 6.0 * global.scale_factor);
        let tex_sample = textureSample(t_images, s_images, in.local_uv, icon.image_index).rgb;
        out_color = mix(tex_sample, vec3(0.24), icon.activity_param);
    }

    let anti_alias_unit = fwidth(dist_to_shape) * 0.5;
    let shape_mask = 1.0 - smoothstep(-anti_alias_unit, anti_alias_unit, dist_to_shape);

    // Subtle outline border
    let border_width = global.scale_factor;
    let border_inner_mask = smoothstep(-border_width - anti_alias_unit, -border_width + anti_alias_unit, dist_to_shape);
    out_color = mix(out_color, vec3(0.15), border_inner_mask);

    let final_alpha = shape_mask * icon.alpha;
    return vec4(out_color * final_alpha, final_alpha);
}

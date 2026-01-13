struct Uniform {
    screen_size: vec2<f32>,
    mouse_pos: vec2<f32>,
    playhead_x: f32,
    time: f32,
    scale_factor: f32,
};

struct IconInstance {
    pos: vec2<f32>,
    alpha: f32,
    variant: f32, // 1.0 for star, 0.0 for squircle
    param: f32,
    image_index: i32,
    _padding: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniform;
@group(0) @binding(1) var<storage, read> icons: array<IconInstance>;
@group(0) @binding(2) var t_images: texture_2d_array<f32>;
@group(0) @binding(3) var s_images: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(flat) icon_id: u32,
    @location(2) @interpolate(flat) radius: f32,
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    let icon = icons[i_idx];

    // Calculate growth from mouse position
    let d_mouse = distance(icon.pos, uniforms.mouse_pos) / uniforms.scale_factor;
    let growth = 1.0 + 0.6 * (1.0 - smoothstep(8.0, 24.0, d_mouse));

    // Scale the quad geometry
    let radius = 11.0 * uniforms.scale_factor * growth;
    let corner = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));
    let pixel_pos = icon.pos + (corner - 0.5) * (radius * 2.0);

    let ndc = (pixel_pos / uniforms.screen_size) * 2.0 - 1.0;
    return VertexOutput(
        vec4(ndc.x, -ndc.y, 0.0, 1.0),
        corner,
        i_idx,
        radius
    );
}

fn sd_squircle(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    let n = 4.0;
    return pow(pow(max(q.x, 0.0), n) + pow(max(q.y, 0.0), n), 1.0/n) - r + min(max(q.x, q.y), 0.0);
}

fn sd_star(p: vec2<f32>, r: f32, rf: f32) -> f32 {
    let k1 = vec2(0.80901699, -0.58778525);
    let k2 = vec2(-k1.x, k1.y);
    var p_l = vec2(p.x, -p.y);
    p_l.x = abs(p_l.x);
    p_l -= 2.0 * max(dot(k1, p_l), 0.0) * k1;
    p_l -= 2.0 * max(dot(k2, p_l), 0.0) * k2;
    p_l.x = abs(p_l.x);
    p_l.y -= r;
    let ba = rf * vec2(-k1.y, k1.x) - vec2(0.0, r);
    let h = clamp(dot(p_l, ba) / dot(ba, ba), 0.0, 1.0);
    return length(p_l - ba * h) * sign(p_l.y * ba.x - p_l.x * ba.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let icon = icons[in.icon_id];
    let p = (in.uv - 0.5) * (in.radius * 2.0);

    var color: vec3<f32>;
    var dist: f32;
    if icon.variant > 0.5 {
        dist = sd_star(p, in.radius * 0.5, in.radius * 0.32) - in.radius * 0.1 * uniforms.scale_factor;
        let line_pos = (in.uv.x - icon.param);
        let split = clamp(line_pos / fwidth(line_pos) + 0.5, 0.0, 1.0);
        color = mix(vec3(1.0, 0.85, 0.2), vec3(0.33), split);
    } else {
        dist = sd_squircle(p, vec2(in.radius * 0.6), 6.0 * uniforms.scale_factor);
        let tex = textureSample(t_images, s_images, in.uv, icon.image_index).rgb;
        color = mix(tex, vec3(0.24), icon.param);
    }
    // Sharp anti-aliasing
    let unit = fwidth(dist) * 0.5;
    let mask = 1.0 - smoothstep(-unit, unit, dist);

    // Border
    let border_w = uniforms.scale_factor;
    let border_mask = smoothstep(-border_w - unit, -border_w + unit, dist);
    color = mix(color, vec3(0.15), border_mask);

    // Alpha blending instead of discard for better performance
    let final_alpha = mask * icon.alpha;
    return vec4(color * final_alpha, final_alpha);
}

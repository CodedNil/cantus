struct Uniform {
    screen_size: vec2<f32>,
    time: f32,
    scale_factor: f32,
    mouse_pos: vec2<f32>,
};

struct IconInstance {
    pos: vec2<f32>,
    alpha: f32,
    variant: f32,
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
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    let icon = icons[i_idx];
    let render_size = 40.0 * uniforms.scale_factor;
    let corner = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));
    let pixel_pos = icon.pos + (corner - 0.5) * render_size;
    let ndc = (pixel_pos / uniforms.screen_size) * 2.0 - 1.0;
    return VertexOutput(vec4(ndc.x, -ndc.y, 0.0, 1.0), corner, i_idx);
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
    let d_mouse = distance(icon.pos, uniforms.mouse_pos);
    let growth = 1.0 + 0.6 * (1.0 - smoothstep(8.0 * uniforms.scale_factor, 24.0 * uniforms.scale_factor, d_mouse));

    let size = 16.0 * uniforms.scale_factor * growth;
    let p = (in.uv - 0.5) * 40.0 * uniforms.scale_factor;

    var color: vec3<f32>;
    var dist: f32;
    if icon.variant > 0.5 {
        dist = sd_star(p, size * 0.38, size * 0.24) - 4.0;
        let split_mask = clamp((in.uv.x - icon.param) / fwidth(in.uv.x) + 0.5, 0.0, 1.0);
        color = mix(vec3(1.0, 0.85, 0.2), vec3(0.33), split_mask);
    } else {
        dist = sd_squircle(p, vec2(size * 0.5), 12.0 * uniforms.scale_factor);
        color = mix(textureSample(t_images, s_images, p / size + 0.5, icon.image_index).rgb, vec3(0.24), icon.param);
    }

    let mask = 1.0 - smoothstep(-0.5, 0.5, dist);
    let border_w = 1.0 * uniforms.scale_factor;
    let border = smoothstep(-border_w - 0.5, -border_w + 0.5, dist);
    color = mix(color, vec3(0.15), border);

    if mask <= 0.01 { discard; }
    return vec4(color * mask * icon.alpha, mask * icon.alpha);
}

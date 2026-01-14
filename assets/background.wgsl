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
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    let pill = pills[i_idx];
    let margin = 16.0; // Margin for shadows/rounding

    let unit_coord = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));
    let pill_size = vec2(pill.rect.y, global.layer_metrics.y);

    // Calculate local pixel position relative to pill top-left, including margin
    let local_pixel = unit_coord * (pill_size + 2.0 * margin) - margin;
    let pixel_pos = vec2(pill.rect.x, global.layer_metrics.x) + local_pixel;

    var out: VertexOutput;
    let ndc = (pixel_pos / global.screen_size) * 2.0 - 1.0;
    out.clip_pos = vec4(ndc.x, -ndc.y, 0.0, 1.0);
    out.local_uv = local_pixel / pill_size;
    out.world_uv = local_pixel / global.screen_size.y;
    out.pill_idx = i_idx;
    out.pixel_pos = pixel_pos;
    return out;
}

fn sd_squircle(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return pow(pow(max(q.x, 0.0), 4.0) + pow(max(q.y, 0.0), 4.0), 0.25) - r + min(max(q.x, q.y), 0.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pill = pills[in.pill_idx];
    let pill_size = vec2(pill.rect.y, global.layer_metrics.y);
    let rounding = 22.0 * global.scale_factor;

    // Geometry Mask
    let dist = sd_squircle((in.local_uv - 0.5) * pill_size, pill_size * 0.5, rounding);
    let mask = 1.0 - smoothstep(-0.5, 0.5, dist);
    let shadow = (1.0 - smoothstep(0.0, 8.0, dist)) * 0.3;
    if (mask <= 0.0 && shadow <= 0.0) { discard; }

    // Animated Noise Field
    let t = global.time * 0.8;
    var p = in.world_uv * 2.5 + f32(pill.colors[0] % 8u) * 7.123;
    let rot = mat2x2<f32>(0.8, -0.6, 0.6, 0.8);

    for(var i = 1.0; i <= 4.0; i += 1.0) {
        p += sin(p.yx + vec2(t, t * 0.7) + i) * 0.5;
        p *= rot;
    }

    // Color Mixing
    let c0 = unpack4x8unorm(pill.colors[0]).rgb;
    let c1 = unpack4x8unorm(pill.colors[1]).rgb;
    let c2 = unpack4x8unorm(pill.colors[2]).rgb;
    let c3 = unpack4x8unorm(pill.colors[3]).rgb;

    let w = sin(p.xyx * 0.4 + vec3(t * 0.1, -t * 0.15 + 2.0, t * 0.05 + 1.0)) * 0.5 + 0.5;
    var color = mix(mix(mix(c0, c1, w.x), c2, w.y), c3, w.z);

    // Polish & Processing
    let luma = dot(color, vec3(0.2126, 0.7152, 0.0722));
    color = mix(vec3(luma), color, 0.85); // Saturation
    color = (color - 0.5) * 1.05 + 0.5;    // Contrast
    color *= (1.0 - smoothstep(0.4, 1.2, luma) * 0.4); // Highlight compression
    color = mix(color * 1.1, color, smoothstep(0.0, -35.0, dist) * 0.5); // Inner glow

    // Darken Past Timeline
    color = mix(color, color * 0.5, smoothstep(global.playhead_x + 0.5, global.playhead_x - 0.5, in.pixel_pos.x));

    // Expansion Circle Effect
    let anim_t = (global.time - global.expansion_time) * 0.95;
    if (anim_t >= 0.0 && anim_t < 1.0) {
        let ripple = (1.0 - clamp(anim_t + 0.4, 0.0, 1.0)) * (1.0 - smoothstep(500.0 * anim_t - 2.0, 500.0 * anim_t, length(in.pixel_pos - global.expansion_xy)));
        color = mix(color, vec3(1.0, 0.88, 0.824), ripple);
    }

    // Cover Art
    let img_x = pill_size.x - pill_size.y;
    let local_x = in.local_uv.x * pill_size.x;
    if (pill.image_index >= 0 && local_x >= img_x) {
        let uv = vec2((local_x - img_x) / pill_size.y, in.local_uv.y);
        let tex = textureSample(t_images, s_images, uv, pill.image_index);
        let img_m = 1.0 - smoothstep(-0.5, 0.5, sd_squircle((uv - 0.5) * pill_size.y, vec2(pill_size.y * 0.5), rounding));
        color = mix(color, tex.rgb, img_m * tex.a);
    }

    return vec4(color * mask * pill.alpha, max(mask, shadow) * pill.alpha);
}

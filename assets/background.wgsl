struct Uniform {
    screen_size: vec2<f32>,
    time: f32,
    scale_factor: f32,
};

struct BackgroundPill {
    rect: vec4<f32>,
    dark_width: f32,
    alpha: f32,
    colors: array<u32, 4>,
    expansion_pos: vec2<f32>,
    expansion_time: f32,
    image_index: i32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniform;
@group(0) @binding(1) var<storage, read> pills: array<BackgroundPill>;
@group(0) @binding(2) var t_images: texture_2d_array<f32>;
@group(0) @binding(3) var s_images: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_uv: vec2<f32>,
    @location(2) @interpolate(flat) instance_index: u32,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> VertexOutput {
    let pill = pills[ii];
    let margin = 16.0;

    let corner = vec2<f32>(f32(vi % 2u), f32(vi / 2u));
    let local_pos = corner * (pill.rect.zw + 2.0 * margin) - margin;
    let pixel_pos = pill.rect.xy + local_pos;

    // Combined NDC and Y-flip
    let ndc = (pixel_pos / uniforms.screen_size) * 2.0 - 1.0;
    return VertexOutput(vec4(ndc.x, -ndc.y, 0.0, 1.0), local_pos / pill.rect.zw, local_pos / uniforms.screen_size.y, ii);
}

fn sd_squircle(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    let n = 4.0;
    return pow(pow(max(q.x, 0.0), n) + pow(max(q.y, 0.0), n), 1.0/n) - r + min(max(q.x, q.y), 0.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pill = pills[in.instance_index];
    let rounding = 28.0 * uniforms.scale_factor;

    // Shape Masking
    let d = sd_squircle((in.uv - 0.5) * pill.rect.zw, pill.rect.zw * 0.5, rounding);
    let edge_mask = 1.0 - smoothstep(-0.5, 0.5, d);
    let shadow_mask = (1.0 - smoothstep(0.0, 8.0, d)) * 0.3;

    if (edge_mask <= 0.0 && shadow_mask <= 0.0) { discard; }

    // Turbulent Color Field
    let t = uniforms.time * 0.8;
    let seed = f32(pill.colors[0] % 8u) * 0.123;
    var p = in.world_uv * 2.5 + seed;

    // Turbulent Color Field
    for(var i: f32 = 0.0; i < 4.0; i += 1.0) {
        let wave_offset = i + seed;
        p.x += sin(p.y + wave_offset + t) * 0.5;
        p.y += sin(p.x + wave_offset * 1.5 + t * 0.7) * 0.5;
        p = mat2x2<f32>(0.8, -0.6, 0.6, 0.8) * p;
    }

    let colors = array<vec3<f32>, 4>(
        unpack4x8unorm(pill.colors[0]).rgb, unpack4x8unorm(pill.colors[1]).rgb,
        unpack4x8unorm(pill.colors[2]).rgb, unpack4x8unorm(pill.colors[3]).rgb
    );

    // Color Selection & Mixing
    let weights = vec3(
        sin(p.x * 0.4 + t * 0.1) * 0.5 + 0.5,
        sin(p.y * 0.4 - t * 0.15 + 2.0) * 0.5 + 0.5,
        sin((p.x + p.y) * 0.3 + t * 0.05 + 1.0) * 0.5 + 0.5
    );

    var color = unpack4x8unorm(pill.colors[0]).rgb;
    color = mix(color, unpack4x8unorm(pill.colors[1]).rgb, weights.x);
    color = mix(color, unpack4x8unorm(pill.colors[2]).rgb, weights.y);
    color = mix(color, unpack4x8unorm(pill.colors[3]).rgb, weights.z);

    // Post-processing
    let luma = dot(color, vec3(0.2126, 0.7152, 0.0722));
    color = mix(vec3(luma), color, 0.85); // Vibrancy
    color = (color - 0.5) * 1.05 + 0.5; // Contrast
    color = color * (1.0 - smoothstep(0.4, 1.2, luma) * 0.4); // Compress highlights
    color = mix(color * 1.1, color, smoothstep(0.0, -35.0, d) * 0.5); // Inner glow

    // --- DARK TRACK TO THE LEFT ---
    color = mix(color, color * 0.5, step(in.uv.x * pill.rect.z, pill.dark_width));

    // --- EXPANDING CIRCLE (ANIMATION OR CLICK) ---
    let anim_lerp = (uniforms.time - pill.expansion_time) * 0.95;
    if (anim_lerp >= 0.0 && anim_lerp < 1.0) {
        let center = pill.expansion_pos - pill.rect.xy - pill.rect.zw * 0.5;
        let dist = length(((in.uv - 0.5) * pill.rect.zw) - center);
        let circle_alpha = (1.0 - clamp(anim_lerp + 0.4, 0.0, 1.0)) * (1.0 - smoothstep(500.0 * anim_lerp - 2.0, 500.0 * anim_lerp, dist));
        color = mix(color, vec3(1.0, 0.88, 0.824), circle_alpha);
    }

    // --- ALBUM IMAGE ---
    let image_start_x = pill.rect.z - pill.rect.w;
    let local_x = in.uv.x * pill.rect.z;
    if (pill.image_index >= 0 && local_x >= image_start_x) {
        let image_uv = vec2<f32>((local_x - image_start_x) / pill.rect.w, in.uv.y);
        let tex = textureSample(t_images, s_images, image_uv, pill.image_index);
        let image_d = sd_squircle((image_uv - 0.5) * pill.rect.w, vec2<f32>(pill.rect.w * 0.5), rounding);
        color = mix(color, tex.rgb, (1.0 - smoothstep(-0.5, 0.5, image_d)) * tex.a);
    }

    return vec4(color * edge_mask * pill.alpha, max(edge_mask, shadow_mask) * pill.alpha);
}

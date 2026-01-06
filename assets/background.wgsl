struct Uniform {
    screen_size: vec2<f32>,
    time: f32,
    _padding: f32,
};

struct BackgroundPill {
    rect: vec4<f32>,
    radii: vec2<f32>, // left, right
    colors: array<u32, 4>,
    alpha: f32,
    _padding: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniform;
@group(0) @binding(1) var<storage, read> pills: array<BackgroundPill>;

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

    // Standard 0-1 corner calculation
    let corner = vec2<f32>(f32(vi % 2u), f32(vi / 2u));
    let size_with_margin = pill.rect.zw + 2.0 * margin;
    let local_pos = corner * size_with_margin - margin;

    let pixel_pos = pill.rect.xy + local_pos;
    let ndc = (pixel_pos / uniforms.screen_size) * 2.0 - 1.0;

    // local_uv is for the rounded rect (0.0 to 1.0 over the pill rect)
    let local_uv = local_pos / pill.rect.zw;

    // world_uv is anchored to the pill's top-left in screen-height units.
    // This makes the noise move WITH the pill, but not STRETCH with the pill.
    let world_uv = local_pos / uniforms.screen_size.y;

    return VertexOutput(vec4(ndc.x, -ndc.y, 0.0, 1.0), local_uv, world_uv, ii);
}

fn sd_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: vec2<f32>) -> f32 {
    let r_val = select(r.x, r.y, p.x > 0.0);
    let q = abs(p) - b + r_val;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2(0.0))) - r_val;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pill = pills[in.instance_index];

    // Shape Masking
    let local_p = (in.uv - 0.5) * pill.rect.zw;
    let d = sd_rounded_rect(local_p, pill.rect.zw * 0.5, pill.radii);

    let edge_mask = 1.0 - smoothstep(-0.5, 0.5, d);
    let shadow_mask = (1.0 - smoothstep(0.0, 8.0, d)) * 0.3;

    if (edge_mask <= 0.0 && shadow_mask <= 0.0) { discard; }

    // Turbulent Color Field
    // We use world_uv which is anchored to the pill origin but spans screen units.
    let t = uniforms.time * 1.5;
    var p = in.world_uv * 2.5;

    // Iterative sine-wave turbulence (Cheap Turbulence)
    for(var i: f32 = 0.0; i < 4.0; i += 1.0) {
        p.x += sin(p.y + i + t) * 0.5;
        p.y += sin(p.x + i * 1.5 + t * 0.8) * 0.5;
        p = mat2x2<f32>(0.8, -0.6, 0.6, 0.8) * p;
    }

    let colors = array<vec3<f32>, 4>(
        unpack4x8unorm(pill.colors[0]).rgb, unpack4x8unorm(pill.colors[1]).rgb,
        unpack4x8unorm(pill.colors[2]).rgb, unpack4x8unorm(pill.colors[3]).rgb
    );

    // Pick colors based on warped coordinates
    let weight_a = sin(p.x * 0.5) * 0.5 + 0.5;
    let weight_b = sin(p.y * 0.5 + 2.0) * 0.5 + 0.5;
    let weight_c = sin((p.x + p.y) * 0.3 + 1.0) * 0.5 + 0.5;

    var color = mix(colors[0], colors[1], weight_a);
    color = mix(color, colors[2], weight_b);
    color = mix(color, colors[3], weight_c);

    // Post-processing (Vibrancy, Contrast, and Soft Highlight Compression)
    let luma = dot(color, vec3(0.2126, 0.7152, 0.0722));
    color = mix(vec3(luma), color, 0.85); // Vibrancy
    color = (color - 0.5) * 1.05 + 0.5;   // Contrast

    // Compress highlights: Darkens bright areas without crushing the blacks/midtones
    color = color * (1.0 - smoothstep(0.4, 1.2, luma) * 0.4);

    // Subtle inner glow
    let inner_glow = smoothstep(0.0, -35.0, d);
    color = mix(color * 1.1, color, inner_glow * 0.5);

    return vec4(color * edge_mask * pill.alpha, max(edge_mask, shadow_mask) * pill.alpha);
}

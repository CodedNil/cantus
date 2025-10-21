struct WarpUniforms {
    resolution: vec4<f32>, // xy = framebuffer size, zw = inverse size
    params: vec4<f32>,     // x = time, y = warp strength, z = swirl strength, w = texture aspect
};

@group(0) @binding(0) var background_tex: texture_2d<f32>;
@group(0) @binding(1) var background_sampler: sampler;
@group(1) @binding(0) var<uniform> uniforms: WarpUniforms;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs(@builtin(vertex_index) index: u32) -> VertexOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>(3.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );
    var out: VertexOut;
    out.position = vec4<f32>(positions[index], 0.0, 1.0);
    return out;
}

@fragment
fn fs(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = uniforms.resolution.xy;
    let inv_resolution = uniforms.resolution.zw;
    let time = uniforms.params.x;
    let warp_strength = uniforms.params.y;
    let swirl_strength = uniforms.params.z;
    let texture_aspect = uniforms.params.w;

    let uv = frag_pos.xy * inv_resolution;
    let centered = uv - vec2<f32>(0.5, 0.5);
    let radius = length(centered);

    let swirl_angle = swirl_strength * warp_strength * sin(time * 0.6 + radius * 5.5);
    let s = sin(swirl_angle);
    let c = cos(swirl_angle);
    let rotated = vec2<f32>(
        centered.x * c - centered.y * s,
        centered.x * s + centered.y * c,
    );

    let wave = warp_strength * 0.045 * vec2<f32>(
        sin(time * 1.3 + uv.y * 12.0),
        cos(time * 1.1 + uv.x * 10.0),
    );

    let ripple = warp_strength * 0.035 * radius * vec2<f32>(
        cos(time * 0.7 + radius * 18.0),
        sin(time * 0.9 + radius * 20.0),
    );

    var warped = rotated + wave + ripple + vec2<f32>(0.5, 0.5);
    let edge_blend = clamp(radius * 1.35, 0.0, 1.0);
    warped = warped * (1.0 - edge_blend) + uv * edge_blend;

    let target_aspect = resolution.x * inv_resolution.y;
    var sample_uv = clamp(warped, vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0));
    if (texture_aspect > 0.0 && abs(texture_aspect - target_aspect) > 0.001) {
        if (texture_aspect > target_aspect) {
            let scale = target_aspect / texture_aspect;
            sample_uv.x = clamp(sample_uv.x * scale + (1.0 - scale) * 0.5, 0.0, 1.0);
        } else {
            let scale = texture_aspect / target_aspect;
            sample_uv.y = clamp(sample_uv.y * scale + (1.0 - scale) * 0.5, 0.0, 1.0);
        }
    }

    var color = textureSample(background_tex, background_sampler, sample_uv).rgb;
    let vignette = smoothstep(0.95, 0.35, radius);
    color *= mix(1.0, vignette, 0.4);

    return vec4<f32>(color, 1.0);
}

const WARP_STRENGTH: f32 = 2.0;
const SWIRL_STRENGTH: f32 = 0.8;
const WARP_TIME_SCALE: f32 = 0.8;
const EDGE_BLEND_POWER: f32 = 1.35;
const VIBRANCY_BOOST: f32 = 0.35;
const DARKEN_STRENGTH: f32 = 0.15;
const SHADOW_LIFT: f32 = 0.3;

struct WarpUniforms {
    params: vec4<f32>,
};

@group(0) @binding(0) var background_tex: texture_2d<f32>;
@group(0) @binding(1) var background_sampler: sampler;
@group(0) @binding(2) var<uniform> uniforms: WarpUniforms;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) index: u32) -> VertexOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>(3.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );
    let pos = positions[index];
    var out: VertexOut;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    let uv = pos * 0.5 + vec2<f32>(0.5, 0.5);
    out.uv = vec2<f32>(uv.x, 1.0 - uv.y);
    return out;
}

@fragment
fn fs(in: VertexOut) -> @location(0) vec4<f32> {
    let time = uniforms.params.x * WARP_TIME_SCALE;

    let uv = in.uv;
    let centered = uv - vec2<f32>(0.5, 0.5);
    let radius = length(centered);

    let swirl_angle = SWIRL_STRENGTH * WARP_STRENGTH * sin(time * 0.6 + radius * 5.5);
    let s = sin(swirl_angle);
    let c = cos(swirl_angle);
    let rotated = vec2<f32>(
        centered.x * c - centered.y * s,
        centered.x * s + centered.y * c,
    );

    let wave = WARP_STRENGTH * 0.045 * vec2<f32>(
        sin(time * 1.3 + uv.y * 12.0),
        cos(time * 1.1 + uv.x * 10.0),
    );

    let ripple = WARP_STRENGTH * 0.035 * radius * vec2<f32>(
        cos(time * 0.7 + radius * 18.0),
        sin(time * 0.9 + radius * 20.0),
    );

    var warped = rotated + wave + ripple + vec2<f32>(0.5, 0.5);
    let edge_blend = clamp(radius * EDGE_BLEND_POWER, 0.0, 1.0);
    warped = warped * (1.0 - edge_blend) + uv * edge_blend;

    var color = textureSample(background_tex, background_sampler, warped).rgb;
    color = max(color, vec3<f32>(SHADOW_LIFT));

    let luma = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(vec3<f32>(luma), color, 1.0 + VIBRANCY_BOOST);

    let contrast = mix(color * color, color, DARKEN_STRENGTH);
    color = clamp(contrast, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}

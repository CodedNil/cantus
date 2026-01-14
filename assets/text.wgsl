struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Uniform {
    screen_size: vec2<f32>,
};

struct TextInstance {
    rect: vec4<f32>,
    uv_rect: vec4<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniform;
@group(0) @binding(1) var<storage, read> instances: array<TextInstance>;
@group(0) @binding(2) var t_atlas: texture_2d<f32>;
@group(0) @binding(3) var s_atlas: sampler;

@vertex
fn vs_main(
    @builtin(vertex_index) v_idx: u32,
    @builtin(instance_index) i_idx: u32,
) -> VertexOutput {
    let instance = instances[i_idx];

    let p = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));

    let world_pos = instance.rect.xy + p * instance.rect.zw;
    let normalized = (world_pos / uniforms.screen_size) * 2.0 - 1.0;

    var out: VertexOutput;
    out.position = vec4<f32>(normalized.x, -normalized.y, 0.0, 1.0);

    // Sample exactly in line with the quad geometry
    out.uv = instance.uv_rect.xy + vec2<f32>(p.x, 1.0 - p.y) * instance.uv_rect.zw;
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let s = textureSample(t_atlas, s_atlas, in.uv).rgb;
    let dist = max(min(s.r, s.g), min(max(s.r, s.g), s.b)) - 0.5;
    let opacity = clamp(dist / fwidth(dist) + 0.5, 0.0, 1.0);
    return vec4<f32>(in.color.rgb, in.color.a * opacity);
}

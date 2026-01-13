struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Uniform {
    screen_size: vec2<f32>,
    bar_height: vec2<f32>,
    mouse_pos: vec2<f32>,
    playhead_x: f32,
    time: f32,
    expansion_xy: vec2<f32>,
    expansion_time: f32,
    scale_factor: f32,
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
    let pos = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );

    let p = pos[v_idx];
    let world_pos = instance.rect.xy + p * instance.rect.zw;
    let normalized_pos = (world_pos / uniforms.screen_size) * 2.0 - 1.0;

    var out: VertexOutput;
    out.position = vec4<f32>(normalized_pos.x, -normalized_pos.y, 0.0, 1.0);

    out.uv = instance.uv_rect.xy + vec2<f32>(p.x, 1.0 - p.y) * instance.uv_rect.zw;
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSampleLevel(t_atlas, s_atlas, in.uv, 0.0).rgb;
    let sig_dist = median(sample.r, sample.g, sample.b) - 0.5;

    let opacity = clamp(sig_dist / max(fwidth(sig_dist), 0.0001) + 0.5, 0.0, 1.0);
    return vec4<f32>(in.color.rgb, in.color.a * opacity);
}

fn median(r: f32, g: f32, b: f32) -> f32 {
    return max(min(r, g), min(max(r, g), b));
}

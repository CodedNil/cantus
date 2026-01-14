struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Uniform {
    screen_size: vec2<f32>,
    padding: vec2<f32>,
};

struct TextInstance {
    rect: vec4<f32>,    // [x, y, width, height] in screen pixels
    uv_rect: vec4<f32>, // [u, v, width, height] in 0..1 atlas space
    color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> global: Uniform;
@group(0) @binding(1) var<storage, read> instances: array<TextInstance>;
@group(0) @binding(2) var t_atlas: texture_2d<f32>;
@group(0) @binding(3) var s_atlas: sampler;

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    let instance = instances[i_idx];

    // Generate quad: (0,0), (1,0), (0,1), (1,1)
    let unit_coord = vec2<f32>(f32(v_idx % 2u), f32(v_idx / 2u));

    let pixel_pos = instance.rect.xy + unit_coord * instance.rect.zw;
    let normalized_pos = (pixel_pos / global.screen_size) * 2.0 - 1.0;

    var out: VertexOutput;
    out.clip_pos = vec4<f32>(normalized_pos.x, -normalized_pos.y, 0.0, 1.0);

    // Flip Y for UVs to match atlas orientation
    out.uv = instance.uv_rect.xy + vec2<f32>(unit_coord.x, 1.0 - unit_coord.y) * instance.uv_rect.zw;
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(t_atlas, s_atlas, in.uv).rgb;

    // MSDF median for multi-channel distance field
    let distance = max(min(sample.r, sample.g), min(max(sample.r, sample.g), sample.b)) - 0.5;

    // Standard screen-space AA with a slight sharpening factor
    let smooth_width = fwidth(distance) * 0.7071;
    let opacity = clamp(distance / max(smooth_width, 0.0001) + 0.5, 0.0, 1.0);

    // Soften glyph weight to prevent harsh aliasing
    return vec4<f32>(in.color.rgb, in.color.a * pow(opacity, 1.1));
}

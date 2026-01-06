struct Uniform {
    screen_size: vec2<f32>,
    time: f32,
    scale_factor: f32,
};

struct IconInstance {
    pos: vec2<f32>,
    size: f32,
    alpha: f32,
    variant: f32,
    param: f32,
    image_index: i32,
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
fn vs_main(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let icon = icons[instance_index];

    let corner = vec2<f32>(f32(vertex_index % 2u), f32(vertex_index / 2u));
    let pixel_pos = icon.pos + (corner - 0.5) * icon.size;

    let ndc = (pixel_pos / uniforms.screen_size) * 2.0 - 1.0;
    return VertexOutput(vec4(ndc.x, -ndc.y, 0.0, 1.0), corner, instance_index);
}

fn sd_rounded_rect(p: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - size + radius;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn sd_star(p: vec2<f32>, radius: f32, inner_radius: f32) -> f32 {
    let k1 = vec2<f32>(0.80901699, -0.58778525);
    let k2 = vec2<f32>(-k1.x, k1.y);
    
    var p_local = vec2<f32>(p.x, -p.y); // Vertical flip
    p_local.x = abs(p_local.x);
    p_local -= 2.0 * max(dot(k1, p_local), 0.0) * k1;
    p_local -= 2.0 * max(dot(k2, p_local), 0.0) * k2;
    p_local.x = abs(p_local.x);
    p_local.y -= radius;
    
    let segment = inner_radius * vec2<f32>(-k1.y, k1.x) - vec2<f32>(0.0, radius);
    let h = clamp(dot(p_local, segment) / dot(segment, segment), 0.0, radius);
    let dist = length(p_local - segment * h) * sign(p_local.y * segment.x - p_local.x * segment.y);
    
    return dist - 1.8; // Rounding by offsetting the distance field
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let icon = icons[in.icon_id];
    let icon_scale = 0.85;
    let p = (in.uv - 0.5) * icon.size / icon_scale;

    var color = vec3<f32>(0.0);
    var dist = 0.0;

    if icon.variant > 0.5 { // Star
        dist = sd_star(p, icon.size * 0.42, icon.size * 0.20);

        let star_rgb = vec3<f32>(1.0, 0.85, 0.2);
        let bg_rgb   = vec3<f32>(0.33, 0.33, 0.33);
        color = mix(bg_rgb, star_rgb, step(in.uv.x, icon.param));
    } else { // Rounded Rectangle (Playlist)
        let corner_radius = 2.0 * uniforms.scale_factor;
        dist = sd_rounded_rect(p, vec2<f32>(icon.size * 0.5), corner_radius);

        if icon.image_index >= 0 {
            let zoom = 4.0;
            let zoomed_uv = (in.uv * icon.size + zoom) / (icon.size + zoom * 2.0);
            color = textureSample(t_images, s_images, zoomed_uv, icon.image_index).rgb;
            if icon.param > 0.0 {
                color = mix(color, vec3<f32>(0.24, 0.24, 0.24), icon.param); // #3C3C3C
            }
        } else {
            color = vec3<f32>(0.24, 0.24, 0.24);
        }
    }

    let border_color = vec3<f32>(0.15, 0.15, 0.15);
    let border_width = 1.2;

    // Antialiased mask and border
    let mask = 1.0 - smoothstep(-0.5, 0.5, dist);
    color = mix(color, border_color, smoothstep(-border_width - 0.3, -border_width + 0.3, dist));

    if mask <= 0.01 { discard; }

    let final_alpha = mask * icon.alpha;
    return vec4(color * final_alpha, final_alpha);
}

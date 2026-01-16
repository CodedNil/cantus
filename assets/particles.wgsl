struct GlobalUniforms {
    screen_size: vec2<f32>,
    bar_height: vec2<f32>, // [start_y, height]
    mouse_pos: vec2<f32>,
    mouse_pressure: f32,
    playhead_x: f32,
    expansion_xy: vec2<f32>,
    expansion_time: f32,
    time: f32,
    scale_factor: f32,
};

struct Particle {
    spawn_pos: vec2<f32>,
    spawn_vel: vec2<f32>,
    end_time: f32,
    color: u32,
};

@group(0) @binding(0) var<uniform> global: GlobalUniforms;
@group(0) @binding(1) var<storage, read> particles: array<Particle>;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    let p = particles[i_idx];

    // Unpack Color and Duration
    let color_vec = unpack4x8unorm(p.color);
    let rgb = color_vec.rgb;
    let duration = (color_vec.a * 255.0) / 100.0;

    // Calculate Timing
    let spawn_time = p.end_time - duration;
    let dt = global.time - spawn_time;

    // Discard inactive particles
    if (dt < 0.0 || dt > duration) {
        return VertexOutput(vec4(0.0), vec4(0.0), vec2(0.0));
    }

    let p_life = dt / duration;
    let p_life_inv = 1.0 - p_life;
    let scale = global.scale_factor;

    // Initial velocity + linear gravity
    let pos = p.spawn_pos + p.spawn_vel * dt * scale;
    let dir = normalize(p.spawn_vel * scale);
    let perp = vec2(-dir.y, dir.x);

    // Expand length from 0 on spawn to full stretch
    let stretch = smoothstep(0.0, 0.1, dt / duration);
    let growth = p_life + 0.5;
    let half_len = 5.0 * scale * growth;
    let half_thick = 2.5 * scale * growth;

    // Build oriented quad
    let uv = array<vec2<f32>, 4>(vec2(-1.,-1.), vec2(1.,-1.), vec2(-1.,1.), vec2(1.,1.))[v_idx];
    let world_pos = pos + (dir * uv.x * half_len) + (perp * uv.y * half_thick);

    // Saturated color with a bright core
    let luma = dot(rgb, vec3(0.299, 0.587, 0.114));
    let spark_color = mix(mix(vec3(luma), rgb, 2.0), vec3(1.0), 0.2) * 2.0;

    var out: VertexOutput;
    out.clip_pos = vec4((world_pos / global.screen_size * 2.0 - 1.0) * vec2(1.0, -1.0), 0.0, 1.0);
    out.color = vec4(spark_color, p_life_inv * smoothstep(0.0, 0.15, dt) * 0.3);
    out.uv = uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Pill-shaped falloff
    let dist = length(in.uv * vec2(0.8, 1.0));
    let alpha = in.color.a * smoothstep(1.0, 0.2, dist);

    if (alpha <= 0.0) { discard; }
    return vec4(in.color.rgb * alpha, alpha);
}

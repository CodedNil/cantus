struct Uniform {
    screen_size: vec2<f32>,
    time: f32,
    line_x: f32,
};

struct Particle {
    spawn_y: f32,
    spawn_time: f32,
    duration: f32,
    color: u32,
    spawn_vel: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniform;
@group(0) @binding(1) var<storage, read> particles: array<Particle>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    let pos = array<vec2<f32>, 4>(vec2(-1., -1.), vec2(1., -1.), vec2(-1., 1.), vec2(1., 1.));
    let uv = array<vec2<f32>, 4>(vec2(0., 1.), vec2(1., 1.), vec2(0., 0.), vec2(1., 0.));
    return VertexOutput(vec4(pos[vi], 0., 1.), uv[vi]);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_pos = in.uv * uniforms.screen_size;
    var final_color = vec4(0.);

    for (var i = 0u; i < 64u; i++) {
        let p = particles[i];
        let dt = uniforms.time - p.spawn_time;
        if (dt < 0. || dt > p.duration) { continue; }

        let p_life = 1. - (dt / p.duration);
        let fade = p_life * smoothstep(0., 0.05, dt);

        let pos = vec2(uniforms.line_x, p.spawn_y) + p.spawn_vel * dt + vec2(0., 150. * dt * dt);
        let dir = normalize(p.spawn_vel + vec2(0., 300. * dt));

        let len = mix(8., 12., p_life);
        let thickness = mix(2.5, 4.0, p_life);

        let pa = (pixel_pos - pos) + (dir * len * 0.5);
        let ba = dir * len;
        let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);
        let dist = length(pa - ba * h);

        if (dist < thickness) {
            let intensity = 1. - (dist / thickness);
            let alpha = fade * intensity * intensity;
            let color = mix(unpack4x8unorm(p.color).rgb, vec3(1.), intensity * 0.5);
            final_color += vec4(color * alpha * 1.5, alpha);
        }
    }

    if (final_color.a <= 0.) { discard; }
    return final_color;
}

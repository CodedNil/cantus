use crate::{pill_coverage, pill_fragment, pill_interaction, pill_sheen, pill_vertex};
use cantus_shared::{GlobalUniforms, WeatherPill, smoothstep};
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn noise(p: Vec2) -> f32 {
    ((p.dot(vec2(127.1, 311.7))).sin() * 43_758.547).fract()
}

fn sky(p: Vec2, size: Vec2, pill: WeatherPill, time: f32) -> Vec3 {
    let uv = p / size;
    let [sun_x, sun_y] = pill.sun;
    let [current, near_weather, far_weather] = pill.conditions;
    let daylight = smoothstep(-0.2, 0.12, sun_y);
    let twilight = smoothstep(-0.45, 0.05, sun_y) * (1.0 - daylight);
    let vertical = smoothstep(1.0, 0.0, uv.y);
    let mut color = vec3(0.018, 0.035, 0.11)
        .lerp(vec3(0.12, 0.08, 0.24), vertical)
        .lerp(
            vec3(0.08, 0.34, 0.62).lerp(vec3(0.32, 0.67, 0.87), vertical),
            daylight,
        );
    color += vec3(0.95, 0.25, 0.12) * twilight * (1.0 - uv.y) * 0.65;

    let sun = vec2(sun_x * size.x, size.y * (0.78 - sun_y * 0.55));
    let sun_glow = smoothstep(34.0, 0.0, p.distance(sun)) * daylight;
    color += vec3(1.0, 0.42, 0.08) * sun_glow * sun_glow * 0.9;

    let edge = (uv.x - 0.5).abs() * 2.0;
    let weather = Vec3::from_array(current).max(
        Vec3::from_array(near_weather) * smoothstep(0.22, 0.86, edge)
            + Vec3::from_array(far_weather) * smoothstep(0.68, 0.98, edge),
    );
    let cloud = weather.x;
    let clouds = smoothstep(
        0.72 - cloud * 0.32,
        0.9,
        noise(uv * vec2(8.0, 2.3) + vec2(time * 0.025, 0.0)),
    ) * smoothstep(0.92, 0.12, uv.y);
    color = color.lerp(vec3(0.52, 0.58, 0.66), clouds * (0.25 + cloud * 0.62));

    let rain = smoothstep(0.93, 1.0, (p.x * 0.17 + (p.y + time * 85.0) * 0.07).fract())
        * smoothstep(0.15, 0.9, uv.y)
        * weather.y;
    color += vec3(0.35, 0.7, 1.0) * rain * 0.75;
    let fog = weather.z * smoothstep(0.1, 0.9, uv.y) * 0.52;
    color.lerp(vec3(0.62, 0.68, 0.72), fog)
}

#[spirv(vertex)]
pub fn vs_weather(
    #[spirv(vertex_index)] vertex: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] weather: &[WeatherPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel: &mut Vec2,
) {
    let pill = weather[0];
    (*out_pos, *out_pixel) = pill_vertex(vertex, global, pill.x, pill.width);
}

#[spirv(fragment)]
pub fn fs_weather(
    #[spirv(location = 0)] pixel: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] weather: &[WeatherPill],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = weather[0];
    let interaction = pill_interaction(pixel, global);
    let bulge = interaction.bulge();
    let (local, size, dist) = pill_fragment(pixel, global, pill.x, pill.width, bulge);
    let (mask, shadow) = pill_coverage(dist);
    if mask.max(shadow) <= 0.0 {
        kill();
    }
    let refracted = interaction.refract(local, size, dist);
    let color =
        sky(refracted * size, size, pill, global.time) + pill_sheen(refracted.y, dist, interaction);
    let color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
    let opacity = 0.96;
    *out_color = (color * mask * opacity).extend(mask.max(shadow) * opacity);
}

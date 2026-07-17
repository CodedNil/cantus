use crate::{pill_coverage, pill_fragment, pill_interaction, pill_sheen, pill_vertex};
use cantus_shared::{GlobalUniforms, WeatherCondition as Condition, WeatherPill, smoothstep};
use core::f32::consts::TAU;
use spirv_std::{
    arch::kill,
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn noise(p: Vec2) -> f32 {
    let value = p.dot(vec2(127.1, 311.7)).sin() * 43_758.547;
    value - value.floor()
}

fn value_noise(p: Vec2) -> f32 {
    let cell = p.floor();
    let f = p - cell;
    let blend = f * f * (3.0 - 2.0 * f);
    let bottom = noise(cell).lerp(noise(cell + vec2(1.0, 0.0)), blend.x);
    let top = noise(cell + vec2(0.0, 1.0)).lerp(noise(cell + 1.0), blend.x);
    bottom.lerp(top, blend.y)
}

fn cloud_noise(mut p: Vec2) -> f32 {
    let mut density = 0.0;
    let mut amplitude = 0.533;
    for _ in 0..4 {
        density += value_noise(p) * amplitude;
        p = vec2(p.x * 1.7 + p.y * 1.1, p.y * 1.7 - p.x * 1.1) + vec2(2.3, -1.7);
        amplitude *= 0.5;
    }
    density
}

fn particles(
    p: Vec2,
    movement: Vec2,
    cell_size: Vec2,
    stretch: f32,
    radius: f32,
    density: f32,
) -> f32 {
    let q = p - movement;
    let cell = (q / cell_size).floor();
    let center = (cell + 0.2 + vec2(noise(cell), noise(cell + 17.3)) * 0.6) * cell_size;
    let distance = ((q - center) * vec2(1.0, stretch)).length();
    smoothstep(radius + 0.45, radius - 0.15, distance)
        * smoothstep(1.0 - density, 1.0, noise(cell + 31.7))
}

fn rain_layer(p: Vec2, time: f32, wind: f32, depth: f32, offset: f32) -> f32 {
    let speed = 72.0 + depth * 38.0;
    let slant = 0.2 + wind * 0.42;
    let q = p - vec2(time * speed * slant, time * speed) + offset;
    let cell_size = vec2(22.0 - depth * 5.0, 26.0 - depth * 3.0);
    let cell = (q / cell_size).floor();
    let random = vec2(noise(cell), noise(cell + 17.3));
    let local = q - (cell + 0.15 + random * 0.7) * cell_size;
    let curve =
        local.x - local.y * slant + (local.y * 0.18 + random.x * TAU).sin() * (0.2 + wind * 0.35);
    smoothstep(0.9, 0.12, curve.abs())
        * smoothstep(9.0 + depth * 2.0, 6.0 + depth * 2.0, local.y.abs())
        * smoothstep(0.64 - depth * 0.16, 1.0, noise(cell + 31.7))
}

fn scene(p: Vec2, size: Vec2, pill: WeatherPill, weather: Condition, time: f32) -> Vec3 {
    let [cloud, fog, wind, lightning] = weather.atmosphere.to_array();
    let [rain, showers, snow, hail] = weather.precipitation.to_array();
    let uv = p / size;
    let shower = value_noise(vec2(uv.x * 4.0 - time * 0.18, time * 0.12));
    let rain_amount = (rain + showers * smoothstep(0.38, 0.72, shower)).clamp(0.0, 1.0);
    let [sun_x, sun_y] = pill.sun;
    let daylight = smoothstep(-0.08, 0.3, sun_y);
    let twilight = smoothstep(-0.45, 0.12, sun_y) * (1.0 - smoothstep(0.08, 0.38, sun_y));
    let vertical = smoothstep(1.0, 0.0, uv.y);
    let mut color = vec3(0.018, 0.035, 0.11)
        .lerp(vec3(0.12, 0.08, 0.24), vertical)
        .lerp(
            vec3(0.08, 0.34, 0.62).lerp(vec3(0.32, 0.67, 0.87), vertical),
            daylight,
        );
    color = color.lerp(
        vec3(0.96, 0.24, 0.06).lerp(vec3(0.28, 0.07, 0.3), vertical),
        twilight * 0.82,
    );

    let sun = vec2(sun_x * size.x, size.y * (0.78 - sun_y * 0.55));
    let text_clearance =
        smoothstep(0.48, 0.3, (uv.x - 0.5).abs()) * smoothstep(0.4, 0.18, (uv.y - 0.5).abs());
    let sun_glow = smoothstep(40.0, 0.0, p.distance(sun))
        * daylight
        * (1.0 - cloud * cloud * 0.82)
        * (1.0 - text_clearance * 0.3);
    color += vec3(1.0, 0.46, 0.1) * sun_glow * sun_glow * 0.9;

    let stars = particles(p, Vec2::ZERO, Vec2::splat(18.0), 1.0, 0.55, 0.25)
        * (1.0 - daylight)
        * (1.0 - cloud);
    color += Vec3::splat(stars * (0.3 + vertical * 0.7));

    let cloud_uv = p / size.y;
    let drift = vec2(time * (0.045 + wind * 0.16), -0.15);
    let turbulence = vec2(
        value_noise(cloud_uv * 2.4 + 2.4),
        value_noise(cloud_uv * 2.1 + 8.7),
    ) - 0.5;
    let cloud_field = cloud_noise(cloud_uv * 0.82 + drift + turbulence * 0.2) * 0.62
        + cloud_noise(cloud_uv * 0.28 + drift * 0.35 - turbulence * 0.1) * 0.38;
    let cloud_density = smoothstep(0.5 - cloud * 0.22, 0.72 - cloud * 0.08, cloud_field);
    let cloud_mask = (cloud * (0.12 + cloud_density * 0.72) + smoothstep(0.76, 1.0, cloud) * 0.15)
        .clamp(0.0, 0.94)
        * (0.75 + vertical * 0.25);
    let cloud_light = smoothstep(0.18, 0.88, cloud_field);
    let cloud_color = vec3(0.45, 0.52, 0.62)
        .lerp(vec3(0.72, 0.77, 0.82), cloud_light)
        .lerp(
            vec3(0.12, 0.16, 0.23).lerp(vec3(0.37, 0.43, 0.52), cloud_light),
            (rain + showers + lightning).clamp(0.0, 1.0),
        );
    color = color.lerp(cloud_color, cloud_mask);

    color = color.lerp(vec3(0.1, 0.17, 0.25), rain_amount * 0.14);
    let rain = (rain_layer(p, time, wind, 1.0, 0.0) + rain_layer(p, time, wind, 0.35, 37.0))
        * smoothstep(0.05, 0.95, uv.y)
        * rain_amount;
    color += vec3(0.52, 0.72, 0.9) * rain * 0.42;

    let snow = (particles(
        p,
        vec2(time * wind * 12.0, time * 15.0),
        Vec2::splat(18.0),
        1.0,
        1.0,
        0.72,
    ) + particles(
        p + 31.0,
        vec2(time * wind * 8.0, time * 10.0),
        Vec2::splat(25.0),
        1.0,
        1.3,
        0.65,
    )) * snow;
    color = color.lerp(Vec3::splat(0.96), snow.clamp(0.0, 0.92));

    let hail = particles(
        p,
        vec2(time * 85.0 * (0.15 + wind * 0.35), time * 85.0),
        Vec2::splat(23.0),
        1.0,
        0.22,
        0.3,
    ) * hail;
    color = color.lerp(vec3(0.75, 0.86, 0.94), hail * 0.7);

    let flash = smoothstep(0.92, 1.0, (time * 2.7).sin()) * lightning;
    color = color.lerp(vec3(0.65, 0.74, 0.96), flash * 0.55);

    let fog_field = value_noise(vec2(uv.x * 5.0 + time * 0.035, uv.y * 2.0));
    let fog = fog * (0.22 + smoothstep(0.2, 0.8, fog_field) * 0.66);
    color.lerp(
        vec3(0.54, 0.61, 0.67).lerp(vec3(0.76, 0.79, 0.8), fog_field),
        fog,
    )
}

fn sky(p: Vec2, size: Vec2, pill: WeatherPill, time: f32) -> Vec3 {
    let edge = ((p / size).x - 0.5).abs() * 2.0;
    let weather = if edge <= 0.6 {
        pill.conditions[0]
    } else {
        let (from, start) = if edge <= 0.8 { (0, 0.6) } else { (1, 0.8) };
        let (from, to) = (pill.conditions[from], pill.conditions[from + 1]);
        let blend = smoothstep(start, start + 0.2, edge);
        Condition {
            atmosphere: from.atmosphere.lerp(to.atmosphere, blend),
            precipitation: from.precipitation.lerp(to.precipitation, blend),
        }
    };
    scene(p, size, pill, weather, time)
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
    let (local, size, dist) = pill_fragment(pixel, global, pill.x, pill.width, interaction.bulge());
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

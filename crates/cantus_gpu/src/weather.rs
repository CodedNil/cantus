use crate::{pill_fragment, pill_sheen, pill_vertex, sd_rounded_box, smooth_union};
use cantus_shared::{
    GlobalUniforms, WEATHER_CALENDAR_ARROW_RADIUS, WEATHER_CALENDAR_ARROW_X,
    WEATHER_CALENDAR_EXTENSION, WEATHER_CALENDAR_TITLE_Y, WeatherCondition as Condition,
    WeatherPill, smoothstep,
};
use core::f32::consts::TAU;
use spirv_std::{
    arch::kill,
    glam::{UVec2, Vec2, Vec3, Vec4, uvec2, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn hash(p: Vec2) -> Vec2 {
    let mut value = uvec2(p.x as i32 as u32, p.y as i32 as u32);
    value = value * 1_664_525 + UVec2::splat(1_013_904_223);
    value.x += value.y * 1_664_525;
    value.y += value.x * 1_664_525;
    value ^= value >> 16;
    value.x += value.y * 1_664_525;
    value.y += value.x * 1_664_525;
    value ^= value >> 16;
    vec2(value.x as f32, value.y as f32) * 2.328_306_4e-10
}

fn simplex_noise(p: Vec2) -> f32 {
    const K1: f32 = 0.366_025_42;
    const K2: f32 = 0.211_324_87;
    let cell = (p + (p.x + p.y) * K1).floor();
    let a = p - cell + (cell.x + cell.y) * K2;
    let corner = if a.x > a.y {
        vec2(1.0, 0.0)
    } else {
        vec2(0.0, 1.0)
    };
    let b = a - corner + K2;
    let c = a - 1.0 + 2.0 * K2;
    let contribution = |offset: Vec2, point: Vec2| {
        let falloff = (0.5 - point.dot(point)).max(0.0);
        let gradient = hash(cell + offset) * 2.0 - 1.0;
        falloff * falloff * falloff * falloff * point.dot(gradient)
    };
    70.0 * (contribution(Vec2::ZERO, a) + contribution(corner, b) + contribution(Vec2::ONE, c))
}

fn fbm(mut p: Vec2) -> f32 {
    let mut density = 0.0;
    let mut amplitude = 0.5;
    for _ in 0..4 {
        density += simplex_noise(p) * amplitude;
        p = vec2(p.x * 1.6 + p.y * 1.2, p.y * 1.6 - p.x * 1.2);
        amplitude *= 0.5;
    }
    0.5 + density * 0.5
}

fn cloud_mass(p: Vec2, scale: f32, time: f32) -> f32 {
    fbm(p / scale * 0.14 + vec2(time * 0.012, 6.1))
}

fn clouds(p: Vec2, scale: f32, time: f32) -> Vec2 {
    let mass = cloud_mass(p, scale, time);
    let billows = fbm(p / scale * 0.287 + vec2(time * 0.018, -3.7));
    let shape = mass + (billows - 0.5) * 0.2;
    vec2(
        smoothstep(0.43, 0.69, shape),
        smoothstep(0.42, 0.72, billows) * 0.55 + smoothstep(0.48, 0.7, mass) * 0.45,
    )
}

fn god_rays(p: Vec2, sun: Vec2, width: f32, cloud_scale: f32, time: f32) -> f32 {
    let ray = p - sun;
    let depth = ray.y.max(0.0);
    let occlusion = smoothstep(0.42, 0.68, cloud_mass(p.lerp(sun, 0.3), cloud_scale, time))
        + smoothstep(0.42, 0.68, cloud_mass(p.lerp(sun, 0.6), cloud_scale, time));
    let transmission = 1.0 - occlusion * 0.5;
    transmission
        * (0.35 + smoothstep(1.0, 0.0, (ray.x / (depth * 0.55 + 24.0)).abs()) * 0.65)
        * smoothstep(5.0, 55.0, depth)
        * smoothstep(width * 0.85, 35.0, ray.length())
}

fn particles(p: Vec2, movement: Vec2, cell_size: f32, radius: f32, density: f32) -> f32 {
    let q = p - movement;
    let cell = (q / cell_size).floor();
    let random = hash(cell);
    let center = (cell + 0.2 + random * 0.6) * cell_size;
    smoothstep(radius + 0.45, radius - 0.15, q.distance(center))
        * smoothstep(1.0 - density, 1.0, hash(cell + 31.7).x)
}

fn rain_layer(p: Vec2, time: f32, depth: f32, offset: f32) -> f32 {
    let speed = 72.0 + depth * 38.0;
    let slant = 0.32;
    let q = p - vec2(time * speed * slant, time * speed) + offset;
    let cell_size = vec2(22.0 - depth * 5.0, 26.0 - depth * 3.0);
    let cell = (q / cell_size).floor();
    let random = hash(cell);
    let local = q - (cell + 0.15 + random * 0.7) * cell_size;
    let curve = local.x - local.y * slant + (local.y * 0.18 + random.x * TAU).sin() * 0.32;
    smoothstep(0.9, 0.12, curve.abs())
        * smoothstep(9.0 + depth * 2.0, 6.0 + depth * 2.0, local.y.abs())
        * smoothstep(0.64 - depth * 0.16, 1.0, hash(cell + 31.7).x)
}

pub(crate) fn scene(
    p: Vec2,
    size: Vec2,
    cloud_scale: f32,
    [sun_x, sun_y]: [f32; 2],
    weather: Condition,
    time: f32,
    sun_presence: f32,
) -> Vec3 {
    let uv = p / size;
    let daylight = smoothstep(-0.02, 0.12, sun_y);
    let twilight = smoothstep(0.18, 0.0, sun_y) * smoothstep(-0.45, -0.05, sun_y);
    let vertical = smoothstep(1.0, 0.0, uv.y);
    let mut color = vec3(0.006, 0.012, 0.035)
        .lerp(vec3(0.025, 0.04, 0.095), vertical)
        .lerp(
            vec3(0.08, 0.34, 0.62).lerp(vec3(0.32, 0.67, 0.87), vertical),
            daylight,
        );
    let sunset = vec3(0.68, 0.38, 0.3).lerp(vec3(0.24, 0.22, 0.36), vertical);
    color = color.lerp(sunset, twilight * 0.7);

    let sun_x = sun_x * size.x;
    let horizon_x = (sun_x / size.x - 0.5) * 2.0;
    let sun = vec2(
        sun_x,
        size.y + horizon_x * horizon_x * 20.0 - sun_y * size.y * 0.82,
    );
    let sun_visibility = daylight * smoothstep(-0.02, 0.04, sun_y);
    let low_sun = smoothstep(0.55, 0.02, sun_y);
    let sun_color = vec3(0.96, 0.98, 1.0).lerp(vec3(0.98, 0.74, 0.66), low_sun);
    let sun_distance = p.distance(sun);
    let sun_cloud = smoothstep(0.43, 0.69, cloud_mass(sun, cloud_scale, time)) * weather.cloud;
    let sun_clear = sun_visibility * (1.0 - sun_cloud * 0.82) * sun_presence;
    let sun_halo = smoothstep(62.0, 4.0, sun_distance) * sun_clear;
    let sun_core = smoothstep(11.0, 1.0, sun_distance) * sun_clear;
    color = color.lerp(sun_color, sun_halo * 0.24 + sun_core * 0.7);
    let rays =
        god_rays(p, sun, size.x, cloud_scale, time) * sun_clear * (0.14 + weather.cloud * 0.12);
    color += sun_color * rays;

    let stars =
        particles(p, Vec2::ZERO, 18.0, 0.55, 0.25) * (1.0 - daylight) * (1.0 - weather.cloud);
    color += Vec3::splat(stars * (0.3 + vertical * 0.7));

    let cloud_shape = clouds(p, cloud_scale, time);
    let cloud_light = cloud_shape.y;
    let night_cloud = vec3(0.16, 0.2, 0.28).lerp(vec3(0.32, 0.36, 0.43), cloud_light);
    let day_cloud = vec3(0.62, 0.7, 0.78).lerp(vec3(0.92, 0.94, 0.96), cloud_light);
    let dusk_cloud = vec3(0.5, 0.36, 0.4).lerp(vec3(0.76, 0.59, 0.56), cloud_light);
    let cloud_color = night_cloud
        .lerp(day_cloud, daylight)
        .lerp(dusk_cloud, twilight * 0.45);
    let cloud_mask = weather.cloud * cloud_shape.x * 0.82;
    color = color.lerp(cloud_color, cloud_mask);
    color += sun_color * rays * cloud_mask * 0.35;

    color = color.lerp(vec3(0.1, 0.17, 0.25), weather.rain * 0.14);
    let rain = (rain_layer(p, time, 1.0, 0.0) + rain_layer(p, time, 0.35, 37.0))
        * smoothstep(0.05, 0.95, uv.y)
        * weather.rain;
    color += vec3(0.52, 0.72, 0.9) * rain * 0.42;

    let flakes = particles(p, vec2(time * 6.0, time * 15.0), 18.0, 1.0, 0.72)
        + particles(p + 31.0, vec2(time * 4.0, time * 10.0), 25.0, 1.3, 0.65);
    let snow = flakes * weather.snow;
    color = color.lerp(Vec3::splat(0.96), snow.clamp(0.0, 0.92));

    let hail = particles(p, vec2(time * 18.0, time * 85.0), 23.0, 0.22, 0.3) * weather.hail;
    color = color.lerp(vec3(0.75, 0.86, 0.94), hail * 0.7);

    let flash = smoothstep(0.92, 1.0, (time * 2.7).sin()) * weather.lightning;
    color = color.lerp(vec3(0.65, 0.74, 0.96), flash * 0.55);

    let fog_field = fbm(vec2(uv.x * 0.9 + time * 0.008, uv.y * 0.32 + 12.0));
    let fog_density = weather.fog * (0.58 + smoothstep(0.35, 0.7, fog_field) * 0.18);
    color.lerp(vec3(0.63, 0.69, 0.73), fog_density)
}

pub(crate) fn forecast(conditions: [Condition; 3], edge: f32) -> Condition {
    let position = ((edge - 0.6) / 0.2).clamp(0.0, 2.0);
    conditions[0]
        .lerp(conditions[1], smoothstep(0.0, 1.0, position))
        .lerp(conditions[2], smoothstep(1.0, 2.0, position))
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
    let size = vec2(
        pill.width,
        WEATHER_CALENDAR_EXTENSION * pill.calendar_expansion,
    );
    (*out_pos, *out_pixel) = pill_vertex(vertex, global, pill.x, size);
}

#[spirv(fragment)]
pub fn fs_weather(
    #[spirv(location = 0)] pixel: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] weather: &[WeatherPill],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = weather[0];
    let (interaction, _, pill_size, body_dist) = pill_fragment(pixel, global, pill.x, pill.width);
    let expansion = smoothstep(0.0, 1.0, pill.calendar_expansion);
    let body_bottom = global.bar_height.x + global.bar_height.y;
    let calendar_height = WEATHER_CALENDAR_EXTENSION * expansion;
    let calendar_size = vec2(pill.width, calendar_height);
    let calendar_local = pixel - vec2(pill.x, body_bottom);
    let calendar_dist = sd_rounded_box(
        calendar_local - calendar_size * 0.5,
        calendar_size * 0.5,
        (calendar_height * 0.5).min(18.0),
    );
    let (dist, mask, alpha) = interaction.surface(smooth_union(
        body_dist,
        calendar_dist,
        20.0,
        pill.calendar_expansion,
    ));
    if alpha <= 0.0 {
        kill();
    }

    let size = vec2(
        pill_size.x,
        pill_size.y + WEATHER_CALENDAR_EXTENSION * expansion,
    );
    let local = pixel - vec2(pill.x, global.bar_height.x);
    let refracted = interaction.refract(local, size, dist);
    let edge = (local.x / pill.width - 0.5).abs() * 2.0;
    let mut color = scene(
        refracted * size,
        size,
        pill_size.y,
        pill.sun,
        forecast(pill.conditions, edge),
        global.time,
        1.0,
    ) + pill_sheen(refracted.y, dist, interaction);
    let today_presence = smoothstep(0.0, 12.0, pill.today.y);
    let today_distance = calendar_local.distance(pill.today);
    let today = smoothstep(13.0, 11.0, today_distance) * today_presence;
    let today_ring = (smoothstep(16.0, 14.0, today_distance)
        - smoothstep(13.0, 11.0, today_distance))
        * today_presence;
    color = color.lerp(Vec3::splat(0.88), today_ring * 0.55);
    color = color.lerp(color * 0.42 + 0.012, today * 0.82);
    let button = |x| {
        smoothstep(
            WEATHER_CALENDAR_ARROW_RADIUS - 1.0,
            WEATHER_CALENDAR_ARROW_RADIUS - 5.0,
            calendar_local.distance(vec2(x, WEATHER_CALENDAR_TITLE_Y)),
        )
    };
    color += Vec3::splat(
        (button(WEATHER_CALENDAR_ARROW_X) + button(pill.width - WEATHER_CALENDAR_ARROW_X))
            * pill.calendar_expansion
            * 0.055,
    );
    let color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
    *out_color = (color * mask).extend(alpha);
}

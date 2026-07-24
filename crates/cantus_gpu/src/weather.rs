use crate::{
    PillInteraction, pill_fragment, pill_sheen, pill_vertex, sd_capsule_box, sd_chevron,
    sd_rounded_box, smooth_union, stroke,
};
use cantus_shared::{
    GlobalUniforms, WeatherCondition as Condition, WeatherLayout, WeatherPill, smoothstep,
};
use spirv_std::{
    arch::kill,
    glam::{FloatExt, UVec2, Vec2, Vec3, Vec4, uvec2, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

pub fn hash(p: Vec2) -> Vec2 {
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

pub fn cloud_mass(p: Vec2, scale: f32, time: f32) -> f32 {
    fbm(p / scale * 0.14 + vec2(time * 0.012, 6.1))
}

fn god_rays(p: Vec2, sun: Vec2, width: f32, cloud_scale: f32, time: f32) -> f32 {
    let ray = p - sun;
    let depth = ray.y.max(0.0);
    let occlusion = smoothstep(0.42, 0.68, cloud_mass(p.lerp(sun, 0.3), cloud_scale, time))
        + smoothstep(0.42, 0.68, cloud_mass(p.lerp(sun, 0.6), cloud_scale, time));
    (1.0 - occlusion * 0.5)
        * (0.35 + smoothstep(1.0, 0.0, (ray.x / (depth * 0.55 + 24.0)).abs()) * 0.65)
        * smoothstep(5.0, 55.0, depth)
        * smoothstep(width * 0.85, 35.0, ray.length())
}

fn particles(p: Vec2, movement: Vec2, cell_size: f32, radius: f32, density: f32) -> f32 {
    let q = p - movement;
    let cell = (q / cell_size).floor();
    let center = (cell + 0.2 + hash(cell) * 0.6) * cell_size;
    smoothstep(radius + 0.45, radius - 0.15, q.distance(center))
        * smoothstep(1.0 - density, 1.0, hash(cell + 31.7).x)
}

fn rain_layer(p: Vec2, time: f32, depth: f32, seed: f32) -> f32 {
    let seed = vec2(seed, seed * 0.37);
    let layer = hash(seed) * 2.0 - 1.0;
    let drift = vec2(0.22 + layer.x * 0.05, 1.0);
    let q = p - time * (72.0 + depth * 38.0) * drift + layer * 91.0;
    let cell_size = vec2(
        21.0 - depth * 4.5 + layer.y * 1.5,
        31.0 - depth * 3.0 + layer.x * 1.5,
    );
    let cell = (q / cell_size).floor();
    let random = hash(cell + seed);
    let local = q - (cell + random * 0.75) * cell_size;
    let slant = 0.22 + random.x * 0.14 + layer.y * 0.05;
    let lane = (random.y - 0.5) * cell_size.x * 0.65;
    let curve = local.x - local.y * slant - lane;
    let streak = smoothstep(0.72, 0.0, curve.abs());
    let taper = smoothstep(0.0, 5.0 + depth * 2.0, local.y)
        * smoothstep(cell_size.y - 4.0, cell_size.y - 11.0, local.y);
    streak * taper * smoothstep(0.45, 0.95, hash(cell + 31.7).x)
}

fn scene(
    global: &GlobalUniforms,
    interaction: PillInteraction,
    refracted: Vec2,
    size: Vec2,
    dist: f32,
    sun_y: f32,
    weather: Condition,
) -> Vec3 {
    let p = refracted * size;
    let (cloud_scale, time) = (global.bar_height.y, global.time);
    let uv = p / size;
    let daylight = smoothstep(-0.02, 0.12, sun_y);
    let twilight = smoothstep(0.18, 0.0, sun_y) * smoothstep(-0.45, -0.05, sun_y);
    let vertical = smoothstep(1.0, 0.0, uv.y);
    let mut color = vec3(0.006, 0.012, 0.035)
        .lerp(vec3(0.025, 0.04, 0.095), vertical)
        .lerp(
            vec3(0.08, 0.34, 0.62).lerp(vec3(0.32, 0.67, 0.87), vertical),
            daylight,
        )
        .lerp(
            vec3(0.68, 0.38, 0.3).lerp(vec3(0.24, 0.22, 0.36), vertical),
            twilight * 0.7,
        );

    let stars = particles(p, Vec2::ZERO, 18.0, 0.55, 0.25) * (1.0 - daylight);
    color += Vec3::splat(stars * (1.0 - weather.cloud) * (0.3 + vertical * 0.7));

    let mass = cloud_mass(p, cloud_scale, time);
    let billows = fbm(p / cloud_scale * 0.287 + vec2(time * 0.018, -3.7));
    let cloud_shape = smoothstep(0.43, 0.69, mass + (billows - 0.5) * 0.2);
    let cloud_light = smoothstep(0.42, 0.72, billows) * 0.55 + smoothstep(0.48, 0.7, mass) * 0.45;
    let cloud_color = vec3(0.16, 0.2, 0.28)
        .lerp(vec3(0.32, 0.36, 0.43), cloud_light)
        .lerp(
            vec3(0.62, 0.7, 0.78).lerp(vec3(0.92, 0.94, 0.96), cloud_light),
            daylight,
        )
        .lerp(
            vec3(0.5, 0.36, 0.4).lerp(vec3(0.76, 0.59, 0.56), cloud_light),
            twilight * 0.45,
        );
    let cloud_mask = weather.cloud * cloud_shape * 0.82;
    color = color.lerp(cloud_color, cloud_mask);

    color = color.lerp(vec3(0.1, 0.17, 0.25), weather.rain * 0.2);
    let rain = (rain_layer(p, time, 1.0, 0.0)
        + rain_layer(p, time, 0.72, 37.0)
        + rain_layer(p, time, 0.35, 74.0))
        * weather.rain;
    color += vec3(0.52, 0.72, 0.9) * rain * 0.7;

    let snow = particles(p, vec2(time * 6.0, time * 15.0), 18.0, 1.0, 0.72)
        + particles(p + 31.0, vec2(time * 4.0, time * 10.0), 25.0, 1.3, 0.65);
    color = color.lerp(Vec3::splat(0.96), (snow * weather.snow).clamp(0.0, 0.92));

    let hail = particles(p, vec2(time * 18.0, time * 85.0), 23.0, 0.22, 0.3) * weather.hail;
    color = color.lerp(vec3(0.75, 0.86, 0.94), hail * 0.7);

    let flash = smoothstep(0.92, 1.0, (time * 2.7).sin()) * weather.lightning;
    color = color.lerp(vec3(0.65, 0.74, 0.96), flash * 0.55);

    let fog = fbm(vec2(uv.x * 0.9 + time * 0.008, uv.y * 0.32 + 12.0));
    color.lerp(
        vec3(0.63, 0.69, 0.73),
        weather.fog * (0.58 + smoothstep(0.35, 0.7, fog) * 0.18),
    ) + pill_sheen(refracted.y, dist, interaction)
}

fn sun_layer(
    mut color: Vec3,
    point: Vec2,
    size: Vec2,
    [sun_x, sun_y]: [f32; 2],
    weather: Condition,
    time: f32,
) -> Vec3 {
    let sun = vec2(
        16.0 + sun_x * (size.x - 32.0),
        size.y * (0.72 - sun_y.saturate() * 0.45),
    );
    let sun_color =
        vec3(0.96, 0.98, 1.0).lerp(vec3(0.98, 0.74, 0.66), smoothstep(0.55, 0.02, sun_y));
    let clear = smoothstep(-0.02, 0.04, sun_y)
        * (1.0 - smoothstep(0.43, 0.69, cloud_mass(sun, size.y, time)) * weather.cloud * 0.82);
    let distance = point.distance(sun);
    color = color.lerp(
        sun_color,
        (smoothstep(62.0, 4.0, distance) * 0.24 + smoothstep(11.0, 1.0, distance) * 0.7) * clear,
    );
    color
        + sun_color
            * god_rays(point, sun, size.x, size.y, time)
            * clear
            * (0.14 + weather.cloud * 0.12)
}

/// Sky backdrop shared by the weather and status pills, blending the forecast across the pill's width; also returns the refracted pixel position.
pub fn sky_background(
    global: &GlobalUniforms,
    interaction: PillInteraction,
    local: Vec2,
    size: Vec2,
    dist: f32,
    sun_height: f32,
    conditions: [Condition; 3],
) -> (Vec3, Vec2) {
    let refracted = interaction.refract(local, size, dist);
    (
        scene(
            global,
            interaction,
            refracted,
            size,
            dist,
            sun_height,
            blended_conditions(local, size, conditions),
        ),
        refracted * size,
    )
}

fn blended_conditions(local: Vec2, size: Vec2, conditions: [Condition; 3]) -> Condition {
    let position = ((local.x / size.x - 0.5).abs() * 10.0 - 3.0).clamp(0.0, 2.0);
    conditions[0]
        .lerp(conditions[1], smoothstep(0.0, 1.0, position))
        .lerp(conditions[2], smoothstep(1.0, 2.0, position))
}

fn forecast_at<const N: usize>(x: f32, forecasts: &[Condition; N]) -> Condition {
    let position = (x / WeatherLayout::WIDTH * N as f32 - 0.5).clamp(0.0, (N - 1) as f32);
    let index = position.floor() as usize;
    forecasts[index].lerp(
        forecasts[if index + 1 < N { index + 1 } else { index }],
        smoothstep(0.0, 1.0, position - position.floor()),
    )
}

#[spirv(vertex)]
pub fn vs_weather(
    #[spirv(vertex_index)] vertex: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] weather: &[WeatherPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel: &mut Vec2,
) {
    let pill = weather[0];
    let expansion = smoothstep(0.0, 1.0, pill.calendar_expansion);
    (*out_pos, *out_pixel) = pill_vertex(
        vertex,
        global,
        WeatherLayout::expanded_x(pill.x, expansion),
        WeatherLayout::popup_size(expansion),
    );
}

#[spirv(fragment)]
pub fn fs_weather(
    #[spirv(location = 0)] pixel: Vec2,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] weather: &[WeatherPill],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = weather[0];
    let (interaction, main_local, pill_size, body_dist) =
        pill_fragment(pixel, global, pill.x, pill.width);
    let expansion = smoothstep(0.0, 1.0, pill.calendar_expansion);
    let body_bottom = global.bar_height.x + global.bar_height.y;
    let popup_size = WeatherLayout::popup_size(expansion);
    let popup_local = pixel - vec2(WeatherLayout::expanded_x(pill.x, expansion), body_bottom);
    let content_origin = vec2(WeatherLayout::expanded_x(pill.x, 1.0), body_bottom);
    let content_local = pixel - content_origin;
    let top_gap = WeatherLayout::TOP_GAP * expansion;
    let box_size = vec2(popup_size.x, (popup_size.y - top_gap).max(0.0));
    let popup_dist = sd_rounded_box(
        popup_local - vec2(popup_size.x * 0.5, top_gap + box_size.y * 0.5),
        box_size * 0.5,
        (box_size.y * 0.5).min(18.0),
    );
    let main_dist = smooth_union(body_dist, popup_dist, 32.0, expansion);
    let (_, mask, alpha) = interaction.surface(main_dist);
    if alpha <= 0.0 {
        kill();
    }

    let reveal = |row| WeatherLayout::forecast_reveal(expansion, row);
    let moving_center_y =
        |row| WeatherLayout::forecast_center_at(global.bar_height.y, row, reveal(row)).y;
    let row = if content_local.y > (moving_center_y(0.0) + moving_center_y(1.0)) * 0.5 {
        1.0
    } else {
        0.0
    };
    let row_reveal = reveal(row);
    let (row_origin, row_size) = WeatherLayout::forecast_row(global.bar_height.y, row, row_reveal);
    let row_local = content_local - row_origin;
    let row_dist = sd_capsule_box(
        row_local - row_size * 0.5,
        (row_size.x - row_size.y) * 0.5,
        row_size.y * 0.5,
    );
    let forecast_x = content_local.x - WeatherLayout::FORECAST_X;
    let conditions = if row < 0.5 {
        forecast_at(forecast_x, &pill.hourly)
    } else {
        forecast_at(forecast_x, &pill.daily)
    };
    let object_size = vec2(popup_size.x, pill_size.y + popup_size.y);
    let object_local = popup_local + vec2(0.0, pill_size.y);
    let object_conditions = blended_conditions(object_local, object_size, pill.conditions);
    let pill_conditions = blended_conditions(main_local, pill_size, pill.conditions);
    let row_inside = row_dist < 0.5 && row_reveal > 0.0;
    let (scene_local, scene_size, scene_dist, scene_conditions) = if row_inside {
        (
            row_local,
            row_size,
            row_dist,
            object_conditions.lerp(conditions, row_reveal),
        )
    } else {
        (main_local, pill_size, main_dist, pill_conditions)
    };
    let refracted = interaction.refract(scene_local, scene_size, scene_dist);
    let mut color = scene(
        global,
        interaction,
        refracted,
        scene_size,
        scene_dist,
        pill.sun[1],
        scene_conditions,
    );
    if !row_inside && row_dist < 12.0 {
        let (_, row_mask, row_alpha) = interaction.surface(row_dist);
        color *= 1.0 - (row_alpha - row_mask).max(0.0) * row_reveal * 0.45;
    }
    if body_dist < 1.0 {
        color = color.lerp(
            sun_layer(
                color,
                main_local,
                pill_size,
                pill.sun,
                pill_conditions,
                global.time,
            ),
            smoothstep(1.0, -1.0, body_dist),
        );
    }
    let mouse = global.mouse_pos - content_origin;
    let header_reveal = WeatherLayout::header_reveal(expansion);
    let title = |point| {
        sd_rounded_box(
            point - WeatherLayout::TITLE,
            WeatherLayout::TITLE_HALF_SIZE,
            12.0,
        )
    };
    let title_dist = title(content_local);
    let title_hover =
        smoothstep(5.0, -2.0, title(mouse)) * global.mouse_pressure.saturate() * header_reveal;
    color = color.lerp(
        Vec3::ONE,
        (smoothstep(1.0, -1.0, title_dist) * 0.1 + stroke(title_dist, 1.0) * 0.16) * title_hover,
    );
    let today_presence = smoothstep(0.0, 12.0, pill.today.y);
    let today_distance = content_local.distance(pill.today);
    let today = smoothstep(13.0, 11.0, today_distance);
    let ring = smoothstep(16.0, 14.0, today_distance) - today;
    color = color.lerp(Vec3::splat(0.88), ring * today_presence * 0.55);
    color = color.lerp(color * 0.42 + 0.012, today * today_presence * 0.82);
    let arrow_button = |side: f32| {
        let center = WeatherLayout::arrow(side, header_reveal);
        let hover = smoothstep(WeatherLayout::ARROW_RADIUS, 6.0, mouse.distance(center))
            * global.mouse_pressure.saturate();
        let point = (content_local - center) * vec2(-side, 1.0);
        stroke(
            sd_chevron(point, Vec2::splat(5.0 + hover * 1.4)),
            1.6 + hover * 0.5,
        ) * (0.7 + hover * 0.6)
    };
    let arrows = arrow_button(-1.0) + arrow_button(1.0);
    color = color.lerp(Vec3::ONE, (arrows * header_reveal).min(1.0));
    let color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
    *out_color = (color * mask).extend(alpha);
}

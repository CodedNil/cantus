use crate::{
    GAP, GlobalUniforms, UNIT, cloud_mass, fbm, hash, pill_fragment, pill_sheen, pill_vertex,
    sd_capsule_box, sd_rounded_box, smooth_union, smoothstep,
};
use bitfields::bitfield;
use core::f32::consts::PI;
use spirv_std::{
    arch::{Derivative, kill},
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

/// Number of conditions shown in the hourly forecast row.
pub const HOURLY_FORECASTS: usize = 6;
/// Hours between adjacent conditions in the hourly forecast row.
pub const HOURLY_STEP_HOURS: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Data {
    /// Left edge of the collapsed pill in logical pixels.
    pub x: f32,
    /// How open the calendar popup is, from 0 (closed) to 1 (fully expanded).
    pub calendar_expansion: f32,
    /// `[sunrise, sunset]`, as hour-of-day.
    pub sun_hours: [f32; 2],
    /// Hour-of-day of the first hourly forecast sample.
    pub hourly_start: f32,
    /// Conditions at four-hour intervals; the first sample is the current condition.
    pub hourly: [WeatherCondition; HOURLY_FORECASTS],
    /// Condition for each of the next five days.
    pub daily: [WeatherCondition; 5],
}

/// Cloud/rain/snow/lightning/hail conditions, packed into a `u32`.
#[bitfield(u32, new = false)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct WeatherCondition {
    #[bits(4)]
    pub fog_raw: u32,
    #[bits(7)]
    pub cloud_raw: u32,
    #[bits(8)]
    pub rain_raw: u32,
    #[bits(8)]
    pub snow_raw: u32,
    pub lightning: bool,
    #[bits(4)]
    pub hail_raw: u32,
}

impl WeatherCondition {
    const FOUR_BIT_MAX: f32 = 15.0;
    const SEVEN_BIT_MAX: f32 = 127.0;
    const EIGHT_BIT_MAX: f32 = 255.0;

    /// Unpacks `[fog, cloud, rain, snow, lightning, hail]` as 0-1 values.
    pub fn values(self) -> [f32; 6] {
        [
            self.fog_raw() as f32 / Self::FOUR_BIT_MAX,
            self.cloud_raw() as f32 / Self::SEVEN_BIT_MAX,
            self.rain_raw() as f32 / Self::EIGHT_BIT_MAX,
            self.snow_raw() as f32 / Self::EIGHT_BIT_MAX,
            if self.lightning() { 1.0 } else { 0.0 },
            self.hail_raw() as f32 / Self::FOUR_BIT_MAX,
        ]
    }
}

pub const WIDTH: f32 = UNIT * 77.0;
pub const FORECAST_X: f32 = WIDTH + GAP;
pub const EXTENSION: f32 = UNIT * 61.0;
const HEADER_BOTTOM: f32 = UNIT * 14.0;
const REVEAL_START: f32 = 0.5;
const REVEAL_SPREAD: f32 = 0.18;
const REVEAL_DURATION: f32 = 0.24;

#[cfg(feature = "cpu")]
pub fn pill_x(screen_width: f32, status_enabled: bool) -> f32 {
    use crate::status::WIDTH as STATUS_WIDTH;
    screen_width - WIDTH - GAP - f32::from(status_enabled) * (STATUS_WIDTH + GAP)
}

pub fn expanded_x(x: f32, expansion: f32) -> f32 {
    x - FORECAST_X * expansion * 0.5
}

pub fn popup_size(expansion: f32) -> Vec2 {
    Vec2::new(WIDTH + FORECAST_X * expansion, EXTENSION * expansion)
}

pub fn forecast_center(height: f32, row: f32) -> f32 {
    HEADER_BOTTOM + height * 0.5 + row * (height + GAP)
}

pub fn forecast_row(height: f32, row: f32) -> (Vec2, Vec2) {
    let size = Vec2::new(WIDTH - GAP * 2.0, height);
    let center = Vec2::new(FORECAST_X + WIDTH * 0.5, forecast_center(height, row));
    (center - size * 0.5, size)
}

pub fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let delay = REVEAL_START + (y / EXTENSION) * REVEAL_SPREAD;
    smoothstep(delay, delay + REVEAL_DURATION, expansion)
}

/// Sun phase (0 at sunrise, 1 at sunset) and height (-1 to 1) for the given hour.
pub fn sun_position(hour: f32, [sunrise, sunset]: [f32; 2]) -> [f32; 2] {
    let height = |phase: f32| (phase * PI).sin();
    let daylight = sunset - sunrise;
    if hour >= sunrise && hour <= sunset {
        let phase = (hour - sunrise) / daylight;
        [phase, height(phase)]
    } else {
        let night = 24.0 - daylight;
        let phase = if hour < sunrise {
            (hour + 24.0 - sunset) / night
        } else {
            (hour - sunset) / night
        };
        [if hour >= sunset { 1.0 } else { 0.0 }, -height(phase)]
    }
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
    let q = p - time * (72.0 + depth * 38.0) * vec2(0.22 + layer.x * 0.05, 1.0) + layer * 91.0;
    let size = vec2(
        21.0 - depth * 4.5 + layer.y * 1.5,
        31.0 - depth * 3.0 + layer.x * 1.5,
    );
    let cell = (q / size).floor();
    let random = hash(cell + seed);
    let local = q - (cell + random * 0.75) * size;
    let curve =
        local.x - local.y * (0.22 + random.x * 0.14 + layer.y * 0.05) - (random.y - 0.5) * size.x * 0.65;
    smoothstep(0.72, 0.0, curve.abs())
        * smoothstep(0.0, 5.0 + depth * 2.0, local.y)
        * smoothstep(size.y - 4.0, size.y - 11.0, local.y)
        * smoothstep(0.45, 0.95, hash(cell + 31.7).x)
}

fn lerp_conditions(mut from: [f32; 6], to: [f32; 6], amount: f32) -> [f32; 6] {
    for index in 0..from.len() {
        from[index] += (to[index] - from[index]) * amount;
    }
    from
}

/// Sky backdrop for the status pill; also returns the refracted pixel position.
pub(crate) fn scene(
    global: &GlobalUniforms,
    refracted: Vec2,
    size: Vec2,
    dist: f32,
    sun_y: f32,
    weather: [f32; 6],
) -> Vec3 {
    let [
        fog_strength,
        cloud,
        rain_strength,
        snow_strength,
        lightning,
        hail_strength,
    ] = weather;
    let p = refracted * size;
    let (cloud_scale, time) = (global.bar_height.y, global.time);
    let sky_y = p.y / cloud_scale;
    let daylight = smoothstep(-0.04, 0.2, sun_y);
    let blue_hour = smoothstep(-0.32, -0.08, sun_y) * (1.0 - daylight);
    let twilight = smoothstep(-0.18, 0.0, sun_y) * smoothstep(0.2, 0.02, sun_y);
    let vertical = smoothstep(1.0, 0.0, sky_y);
    let mut color = vec3(0.006, 0.012, 0.035)
        .lerp(vec3(0.025, 0.04, 0.095), vertical)
        .lerp(
            vec3(0.08, 0.34, 0.62).lerp(vec3(0.32, 0.67, 0.87), vertical),
            daylight,
        )
        .lerp(
            vec3(0.10, 0.16, 0.30).lerp(vec3(0.22, 0.25, 0.45), vertical),
            blue_hour * 0.8,
        )
        .lerp(
            vec3(0.78, 0.30, 0.20).lerp(vec3(0.38, 0.22, 0.42), vertical),
            twilight * 0.9,
        );

    let stars = particles(p, Vec2::ZERO, 18.0, 0.55, 0.25) * (1.0 - daylight);
    color += Vec3::splat(stars * (1.0 - cloud) * (0.3 + vertical * 0.7));

    let mass = cloud_mass(p, cloud_scale, time);
    let billows = fbm(p / cloud_scale * 0.287 + vec2(time * 0.018, -3.7));
    let cloud_shape = smoothstep(0.35, 0.6, mass + (billows - 0.5) * 0.24);
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
    // Keep a low-frequency veil of the forecast coverage visible
    let cloud_mask = cloud * (0.12 + cloud_shape * 0.7);
    color = color.lerp(cloud_color, cloud_mask);

    color = color.lerp(vec3(0.1, 0.17, 0.25), rain_strength * 0.2);
    let rain = (rain_layer(p, time, 1.0, 0.0)
        + rain_layer(p, time, 0.72, 37.0)
        + rain_layer(p, time, 0.35, 74.0))
        * rain_strength;
    color += vec3(0.52, 0.72, 0.9) * rain * 0.7;

    let snow = particles(p, vec2(time * 6.0, time * 15.0), 18.0, 1.0, 0.72)
        + particles(p + 31.0, vec2(time * 4.0, time * 10.0), 25.0, 1.3, 0.65);
    color = color.lerp(Vec3::splat(0.96), (snow * snow_strength).clamp(0.0, 0.92));

    let hail = particles(p, vec2(time * 18.0, time * 85.0), 23.0, 0.22, 0.3) * hail_strength;
    color = color.lerp(vec3(0.75, 0.86, 0.94), hail * 0.7);

    let flash = smoothstep(0.92, 1.0, (time * 2.7).sin()) * lightning;
    color = color.lerp(vec3(0.65, 0.74, 0.96), flash * 0.55);

    let fog = fbm(vec2(p.x / size.x * 0.9 + time * 0.008, sky_y * 0.32 + 12.0));
    color.lerp(
        vec3(0.63, 0.69, 0.73),
        fog_strength * (0.58 + smoothstep(0.35, 0.7, fog) * 0.18),
    ) + pill_sheen(sky_y, dist)
}

fn sun_layer(
    color: Vec3,
    point: Vec2,
    size: Vec2,
    [sun_x, sun_y]: [f32; 2],
    cloud: f32,
    time: f32,
) -> Vec3 {
    let sun = vec2(
        16.0 + sun_x * (size.x - 32.0),
        size.y * (0.72 - sun_y.saturate() * 0.45),
    );
    let sun_color = vec3(0.96, 0.98, 1.0).lerp(vec3(0.98, 0.74, 0.66), smoothstep(0.55, 0.02, sun_y));
    let clear = smoothstep(-0.02, 0.04, sun_y)
        * (1.0 - smoothstep(0.43, 0.69, cloud_mass(sun, size.y, time)) * cloud * 0.82);
    let distance = point.distance(sun);
    color.lerp(
        sun_color,
        (smoothstep(62.0, 4.0, distance) * 0.24 + smoothstep(11.0, 1.0, distance) * 0.7) * clear,
    )
}

fn forecast_position(x: f32, count: usize) -> f32 {
    (x / WIDTH * count as f32 - 0.5).clamp(0.0, (count - 1) as f32)
}

fn forecast_at<const N: usize>(x: f32, forecasts: &[WeatherCondition; N]) -> [f32; 6] {
    let position = forecast_position(x, N);
    let index = position.floor() as usize;
    lerp_conditions(
        forecasts[index].values(),
        forecasts[(index + 1).min(N - 1)].values(),
        smoothstep(0.0, 1.0, position.fract()),
    )
}

#[spirv(vertex)]
pub fn vertex(
    #[spirv(vertex_index)] vertex: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] weather: &[Data],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel: &mut Vec2,
    #[spirv(location = 1, flat)] out_weather: &mut Vec4,
) {
    let pill = weather[0];
    let x = pill.x;
    let expansion = smoothstep(0.0, 1.0, pill.calendar_expansion);
    let sun = sun_position(global.weather_hour, pill.sun_hours);
    *out_weather = vec3(sun[0], sun[1], sun_position(12.0, pill.sun_hours)[1]).extend(expansion);
    (*out_pos, *out_pixel) =
        pill_vertex(vertex, global, expanded_x(x, expansion), popup_size(expansion));
}

#[spirv(fragment)]
pub fn fragment(
    #[spirv(location = 0)] pixel: Vec2,
    #[spirv(location = 1, flat)] weather_data: Vec4,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] weather: &[Data],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = weather[0];
    let x = pill.x;
    let (interaction, main_local, pill_size, body_dist) = pill_fragment(pixel, global, x, WIDTH);
    let expansion = weather_data.w;
    let body_bottom = global.bar_height.x + global.bar_height.y;
    let popup_size = popup_size(expansion);
    let popup_local = pixel - vec2(expanded_x(x, expansion), body_bottom);
    let content_local = pixel - vec2(expanded_x(x, 1.0), body_bottom);
    let top_gap = GAP * expansion;
    let box_size = vec2(popup_size.x, (popup_size.y - top_gap).max(0.0));
    let popup_dist = sd_rounded_box(
        popup_local - vec2(popup_size.x * 0.5, top_gap + box_size.y * 0.5),
        box_size * 0.5,
        (box_size.y * 0.5).min(18.0),
    );
    let main_dist = smooth_union(body_dist, popup_dist, 32.0, expansion);
    let center_y = |row| forecast_center(global.bar_height.y, row);
    let hourly_row = content_local.y <= (center_y(0.0) + center_y(1.0)) * 0.5;
    let row = if hourly_row { 0.0 } else { 1.0 };
    let row_reveal = reveal_progress(expansion, center_y(row));
    let (row_origin, row_size) = forecast_row(global.bar_height.y, row);
    let row_local = content_local - row_origin;
    let row_dist = sd_capsule_box(
        row_local - row_size * 0.5,
        (row_size.x - row_size.y) * 0.5,
        row_size.y * 0.5,
    );
    let mouse_row_dist = sd_capsule_box(
        (global.mouse_pos - vec2(expanded_x(x, 1.0), body_bottom)) - row_origin - row_size * 0.5,
        (row_size.x - row_size.y) * 0.5,
        row_size.y * 0.5,
    );
    let row_hover = smoothstep(0.5, -0.5, mouse_row_dist) * global.mouse_pressure;
    let row_surface_dist = row_dist - interaction.mouse_bulge * row_hover * 0.5;
    let surface_dist = if expansion > 0.0 && row_reveal > 0.0 {
        interaction.expand(main_dist).min(row_surface_dist)
    } else {
        interaction.expand(main_dist)
    };
    let coverage = |distance: f32| {
        let width = distance.fwidth().max(0.55);
        smoothstep(width, -width, distance)
    };
    let mask = coverage(surface_dist);
    let alpha = mask.max((-surface_dist.max(0.0) * 0.3).exp() * 0.16);
    if alpha <= 0.0 {
        kill();
    }
    let forecast_x = content_local.x - FORECAST_X;
    let conditions = if hourly_row {
        forecast_at(forecast_x, &pill.hourly)
    } else {
        forecast_at(forecast_x, &pill.daily)
    };
    let sun = [weather_data.x, weather_data.y];
    let current_conditions = pill.hourly[0].values();
    let edge = ((main_local.x / pill_size.x).clamp(0.0, 1.0) - 0.5).abs();
    let pill_conditions = lerp_conditions(
        current_conditions,
        pill.hourly[1].values(),
        smoothstep(0.2, 0.3, edge),
    );
    let row_blend = coverage(row_surface_dist) * row_reveal;
    let merged_size = vec2(pill_size.x, pill_size.y + popup_size.y);
    let popup_conditions = lerp_conditions(
        pill_conditions,
        current_conditions,
        smoothstep(8.0, -8.0, popup_dist),
    );
    let row_conditions = lerp_conditions(current_conditions, conditions, row_reveal);
    let scene_local = main_local.lerp(row_local, row_blend);
    let scene_size = merged_size.lerp(row_size, row_blend);
    let scene_conditions = lerp_conditions(popup_conditions, row_conditions, row_blend);
    let refracted = interaction.refract(scene_local, scene_size, main_dist);
    let row_sun_height = if hourly_row {
        sun_position(
            (pill.hourly_start
                + forecast_position(forecast_x, HOURLY_FORECASTS) * HOURLY_STEP_HOURS as f32)
                % 24.0,
            pill.sun_hours,
        )[1]
    } else {
        weather_data.z
    };
    let sun_height = sun[1].lerp(row_sun_height, row_blend);
    let mut color = scene(
        global,
        refracted,
        scene_size,
        main_dist,
        sun_height,
        scene_conditions,
    );
    if body_dist < 1.0 {
        color = color.lerp(
            sun_layer(color, main_local, pill_size, sun, pill_conditions[1], global.time),
            smoothstep(1.0, -1.0, body_dist),
        );
    }
    color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
    let forecast_alpha = 1.0 - row_blend * (1.0 - row_reveal);
    *out_color = (color * mask * forecast_alpha).extend(alpha * forecast_alpha);
}

use crate::render::{
    shader::{
        SdfSurface, cloud_mass, fbm, hash, pill_fragment, pill_interaction, pill_sheen, pill_vertex,
        sd_capsule_box, sd_rounded_box, sdf_coverage,
    },
    shared::{FrameData, GAP, PANEL_START, UNIT, smoothstep},
    text,
};
use core::f32::consts::PI;
use isthmus::{
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    spirv_std::arch::kill,
};

#[cfg(target_arch = "spirv")]
use isthmus::spirv_std::num_traits::Float;

use isthmus::Vertex;
#[cfg(feature = "cpu")]
use {
    crate::{
        app::{
            AppUpdater,
            interaction::{InteractionState, Rect},
            openmeteo::{self, Forecast},
        },
        render::{
            cpu::{Frame, Passes, approach},
            status,
            text::TextStyle,
        },
    },
    arrayvec::ArrayString,
    isthmus::StatePass,
    jiff::{
        Span, Zoned,
        civil::{DateTime, Time},
        tz::Offset,
    },
    std::{array::from_fn, fmt::Write},
};

/// Number of conditions shown in the hourly forecast row.
const HOURLY_FORECASTS: usize = 6;
/// Hours between adjacent conditions in the hourly forecast row.
const HOURLY_STEP_HOURS: usize = 4;
const DAILY_FORECASTS: usize = 5;

#[isthmus::data]
#[derive(Default)]
pub struct WeatherSurface {
    pub x: f32,
    pub calendar_expansion: f32,
    pub sun_hours: [f32; 2],
    pub hourly_start: f32,
    pub today_index: i32,
    pub month_range: [u32; 2],
    pub text_hover: [f32; 3],
    pub hourly_conditions: [WeatherCondition; HOURLY_FORECASTS],
    pub daily_conditions: [WeatherCondition; DAILY_FORECASTS],
    pub text: text::Text<{ DAY_TEXT + GRID_CELLS }, 512>,
}

const WEEKDAY_COUNT: usize = 7;
const GRID_ROWS: usize = 6;
const GRID_CELLS: usize = WEEKDAY_COUNT * GRID_ROWS;
const GRID_ROW_HEIGHT: f32 = UNIT * 6.0;
const GRID_TOP_Y: f32 = UNIT * 24.0;
const WEEKDAY_Y: f32 = UNIT * 17.0;
const TITLE: Vec2 = Vec2::new(WIDTH * 0.5, UNIT * 10.0);
const DETAILS_POS: Vec2 = Vec2::new(FORECAST_X + WIDTH * 0.5, TITLE.y);
/// Month title's zone; fits the title at hover scale and clears the arrows.
const TITLE_HALF_WIDTH: f32 = UNIT * 26.0;

const WEATHER_TEXT: usize = 0;
const TITLE_TEXT: usize = WEATHER_TEXT + 1;
const ARROW_TEXT: usize = TITLE_TEXT + 1;
const DETAILS_TEXT: usize = ARROW_TEXT + 2;
const HOURLY_TEXT: usize = DETAILS_TEXT + 1;
const DAILY_TEXT: usize = HOURLY_TEXT + HOURLY_FORECASTS * 2;
const WEEKDAY_TEXT: usize = DAILY_TEXT + DAILY_FORECASTS * 2;
const DAY_TEXT: usize = WEEKDAY_TEXT + WEEKDAY_COUNT;
const TEXT_COLOR: Vec3 = Vec3::splat(0.94);
const TODAY_COLOR: Vec3 = Vec3::new(1.0, 0.68, 0.68);
#[cfg(feature = "cpu")]
const PRIMARY_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
#[cfg(feature = "cpu")]
const TODAY_STYLE: TextStyle = TextStyle::new(16.0, 900.0);
#[cfg(feature = "cpu")]
const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);
#[cfg(feature = "cpu")]
const WEATHER_STYLE: TextStyle = TextStyle::new(24.0, 600.0);
#[cfg(feature = "cpu")]
const TITLE_STYLE: TextStyle = TextStyle::new(20.0, 750.0);

fn grid_cell(index: usize) -> Vec2 {
    let column_width = WIDTH / WEEKDAY_COUNT as f32;
    vec2(
        (index % WEEKDAY_COUNT) as f32 * column_width + column_width * 0.5,
        GRID_TOP_Y + (index / WEEKDAY_COUNT) as f32 * GRID_ROW_HEIGHT,
    )
}

fn grid_column(x: f32) -> usize {
    let column_width = WIDTH / WEEKDAY_COUNT as f32;
    (x / column_width).floor().clamp(0.0, WEEKDAY_COUNT as f32 - 1.0) as usize
}

fn grid_index(local: Vec2) -> usize {
    let row = ((local.y - GRID_TOP_Y) / GRID_ROW_HEIGHT + 0.5)
        .floor()
        .clamp(0.0, GRID_ROWS as f32 - 1.0) as usize;
    row * WEEKDAY_COUNT + grid_column(local.x)
}

fn forecast_index(local: Vec2, row_height: f32) -> (bool, usize) {
    let daily = local.y > forecast_split(row_height);
    let count = if daily { DAILY_FORECASTS } else { HOURLY_FORECASTS };
    let column = forecast_position(local.x - FORECAST_X, count)
        .round()
        .clamp(0.0, (count - 1) as f32) as usize;
    // Not `f32::from(bool)`: it lowers through a `u8` cast, which needs `OpCapability Int8`.
    let row = if daily { 1.0 } else { 0.0 };
    (
        daily,
        column * 2 + usize::from(local.y >= forecast_center(row_height, row)),
    )
}

#[isthmus::data]
#[derive(Default)]
pub struct WeatherCondition {
    pub fog: f32,
    pub cloud: f32,
    pub rain: f32,
    pub snow: f32,
    pub lightning: f32,
    pub hail: f32,
}

impl WeatherCondition {
    fn lerp(self, to: Self, amount: f32) -> Self {
        let mix = |from, to| from + (to - from) * amount;
        Self {
            fog: mix(self.fog, to.fog),
            cloud: mix(self.cloud, to.cloud),
            rain: mix(self.rain, to.rain),
            snow: mix(self.snow, to.snow),
            lightning: mix(self.lightning, to.lightning),
            hail: mix(self.hail, to.hail),
        }
    }
}

pub const WIDTH: f32 = UNIT * 77.0;
pub const FORECAST_X: f32 = WIDTH + GAP;
pub const EXTENSION: f32 = UNIT * 61.0;
const HEADER_BOTTOM: f32 = UNIT * 14.0;
const REVEAL_START: f32 = 0.5;
const REVEAL_SPREAD: f32 = 0.18;
const REVEAL_DURATION: f32 = 0.24;

fn expanded_x(x: f32, expansion: f32) -> f32 {
    x - FORECAST_X * expansion * 0.5
}

fn popup_size(expansion: f32) -> Vec2 {
    Vec2::new(WIDTH + FORECAST_X * expansion, EXTENSION * expansion)
}

fn forecast_center(height: f32, row: f32) -> f32 {
    HEADER_BOTTOM + height * 0.5 + row * (height + GAP)
}

/// The y that divides the hourly forecast row from the daily one.
fn forecast_split(height: f32) -> f32 {
    (forecast_center(height, 0.0) + forecast_center(height, 1.0)) * 0.5
}

fn forecast_row(height: f32, row: f32) -> (Vec2, Vec2) {
    let size = Vec2::new(WIDTH - GAP * 2.0, height);
    let center = Vec2::new(FORECAST_X + WIDTH * 0.5, forecast_center(height, row));
    (center - size * 0.5, size)
}

fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let delay = REVEAL_START + (y / EXTENSION) * REVEAL_SPREAD;
    smoothstep(delay, delay + REVEAL_DURATION, expansion)
}

/// Sun phase (0 at sunrise, 1 at sunset) and height (-1 to 1) for the given hour.
fn sun_position(hour: f32, [sunrise, sunset]: [f32; 2]) -> [f32; 2] {
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

/// One layer; `kind` is a literal at every call site, so the tables below fold away.
fn precipitation(p: Vec2, time: f32, kind: i32, strength: f32) -> Vec4 {
    let rain = kind == 0;
    let snow = kind == 1;
    let (velocity, cell_size, radius, density, trail) = if rain {
        (vec2(20.0, 110.0), vec2(15.0, 25.0), 0.65, 0.78, 9.0)
    } else if snow {
        (vec2(5.0, 14.0), Vec2::splat(20.0), 1.15, 0.7, 0.4)
    } else {
        (vec2(18.0, 85.0), Vec2::splat(23.0), 0.35, 0.3, 1.2)
    };
    let q = p - velocity * time;
    let cell = (q / cell_size).floor();
    let random = hash(cell + kind as f32 * 31.7);
    let center = (cell + 0.15 + random * 0.7) * cell_size;
    let direction = vec2(0.2, 1.0);
    let segment = direction * trail;
    let offset = q - center;
    let along = (offset.dot(segment) / segment.length_squared()).clamp(0.0, 1.0);
    let distance = (offset - segment * along).length();
    let particle = smoothstep(radius + 0.45, radius - 0.15, distance)
        * smoothstep(1.0 - density, 1.0, hash(cell + 19.3).x);
    let color = if rain {
        vec3(0.52, 0.72, 0.9)
    } else if snow {
        Vec3::splat(0.96)
    } else {
        vec3(0.75, 0.86, 0.94)
    };
    color.extend((particle * strength * if snow { 0.92 } else { 0.7 }).saturate())
}

/// How much each sky palette contributes at a given sun height.
#[derive(Clone, Copy)]
pub struct SkyPhase {
    daylight: f32,
    blue_hour: f32,
    twilight: f32,
}

impl SkyPhase {
    fn new(sun_y: f32) -> Self {
        let daylight = smoothstep(-0.04, 0.2, sun_y);
        Self {
            daylight,
            blue_hour: smoothstep(-0.32, -0.08, sun_y) * (1.0 - daylight),
            twilight: smoothstep(-0.18, 0.0, sun_y) * smoothstep(0.2, 0.02, sun_y),
        }
    }

    /// Blends the palettes, so a day/night crossfade never passes through sunset.
    fn lerp(self, to: Self, amount: f32) -> Self {
        let mix = |from: f32, to: f32| from + (to - from) * amount;
        Self {
            daylight: mix(self.daylight, to.daylight),
            blue_hour: mix(self.blue_hour, to.blue_hour),
            twilight: mix(self.twilight, to.twilight),
        }
    }
}

/// Sky backdrop for the status pill, sampled at pixel position `p`.
pub fn scene(
    frame: &FrameData,
    p: Vec2,
    width: f32,
    dist: f32,
    phase: SkyPhase,
    weather: WeatherCondition,
) -> Vec3 {
    let WeatherCondition {
        fog: fog_strength,
        cloud,
        rain: rain_strength,
        lightning,
        ..
    } = weather;
    let (cloud_scale, time) = (frame.panel_height, frame.time);
    let sky_y = p.y / cloud_scale;
    let vertical = smoothstep(1.0, 0.0, sky_y);
    let mut color = vec3(0.006, 0.012, 0.035)
        .lerp(vec3(0.025, 0.04, 0.095), vertical)
        .lerp(
            vec3(0.08, 0.34, 0.62).lerp(vec3(0.32, 0.67, 0.87), vertical),
            phase.daylight,
        )
        .lerp(
            vec3(0.10, 0.16, 0.30).lerp(vec3(0.22, 0.25, 0.45), vertical),
            phase.blue_hour * 0.8,
        )
        .lerp(
            vec3(0.78, 0.30, 0.20).lerp(vec3(0.38, 0.22, 0.42), vertical),
            phase.twilight * 0.9,
        );

    let star_cell = (p / 18.0).floor();
    let star_center = (star_cell + 0.2 + hash(star_cell) * 0.6) * 18.0;
    let stars = smoothstep(1.0, 0.4, p.distance(star_center))
        * smoothstep(0.75, 1.0, hash(star_cell + 31.7).x)
        * (1.0 - phase.daylight);
    color += Vec3::splat(stars * (1.0 - cloud) * (0.3 + vertical * 0.7));

    if cloud > 1.0 / 1024.0 {
        let mass = cloud_mass(p, cloud_scale, time);
        let billows = fbm(p / cloud_scale * 0.287 + vec2(time * 0.018, -3.7));
        let cloud_shape = smoothstep(0.35, 0.6, mass + (billows - 0.5) * 0.24);
        let cloud_light = smoothstep(0.42, 0.72, billows) * 0.55 + smoothstep(0.48, 0.7, mass) * 0.45;
        let cloud_color = vec3(0.16, 0.2, 0.28)
            .lerp(vec3(0.32, 0.36, 0.43), cloud_light)
            .lerp(
                vec3(0.62, 0.7, 0.78).lerp(vec3(0.92, 0.94, 0.96), cloud_light),
                phase.daylight,
            )
            .lerp(
                vec3(0.5, 0.36, 0.4).lerp(vec3(0.76, 0.59, 0.56), cloud_light),
                phase.twilight * 0.45,
            );
        color = color.lerp(cloud_color, cloud * (0.12 + cloud_shape * 0.7));
    }

    color = color.lerp(vec3(0.1, 0.17, 0.25), rain_strength * 0.2);
    if weather.rain > 1.0 / 1024.0 {
        let particle = precipitation(p, time, 0, weather.rain);
        color = color.lerp(particle.truncate(), particle.w);
    }
    if weather.snow > 1.0 / 1024.0 {
        let particle = precipitation(p, time, 1, weather.snow);
        color = color.lerp(particle.truncate(), particle.w);
    }
    if weather.hail > 1.0 / 1024.0 {
        let particle = precipitation(p, time, 2, weather.hail);
        color = color.lerp(particle.truncate(), particle.w);
    }

    let flash = smoothstep(0.92, 1.0, (time * 2.7).sin()) * lightning;
    color = color.lerp(vec3(0.65, 0.74, 0.96), flash * 0.55);

    if fog_strength > 1.0 / 1024.0 {
        let fog = fbm(vec2(p.x / width * 0.9 + time * 0.008, sky_y * 0.32 + 12.0));
        color = color.lerp(
            vec3(0.63, 0.69, 0.73),
            fog_strength * (0.58 + smoothstep(0.35, 0.7, fog) * 0.18),
        );
    }
    color + pill_sheen(sky_y, dist)
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
    let obstruction = if cloud > 1.0 / 1024.0 {
        smoothstep(0.43, 0.69, cloud_mass(sun, size.y, time)) * cloud * 0.82
    } else {
        0.0
    };
    let clear = smoothstep(-0.02, 0.04, sun_y) * (1.0 - obstruction);
    let distance = point.distance(sun);
    color.lerp(
        sun_color,
        (smoothstep(62.0, 4.0, distance) * 0.24 + smoothstep(11.0, 1.0, distance) * 0.7) * clear,
    )
}

fn forecast_position(x: f32, count: usize) -> f32 {
    (x / WIDTH * count as f32 - 0.5).clamp(0.0, (count - 1) as f32)
}

struct ForecastSample {
    local: Vec2,
    size: Vec2,
    surface: SdfSurface,
    reveal: f32,
    position: f32,
    daily: bool,
}

impl ForecastSample {
    fn weather(&self, pill: &WeatherSurface) -> (WeatherCondition, f32) {
        let index = self.position.floor() as usize;
        if self.daily {
            (
                pill.daily_conditions[index].lerp(
                    pill.daily_conditions[(index + 1).min(DAILY_FORECASTS - 1)],
                    smoothstep(0.0, 1.0, self.position.fract()),
                ),
                sun_position(12.0, pill.sun_hours)[1],
            )
        } else {
            (
                pill.hourly_conditions[index].lerp(
                    pill.hourly_conditions[(index + 1).min(HOURLY_FORECASTS - 1)],
                    smoothstep(0.0, 1.0, self.position.fract()),
                ),
                sun_position(
                    (pill.hourly_start + self.position * HOURLY_STEP_HOURS as f32) % 24.0,
                    pill.sun_hours,
                )[1],
            )
        }
    }
}

fn sample_forecast(pixel: Vec2, frame: &FrameData, pill: &WeatherSurface) -> ForecastSample {
    let body_bottom = PANEL_START + frame.panel_height;
    let content_origin = vec2(expanded_x(pill.x, 1.0), body_bottom);
    let content = pixel - content_origin;
    let daily = content.y > forecast_split(frame.panel_height);
    let row = if daily { 1.0 } else { 0.0 };
    let reveal = reveal_progress(pill.calendar_expansion, forecast_center(frame.panel_height, row));
    let (full_origin, full_size) = forecast_row(frame.panel_height, row);
    let size = full_size * reveal;
    let origin = full_origin + (full_size - size) * 0.5;
    let local = content - origin;
    let capsule = |point: Vec2| {
        if reveal <= 0.001 {
            f32::MAX
        } else {
            sd_capsule_box(point - size * 0.5, (size.x - size.y) * 0.5, size.y * 0.5)
        }
    };
    let forecast_x = local.x / size.x.max(0.001) * WIDTH;
    let position = if daily {
        forecast_position(forecast_x, DAILY_FORECASTS)
    } else {
        forecast_position(forecast_x, HOURLY_FORECASTS)
    };
    ForecastSample {
        local,
        size,
        surface: SdfSurface::new(capsule(local), capsule(frame.mouse_pos - content_origin - origin)),
        reveal,
        position,
        daily,
    }
}

#[cfg(feature = "cpu")]
#[derive(Default)]
struct ForecastItem {
    text: [String; 2],
    hover_text: String,
}

#[isthmus::pass]
pub struct TempestasPass {
    pub pill: StatePass<Self>,
    temperature: String,
    utc_offset: Option<Offset>,
    details: String,
    hourly: [ForecastItem; HOURLY_FORECASTS],
    daily: [ForecastItem; DAILY_FORECASTS],
    month_offset: i32,
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub pixel: Vec2,
    #[gpu(flat)]
    pub weather: Vec4,
}

#[isthmus::pass]
impl TempestasPass {
    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] pill: WeatherSurface,
    ) -> Vertex<Varyings> {
        let expansion = smoothstep(0.0, 1.0, pill.calendar_expansion);
        let sun = sun_position(frame.weather_hour, pill.sun_hours);
        let weather = vec3(sun[0], sun[1], sun_position(12.0, pill.sun_hours)[1]).extend(expansion);
        // `pill_vertex` reserves space around the animated SDF for bulge and AA.
        let (position, pixel) = pill_vertex(
            vertex,
            frame,
            expanded_x(pill.x, expansion),
            popup_size(expansion),
        );
        Vertex {
            position,
            varyings: Varyings { pixel, weather },
        }
    }

    #[gpu]
    pub fn fragment(
        Varyings { pixel, weather }: Varyings,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] pill: WeatherSurface,
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
    ) -> Vec4 {
        let (_, body_local, body_size, body_surface) = pill_fragment(pixel, frame, pill.x, WIDTH);
        let expansion = weather.w;
        let body_bottom = PANEL_START + frame.panel_height;
        let popup_size = popup_size(expansion);
        let popup_origin = vec2(expanded_x(pill.x, expansion), body_bottom);
        let popup_local = pixel - popup_origin;
        let content_local = pixel - vec2(expanded_x(pill.x, 1.0), body_bottom);
        let top_gap = GAP * expansion;
        let box_size = vec2(popup_size.x, (popup_size.y - top_gap).max(0.0));
        let popup = |point: Vec2| {
            sd_rounded_box(
                point - vec2(popup_size.x * 0.5, top_gap + box_size.y * 0.5),
                box_size * 0.5,
                (box_size.y * 0.5).min(18.0),
            )
        };
        let main_surface = body_surface.smooth_union(
            SdfSurface::new(popup(popup_local), popup(frame.mouse_pos - popup_origin)),
            56.0,
            expansion,
        );
        let interaction = pill_interaction(pixel, frame);
        let main_surface_distance = interaction.expand(main_surface);

        let row = sample_forecast(pixel, frame, pill);
        let row_surface_distance = interaction.expand(row.surface);
        let surface_distance = main_surface_distance.min(row_surface_distance);
        let mask = sdf_coverage(surface_distance);
        let alpha = mask.max((-surface_distance.max(0.0) * 0.3).exp() * 0.16);
        if alpha <= 1.0 / 1024.0 {
            kill();
        }

        let current = pill.hourly_conditions[0];
        let edge = ((body_local.x / body_size.x).clamp(0.0, 1.0) - 0.5).abs();
        let body_conditions = current.lerp(pill.hourly_conditions[1], smoothstep(0.2, 0.25, edge));
        let main_conditions = body_conditions.lerp(current, expansion);
        let main_refracted = interaction.refract(body_local, body_size, main_surface_distance);
        let row_refracted = if row.reveal > 0.001 {
            interaction.refract(row.local, row.size, row_surface_distance)
        } else {
            main_refracted
        };
        let row_blend = sdf_coverage(row_surface_distance) * row.reveal;
        let (row_conditions, row_sun_height) = row.weather(pill);
        let sample = (main_refracted * body_size).lerp(row_refracted * row.size, row_blend);
        let scene_width = body_size.x + (row.size.x - body_size.x) * row_blend;
        let scene_distance = main_surface_distance
            + (row_surface_distance.min(1000.0) - main_surface_distance) * row_blend;
        let conditions = main_conditions.lerp(row_conditions, row_blend);
        let phase = SkyPhase::new(weather.y).lerp(SkyPhase::new(row_sun_height), row_blend);
        let mut color = scene(frame, sample, scene_width, scene_distance, phase, conditions);
        if body_surface.distance < 1.0 {
            color = color.lerp(
                sun_layer(
                    color,
                    body_local,
                    body_size,
                    [weather.x, weather.y],
                    body_conditions.cloud,
                    frame.time,
                ),
                smoothstep(1.0, -body_size.y * 0.25, body_surface.distance),
            );
        }
        let mut line = WEATHER_TEXT;
        let mut line_local = body_local;
        let mut line_color = TEXT_COLOR;
        let line_visibility = if pill.calendar_expansion > 0.0 && content_local.y >= 0.0 {
            line = DETAILS_TEXT;
            line_local = content_local;
            let mut line_y = DETAILS_POS.y;
            let mut line_opacity = 1.0;
            if content_local.x < WIDTH {
                if content_local.y < (TITLE.y + WEEKDAY_Y) * 0.5 {
                    let side = content_local.x - WIDTH * 0.5;
                    line = if side < -TITLE_HALF_WIDTH {
                        ARROW_TEXT
                    } else if side > TITLE_HALF_WIDTH {
                        ARROW_TEXT + 1
                    } else {
                        TITLE_TEXT
                    };
                    line_y = TITLE.y;
                } else if content_local.y < (WEEKDAY_Y + GRID_TOP_Y) * 0.5 {
                    line = WEEKDAY_TEXT + grid_column(content_local.x);
                    line_y = WEEKDAY_Y;
                    line_opacity = 0.75;
                } else {
                    let day = grid_index(content_local);
                    line = DAY_TEXT + day;
                    line_y = grid_cell(day).y;
                    if day as i32 == pill.today_index {
                        line_color = TODAY_COLOR;
                    }
                    if day < pill.month_range[0] as usize || day >= pill.month_range[1] as usize {
                        line_opacity = 0.32;
                    }
                }
            } else if content_local.x >= FORECAST_X && content_local.y >= HEADER_BOTTOM {
                let (daily, index) = forecast_index(content_local, frame.panel_height);
                line = if daily { DAILY_TEXT } else { HOURLY_TEXT } + index;
                line_y = forecast_center(frame.panel_height, if daily { 1.0 } else { 0.0 });
            }
            reveal_progress(pill.calendar_expansion, line_y) * line_opacity
        } else {
            1.0
        };
        let distance = text::line_distance(&pill.text, line, glyphs, edges, line_local);
        color = color.lerp(line_color, text::alpha(distance) * line_visibility);
        color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
        (color * mask).extend(alpha)
    }

    const WEEKDAYS: [&str; WEEKDAY_COUNT] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const ORDINALS: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];

    fn pill_x(screen_width: f32, status_width: f32) -> f32 {
        screen_width - WIDTH - GAP - status_width
    }

    const fn pill_rect(x: f32, height: f32) -> Rect {
        Rect::pill(x, WIDTH, height)
    }

    fn visible_rects(x: f32, height: f32, expansion: f32) -> [Rect; 2] {
        let pill = Self::pill_rect(x, height);
        let expansion = smoothstep(0.0, 1.0, expansion);
        let size = popup_size(expansion);
        let x = expanded_x(pill.x0, expansion);
        [pill, Rect::new(x, pill.y1, x + size.x, pill.y1 + size.y)]
    }

    pub fn new(passes: &Passes<'_>, location: [f32; 2], updater: AppUpdater, font: &text::Font) -> Self {
        let [latitude, longitude] = location;
        openmeteo::spawn_refresh_loop(latitude, longitude, updater);
        let pill = passes.state_with(
            Resources {
                glyphs: &font.glyphs,
                edges: &font.edges,
            },
            WeatherSurface {
                sun_hours: [6.0, 18.0],
                today_index: -1,
                ..Default::default()
            },
        );
        Self {
            pill,
            temperature: String::new(),
            utc_offset: None,
            details: "Weather unavailable".into(),
            hourly: Default::default(),
            daily: Default::default(),
            month_offset: 0,
        }
    }

    fn collapsed_label(&mut self, x: f32, height: f32, frame: &mut Frame) -> (ArrayString<64>, f32) {
        let hovered = Self::visible_rects(x, height, self.pill.calendar_expansion)
            .into_iter()
            .any(|rect| frame.interaction.contains(rect));
        approach(
            &mut self.pill.calendar_expansion,
            f32::from(hovered),
            frame.delta_time.min(1.0 / 30.0) * 3.0,
        );
        let time = Zoned::now();
        let hour = self.utc_offset.map_or_else(
            || Self::hour_of_day(time.datetime()),
            |offset| Self::hour_of_day(offset.to_datetime(time.timestamp())),
        );
        self.pill.x = x;
        let clock = time.strftime("%a %d %b  %H:%M:%S");
        let mut label = ArrayString::new();
        if self.temperature.is_empty() {
            write!(label, "{clock}").unwrap();
        } else {
            write!(label, "{}   {clock}", self.temperature).unwrap();
        }
        (label, hour)
    }

    pub fn update(
        &mut self,
        font: &text::Font,
        status: Option<&mut status::StatusPass>,
        frame: &mut Frame,
    ) {
        let height = frame.config.height;
        let x = Self::pill_x(frame.shared.screen_size.x, frame.shared.status_width);
        let (weather_label, hour) = self.collapsed_label(x, height, frame);
        frame.shared.weather_hour = hour;
        if let Some(status) = status {
            status.pill.sun_height = sun_position(hour, self.pill.sun_hours)[1];
            status.pill.conditions = self.pill.hourly_conditions[0];
        }
        self.pill.text.clear();
        self.pill.text.centered(
            font,
            &weather_label,
            WEATHER_STYLE,
            vec2(WIDTH * 0.5, height * 0.46),
        );
        self.calendar_labels(font, x, height, frame.delta_time, frame.interaction);
    }

    fn calendar_labels(
        &mut self,
        font: &text::Font,
        x: f32,
        height: f32,
        delta_time: f32,
        interaction: &mut InteractionState,
    ) {
        let bounds = Self::pill_rect(x, height);
        if self.pill.calendar_expansion <= 0.0 {
            self.pill.text_hover = [0.0; 3];
            interaction.input_region(bounds);
            return;
        }
        let origin = Vec2::new(expanded_x(bounds.x0, 1.0), bounds.y1);
        let reveal = reveal_progress(self.pill.calendar_expansion, TITLE.y);
        let title = interaction.surface(Rect::from_center(
            origin + TITLE,
            Vec2::new(TITLE_HALF_WIDTH, UNIT * 4.0),
        ));
        if title.clicked {
            self.month_offset = 0;
        }
        let hover_blend = 1.0 - (-delta_time * 14.0).exp();
        self.pill.text_hover[0] += (f32::from(title.hovered) - self.pill.text_hover[0]) * hover_blend;

        let arrows = [-1.0f32, 1.0].map(|side| {
            let position = Vec2::new(
                WIDTH * 0.5 + side * (WIDTH * 0.5 - UNIT * 7.0) * reveal,
                TITLE.y - (1.0 - reveal) * UNIT * 3.0,
            );
            let response =
                interaction.surface(Rect::from_center(origin + position, Vec2::splat(UNIT * 5.0)));
            if response.clicked {
                self.month_offset = (self.month_offset + side as i32).clamp(-1200, 1200);
            }
            let index = usize::from(side > 0.0) + 1;
            self.pill.text_hover[index] +=
                (f32::from(response.hovered) - self.pill.text_hover[index]) * hover_blend;
            position
        });

        let today = Zoned::now().date();
        let month = today
            .first_of_month()
            .saturating_add(Span::new().months(self.month_offset));
        let title_hover = self.pill.text_hover[0];
        self.pill.text.centered(
            font,
            &month.strftime("%B %Y").to_string(),
            TITLE_STYLE
                .scaled(1.0 + title_hover * 0.2)
                .with_weight_mix(0.5 + title_hover * 0.5),
            TITLE,
        );
        for (index, (content, position)) in ["<", ">"].into_iter().zip(arrows).enumerate() {
            let hover = self.pill.text_hover[index + 1];
            self.pill.text.centered(
                font,
                content,
                TITLE_STYLE
                    .scaled(1.0 + hover * 0.35)
                    .with_weight_mix(0.5 + hover * 0.5),
                position,
            );
        }

        let forecasts = [&self.hourly[..], &self.daily[..]];
        let hovered_forecast = if self.pill.calendar_expansion > 0.01 {
            forecasts.into_iter().enumerate().find_map(|(row, forecasts)| {
                let (row_origin, size) = forecast_row(height, row as f32);
                let column_width = size.x / forecasts.len() as f32;
                (0..forecasts.len()).find_map(|column| {
                    let x = origin.x + row_origin.x + column as f32 * column_width;
                    let rect = Rect::new(
                        x,
                        bounds.y1 + row_origin.y,
                        x + column_width,
                        bounds.y1 + row_origin.y + size.y,
                    );
                    interaction
                        .contains(rect)
                        .then(|| forecasts[column].hover_text.as_str())
                })
            })
        } else {
            None
        };
        self.pill.text.centered(
            font,
            hovered_forecast.unwrap_or(&self.details),
            DETAILS_STYLE,
            DETAILS_POS,
        );

        for (row, forecasts) in forecasts.into_iter().enumerate() {
            for (column, forecast) in forecasts.iter().enumerate() {
                for (line, content) in forecast.text.iter().enumerate() {
                    let position = Vec2::new(
                        FORECAST_X + (column as f32 + 0.5) * WIDTH / forecasts.len() as f32,
                        forecast_center(height, row as f32) + (line as f32 * 2.0 - 1.0) * GAP,
                    );
                    self.pill.text.centered(font, content, DETAILS_STYLE, position);
                }
            }
        }

        for (column, weekday) in Self::WEEKDAYS.iter().enumerate() {
            self.pill.text.centered(
                font,
                weekday,
                DETAILS_STYLE,
                Vec2::new(grid_cell(column).x, WEEKDAY_Y),
            );
        }

        let grid_start = month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
        self.pill.today_index = -1;
        self.pill.month_range = [GRID_CELLS as u32, 0];
        for index in 0..GRID_CELLS {
            let date = grid_start.saturating_add(Span::new().days(index as i64));
            let mut text = ArrayString::<2>::new();
            write!(text, "{}", date.day()).unwrap();
            if date.month() == month.month() {
                self.pill.month_range[0] = self.pill.month_range[0].min(index as u32);
                self.pill.month_range[1] = index as u32 + 1;
            }
            if date == today {
                self.pill.today_index = index as i32;
            }
            let today = date == today;
            self.pill.text.centered(
                font,
                &text,
                if today { TODAY_STYLE } else { PRIMARY_STYLE },
                grid_cell(index),
            );
        }

        for rect in Self::visible_rects(x, height, self.pill.calendar_expansion) {
            interaction.input_region(rect);
        }
    }

    pub fn apply_forecast(&mut self, forecast: &Forecast) {
        let raw = &forecast.current;
        self.utc_offset = Offset::from_seconds(forecast.utc_offset_seconds).ok();
        self.temperature = format!("{:.1}°C", raw.temperature_2m);
        self.pill.sun_hours =
            [forecast.daily.sunrise[0], forecast.daily.sunset[0]].map(Self::hour_of_day);
        self.pill.hourly_start = Self::hour_of_day(forecast.hourly.time[0]);
        self.hourly = from_fn(|index| {
            let source = index * HOURLY_STEP_HOURS;
            let time = forecast.hourly.time[source];
            self.pill.hourly_conditions[index] =
                openmeteo::coded_conditions(forecast.hourly.weather_code[source]);
            ForecastItem {
                text: [
                    time.strftime("%H:%M").to_string(),
                    format!("{:.0}°", forecast.hourly.temperature_2m[source]),
                ],
                hover_text: format!(
                    "{} {} {:.0}°",
                    time.strftime("%H:%M"),
                    openmeteo::weather_code(forecast.hourly.weather_code[source]),
                    forecast.hourly.temperature_2m[source]
                ),
            }
        });
        self.pill.hourly_conditions[0] = openmeteo::coded_conditions(raw.weather_code);
        self.daily = from_fn(|day| {
            let day = day + 1;
            let date = forecast.daily.sunrise[day];
            let date_day = date.day();
            let suffix = if (11..=13).contains(&(date_day % 100)) {
                "th"
            } else {
                Self::ORDINALS[(date_day % 10) as usize]
            };
            self.pill.daily_conditions[day - 1] =
                openmeteo::coded_conditions(forecast.daily.weather_code[day]);
            ForecastItem {
                text: [
                    date.strftime("%a").to_string(),
                    format!(
                        "{:.0}°/{:.0}°",
                        forecast.daily.temperature_2m_max[day], forecast.daily.temperature_2m_min[day]
                    ),
                ],
                hover_text: format!(
                    "{}{} {} {:.0}°/{:.0}°",
                    date.strftime("%A %-d"),
                    suffix,
                    openmeteo::weather_code(forecast.daily.weather_code[day]),
                    forecast.daily.temperature_2m_max[day],
                    forecast.daily.temperature_2m_min[day]
                ),
            }
        });
        self.details = format!(
            "{} · Humidity {}% · Wind {:.0} km/h",
            openmeteo::weather_code(raw.weather_code),
            raw.relative_humidity_2m,
            raw.wind_speed_10m
        );
    }

    fn hour_of_day(time: DateTime) -> f32 {
        time.time().duration_since(Time::midnight()).as_secs_f32() / 3600.0
    }
}

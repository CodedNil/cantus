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
    Vertex,
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    spirv_std::arch::kill,
};

#[cfg(target_arch = "spirv")]
use isthmus::spirv_std::num_traits::Float;

#[cfg(feature = "cpu")]
use {
    crate::{
        app::interaction::{InteractionState, Rect},
        render::{
            cpu::{Frame, Passes, approach},
            status,
            text::TextStyle,
        },
    },
    arrayvec::{ArrayString, ArrayVec},
    isthmus::Storage,
    jiff::{
        Span, Timestamp, Zoned,
        civil::{DateTime, Time},
        tz::{Offset, TimeZone},
    },
    std::{fmt::Write, mem, sync::mpsc::Receiver},
    tracing::warn,
};

/// Number of conditions shown in the hourly forecast row.
const HOURLY_FORECASTS: usize = 6;
/// Hours between adjacent conditions in the hourly forecast row.
const HOURLY_STEP_HOURS: usize = 4;
const DAILY_FORECASTS: usize = 5;
pub const MAX_WORLD_CLOCKS: usize = 3;
pub const WIDTH: f32 = UNIT * 77.0;
pub const EXTENSION: f32 = UNIT * 61.0;
const FORECAST_X: f32 = WIDTH + GAP;

// Calendar geometry.
const WEEKDAY_COUNT: usize = 7;
const GRID_ROWS: usize = 6;
const GRID_CELLS: usize = WEEKDAY_COUNT * GRID_ROWS;
const GRID_ROW_HEIGHT: f32 = UNIT * 6.0;
const GRID_TOP_Y: f32 = UNIT * 24.0;
const WEEKDAY_Y: f32 = UNIT * 17.0;
const TITLE: Vec2 = Vec2::new(WIDTH * 0.5, UNIT * 10.0);

// Offsets into the spatially indexed text-line buffer.
const TITLE_LINE: usize = 1;
const ARROW_LINES: usize = 2;
const DETAILS_LINE: usize = 4;
const FORECAST_LINES: usize = 5;
const DAILY_LINES: usize = FORECAST_LINES + HOURLY_FORECASTS * 2;
const WEEKDAY_LINES: usize = DAILY_LINES + DAILY_FORECASTS * 2;
const GRID_LINES: usize = WEEKDAY_LINES + WEEKDAY_COUNT;
const WORLD_CLOCK_LINES: usize = GRID_LINES + GRID_CELLS;
#[cfg(feature = "cpu")]
const WEEKDAYS: [&str; WEEKDAY_COUNT] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
#[cfg(feature = "cpu")]
const ORDINALS: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];
#[cfg(feature = "cpu")]
const MAX_TEXT_LINES: usize = WORLD_CLOCK_LINES + MAX_WORLD_CLOCKS * 2;
#[cfg(feature = "cpu")]
pub(crate) const TEXT_GLYPHS: usize = 768;

#[cfg(feature = "cpu")]
const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);
#[cfg(feature = "cpu")]
const WEATHER_STYLE: TextStyle = TextStyle::new(24.0, 600.0);
#[cfg(feature = "cpu")]
const TITLE_STYLE: TextStyle = TextStyle::new(20.0, 750.0);
#[cfg(feature = "cpu")]
const CLOCK_STYLE: TextStyle = TextStyle::new(12.0, 700.0);

#[isthmus::data]
#[derive(Default)]
pub struct WeatherSurface {
    pub x: f32,
    pub calendar_expansion: f32,
    pub sun_hours: [f32; 2],
    pub hourly_start: f32,
    pub text_hover: [f32; 3],
    pub hourly_conditions: [WeatherCondition; HOURLY_FORECASTS],
    pub daily_conditions: [WeatherCondition; DAILY_FORECASTS],
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

#[cfg(feature = "cpu")]
#[derive(Default)]
struct ForecastItem {
    text: [String; 2],
    hover_text: String,
}

#[cfg(feature = "cpu")]
struct WorldClock {
    label: String,
    timezone: TimeZone,
    weather: String,
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub pixel: Vec2,
    #[gpu(flat)]
    pub weather: Vec4,
}

#[isthmus::pass]
pub struct TempestasPass {
    pub pill: isthmus::Instance<Self>,
    text_lines: Storage<text::Line>,
    labels: Vec<text::Line>,
    temperature: String,
    utc_offset: Option<Offset>,
    details: String,
    hourly: [ForecastItem; HOURLY_FORECASTS],
    daily: [ForecastItem; DAILY_FORECASTS],
    timezones: ArrayVec<WorldClock, MAX_WORLD_CLOCKS>,
    month_offset: i32,
    forecast_updates: Receiver<monitor::Update>,
}

#[cfg(feature = "cpu")]
struct Labels<'a> {
    lines: &'a mut Vec<text::Line>,
    text: &'a mut text::Renderer,
    origin: Vec2,
}

#[cfg(feature = "cpu")]
impl Labels<'_> {
    fn centered(&mut self, content: &str, style: TextStyle, center: Vec2, color: Vec4) {
        self.lines.push(
            self.text
                .centered(content, style, self.origin + center)
                .with_color(color),
        );
    }

    fn pair(
        &mut self,
        content: [&str; 2],
        style: TextStyle,
        center: Vec2,
        spacing: f32,
        opacity: [f32; 2],
    ) {
        for (line, content) in content.into_iter().enumerate() {
            self.centered(
                content,
                style,
                center + vec2(0.0, (line as f32 * 2.0 - 1.0) * spacing),
                text::COLOR.extend(opacity[line]),
            );
        }
    }
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

#[cfg(feature = "cpu")]
fn grid_cell(index: usize) -> Vec2 {
    let column_width = WIDTH / WEEKDAY_COUNT as f32;
    vec2(
        (index % WEEKDAY_COUNT) as f32 * column_width + column_width * 0.5,
        GRID_TOP_Y + (index / WEEKDAY_COUNT) as f32 * GRID_ROW_HEIGHT,
    )
}

fn expanded_x(x: f32, expansion: f32) -> f32 {
    x - FORECAST_X * expansion * 0.5
}

fn popup_size(expansion: f32) -> Vec2 {
    Vec2::new(WIDTH + FORECAST_X * expansion, EXTENSION * expansion)
}

fn forecast_center(height: f32, row: f32) -> f32 {
    UNIT * 14.0 + height * 0.5 + row * (height + GAP)
}

/// The y that divides the hourly forecast row from the daily one.
fn forecast_split(height: f32) -> f32 {
    UNIT * 14.0 + height + GAP * 0.5
}

fn forecast_row(height: f32, row: f32) -> (Vec2, Vec2) {
    let size = Vec2::new(WIDTH - GAP * 2.0, height);
    let center = Vec2::new(FORECAST_X + WIDTH * 0.5, forecast_center(height, row));
    (center - size * 0.5, size)
}

fn world_clock_center(height: f32, index: usize) -> f32 {
    forecast_center(height, 1.0) + height * 0.5 + UNIT * (3.5 + index as f32 * 7.0)
}

fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let delay = 0.5 + (y / EXTENSION) * 0.18;
    smoothstep(delay, delay + 0.24, expansion)
}

#[isthmus::outline]
fn cell_index(value: f32, start: f32, size: f32, last: f32) -> usize {
    ((value - start) / size).floor().max(0.0).min(last) as usize
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

/// Daylight, blue-hour and twilight palette weights for a sun height.
fn sky_phase(sun_y: f32) -> Vec3 {
    let daylight = smoothstep(-0.04, 0.2, sun_y);
    vec3(
        daylight,
        smoothstep(-0.32, -0.08, sun_y) * (1.0 - daylight),
        smoothstep(-0.18, 0.0, sun_y) * smoothstep(0.2, 0.02, sun_y),
    )
}

fn scene(
    frame: &FrameData,
    p: Vec2,
    width: f32,
    dist: f32,
    phase: Vec3,
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
            phase.x,
        )
        .lerp(
            vec3(0.10, 0.16, 0.30).lerp(vec3(0.22, 0.25, 0.45), vertical),
            phase.y * 0.8,
        )
        .lerp(
            vec3(0.78, 0.30, 0.20).lerp(vec3(0.38, 0.22, 0.42), vertical),
            phase.z * 0.9,
        );

    let star_cell = (p / 18.0).floor();
    let star_center = (star_cell + 0.2 + hash(star_cell) * 0.6) * 18.0;
    let stars = smoothstep(1.0, 0.4, p.distance(star_center))
        * smoothstep(0.75, 1.0, hash(star_cell + 31.7).x)
        * (1.0 - phase.x);
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
                phase.x,
            )
            .lerp(
                vec3(0.5, 0.36, 0.4).lerp(vec3(0.76, 0.59, 0.56), cloud_light),
                phase.z * 0.45,
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

fn main_surface(
    pixel: Vec2,
    frame: &FrameData,
    pill_x: f32,
    expansion: f32,
    body: SdfSurface,
) -> SdfSurface {
    let size = popup_size(expansion);
    let origin = vec2(expanded_x(pill_x, expansion), PANEL_START + frame.panel_height);
    let gap = GAP * expansion;
    let box_size = vec2(size.x, (size.y - gap).max(0.0));
    let popup = |point: Vec2| {
        sd_rounded_box(
            point - origin - vec2(size.x * 0.5, gap + box_size.y * 0.5),
            box_size * 0.5,
            (box_size.y * 0.5).min(18.0),
        )
    };
    body.smooth_union(
        SdfSurface::new(popup(pixel), popup(frame.mouse_pos)),
        56.0,
        expansion,
    )
}

struct ForecastSample {
    local: Vec2,
    size: Vec2,
    surface: SdfSurface,
    reveal: f32,
    conditions: WeatherCondition,
    sun_height: f32,
}

fn sample_forecast(pixel: Vec2, frame: &FrameData, pill: &WeatherSurface) -> ForecastSample {
    let daily = pixel.y - PANEL_START - frame.panel_height > forecast_split(frame.panel_height);
    let row = if daily { 1.0 } else { 0.0 };
    let reveal = reveal_progress(pill.calendar_expansion, forecast_center(frame.panel_height, row));
    let (row_origin, full_size) = forecast_row(frame.panel_height, row);
    let size = full_size * reveal;
    let origin = vec2(expanded_x(pill.x, 1.0), PANEL_START + frame.panel_height)
        + row_origin
        + (full_size - size) * 0.5;
    let capsule = |point: Vec2| {
        if reveal <= 0.001 {
            f32::MAX
        } else {
            sd_capsule_box(point - origin - size * 0.5, (size.x - size.y) * 0.5, size.y * 0.5)
        }
    };
    let local = pixel - origin;
    let count = if daily { DAILY_FORECASTS } else { HOURLY_FORECASTS };
    let position = (local.x / size.x.max(0.001) * count as f32 - 0.5).clamp(0.0, (count - 1) as f32);
    let index = position.floor() as usize;
    let amount = smoothstep(0.0, 1.0, position.fract());
    let (conditions, next, hour) = if daily {
        (
            pill.daily_conditions[index],
            pill.daily_conditions[(index + 1).min(DAILY_FORECASTS - 1)],
            12.0,
        )
    } else {
        (
            pill.hourly_conditions[index],
            pill.hourly_conditions[(index + 1).min(HOURLY_FORECASTS - 1)],
            (pill.hourly_start + position * HOURLY_STEP_HOURS as f32) % 24.0,
        )
    };
    ForecastSample {
        local,
        size,
        surface: SdfSurface::new(capsule(pixel), capsule(frame.mouse_pos)),
        reveal,
        conditions: conditions.lerp(next, amount),
        sun_height: sun_position(hour, pill.sun_hours)[1],
    }
}

/// Selects the only text line whose spatial cell can cover `pixel`.
fn text_line_index(pixel: Vec2, frame: &FrameData, pill: &WeatherSurface) -> usize {
    let popup_origin = vec2(expanded_x(pill.x, 1.0), PANEL_START + frame.panel_height);
    if pixel.y < popup_origin.y {
        return 0;
    }
    let local = pixel - popup_origin;
    if local.x >= FORECAST_X {
        let first_forecast = forecast_center(frame.panel_height, 0.0);
        if local.y < (TITLE.y + first_forecast) * 0.5 {
            return DETAILS_LINE;
        }
        let first_clock = world_clock_center(frame.panel_height, 0);
        if local.y > first_clock - UNIT * 3.5 {
            let clock = cell_index(
                local.y,
                first_clock - UNIT * 3.5,
                UNIT * 7.0,
                (MAX_WORLD_CLOCKS - 1) as f32,
            );
            let center = world_clock_center(frame.panel_height, clock);
            return WORLD_CLOCK_LINES + clock * 2 + usize::from(local.y > center);
        }
        let daily = local.y > forecast_split(frame.panel_height);
        let (base, count, center) = if daily {
            (
                DAILY_LINES,
                DAILY_FORECASTS,
                forecast_center(frame.panel_height, 1.0),
            )
        } else {
            (FORECAST_LINES, HOURLY_FORECASTS, first_forecast)
        };
        let column = cell_index(local.x, FORECAST_X, WIDTH / count as f32, (count - 1) as f32);
        return base + column * 2 + usize::from(local.y > center);
    }
    if local.y < (TITLE.y + WEEKDAY_Y) * 0.5 {
        let reveal = reveal_progress(pill.calendar_expansion, TITLE.y);
        let offset = (WIDTH * 0.5 - UNIT * 7.0) * reveal;
        if (local.x - (TITLE.x - offset)).abs() < UNIT * 5.0 {
            return ARROW_LINES;
        }
        if (local.x - (TITLE.x + offset)).abs() < UNIT * 5.0 {
            return ARROW_LINES + 1;
        }
        return TITLE_LINE;
    }
    let column = cell_index(
        local.x,
        0.0,
        WIDTH / WEEKDAY_COUNT as f32,
        (WEEKDAY_COUNT - 1) as f32,
    );
    if local.y < (WEEKDAY_Y + GRID_TOP_Y) * 0.5 {
        return WEEKDAY_LINES + column;
    }
    let row = cell_index(
        local.y,
        GRID_TOP_Y - GRID_ROW_HEIGHT * 0.5,
        GRID_ROW_HEIGHT,
        (GRID_ROWS - 1) as f32,
    );
    GRID_LINES + row * WEEKDAY_COUNT + column
}

#[isthmus::pass]
impl TempestasPass {
    pub fn new(passes: &Passes<'_>, text: &text::Renderer, timezones: &[String]) -> Self {
        let mut forecast_timezones = Vec::with_capacity(timezones.len());
        let timezones: ArrayVec<_, MAX_WORLD_CLOCKS> = timezones
            .iter()
            .filter_map(|name| {
                let timezone = TimeZone::get(name)
                    .inspect_err(|error| warn!(timezone = name, %error, "Ignoring invalid timezone"))
                    .ok()?;
                forecast_timezones.push(name.clone());
                Some(WorldClock {
                    label: name.rsplit('/').next().unwrap_or(name).replace('_', " "),
                    timezone,
                    weather: String::from("Weather unavailable"),
                })
            })
            .collect();
        let forecast_updates = monitor::start(forecast_timezones);
        let text_lines = passes.storage_with_capacity("Tempestas Text", MAX_TEXT_LINES);
        let (placed_glyphs, glyphs, edges) = text.resources();
        let pill = passes.instance(
            (&text_lines, placed_glyphs, glyphs, edges),
            WeatherSurface {
                sun_hours: [6.0, 18.0],
                ..Default::default()
            },
        );
        Self {
            pill,
            text_lines,
            labels: Vec::with_capacity(MAX_TEXT_LINES),
            temperature: String::new(),
            utc_offset: None,
            details: "Weather unavailable".into(),
            hourly: Default::default(),
            daily: Default::default(),
            timezones,
            month_offset: 0,
            forecast_updates,
        }
    }

    pub fn update(
        &mut self,
        text: &mut text::Renderer,
        status: Option<&mut status::StatusPass>,
        frame: &mut Frame,
    ) {
        while let Ok(update) = self.forecast_updates.try_recv() {
            monitor::apply(self, update);
        }
        let height = frame.config.height;
        let x = Self::pill_x(frame.shared.screen_size.x, frame.shared.status_width);
        let (weather_label, hour) = self.collapsed_label(x, height, frame);
        frame.shared.weather_hour = hour;
        if let Some(status) = status {
            status.pill.sun_height = sun_position(hour, self.pill.sun_hours)[1];
            status.pill.conditions = self.pill.hourly_conditions[0];
        }
        let mut lines = mem::take(&mut self.labels);
        lines.clear();
        let mut labels = Labels {
            lines: &mut lines,
            text,
            origin: vec2(x, PANEL_START),
        };
        labels.centered(
            &weather_label,
            WEATHER_STYLE,
            vec2(WIDTH * 0.5, height * 0.46),
            text::COLOR.extend(1.0),
        );
        self.calendar_labels(&mut labels, x, height, frame.delta_time, frame.interaction);
        lines.resize_with(MAX_TEXT_LINES, text::Line::default);
        self.text_lines.upload(&lines);
        self.labels = lines;
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

    fn calendar_labels(
        &mut self,
        text: &mut Labels<'_>,
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
        text.origin = origin;
        let reveal = reveal_progress(self.pill.calendar_expansion, TITLE.y);
        let title = interaction.surface(Rect::from_center(
            origin + TITLE,
            Vec2::new(UNIT * 26.0, UNIT * 4.0),
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
        let expansion = self.pill.calendar_expansion;
        text.centered(
            &month.strftime("%B %Y").to_string(),
            TITLE_STYLE
                .scaled(1.0 + title_hover * 0.2)
                .with_weight_mix(0.5 + title_hover * 0.5),
            TITLE,
            text::COLOR.extend(reveal_progress(expansion, TITLE.y)),
        );
        for (index, (content, position)) in ["<", ">"].into_iter().zip(arrows).enumerate() {
            let hover = self.pill.text_hover[index + 1];
            text.centered(
                content,
                TITLE_STYLE
                    .scaled(1.0 + hover * 0.35)
                    .with_weight_mix(0.5 + hover * 0.5),
                position,
                text::COLOR.extend(reveal_progress(expansion, TITLE.y)),
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
        let details = Vec2::new(FORECAST_X + WIDTH * 0.5, TITLE.y);
        text.centered(
            hovered_forecast.unwrap_or(&self.details),
            DETAILS_STYLE,
            details,
            text::COLOR.extend(reveal_progress(expansion, details.y)),
        );

        for (row, forecasts) in forecasts.into_iter().enumerate() {
            let center = forecast_center(height, row as f32);
            let opacity = reveal_progress(expansion, center);
            for (column, forecast) in forecasts.iter().enumerate() {
                let x = FORECAST_X + (column as f32 + 0.5) * WIDTH / forecasts.len() as f32;
                text.pair(
                    [&forecast.text[0], &forecast.text[1]],
                    DETAILS_STYLE,
                    vec2(x, center),
                    GAP,
                    [opacity; 2],
                );
            }
        }

        for (column, weekday) in WEEKDAYS.iter().enumerate() {
            text.centered(
                weekday,
                DETAILS_STYLE,
                Vec2::new(grid_cell(column).x, WEEKDAY_Y),
                text::COLOR.extend(reveal_progress(expansion, WEEKDAY_Y) * 0.75),
            );
        }

        let grid_start = month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
        for index in 0..GRID_CELLS {
            let date = grid_start.saturating_add(Span::new().days(index as i64));
            let mut label = ArrayString::<2>::new();
            write!(label, "{}", date.day()).unwrap();
            let today = date == today;
            let (color, style) = if today {
                (vec3(1.0, 0.68, 0.68), TextStyle::new(16.0, 900.0))
            } else {
                (text::COLOR, TextStyle::new(16.0, 700.0))
            };
            let month_opacity = if date.month() == month.month() { 1.0 } else { 0.32 };
            let position = grid_cell(index);
            text.centered(
                &label,
                style,
                position,
                color.extend(reveal_progress(expansion, position.y) * month_opacity),
            );
        }

        let now = Timestamp::now();
        for (index, timezone) in self.timezones.iter().enumerate() {
            let center = world_clock_center(height, index);
            let local = now.to_zoned(timezone.timezone.clone());
            let mut clock = ArrayString::<64>::new();
            write!(clock, "{} · {}", timezone.label, local.strftime("%H:%M")).unwrap();
            let opacity = reveal_progress(expansion, center);
            text.pair(
                [&clock, &timezone.weather],
                CLOCK_STYLE,
                vec2(FORECAST_X + WIDTH * 0.5, center),
                GAP * 0.7,
                [opacity, opacity * 0.75],
            );
        }

        for rect in Self::visible_rects(x, height, self.pill.calendar_expansion) {
            interaction.input_region(rect);
        }
    }

    fn hour_of_day(time: DateTime) -> f32 {
        time.time().duration_since(Time::midnight()).as_secs_f32() / 3600.0
    }

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
        #[gpu(resource)] text_lines: &[text::Line],
        #[gpu(resource)] placed_glyphs: &[text::PlacedGlyph],
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
    ) -> Vec4 {
        let (_, body_local, body_size, body_surface) = pill_fragment(pixel, frame, pill.x, WIDTH);
        let expansion = weather.w;
        let main_surface = main_surface(pixel, frame, pill.x, expansion, body_surface);
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
        let sample = (main_refracted * body_size).lerp(row_refracted * row.size, row_blend);
        let scene_width = body_size.x + (row.size.x - body_size.x) * row_blend;
        let scene_distance = main_surface_distance
            + (row_surface_distance.min(1000.0) - main_surface_distance) * row_blend;
        let conditions = main_conditions.lerp(row.conditions, row_blend);
        let phase = sky_phase(weather.y).lerp(sky_phase(row.sun_height), row_blend);
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
        color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
        let label = text_lines[text_line_index(pixel, frame, pill)];
        let label_color = label.color.to_vec4();
        let text_alpha = text::line_alpha(label, placed_glyphs, glyphs, edges, pixel) * label_color.w;
        color = color * (1.0 - text_alpha) + label_color.truncate() * text_alpha;
        (color * mask).extend(alpha)
    }
}

#[cfg(feature = "cpu")]
mod monitor {
    use super::{ForecastItem, HOURLY_STEP_HOURS, ORDINALS, TempestasPass, WeatherCondition};
    use ashpd::desktop::location::{Accuracy, CreateSessionOptions, LocationProxy, StartOptions};
    use futures_util::StreamExt;
    use jiff::{
        civil::DateTime,
        tz::{Offset, TimeZone},
    };
    use serde::{Deserialize, de::DeserializeOwned};
    use std::{
        array::from_fn,
        sync::mpsc::{self, Receiver, Sender},
        thread,
        time::Duration,
    };
    use tracing::{info, warn};

    const WEATHER_FIELDS: &str = "temperature_2m,weather_code";
    const REFRESH_INTERVAL: Duration = Duration::from_mins(15);
    const RETRY_INTERVAL: Duration = Duration::from_secs(30);

    pub(super) struct Update(Vec<(usize, Forecast)>);

    #[derive(Deserialize)]
    struct Forecast {
        utc_offset_seconds: i32,
        current: Current,
        hourly: Hourly,
        daily: Daily,
    }

    #[derive(Deserialize)]
    struct Current {
        weather_code: u8,
        temperature_2m: f32,
        relative_humidity_2m: u8,
        wind_speed_10m: f32,
    }

    #[derive(Deserialize)]
    struct Hourly {
        weather_code: [u8; 24],
        time: [DateTime; 24],
        temperature_2m: [f32; 24],
    }

    #[derive(Deserialize)]
    struct Daily {
        weather_code: [u8; 6],
        temperature_2m_max: [f32; 6],
        temperature_2m_min: [f32; 6],
        sunrise: [DateTime; 6],
        sunset: [DateTime; 6],
    }

    #[derive(Deserialize)]
    struct SearchResults {
        #[serde(default)]
        results: Vec<Place>,
    }

    #[derive(Deserialize)]
    struct Place {
        latitude: f32,
        longitude: f32,
        timezone: String,
    }

    macro_rules! weather_codes {
        ($($code:literal => $name:literal { $($field:ident: $value:literal),* };)*) => {
            fn weather(code: u8) -> (&'static str, WeatherCondition) {
                match code {
                    $($code => ($name, WeatherCondition {
                        $($field: $value,)*
                        ..Default::default()
                    }),)*
                    _ => ("Unknown weather", Default::default()),
                }
            }
        };
    }

    weather_codes! {
        0 => "Clear" { };
        1 => "Mainly Clear" { cloud: 0.25 };
        2 => "Partly Cloudy" { cloud: 0.55 };
        3 => "Overcast" { cloud: 0.8 };
        45 => "Fog" { fog: 0.6 };
        48 => "Rime Fog" { fog: 0.75 };
        51 => "Light Drizzle" { rain: 0.15 };
        53 => "Moderate Drizzle" { rain: 0.3 };
        55 => "Dense Drizzle" { rain: 0.45 };
        56 => "Light Freezing Drizzle" { rain: 0.2 };
        57 => "Dense Freezing Drizzle" { rain: 0.4 };
        61 => "Light Rain" { rain: 0.3 };
        63 => "Moderate Rain" { rain: 0.6 };
        65 => "Heavy Rain" { rain: 1.0 };
        66 => "Light Freezing Rain" { rain: 0.35 };
        67 => "Heavy Freezing Rain" { rain: 0.9 };
        71 => "Light Snow" { snow: 0.3 };
        73 => "Moderate Snow" { snow: 0.6 };
        75 => "Heavy Snow" { snow: 1.0 };
        77 => "Snow Grains" { snow: 0.25 };
        80 => "Light Rain Showers" { rain: 0.35 };
        81 => "Moderate Rain Showers" { rain: 0.65 };
        82 => "Violent Rain Showers" { rain: 1.0 };
        85 => "Light Snow Showers" { snow: 0.35 };
        86 => "Heavy Snow Showers" { snow: 0.9 };
        95 => "Thunderstorm" { rain: 0.7, lightning: 1.0 };
        96 => "Thunderstorm Light Hail" { rain: 0.75, lightning: 1.0, hail: 0.6 };
        99 => "Thunderstorm Heavy Hail" { rain: 0.85, lightning: 1.0, hail: 1.0 };
    }

    pub(super) fn start(timezones: Vec<String>) -> Receiver<Update> {
        let (forecast_tx, forecast_updates) = mpsc::channel();
        let locations = location_monitor();
        thread::spawn(move || refresh_loop(&timezones, &forecast_tx, &locations));
        forecast_updates
    }

    fn location_monitor() -> Receiver<[f32; 2]> {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            if let Err(error) = pollster::block_on(stream_location(sender)) {
                warn!(%error, "Location portal unavailable");
            }
        });
        receiver
    }

    async fn stream_location(sender: Sender<[f32; 2]>) -> Result<(), String> {
        let proxy = LocationProxy::new().await.map_err(|error| error.to_string())?;
        let session = proxy
            .create_session(CreateSessionOptions::default().set_accuracy(Accuracy::City))
            .await
            .map_err(|error| error.to_string())?;
        let mut updates = proxy
            .receive_location_updated()
            .await
            .map_err(|error| error.to_string())?;
        proxy
            .start(&session, None, StartOptions::default())
            .await
            .map_err(|error| error.to_string())?
            .response()
            .map_err(|error| error.to_string())?;
        while let Some(value) = updates.next().await {
            if sender
                .send([value.latitude() as f32, value.longitude() as f32])
                .is_err()
            {
                break;
            }
        }
        session.close().await.map_err(|error| error.to_string())
    }

    fn refresh_loop(
        timezones: &[String],
        forecast_tx: &Sender<Update>,
        locations_rx: &Receiver<[f32; 2]>,
    ) {
        let mut locations = vec![None; timezones.len() + 1];
        let mut portal_location = false;
        if let Some(timezone) = TimeZone::system().iana_name() {
            match geocode(timezone) {
                Ok(location) => {
                    info!(
                        timezone,
                        latitude = location[0],
                        longitude = location[1],
                        "Using timezone fallback weather location"
                    );
                    locations[0] = Some(location);
                }
                Err(error) => warn!(%error, timezone, "Failed to locate system timezone"),
            }
        } else {
            warn!("System timezone has no IANA name; waiting for the location portal");
        }
        loop {
            while let Ok(location) = locations_rx.try_recv() {
                locations[0] = Some(location);
                portal_location = true;
            }
            let mut retry = false;
            let mut forecasts = Vec::with_capacity(locations.len());
            let mut ready = Vec::with_capacity(locations.len());
            for (index, location) in locations.iter_mut().enumerate() {
                if index > 0 && location.is_none() {
                    *location = geocode(&timezones[index - 1])
                        .inspect_err(|error| {
                            retry = true;
                            warn!(%error, timezone = timezones[index - 1], "Failed to locate timezone");
                        })
                        .ok();
                }
                let Some([latitude, longitude]) = *location else {
                    retry = true;
                    continue;
                };
                ready.push((index, [latitude, longitude]));
            }
            match fetch(&ready) {
                Ok(results) => {
                    for ((index, [latitude, longitude]), forecast) in ready.into_iter().zip(results) {
                        if index == 0 {
                            info!(
                                source = if portal_location {
                                    "portal"
                                } else {
                                    "timezone fallback"
                                },
                                latitude,
                                longitude,
                                temperature = forecast.current.temperature_2m,
                                "Current weather refreshed"
                            );
                        }
                        forecasts.push((index, forecast));
                    }
                }
                Err(error) => {
                    retry = true;
                    warn!(%error, "Failed to refresh weather");
                }
            }
            if !forecasts.is_empty() && forecast_tx.send(Update(forecasts)).is_err() {
                return;
            }
            let interval = if retry { RETRY_INTERVAL } else { REFRESH_INTERVAL };
            match locations_rx.recv_timeout(interval) {
                Ok(location) => {
                    locations[0] = Some(location);
                    portal_location = true;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => thread::sleep(interval),
            }
        }
    }

    pub(super) fn apply(pass: &mut TempestasPass, Update(forecasts): Update) {
        for (index, forecast) in forecasts {
            apply_forecast(pass, index, &forecast);
        }
    }

    fn apply_forecast(pass: &mut TempestasPass, index: usize, forecast: &Forecast) {
        let Forecast {
            utc_offset_seconds,
            current,
            hourly,
            daily,
        } = forecast;
        if let Some(timezone) = index
            .checked_sub(1)
            .and_then(|index| pass.timezones.get_mut(index))
        {
            timezone.weather = format!(
                "{} · {:.0}°/{:.0}°",
                weather(daily.weather_code[0]).0,
                daily.temperature_2m_max[0],
                daily.temperature_2m_min[0],
            );
            return;
        }
        if index != 0 {
            return;
        }
        pass.utc_offset = Offset::from_seconds(*utc_offset_seconds).ok();
        pass.temperature = format!("{:.1}°C", current.temperature_2m);
        pass.pill.sun_hours = [daily.sunrise[0], daily.sunset[0]].map(TempestasPass::hour_of_day);
        pass.pill.hourly_start = TempestasPass::hour_of_day(hourly.time[0]);
        pass.hourly = from_fn(|index| {
            let source = index * HOURLY_STEP_HOURS;
            let time = hourly.time[source];
            let (description, conditions) = weather(hourly.weather_code[source]);
            pass.pill.hourly_conditions[index] = conditions;
            ForecastItem {
                text: [
                    time.strftime("%H:%M").to_string(),
                    format!("{:.0}°", hourly.temperature_2m[source]),
                ],
                hover_text: format!(
                    "{} {description} {:.0}°",
                    time.strftime("%H:%M"),
                    hourly.temperature_2m[source]
                ),
            }
        });
        pass.pill.hourly_conditions[0] = weather(current.weather_code).1;
        pass.daily = from_fn(|index| {
            let day = index + 1;
            let date = daily.sunrise[day];
            let number = date.day();
            let suffix = if (11..=13).contains(&(number % 100)) {
                "th"
            } else {
                ORDINALS[(number % 10) as usize]
            };
            let (description, conditions) = weather(daily.weather_code[day]);
            pass.pill.daily_conditions[index] = conditions;
            ForecastItem {
                text: [
                    date.strftime("%a").to_string(),
                    format!(
                        "{:.0}°/{:.0}°",
                        daily.temperature_2m_max[day], daily.temperature_2m_min[day]
                    ),
                ],
                hover_text: format!(
                    "{}{suffix} {description} {:.0}°/{:.0}°",
                    date.strftime("%A %-d"),
                    daily.temperature_2m_max[day],
                    daily.temperature_2m_min[day]
                ),
            }
        });
        pass.details = format!(
            "{} · Humidity {}% · Wind {:.0} km/h",
            weather(current.weather_code).0,
            current.relative_humidity_2m,
            current.wind_speed_10m
        );
    }

    fn geocode(timezone: &str) -> Result<[f32; 2], String> {
        let city = timezone.rsplit('/').next().unwrap_or(timezone).replace('_', " ");
        let query: String = form_urlencoded::byte_serialize(city.as_bytes()).collect();
        let results: SearchResults = get_json(format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={query}&count=10"
        ))?;
        let place = results
            .results
            .iter()
            .find(|place| place.timezone == timezone)
            .or_else(|| results.results.first())
            .ok_or_else(|| format!("no place found for {city}"))?;
        Ok([place.latitude, place.longitude])
    }

    fn fetch(locations: &[(usize, [f32; 2])]) -> Result<Vec<Forecast>, String> {
        if locations.is_empty() {
            return Ok(Vec::new());
        }
        let latitude = locations
            .iter()
            .map(|(_, [latitude, _])| latitude.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let longitude = locations
            .iter()
            .map(|(_, [_, longitude])| longitude.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current={WEATHER_FIELDS},relative_humidity_2m,wind_speed_10m&hourly={WEATHER_FIELDS}&forecast_hours=24&daily=weather_code,temperature_2m_max,temperature_2m_min,sunrise,sunset&temperature_unit=celsius&timezone=auto&forecast_days=6"
        );
        if locations.len() == 1 {
            get_json(url).map(|forecast| vec![forecast])
        } else {
            get_json(url)
        }
    }

    fn get_json<T: DeserializeOwned>(url: String) -> Result<T, String> {
        let mut response = ureq::get(url).call().map_err(|error| error.to_string())?;
        response.body_mut().read_json().map_err(|error| error.to_string())
    }
}

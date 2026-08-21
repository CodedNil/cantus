use crate::render::{
    FrameData, GAP, PANEL_START, UNIT,
    shader::{
        PillInteraction, SdfSurface, cloud_mass, fbm, hash, pill_fragment, pill_interaction, pill_sheen,
        pill_vertex, sd_capsule_box, sd_rounded_box, sdf_coverage,
    },
    smoothstep, text,
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
        app::{
            AppUpdater, Background,
            interaction::{InteractionState, Rect},
        },
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
    reqwest::Client,
    std::{fmt::Write, mem},
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

/// Labels are found through a lookup grid over the pill and its expanded popup: each cell names
/// up to two labels whose padded bounds reach it, so the fragment samples two lines rather than
/// all of them. Two slots suffice because only stacked or neighbouring labels share a cell.
const TEXT_COLUMNS: usize = 24;
const TEXT_ROWS: usize = 36;
#[cfg(feature = "cpu")]
mod host {
    use super::{MAX_WORLD_CLOCKS, TextStyle, UNIT, Vec2, WIDTH};

    pub const WEEKDAY_COUNT: usize = 7;
    pub const GRID_CELLS: usize = WEEKDAY_COUNT * 6;
    pub const GRID_ROW_HEIGHT: f32 = UNIT * 6.0;
    pub const GRID_TOP_Y: f32 = UNIT * 24.0;
    pub const WEEKDAY_Y: f32 = UNIT * 17.0;
    pub const TITLE: Vec2 = Vec2::new(WIDTH * 0.5, UNIT * 10.0);
    pub const WEEKDAYS: [&str; WEEKDAY_COUNT] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    pub const ORDINALS: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];
    pub const TEXT_CELLS: usize = super::TEXT_COLUMNS * super::TEXT_ROWS;
    pub const MAX_TEXT_LINES: usize = 17
        + (super::HOURLY_FORECASTS + super::DAILY_FORECASTS + MAX_WORLD_CLOCKS) * 2
        + WEEKDAY_COUNT
        + GRID_CELLS;
    pub const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);
    pub const WEATHER_STYLE: TextStyle = TextStyle::new(24.0, 600.0);
    pub const TITLE_STYLE: TextStyle = TextStyle::new(20.0, 750.0);
    pub const CLOCK_STYLE: TextStyle = TextStyle::new(12.0, 700.0);
}

#[cfg(feature = "cpu")]
use host::{
    CLOCK_STYLE, DETAILS_STYLE, GRID_CELLS, GRID_ROW_HEIGHT, GRID_TOP_Y, MAX_TEXT_LINES, ORDINALS,
    TEXT_CELLS, TITLE, TITLE_STYLE, WEATHER_STYLE, WEEKDAY_COUNT, WEEKDAY_Y, WEEKDAYS,
};

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
    text_cells: Storage<u32>,
    labels: Vec<text::Line>,
    cells: Box<[u32; TEXT_CELLS]>,
    temperature: String,
    utc_offset: Option<Offset>,
    details: String,
    hourly: [ForecastItem; HOURLY_FORECASTS],
    daily: [ForecastItem; DAILY_FORECASTS],
    timezones: ArrayVec<WorldClock, MAX_WORLD_CLOCKS>,
    month_offset: i32,
}

#[cfg(feature = "cpu")]
struct Labels<'a> {
    lines: &'a mut Vec<text::Line>,
    cells: &'a mut [u32; TEXT_CELLS],
    text: &'a mut text::Renderer,
    /// Where popup-relative label positions start from.
    origin: Vec2,
    /// Lookup grid origin and cell size, in surface pixels.
    grid: (Vec2, Vec2),
    /// Popup progress driving each label's staggered fade; 1.0 leaves them fully visible.
    expansion: f32,
}

#[cfg(feature = "cpu")]
impl Labels<'_> {
    fn centered(&mut self, content: &str, style: TextStyle, center: Vec2, opacity: f32) {
        self.colored(content, style, center, text::COLOR, opacity);
    }

    fn colored(&mut self, content: &str, style: TextStyle, center: Vec2, color: Vec3, opacity: f32) {
        let alpha = reveal_progress(self.expansion, center.y) * opacity;
        let line = self
            .text
            .centered(content, style, self.origin + center)
            .with_color(color.extend(alpha));
        let index = self.lines.len() as u32;
        self.lines.push(line);

        let (grid_origin, cell) = self.grid;
        let last = vec2(TEXT_COLUMNS as f32, TEXT_ROWS as f32) - 1.0;
        let first_cell = ((line.min - grid_origin) / cell).floor().clamp(Vec2::ZERO, last);
        let last_cell = ((line.max - grid_origin) / cell).floor().clamp(first_cell, last);
        for row in first_cell.y as usize..=last_cell.y as usize {
            for column in first_cell.x as usize..=last_cell.x as usize {
                // Two label indices share each cell word; the low half fills first.
                let slot = &mut self.cells[row * TEXT_COLUMNS + column];
                if *slot == 0 {
                    *slot = index;
                } else if *slot >> 16 == 0 {
                    *slot |= index << 16;
                }
            }
        }
    }

    /// Two stacked labels sharing a center, the second dimmer than the first.
    fn pair(&mut self, content: [&str; 2], style: TextStyle, center: Vec2, spacing: f32, dim: f32) {
        for (line, content) in content.into_iter().enumerate() {
            let offset = vec2(0.0, (line as f32 * 2.0 - 1.0) * spacing);
            self.centered(content, style, center + offset, if line == 0 { 1.0 } else { dim });
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

#[cfg(feature = "cpu")]
fn world_clock_center(height: f32, index: usize) -> f32 {
    forecast_center(height, 1.0) + height * 0.5 + UNIT * (3.5 + index as f32 * 7.0)
}

/// Origin and cell size of the label lookup grid, which spans the pill plus its full popup.
fn text_grid(frame: &FrameData, pill_x: f32) -> (Vec2, Vec2) {
    let size = vec2(WIDTH + FORECAST_X, frame.panel_height + EXTENSION);
    let cell = size / vec2(TEXT_COLUMNS as f32, TEXT_ROWS as f32);
    (vec2(expanded_x(pill_x, 1.0), PANEL_START), cell)
}

#[cfg(feature = "cpu")]
fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let delay = 0.5 + (y / EXTENSION) * 0.18;
    smoothstep(delay, delay + 0.24, expansion)
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
pub(crate) fn sky_phase(sun_y: f32) -> Vec3 {
    let daylight = smoothstep(-0.04, 0.2, sun_y);
    vec3(
        daylight,
        smoothstep(-0.32, -0.08, sun_y) * (1.0 - daylight),
        smoothstep(-0.18, 0.0, sun_y) * smoothstep(0.2, 0.02, sun_y),
    )
}

pub(crate) fn scene(
    frame: &FrameData,
    p: Vec2,
    width: f32,
    dist: f32,
    phase: Vec3,
    weather: WeatherCondition,
) -> Vec3 {
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
    color += Vec3::splat(stars * (1.0 - weather.cloud) * (0.3 + vertical * 0.7));

    if weather.cloud > 1.0 / 1024.0 {
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
        color = color.lerp(cloud_color, weather.cloud * (0.12 + cloud_shape * 0.7));
    }

    color = color.lerp(vec3(0.1, 0.17, 0.25), weather.rain * 0.2);
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

    let flash = smoothstep(0.92, 1.0, (time * 2.7).sin()) * weather.lightning;
    color = color.lerp(vec3(0.65, 0.74, 0.96), flash * 0.55);

    if weather.fog > 1.0 / 1024.0 {
        let fog = fbm(vec2(p.x / width * 0.9 + time * 0.008, sky_y * 0.32 + 12.0));
        color = color.lerp(
            vec3(0.63, 0.69, 0.73),
            weather.fog * (0.58 + smoothstep(0.35, 0.7, fog) * 0.18),
        );
    }
    color + pill_sheen(dist)
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
    body.smooth_union(SdfSurface::sample(pixel, frame.mouse_pos, popup), 56.0, expansion)
}

struct ForecastSample {
    local: Vec2,
    size: Vec2,
    surface: SdfSurface,
    conditions: WeatherCondition,
    sun_height: f32,
}

fn sample_forecast(pixel: Vec2, frame: &FrameData, pill: &WeatherSurface) -> ForecastSample {
    let daily = pixel.y - PANEL_START - frame.panel_height > forecast_split(frame.panel_height);
    let row = if daily { 1.0 } else { 0.0 };
    let (row_origin, size) = forecast_row(frame.panel_height, row);
    let origin = vec2(expanded_x(pill.x, 1.0), PANEL_START + frame.panel_height) + row_origin;
    let capsule =
        |point: Vec2| sd_capsule_box(point - origin - size * 0.5, (size.x - size.y) * 0.5, size.y * 0.5);
    let local = pixel - origin;
    let count = if daily { DAILY_FORECASTS } else { HOURLY_FORECASTS };
    let position = (local.x / size.x * count as f32 - 0.5).clamp(0.0, (count - 1) as f32);
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
        surface: SdfSurface::sample(pixel, frame.mouse_pos, capsule),
        conditions: conditions.lerp(next, amount),
        sun_height: sun_position(hour, pill.sun_hours)[1],
    }
}

#[isthmus::pass]
impl TempestasPass {
    pub(crate) fn new(
        passes: &Passes<'_>,
        text: &text::Renderer,
        timezones: &[String],
        background: &Background,
        updater: AppUpdater,
        http: Client,
    ) -> Self {
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
        monitor::start(forecast_timezones, background, updater, http);
        let text_lines = passes.storage_with_capacity("Tempestas Text", MAX_TEXT_LINES);
        let text_cells = passes.storage_with_capacity("Tempestas Text Grid", TEXT_CELLS);
        let (placed_glyphs, glyphs, edges) = text.resources();
        let pill = passes.instance(
            (&text_lines, &text_cells, placed_glyphs, glyphs, edges),
            WeatherSurface {
                sun_hours: [6.0, 18.0],
                ..Default::default()
            },
        );
        Self {
            pill,
            text_lines,
            text_cells,
            labels: Vec::with_capacity(MAX_TEXT_LINES),
            cells: Box::new([0; TEXT_CELLS]),
            temperature: String::new(),
            utc_offset: None,
            details: "Weather unavailable".into(),
            hourly: Default::default(),
            daily: Default::default(),
            timezones,
            month_offset: 0,
        }
    }

    pub fn update(
        &mut self,
        text: &mut text::Renderer,
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
        let mut lines = mem::take(&mut self.labels);
        let mut cells = mem::replace(&mut self.cells, Box::new([0; TEXT_CELLS]));
        lines.clear();
        // Index 0 is an empty line, so a cell with no label needs no sentinel of its own.
        lines.push(text::Line::default());
        cells.fill(0);
        let mut labels = Labels {
            lines: &mut lines,
            cells: &mut cells,
            text,
            origin: vec2(x, PANEL_START),
            grid: text_grid(frame.shared, x),
            expansion: 1.0,
        };
        labels.centered(
            &weather_label,
            WEATHER_STYLE,
            vec2(WIDTH * 0.5, height * 0.46),
            1.0,
        );
        self.calendar_labels(&mut labels, x, height, frame.delta_time, frame.interaction);
        self.text_lines.upload(&lines);
        self.text_cells.upload(&cells[..]);
        self.labels = lines;
        self.cells = cells;
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
        let surfaces = Self::visible_rects(x, height, self.pill.calendar_expansion);
        let expansion = self.pill.calendar_expansion;
        text.expansion = expansion;
        let reveal = reveal_progress(expansion, TITLE.y);
        let hover_blend = 1.0 - (-delta_time * 14.0).exp();

        let title = interaction.surface(Rect::from_center(
            origin + TITLE,
            Vec2::new(UNIT * 26.0, UNIT * 4.0),
        ));
        if title.clicked {
            self.month_offset = 0;
        }
        self.pill.text_hover[0] += (f32::from(title.hovered) - self.pill.text_hover[0]) * hover_blend;

        let today = Zoned::now().date();
        let month = today
            .first_of_month()
            .saturating_add(Span::new().months(self.month_offset));
        let title_hover = self.pill.text_hover[0];
        text.centered(
            &month.strftime("%B %Y").to_string(),
            TITLE_STYLE
                .scaled(1.0 + title_hover * 0.2)
                .with_weight_mix(0.5 + title_hover * 0.5),
            TITLE,
            1.0,
        );

        for (side, glyph) in [(-1.0f32, "<"), (1.0, ">")] {
            let position = Vec2::new(
                WIDTH * 0.5 + side * (WIDTH * 0.5 - UNIT * 7.0) * reveal,
                TITLE.y - (1.0 - reveal) * UNIT * 3.0,
            );
            let response =
                interaction.surface(Rect::from_center(origin + position, Vec2::splat(UNIT * 5.0)));
            if response.clicked {
                self.month_offset = (self.month_offset + side as i32).clamp(-1200, 1200);
            }
            let hover = &mut self.pill.text_hover[usize::from(side > 0.0) + 1];
            *hover += (f32::from(response.hovered) - *hover) * hover_blend;
            let style = TITLE_STYLE
                .scaled(1.0 + *hover * 0.35)
                .with_weight_mix(0.5 + *hover * 0.5);
            text.centered(glyph, style, position, 1.0);
        }

        // The hovered forecast column takes over the details line while the pointer is on it.
        let forecasts = [&self.hourly[..], &self.daily[..]];
        let hovered = forecasts.into_iter().enumerate().find_map(|(row, items)| {
            let (row_origin, size) = forecast_row(height, row as f32);
            let left = origin.x + row_origin.x;
            let top = bounds.y1 + row_origin.y;
            if !interaction.contains(Rect::new(left, top, left + size.x, top + size.y)) {
                return None;
            }
            let column = ((interaction.pointer.x - left) / size.x * items.len() as f32) as usize;
            Some(items[column.min(items.len() - 1)].hover_text.as_str())
        });
        text.centered(
            hovered.unwrap_or(&self.details),
            DETAILS_STYLE,
            Vec2::new(FORECAST_X + WIDTH * 0.5, TITLE.y),
            1.0,
        );

        for (row, items) in forecasts.into_iter().enumerate() {
            let center = forecast_center(height, row as f32);
            for (column, forecast) in items.iter().enumerate() {
                let x = FORECAST_X + (column as f32 + 0.5) * WIDTH / items.len() as f32;
                let [primary, secondary] = &forecast.text;
                text.pair([primary, secondary], DETAILS_STYLE, vec2(x, center), GAP, 1.0);
            }
        }

        for (column, weekday) in WEEKDAYS.iter().enumerate() {
            let position = Vec2::new(grid_cell(column).x, WEEKDAY_Y);
            text.centered(weekday, DETAILS_STYLE, position, 0.75);
        }

        let grid_start = month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
        for index in 0..GRID_CELLS {
            let date = grid_start.saturating_add(Span::new().days(index as i64));
            let mut label = ArrayString::<2>::new();
            write!(label, "{}", date.day()).unwrap();
            let is_today = date == today;
            text.colored(
                &label,
                TextStyle::new(16.0, if is_today { 900.0 } else { 700.0 }),
                grid_cell(index),
                if is_today {
                    vec3(1.0, 0.68, 0.68)
                } else {
                    text::COLOR
                },
                if date.month() == month.month() { 1.0 } else { 0.32 },
            );
        }

        let now = Timestamp::now();
        for (index, timezone) in self.timezones.iter().enumerate() {
            let local = now.to_zoned(timezone.timezone.clone());
            let mut clock = ArrayString::<64>::new();
            write!(clock, "{} · {}", timezone.label, local.strftime("%H:%M")).unwrap();
            let center = vec2(FORECAST_X + WIDTH * 0.5, world_clock_center(height, index));
            text.pair([&clock, &timezone.weather], CLOCK_STYLE, center, GAP * 0.7, 0.75);
        }

        for rect in surfaces {
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
            PANEL_START,
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
        #[gpu(resource)] text_cells: &[u32],
        #[gpu(resource)] placed_glyphs: &[text::PlacedGlyph],
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
    ) -> Vec4 {
        let (_, body_local, body_size, body_surface) =
            pill_fragment(pixel, frame, pill.x, PANEL_START, WIDTH);
        let expansion = weather.w;
        let interaction = pill_interaction(pixel, frame);

        // The pill merged with its popup is the whole shape; forecast rows only pick a scene.
        let surface_distance =
            interaction.expand(main_surface(pixel, frame, pill.x, expansion, body_surface));
        let (_, mask, alpha) = PillInteraction::shade(surface_distance);
        if alpha <= 1.0 / 1024.0 {
            kill();
        }

        let row = sample_forecast(pixel, frame, pill);
        let row_surface_distance = interaction.expand(row.surface);
        let current = pill.hourly_conditions[0];
        let edge = ((body_local.x / body_size.x).clamp(0.0, 1.0) - 0.5).abs();
        let body_conditions = current.lerp(pill.hourly_conditions[1], smoothstep(0.05, 0.25, edge));
        let main_conditions = body_conditions.lerp(current, expansion);
        let main_refracted = interaction.refract(body_local, body_size, surface_distance);
        let row_refracted = interaction.refract(row.local, row.size, row_surface_distance);
        let row_blend = sdf_coverage(row_surface_distance);
        let in_row = row_blend > 0.001;
        let (sample, width, distance, phase, conditions) = if in_row {
            (
                row_refracted * row.size,
                row.size.x,
                row_surface_distance,
                sky_phase(row.sun_height),
                row.conditions,
            )
        } else {
            (
                main_refracted * body_size,
                body_size.x,
                surface_distance,
                sky_phase(weather.y),
                main_conditions,
            )
        };
        let mut color = scene(frame, sample, width, distance, phase, conditions);
        // Only the antialiased boundary needs both nonlinear scene evaluations.
        if in_row && row_blend < 0.999 {
            let main_color = scene(
                frame,
                main_refracted * body_size,
                body_size.x,
                surface_distance,
                sky_phase(weather.y),
                main_conditions,
            );
            color = main_color.lerp(color, row_blend);
        }
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

        // Both labels a grid cell can name, so overlapping padded bounds never drop a glyph.
        let (grid_origin, cell_size) = text_grid(frame, pill.x);
        let cell = ((pixel - grid_origin) / cell_size).floor().clamp(
            Vec2::ZERO,
            vec2(TEXT_COLUMNS as f32 - 1.0, TEXT_ROWS as f32 - 1.0),
        );
        let names = *isthmus::reference(text_cells, (cell.y * TEXT_COLUMNS as f32 + cell.x) as usize);
        let mut slot = 0;
        while slot < 2 {
            let label = *isthmus::reference(text_lines, ((names >> (slot * 16)) & 0xffff) as usize);
            let label_color = label.color.to_vec4();
            let text_alpha =
                text::line_alpha(label, placed_glyphs, glyphs, edges, pixel) * label_color.w;
            color = color * (1.0 - text_alpha) + label_color.truncate() * text_alpha;
            slot += 1;
        }
        (color * mask).extend(alpha)
    }
}

#[cfg(feature = "cpu")]
mod monitor {
    use super::{ForecastItem, HOURLY_STEP_HOURS, ORDINALS, TempestasPass, WeatherCondition};
    use crate::app::{AppUpdater, Background, send_update};
    use futures_util::StreamExt;
    use jiff::{
        civil::DateTime,
        tz::{Offset, TimeZone},
    };
    use reqwest::Client;
    use serde::{Deserialize, de::DeserializeOwned};
    use std::{array::from_fn, collections::HashMap, time::Duration};
    use tokio::{
        sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
        time::{sleep, timeout},
    };
    use tracing::warn;
    use zbus::{
        Connection, Proxy,
        proxy::Builder as ProxyBuilder,
        proxy::CacheProperties,
        zvariant::{OwnedObjectPath, OwnedValue, Value},
    };

    const WEATHER_FIELDS: &str = "temperature_2m,weather_code";
    const REFRESH_INTERVAL: Duration = Duration::from_mins(15);
    const RETRY_INTERVAL: Duration = Duration::from_secs(30);

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

    pub(super) fn start(
        timezones: Vec<String>,
        background: &Background,
        updater: AppUpdater,
        http: Client,
    ) {
        let (location_tx, locations) = mpsc::unbounded_channel();
        background.spawn(async move {
            if let Err(error) = stream_location(&location_tx).await {
                warn!(%error, "Location portal unavailable");
            }
            None
        });
        background.spawn(async move {
            refresh_loop(&http, &timezones, &updater, locations).await;
            None
        });
    }

    async fn stream_location(sender: &UnboundedSender<[f32; 2]>) -> Result<(), String> {
        const DESTINATION: &str = "org.freedesktop.portal.Desktop";
        const PORTAL_PATH: &str = "/org/freedesktop/portal/desktop";
        let connection = Connection::session().await.map_err(|error| error.to_string())?;
        let location: Proxy<'_> = ProxyBuilder::new(&connection)
            .destination(DESTINATION)
            .and_then(|builder| builder.path(PORTAL_PATH))
            .and_then(|builder| builder.interface("org.freedesktop.portal.Location"))
            .map_err(|error| error.to_string())?
            .cache_properties(CacheProperties::No)
            .build()
            .await
            .map_err(|error| error.to_string())?;
        let session_token = format!("cantus_{:x}", fastrand::u64(..));
        let session_path: OwnedObjectPath = location
            .call(
                "CreateSession",
                &HashMap::from([
                    ("session_handle_token", Value::from(session_token.as_str())),
                    ("accuracy", Value::from(2u32)),
                ]),
            )
            .await
            .map_err(|error| error.to_string())?;
        let session: Proxy<'_> = ProxyBuilder::new(&connection)
            .destination(DESTINATION)
            .and_then(|builder| builder.path(session_path.clone()))
            .and_then(|builder| builder.interface("org.freedesktop.portal.Session"))
            .map_err(|error| error.to_string())?
            .cache_properties(CacheProperties::No)
            .build()
            .await
            .map_err(|error| error.to_string())?;

        let updates = location
            .receive_signal("LocationUpdated")
            .await
            .map_err(|error| error.to_string())?;
        let request_token = format!("cantus_{:x}", fastrand::u64(..));
        let sender_name = connection
            .unique_name()
            .expect("session bus connection has no unique name")
            .trim_start_matches(':')
            .replace('.', "_");
        let request_path =
            format!("/org/freedesktop/portal/desktop/request/{sender_name}/{request_token}");
        let request: Proxy<'_> = ProxyBuilder::new(&connection)
            .destination(DESTINATION)
            .and_then(|builder| builder.path(request_path))
            .and_then(|builder| builder.interface("org.freedesktop.portal.Request"))
            .map_err(|error| error.to_string())?
            .cache_properties(CacheProperties::No)
            .build()
            .await
            .map_err(|error| error.to_string())?;
        let mut response = request
            .receive_signal("Response")
            .await
            .map_err(|error| error.to_string())?;
        let returned_path: OwnedObjectPath = location
            .call(
                "Start",
                &(
                    &session_path,
                    "",
                    HashMap::from([("handle_token", Value::from(request_token.as_str()))]),
                ),
            )
            .await
            .map_err(|error| error.to_string())?;
        if returned_path.as_str() != request.path().as_str() {
            return Err("location portal returned an unexpected request path".into());
        }
        let response = response
            .next()
            .await
            .ok_or("location portal closed without responding")?
            .body()
            .deserialize::<(u32, HashMap<String, OwnedValue>)>()
            .map_err(|error| error.to_string())?;
        if response.0 != 0 {
            return Err(format!("location portal rejected request ({})", response.0));
        }

        let mut updates = updates;
        while let Some(message) = updates.next().await {
            let (_, values) = message
                .body()
                .deserialize::<(OwnedObjectPath, HashMap<String, OwnedValue>)>()
                .map_err(|error| error.to_string())?;
            let coordinate = |name| {
                values
                    .get(name)
                    .ok_or_else(|| format!("location update omitted {name}"))?
                    .downcast_ref::<f64>()
                    .map(|value| value as f32)
                    .map_err(|error| error.to_string())
            };
            if sender
                .send([coordinate("Latitude")?, coordinate("Longitude")?])
                .is_err()
            {
                break;
            }
        }
        session
            .call::<_, _, ()>("Close", &())
            .await
            .map_err(|error| error.to_string())
    }

    async fn refresh_loop(
        http: &Client,
        timezones: &[String],
        updater: &AppUpdater,
        mut locations_rx: UnboundedReceiver<[f32; 2]>,
    ) {
        let mut locations = vec![None; timezones.len() + 1];
        if let Some(timezone) = TimeZone::system().iana_name() {
            match geocode(http, timezone).await {
                Ok(location) => locations[0] = Some(location),
                Err(error) => warn!(%error, timezone, "Failed to locate system timezone"),
            }
        } else {
            warn!("System timezone has no IANA name; waiting for the location portal");
        }
        loop {
            while let Ok(location) = locations_rx.try_recv() {
                locations[0] = Some(location);
            }
            let mut retry = false;
            let mut ready = Vec::with_capacity(locations.len());
            for (index, location) in locations.iter_mut().enumerate() {
                if index > 0 && location.is_none() {
                    *location = geocode(http, &timezones[index - 1])
                        .await
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
            let forecasts = match fetch(http, &ready).await {
                Ok(results) => ready
                    .into_iter()
                    .zip(results)
                    .map(|((index, _), forecast)| (index, forecast))
                    .collect(),
                Err(error) => {
                    retry = true;
                    warn!(%error, "Failed to refresh weather");
                    Vec::new()
                }
            };
            if !forecasts.is_empty()
                && !send_update(updater, move |app| {
                    if let Some(program) = app.render.program.as_mut()
                        && let Some(pass) = program.passes_mut().tempestas.as_mut()
                    {
                        for (index, forecast) in forecasts {
                            apply_forecast(pass, index, &forecast);
                        }
                    }
                })
            {
                break;
            }
            let interval = if retry { RETRY_INTERVAL } else { REFRESH_INTERVAL };
            match timeout(interval, locations_rx.recv()).await {
                Ok(Some(location)) => locations[0] = Some(location),
                Ok(None) => sleep(interval).await,
                Err(_) => {}
            }
        }
    }

    fn apply_forecast(pass: &mut TempestasPass, index: usize, forecast: &Forecast) {
        let Forecast {
            utc_offset_seconds,
            current,
            hourly,
            daily,
        } = forecast;
        // Index 0 is the local forecast; the rest fill in each configured world clock.
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
            let (description, conditions) = weather(hourly.weather_code[source]);
            pass.pill.hourly_conditions[index] = conditions;
            let (time, degrees) = (
                hourly.time[source].strftime("%H:%M").to_string(),
                format!("{:.0}°", hourly.temperature_2m[source]),
            );
            ForecastItem {
                hover_text: format!("{time} {description} {degrees}"),
                text: [time, degrees],
            }
        });
        pass.pill.hourly_conditions[0] = weather(current.weather_code).1;
        pass.daily = from_fn(|index| {
            let day = index + 1;
            let date = daily.sunrise[day];
            let number = date.day();
            let suffix = match number % 100 {
                11..=13 => "th",
                _ => ORDINALS[(number % 10) as usize],
            };
            let (description, conditions) = weather(daily.weather_code[day]);
            pass.pill.daily_conditions[index] = conditions;
            let range = format!(
                "{:.0}°/{:.0}°",
                daily.temperature_2m_max[day], daily.temperature_2m_min[day]
            );
            ForecastItem {
                hover_text: format!("{}{suffix} {description} {range}", date.strftime("%A %-d")),
                text: [date.strftime("%a").to_string(), range],
            }
        });
        pass.details = format!(
            "{} · Humidity {}% · Wind {:.0} km/h",
            weather(current.weather_code).0,
            current.relative_humidity_2m,
            current.wind_speed_10m
        );
    }

    async fn geocode(http: &Client, timezone: &str) -> Result<[f32; 2], String> {
        let city = timezone.rsplit('/').next().unwrap_or(timezone).replace('_', " ");
        let query: String = form_urlencoded::byte_serialize(city.as_bytes()).collect();
        let results: SearchResults = get_json(
            http,
            format!("https://geocoding-api.open-meteo.com/v1/search?name={query}&count=10"),
        )
        .await?;
        let place = results
            .results
            .iter()
            .find(|place| place.timezone == timezone)
            .or_else(|| results.results.first())
            .ok_or_else(|| format!("no place found for {city}"))?;
        Ok([place.latitude, place.longitude])
    }

    async fn fetch(http: &Client, locations: &[(usize, [f32; 2])]) -> Result<Vec<Forecast>, String> {
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
            get_json(http, url).await.map(|forecast| vec![forecast])
        } else {
            get_json(http, url).await
        }
    }

    async fn get_json<T: DeserializeOwned>(http: &Client, url: String) -> Result<T, String> {
        http.get(url)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|error| error.to_string())?
            .json()
            .await
            .map_err(|error| error.to_string())
    }
}

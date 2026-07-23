use crate::{
    AppUpdater, Rect,
    render::{status::GAP, text_render::TextStyle},
    send_update,
};
use arrayvec::ArrayString;
use cantus_shared::{
    StatusPill, WEATHER_CALENDAR_ARROW_RADIUS, WEATHER_CALENDAR_ARROW_X,
    WEATHER_CALENDAR_EXTENSION, WEATHER_CALENDAR_GRID_TOP, WEATHER_CALENDAR_ROW_HEIGHT,
    WEATHER_CALENDAR_TITLE_Y, WeatherCondition, WeatherPill, approach, smoothstep,
};
use glam::{FloatExt, Vec2, vec2};
use jiff::{Span, Zoned, civil::DateTime};
use serde::Deserialize;
use std::{f32::consts::PI, fmt::Write, thread, time::Duration};
use tracing::warn;

pub const WIDTH: f32 = 310.0;
const WEATHER_FIELDS: &str =
    "cloud_cover,visibility,precipitation_probability,rain,showers,snowfall,weather_code";
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let y = (y - 9.0).max(0.0);
    let delay = y * 0.0016 + (y - 87.0).max(0.0) * 0.0014;
    smoothstep(delay, 1.0, expansion)
}

fn calendar_cell(cell: usize) -> Vec2 {
    let row = (cell / 7) as f32;
    vec2(
        (cell % 7) as f32 * WIDTH / 7.0 + WIDTH / 14.0,
        WEATHER_CALENDAR_GRID_TOP + row * WEATHER_CALENDAR_ROW_HEIGHT,
    )
}

fn sun_position(hour: f32, [sunrise, sunset]: [f32; 2]) -> [f32; 2] {
    let daylight = sunset - sunrise;
    if (sunrise..=sunset).contains(&hour) {
        let phase = (hour - sunrise) / daylight;
        [phase, (phase * PI).sin()]
    } else {
        let night = 24.0 - daylight;
        let phase = if hour < sunrise {
            (hour + 24.0 - sunset) / night
        } else {
            (hour - sunset) / night
        };
        [f32::from(hour >= sunset), -(phase * PI).sin()]
    }
}

fn rect(status: &StatusPill, height: f32) -> Rect {
    Rect::pill(status.x - WIDTH - GAP, WIDTH, height)
}

fn calendar_rect(status: &StatusPill, height: f32) -> Rect {
    let mut calendar = rect(status, height);
    calendar.y0 = calendar.y1;
    calendar.y1 += WEATHER_CALENDAR_EXTENSION;
    calendar
}

pub struct Weather {
    temperature: String,
    sun_hours: [f32; 2],
    conditions: [WeatherCondition; 3],
    details: String,
    calendar_expansion: f32,
    month_offset: i32,
}

impl Weather {
    pub fn new([latitude, longitude]: [f32; 2], updater: AppUpdater) -> Self {
        thread::spawn(move || {
            while ureq::get(format!(
                "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current=temperature_2m,relative_humidity_2m,wind_speed_10m,{WEATHER_FIELDS}&hourly={WEATHER_FIELDS}&forecast_hours=7&daily=sunrise,sunset&temperature_unit=celsius&timezone=auto&forecast_days=1"
            ))
            .call()
            .map_err(|error| error.to_string())
            .and_then(|mut response| {
                serde_json::from_reader::<_, Forecast>(response.body_mut().as_reader())
                    .map_err(|error| error.to_string())
            })
            .map_or_else(
                |error| {
                    warn!(%error, "Failed to refresh weather");
                    true
                },
                |forecast| {
                    send_update(&updater, move |app| {
                        if let Some(weather) = &mut app.weather {
                            weather.apply_forecast(&forecast);
                        }
                    })
                },
            ) {
                thread::sleep(Duration::from_mins(15));
            }
        });
        Self {
            temperature: "--.-°C".into(),
            sun_hours: [6.0, 18.0],
            conditions: [WeatherCondition {
                cloud: 0.15,
                ..WeatherCondition::default()
            }; 3],
            details: "Weather unavailable".into(),
            calendar_expansion: 0.0,
            month_offset: 0,
        }
    }

    pub fn scene(
        &mut self,
        status: &StatusPill,
        height: f32,
        mouse: Vec2,
        mouse_active: bool,
        dt: f32,
    ) -> (WeatherPill, ArrayString<64>) {
        let hovered = self.interaction_rect(status, height).contains(mouse);
        approach(
            &mut self.calendar_expansion,
            f32::from(mouse_active && hovered),
            dt.min(1.0 / 30.0) * 3.0,
        );

        let time = Zoned::now();
        let hour = f32::from(time.hour())
            + f32::from(time.minute()) / 60.0
            + f32::from(time.second()) / 3600.0;
        let today = time.date();
        let cell = today.first_of_month().weekday().to_monday_zero_offset() as usize
            + today.day() as usize
            - 1;
        let marker = calendar_cell(cell);
        let marker_reveal =
            reveal_progress(self.calendar_expansion, marker.y) * f32::from(self.month_offset == 0);
        let pill = WeatherPill {
            x: rect(status, height).x0,
            width: WIDTH,
            sun: sun_position(hour, self.sun_hours),
            today: vec2(WIDTH * 0.5, 0.0).lerp(marker, marker_reveal),
            calendar_expansion: self.calendar_expansion,
            conditions: self.conditions,
            padding: 0.0,
        };
        let clock = time.strftime("%a %d %b  %H:%M:%S");
        let mut label = ArrayString::new();
        write!(label, "{}   {clock}", self.temperature).unwrap();
        (pill, label)
    }

    pub fn interaction_rect(&self, status: &StatusPill, height: f32) -> Rect {
        let mut area = rect(status, height);
        area.y1 += WEATHER_CALENDAR_EXTENSION * f32::from(self.calendar_expansion > 0.0);
        area
    }

    pub fn navigate_calendar(&mut self, position: Vec2, status: &StatusPill, height: f32) -> bool {
        if self.calendar_expansion < 0.5 {
            return false;
        }
        let calendar = calendar_rect(status, height);
        let center_x = (calendar.x0 + calendar.x1) * 0.5;
        let title_y = calendar.y0 + WEATHER_CALENDAR_TITLE_Y;
        let hit = |x, size| Rect::from_center(vec2(x, title_y), size).contains(position);
        let arrow = Vec2::splat(WEATHER_CALENDAR_ARROW_RADIUS);

        self.month_offset = if hit(center_x, vec2(90.0, 16.0)) {
            0
        } else if hit(calendar.x0 + WEATHER_CALENDAR_ARROW_X, arrow) {
            (self.month_offset - 1).clamp(-1200, 1200)
        } else if hit(calendar.x1 - WEATHER_CALENDAR_ARROW_X, arrow) {
            (self.month_offset + 1).clamp(-1200, 1200)
        } else {
            return false;
        };
        true
    }

    pub fn calendar_labels(
        &self,
        status: &StatusPill,
        height: f32,
        mut draw: impl FnMut(&str, Vec2, f32, TextStyle),
    ) {
        if self.calendar_expansion <= 0.0 {
            return;
        }
        let today = Zoned::now().date();
        let month = today
            .first_of_month()
            .saturating_add(Span::new().months(self.month_offset));
        let calendar = calendar_rect(status, height);
        let center_x = (calendar.x0 + calendar.x1) * 0.5;
        let origin = vec2(center_x, calendar.y0);
        let mut label = |text: &str, target: Vec2, alpha, style| {
            let eased = reveal_progress(self.calendar_expansion, target.y - calendar.y0);
            draw(text, origin.lerp(target, eased), alpha * eased, style);
        };
        label(
            &self.details,
            origin + vec2(0.0, 9.0),
            1.0,
            TextStyle::DETAILS,
        );

        let title = month.strftime("%B %Y").to_string();
        let title_position = vec2(center_x, calendar.y0 + WEATHER_CALENDAR_TITLE_Y);
        label(&title, title_position, 1.0, TextStyle::PRIMARY);

        let grid_start =
            month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
        for (column, weekday) in WEEKDAYS.into_iter().enumerate() {
            let position = vec2(calendar.x0 + calendar_cell(column).x, calendar.y0 + 69.0);
            label(weekday, position, 0.75, TextStyle::DETAILS);
        }

        for cell in 0..42 {
            let date = grid_start.saturating_add(Span::new().days(cell as i64));
            let mut text = ArrayString::<2>::new();
            write!(text, "{}", date.day()).unwrap();
            let alpha = 0.32 + f32::from(date.month() == month.month()) * 0.68;
            let style = if date == today {
                TextStyle::TODAY
            } else {
                TextStyle::PRIMARY
            };
            let position = vec2(calendar.x0, calendar.y0) + calendar_cell(cell);
            label(&text, position, alpha, style);
        }
    }

    fn apply_forecast(&mut self, forecast: &Forecast) {
        let raw = &forecast.current;
        self.temperature = format!("{:.1}°C", raw.temperature_2m);
        self.sun_hours = [forecast.daily.sunrise[0], forecast.daily.sunset[0]]
            .map(|time| f32::from(time.hour()) + f32::from(time.minute()) / 60.0);
        self.conditions = [
            conditions(&raw.conditions),
            conditions(&forecast.hourly.at(3)),
            conditions(&forecast.hourly.at(6)),
        ];
        self.details = format!(
            "{} · Humidity {}% · Wind {:.0} km/h",
            weather_name(raw.conditions.weather_code),
            raw.relative_humidity_2m,
            raw.wind_speed_10m
        );
    }
}

const fn weather_name(code: u8) -> &'static str {
    match code {
        0 => "Clear skies",
        1 => "Mostly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 | 48 => "Foggy",
        51 | 53 | 55 => "Drizzle",
        56 | 57 => "Freezing drizzle",
        61 | 63 | 65 => "Rain",
        66 | 67 => "Freezing rain",
        71 | 73 | 75 | 77 => "Snow",
        80..=82 => "Rain showers",
        85 | 86 => "Snow showers",
        95..=99 => "Thunderstorms",
        _ => "Unknown weather",
    }
}

#[derive(Deserialize)]
struct Forecast {
    current: Current,
    hourly: RawConditions<[f32; 7], [u8; 7]>,
    daily: Daily,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: f32,
    relative_humidity_2m: u8,
    wind_speed_10m: f32,
    #[serde(flatten)]
    conditions: RawConditions<f32, u8>,
}

#[derive(Deserialize)]
struct RawConditions<T, C> {
    cloud_cover: T,
    visibility: T,
    precipitation_probability: T,
    rain: T,
    showers: T,
    snowfall: T,
    weather_code: C,
}

impl RawConditions<[f32; 7], [u8; 7]> {
    const fn at(&self, index: usize) -> RawConditions<f32, u8> {
        RawConditions {
            cloud_cover: self.cloud_cover[index],
            visibility: self.visibility[index],
            precipitation_probability: self.precipitation_probability[index],
            rain: self.rain[index],
            showers: self.showers[index],
            snowfall: self.snowfall[index],
            weather_code: self.weather_code[index],
        }
    }
}

#[derive(Deserialize)]
struct Daily {
    sunrise: [DateTime; 1],
    sunset: [DateTime; 1],
}

fn conditions(raw: &RawConditions<f32, u8>) -> WeatherCondition {
    let code = raw.weather_code;
    let intensity = match code {
        51 | 56 | 61 | 66 | 71 | 80 | 85 | 95 | 96 => 0.3,
        53 | 63 | 73 | 81 => 0.55,
        55 | 57 | 65 | 67 | 75 | 77 | 82 | 86 | 99 => 0.9,
        _ => 0.0,
    } * raw.precipitation_probability
        / 100.0;
    let amount = |value: f32, scale: f32, expected| {
        (value / scale)
            .saturate()
            .max(intensity * f32::from(expected))
    };
    WeatherCondition {
        cloud: raw.cloud_cover / 100.0,
        fog: (1.0 - raw.visibility / 10_000.0)
            .saturate()
            .max(f32::from(matches!(code, 45 | 48))),
        lightning: f32::from(matches!(code, 95..=99)),
        rain: amount(
            raw.rain + raw.showers,
            4.0,
            matches!(code, 51..=67 | 80..=82 | 95..=99),
        ),
        snow: amount(raw.snowfall, 2.0, matches!(code, 71..=77 | 85..=86)),
        hail: intensity * f32::from(matches!(code, 56 | 57 | 66 | 67 | 96 | 99)),
    }
}

use crate::{
    AppUpdater, Rect,
    render::{status::GAP, text_render::TextStyle},
    send_update,
};
use arrayvec::ArrayString;
use cantus_shared::{
    StatusPill, WeatherCondition, WeatherLayout, WeatherPill, approach, smoothstep,
};
use glam::{FloatExt, Vec2, vec2};
use jiff::{Span, Zoned, civil::DateTime};
use serde::Deserialize;
use std::{f32::consts::PI, fmt::Write, thread, time::Duration};
use tracing::warn;

pub const WIDTH: f32 = WeatherLayout::WIDTH;
const WEATHER_FIELDS: &str =
    "cloud_cover,visibility,precipitation_probability,rain,showers,snowfall,weather_code";
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const HOURLY_SAMPLES: [usize; 6] = [0, 4, 8, 12, 16, 20];

#[derive(Default)]
struct ForecastItem {
    text: [String; 2],
    conditions: WeatherCondition,
    hour: f32,
}

fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let delay = 0.5 + ((y - 9.0).max(0.0) * 0.0008).min(0.18);
    smoothstep(delay, (delay + 0.24).min(0.94), expansion)
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

fn calendar_origin(status: &StatusPill, height: f32) -> Vec2 {
    let normal = rect(status, height);
    vec2(WeatherLayout::expanded_x(normal.x0, 1.0), normal.y1)
}

pub struct Weather {
    temperature: String,
    sun_hours: [f32; 2],
    conditions: [WeatherCondition; 3],
    details: String,
    hourly: [ForecastItem; 6],
    daily: [ForecastItem; 5],
    calendar_expansion: f32,
    month_offset: i32,
    month_animation: f32,
}

impl Weather {
    pub fn new([latitude, longitude]: [f32; 2], updater: AppUpdater) -> Self {
        thread::spawn(move || {
            while ureq::get(format!(
                "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current=temperature_2m,relative_humidity_2m,wind_speed_10m,{WEATHER_FIELDS}&hourly=temperature_2m,{WEATHER_FIELDS}&forecast_hours=24&daily=weather_code,temperature_2m_max,temperature_2m_min,sunrise,sunset&temperature_unit=celsius&timezone=auto&forecast_days=6"
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
            hourly: Default::default(),
            daily: Default::default(),
            calendar_expansion: 0.0,
            month_offset: 0,
            month_animation: 0.0,
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
        approach(&mut self.month_animation, 0.0, dt.min(1.0 / 30.0) * 7.0);

        let time = Zoned::now();
        let hour = f32::from(time.hour())
            + f32::from(time.minute()) / 60.0
            + f32::from(time.second()) / 3600.0;
        let today = time.date();
        let cell = today.first_of_month().weekday().to_monday_zero_offset() as usize
            + today.day() as usize
            - 1;
        let marker = WeatherLayout::cell(cell);
        let marker_reveal =
            reveal_progress(self.calendar_expansion, marker.y) * f32::from(self.month_offset == 0);
        let pill = WeatherPill {
            x: rect(status, height).x0,
            width: WIDTH,
            sun: sun_position(hour, self.sun_hours),
            sun_hours: self.sun_hours,
            hourly_times: self.hourly.each_ref().map(|item| item.hour),
            today: vec2(WIDTH * 0.5, 0.0).lerp(marker, marker_reveal),
            calendar_expansion: self.calendar_expansion,
            conditions: self.conditions,
            hourly: self.hourly.each_ref().map(|item| item.conditions),
            daily: self.daily.each_ref().map(|item| item.conditions),
            padding: 0.0,
        };
        let clock = time.strftime("%a %d %b  %H:%M:%S");
        let mut label = ArrayString::new();
        write!(label, "{}   {clock}", self.temperature).unwrap();
        (pill, label)
    }

    pub fn interaction_rect(&self, status: &StatusPill, height: f32) -> Rect {
        let mut area = rect(status, height);
        if self.calendar_expansion > 0.0 {
            area.x0 = WeatherLayout::expanded_x(area.x0, 1.0);
            area.x1 = area.x0 + WeatherLayout::popup_size(1.0).x;
            area.y1 += WeatherLayout::EXTENSION;
        }
        area
    }

    pub fn navigate_calendar(&mut self, position: Vec2, status: &StatusPill, height: f32) -> bool {
        if self.calendar_expansion < 0.5 {
            return false;
        }
        let local = position - calendar_origin(status, height);
        let hit = |center, size| Rect::from_center(center, size).contains(local);
        let reveal = WeatherLayout::header_reveal(self.calendar_expansion);

        let step = [-1.0, 1.0].into_iter().find(|&side| {
            hit(
                WeatherLayout::arrow(side, reveal),
                Vec2::splat(WeatherLayout::ARROW_RADIUS),
            )
        });
        let new_offset = if hit(WeatherLayout::TITLE, WeatherLayout::TITLE_HALF_SIZE) {
            0
        } else if let Some(step) = step {
            (self.month_offset + step as i32).clamp(-1200, 1200)
        } else {
            return false;
        };
        if new_offset != self.month_offset {
            self.month_animation = (new_offset - self.month_offset).signum() as f32;
            self.month_offset = new_offset;
        }
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
        let origin = calendar_origin(status, height);
        let mut label = |text: &str, target: Vec2, alpha, style| {
            let eased = reveal_progress(self.calendar_expansion, target.y);
            draw(
                text,
                origin + WeatherLayout::pill_center(height).lerp(target, eased),
                alpha * eased,
                style,
            );
        };
        label(
            &self.details,
            WeatherLayout::DETAILS,
            1.0,
            TextStyle::DETAILS,
        );

        for (row, forecasts) in [&self.hourly[..], &self.daily[..]].into_iter().enumerate() {
            let reveal = WeatherLayout::forecast_reveal(self.calendar_expansion, row as f32);
            for (column, forecast) in forecasts.iter().enumerate() {
                for (line, text) in forecast.text.iter().enumerate() {
                    let target = WeatherLayout::forecast_item(
                        height,
                        row as f32,
                        column,
                        forecasts.len(),
                        line,
                    );
                    label(text, target, reveal, TextStyle::DETAILS);
                }
            }
        }

        let month_eased = smoothstep(1.0, 0.0, self.month_animation.abs());
        let month_offset = vec2(
            (1.0 - month_eased) * self.month_animation.signum() * WeatherLayout::MONTH_SLIDE,
            0.0,
        );
        label(
            &month.strftime("%B %Y").to_string(),
            WeatherLayout::TITLE + month_offset,
            month_eased,
            TextStyle::CALENDAR_TITLE,
        );

        let grid_start =
            month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
        for (column, weekday) in WEEKDAYS.iter().enumerate() {
            label(
                weekday,
                WeatherLayout::weekday(column),
                0.75,
                TextStyle::DETAILS,
            );
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
            let position = WeatherLayout::cell(cell) + month_offset;
            label(&text, position, alpha * month_eased, style);
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
        for (item, &source) in self.hourly.iter_mut().zip(&HOURLY_SAMPLES) {
            let time = forecast.hourly.time[source];
            item.text = [
                time.strftime("%H:%M").to_string(),
                format!("{:.0}°", forecast.hourly.temperature_2m[source]),
            ];
            item.hour = f32::from(time.hour()) + f32::from(time.minute()) / 60.0;
            item.conditions = conditions(&forecast.hourly.at(source));
        }
        for (day, item) in self.daily.iter_mut().enumerate() {
            let day = day + 1;
            item.text = [
                forecast.daily.sunrise[day].strftime("%a").to_string(),
                format!(
                    "{:.0}°/{:.0}°",
                    forecast.daily.temperature_2m_max[day], forecast.daily.temperature_2m_min[day]
                ),
            ];
            item.conditions = coded_conditions(forecast.daily.weather_code[day]);
        }
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
    hourly: Hourly,
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

#[derive(Deserialize)]
struct Hourly {
    time: [DateTime; 24],
    temperature_2m: [f32; 24],
    #[serde(flatten)]
    conditions: RawConditions<[f32; 24], [u8; 24]>,
}

impl Hourly {
    const fn at(&self, index: usize) -> RawConditions<f32, u8> {
        RawConditions {
            cloud_cover: self.conditions.cloud_cover[index],
            visibility: self.conditions.visibility[index],
            precipitation_probability: self.conditions.precipitation_probability[index],
            rain: self.conditions.rain[index],
            showers: self.conditions.showers[index],
            snowfall: self.conditions.snowfall[index],
            weather_code: self.conditions.weather_code[index],
        }
    }
}

#[derive(Deserialize)]
struct Daily {
    weather_code: [u8; 6],
    temperature_2m_max: [f32; 6],
    temperature_2m_min: [f32; 6],
    sunrise: [DateTime; 6],
    sunset: [DateTime; 6],
}

fn coded_conditions(code: u8) -> WeatherCondition {
    let intensity = match code {
        51 | 56 | 61 | 66 | 71 | 80 | 85 | 95 | 96 => 0.3,
        53 | 63 | 73 | 81 => 0.55,
        55 | 57 | 65 | 67 | 75 | 77 | 82 | 86 | 99 => 0.9,
        _ => 0.0,
    };
    WeatherCondition {
        cloud: [0.05, 0.25, 0.55]
            .get(code as usize)
            .copied()
            .unwrap_or(0.9),
        fog: f32::from(matches!(code, 45 | 48)),
        lightning: f32::from(matches!(code, 95..=99)),
        rain: intensity * f32::from(matches!(code, 51..=67 | 80..=82 | 95..=99)),
        snow: intensity * f32::from(matches!(code, 71..=77 | 85..=86)),
        hail: intensity * f32::from(matches!(code, 56 | 57 | 66 | 67 | 96 | 99)),
    }
}

fn conditions(raw: &RawConditions<f32, u8>) -> WeatherCondition {
    let mut result = coded_conditions(raw.weather_code);
    let probability = raw.precipitation_probability / 100.0;
    result.cloud = raw.cloud_cover / 100.0;
    result.fog = result.fog.max((1.0 - raw.visibility / 10_000.0).saturate());
    result.rain = (result.rain * probability).max(((raw.rain + raw.showers) / 4.0).saturate());
    result.snow = (result.snow * probability).max((raw.snowfall / 2.0).saturate());
    result.hail *= probability;
    result
}

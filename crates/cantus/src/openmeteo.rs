use crate::render::tempestas::WeatherCondition;
use crate::{AppUpdater, send_update};
use jiff::civil::DateTime;
use serde::Deserialize;
use std::{thread, time::Duration};
use tracing::warn;

const WEATHER_FIELDS: &str = "temperature_2m,weather_code";

/// Fetches and applies a forecast every 15 minutes, retrying every 30 seconds until the first fetch succeeds.
pub fn spawn_refresh_loop(latitude: f32, longitude: f32, updater: AppUpdater) {
    thread::spawn(move || {
        let mut has_data = false;
        while match fetch(latitude, longitude) {
            Ok(forecast) => {
                has_data = true;
                send_update(&updater, move |app| {
                    if let Some(tempestas) = app.render.program().passes_mut().tempestas.as_mut() {
                        tempestas.apply_forecast(&forecast);
                    }
                })
            }
            Err(error) => {
                warn!(%error, "Failed to refresh weather");
                true
            }
        } {
            thread::sleep(if has_data {
                Duration::from_mins(15)
            } else {
                Duration::from_secs(30)
            });
        }
    });
}

fn fetch(latitude: f32, longitude: f32) -> Result<Forecast, String> {
    ureq::get(format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current={WEATHER_FIELDS},relative_humidity_2m,wind_speed_10m&hourly={WEATHER_FIELDS}&forecast_hours=24&daily=weather_code,temperature_2m_max,temperature_2m_min,sunrise,sunset&temperature_unit=celsius&timezone=auto&forecast_days=6"
    ))
    .call()
    .map_err(|error| error.to_string())
    .and_then(|mut response| {
        serde_json::from_reader(response.body_mut().as_reader()).map_err(|error| error.to_string())
    })
}

#[derive(Deserialize)]
pub struct Forecast {
    pub utc_offset_seconds: i32,
    pub current: Current,
    pub hourly: Hourly,
    pub daily: Daily,
}

#[derive(Deserialize)]
pub struct Current {
    pub weather_code: u8,
    pub temperature_2m: f32,
    pub relative_humidity_2m: u8,
    pub wind_speed_10m: f32,
}

#[derive(Deserialize)]
pub struct Hourly {
    pub weather_code: [u8; 24],
    pub time: [DateTime; 24],
    pub temperature_2m: [f32; 24],
}

#[derive(Deserialize)]
pub struct Daily {
    pub weather_code: [u8; 6],
    pub temperature_2m_max: [f32; 6],
    pub temperature_2m_min: [f32; 6],
    pub sunrise: [DateTime; 6],
    pub sunset: [DateTime; 6],
}

/// Derives a condition from a WMO weather code.
pub const fn coded_conditions(code: u8) -> WeatherCondition {
    let intensity = weather(code).1;
    WeatherCondition {
        fog: intensity.fog,
        cloud: intensity.cloud,
        rain: intensity.rain,
        snow: intensity.snow,
        lightning: intensity.lightning,
        hail: intensity.hail,
    }
}

macro_rules! weather_codes {
    ($($code:literal => $name:literal { $($field:ident: $value:literal),* };)*) => {
        const fn weather(code: u8) -> (&'static str, WeatherIntensity) {
            match code {
                $($code => ($name, WeatherIntensity {
                    $($field: $value,)*
                    ..WeatherIntensity::CLEAR
                }),)*
                _ => ("Unknown weather", WeatherIntensity::CLEAR),
            }
        }
    };
}

pub const fn weather_code(code: u8) -> &'static str {
    weather(code).0
}

struct WeatherIntensity {
    fog: f32,
    cloud: f32,
    rain: f32,
    snow: f32,
    lightning: f32,
    hail: f32,
}

impl WeatherIntensity {
    const CLEAR: Self = Self {
        fog: 0.0,
        cloud: 0.0,
        rain: 0.0,
        snow: 0.0,
        lightning: 0.0,
        hail: 0.0,
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

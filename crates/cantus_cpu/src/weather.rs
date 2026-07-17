use crate::{AppUpdater, model::Rect, send_update, status::GAP};
use cantus_shared::{StatusPill, WeatherCondition, WeatherPill};
use glam::Vec4;
use jiff::Zoned;
use serde::Deserialize;
use std::{error::Error, f32::consts::PI, thread, time::Duration};
use tracing::warn;

pub const WIDTH: f32 = 310.0;
const WEATHER_FIELDS: &str = "cloud_cover,visibility,precipitation_probability,rain,showers,snowfall,wind_speed_10m,weather_code";

pub fn rect(status: StatusPill, height: f32) -> Rect {
    Rect::pill(status.x - WIDTH - GAP, WIDTH, height)
}

pub struct Weather {
    temperature: Option<f32>,
    sunrise: f32,
    sunset: f32,
    conditions: [WeatherCondition; 3],
}

impl Weather {
    pub fn new(location: [f32; 2], updater: AppUpdater) -> Self {
        thread::spawn(move || {
            while fetch(location).map_or_else(
                |error| {
                    warn!(%error, "Failed to refresh weather");
                    true
                },
                |weather| send_update(&updater, move |app| app.weather = weather),
            ) {
                thread::sleep(Duration::from_mins(15));
            }
        });
        Self {
            temperature: None,
            sunrise: 6.0,
            sunset: 18.0,
            conditions: [condition([0.15, 0.0, 0.1, 0.0], [0.0; 4]); 3],
        }
    }

    pub fn scene(&self, x: f32) -> (WeatherPill, String) {
        let time = Zoned::now();
        let hour = f32::from(time.hour())
            + f32::from(time.minute()) / 60.0
            + f32::from(time.second()) / 3600.0;
        let sun_phase = (hour - self.sunrise) / (self.sunset - self.sunrise);
        let pill = WeatherPill {
            x,
            width: WIDTH,
            sun: [sun_phase.clamp(0.0, 1.0), (sun_phase * PI).sin()],
            conditions: self.conditions,
        };
        let time = time.strftime("%a %d %b  %H:%M:%S");
        let label = self.temperature.map_or_else(
            || format!("--.-°C   {time}"),
            |temperature| format!("{temperature:.1}°C   {time}"),
        );
        (pill, label)
    }
}

const fn condition(atmosphere: [f32; 4], precipitation: [f32; 4]) -> WeatherCondition {
    WeatherCondition {
        atmosphere: Vec4::from_array(atmosphere),
        precipitation: Vec4::from_array(precipitation),
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
    #[serde(flatten)]
    conditions: RawConditions<f32, u8>,
}

#[derive(Clone, Copy, Deserialize)]
struct RawConditions<T, C> {
    cloud_cover: T,
    visibility: T,
    precipitation_probability: T,
    rain: T,
    showers: T,
    snowfall: T,
    wind_speed_10m: T,
    weather_code: C,
}

#[derive(Deserialize)]
struct Daily {
    sunrise: [String; 1],
    sunset: [String; 1],
}

fn hour(value: &str) -> Option<f32> {
    let (_, time) = value.split_once('T')?;
    let (hour, minute) = time.split_once(':')?;
    Some(hour.parse::<f32>().ok()? + minute.parse::<f32>().ok()? / 60.0)
}

fn conditions(raw: RawConditions<f32, u8>) -> WeatherCondition {
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
            .clamp(0.0, 1.0)
            .max(intensity * f32::from(expected))
    };
    condition(
        [
            raw.cloud_cover / 100.0,
            (1.0 - raw.visibility / 10_000.0)
                .clamp(0.0, 1.0)
                .max(f32::from(matches!(code, 45 | 48))),
            (raw.wind_speed_10m / 60.0).clamp(0.0, 1.0),
            f32::from(matches!(code, 95..=99)),
        ],
        [
            amount(raw.rain, 4.0, matches!(code, 51..=67 | 95..=99)),
            amount(raw.showers, 4.0, matches!(code, 80..=82)),
            amount(raw.snowfall, 2.0, matches!(code, 71..=77 | 85..=86)),
            intensity * f32::from(matches!(code, 56 | 57 | 66 | 67 | 96 | 99)),
        ],
    )
}

fn fetch([latitude, longitude]: [f32; 2]) -> Result<Weather, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current=temperature_2m,{WEATHER_FIELDS}&hourly={WEATHER_FIELDS}&forecast_hours=7&daily=sunrise,sunset&temperature_unit=celsius&timezone=auto&forecast_days=1"
    );
    let mut response = ureq::get(url).call()?;
    let forecast: Forecast = serde_json::from_reader(response.body_mut().as_reader())?;
    let hourly = |index: usize| {
        conditions(RawConditions {
            cloud_cover: forecast.hourly.cloud_cover[index],
            visibility: forecast.hourly.visibility[index],
            precipitation_probability: forecast.hourly.precipitation_probability[index],
            rain: forecast.hourly.rain[index],
            showers: forecast.hourly.showers[index],
            snowfall: forecast.hourly.snowfall[index],
            wind_speed_10m: forecast.hourly.wind_speed_10m[index],
            weather_code: forecast.hourly.weather_code[index],
        })
    };
    Ok(Weather {
        temperature: Some(forecast.current.temperature_2m),
        sunrise: hour(&forecast.daily.sunrise[0]).unwrap_or(6.0),
        sunset: hour(&forecast.daily.sunset[0]).unwrap_or(18.0),
        conditions: [
            conditions(forecast.current.conditions),
            hourly(3),
            hourly(6),
        ],
    })
}

use crate::{AppUpdater, send_update};
use cantus_shared::WeatherPill;
use jiff::Zoned;
use serde::Deserialize;
use std::{error::Error, f32::consts::PI, thread, time::Duration};
use tracing::warn;

pub const WIDTH: f32 = 310.0;

#[derive(Clone, Copy, Default)]
pub struct Weather {
    temperature: Option<f32>,
    sunrise: f32,
    sunset: f32,
    conditions: [[f32; 3]; 3],
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
            sunrise: 6.0,
            sunset: 18.0,
            conditions: [[0.15, 0.0, 0.0], [0.0; 3], [0.0; 3]],
            ..Self::default()
        }
    }

    pub fn scene(&self, x: f32) -> (WeatherPill, String) {
        let time = Zoned::now();
        let weather = *self;
        let now = f32::from(time.hour())
            + f32::from(time.minute()) / 60.0
            + f32::from(time.second()) / 3600.0;
        let sun_phase = (now - weather.sunrise) / (weather.sunset - weather.sunrise);
        let pill = WeatherPill {
            x,
            width: WIDTH,
            sun: [sun_phase.clamp(0.0, 1.0), (sun_phase * PI).sin()],
            conditions: weather.conditions,
        };
        let time = time.strftime("%a %d %b  %H:%M:%S").to_string();
        let label = weather.temperature.map_or_else(
            || format!("--.-°C   {time}"),
            |temperature| format!("{temperature:.1}°C   {time}"),
        );
        (pill, label)
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
    cloud_cover: f32,
    precipitation: f32,
    weather_code: u8,
}

#[derive(Deserialize)]
struct Hourly {
    cloud_cover: [f32; 7],
    precipitation: [f32; 7],
    weather_code: [u8; 7],
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

fn conditions(cloud: f32, precipitation: f32, code: u8) -> [f32; 3] {
    [
        cloud / 100.0,
        (precipitation / 2.0).clamp(0.0, 1.0),
        f32::from(matches!(code, 45 | 48)),
    ]
}

fn fetch([latitude, longitude]: [f32; 2]) -> Result<Weather, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current=temperature_2m,cloud_cover,precipitation,weather_code&hourly=cloud_cover,precipitation,weather_code&forecast_hours=7&daily=sunrise,sunset&temperature_unit=celsius&timezone=auto&forecast_days=1"
    );
    let mut response = ureq::get(url).call()?;
    let forecast: Forecast = serde_json::from_reader(response.body_mut().as_reader())?;
    let current = forecast.current;
    let hourly = |index| {
        conditions(
            forecast.hourly.cloud_cover[index],
            forecast.hourly.precipitation[index],
            forecast.hourly.weather_code[index],
        )
    };
    Ok(Weather {
        temperature: Some(current.temperature_2m),
        sunrise: hour(&forecast.daily.sunrise[0]).unwrap_or(6.0),
        sunset: hour(&forecast.daily.sunset[0]).unwrap_or(18.0),
        conditions: [
            conditions(
                current.cloud_cover,
                current.precipitation,
                current.weather_code,
            ),
            hourly(3),
            hourly(6),
        ],
    })
}

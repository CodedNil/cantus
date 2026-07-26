//! Open-Meteo API: fetching, the raw response shape, weather codes, and deriving conditions.

use crate::{AppUpdater, send_update};
use cantus_gpu::tempo::WeatherCondition;
use jiff::civil::DateTime;
use serde::Deserialize;
use std::{thread, time::Duration};
use strum::{FromRepr, IntoStaticStr};
use tracing::warn;

const WEATHER_FIELDS: &str = "temperature_2m,weather_code";

/// Fetches and applies a forecast every 15 minutes.
pub fn spawn_refresh_loop(latitude: f32, longitude: f32, updater: AppUpdater) {
    thread::spawn(move || {
        while fetch(latitude, longitude).map_or_else(
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
pub fn coded_conditions(code: u8) -> WeatherCondition {
    let [fog, cloud, rain, snow, lightning, hail] =
        WeatherCode::from_repr(code).map_or([0.0, 0.9, 0.0, 0.0, 0.0, 0.0], WeatherCode::values);
    let pack = |value: f32, max: f32| (value.clamp(0.0, 1.0) * max) as u32;
    let mut result = WeatherCondition::from_bits(0);
    result.set_fog_raw(pack(fog, 15.0));
    result.set_cloud_raw(pack(cloud, 127.0));
    result.set_rain_raw(pack(rain, 255.0));
    result.set_snow_raw(pack(snow, 255.0));
    result.set_lightning(lightning > 0.0);
    result.set_hail_raw(pack(hail, 15.0));
    result
}

/// WMO weather interpretation codes, as used by Open-Meteo's `weather_code` field.
#[derive(Copy, Clone, FromRepr, IntoStaticStr)]
#[strum(serialize_all = "title_case")]
#[repr(u8)]
pub enum WeatherCode {
    Clear = 0,
    MainlyClear = 1,
    PartlyCloudy = 2,
    Overcast = 3,
    Fog = 45,
    RimeFog = 48,
    LightDrizzle = 51,
    ModerateDrizzle = 53,
    DenseDrizzle = 55,
    LightFreezingDrizzle = 56,
    DenseFreezingDrizzle = 57,
    LightRain = 61,
    ModerateRain = 63,
    HeavyRain = 65,
    LightFreezingRain = 66,
    HeavyFreezingRain = 67,
    LightSnow = 71,
    ModerateSnow = 73,
    HeavySnow = 75,
    SnowGrains = 77,
    LightRainShowers = 80,
    ModerateRainShowers = 81,
    ViolentRainShowers = 82,
    LightSnowShowers = 85,
    HeavySnowShowers = 86,
    Thunderstorm = 95,
    ThunderstormLightHail = 96,
    ThunderstormHeavyHail = 99,
}

impl WeatherCode {
    pub fn name(code: u8) -> &'static str {
        Self::from_repr(code).map_or("Unknown weather", Into::into)
    }

    const fn values(self) -> [f32; 6] {
        let cloud = match self {
            Self::Clear => 0.05,
            Self::MainlyClear => 0.25,
            Self::PartlyCloudy => 0.55,
            _ => 0.9,
        };
        let fog = match self {
            Self::Fog => 0.6,
            Self::RimeFog => 0.75,
            _ => 0.0,
        };
        let rain = match self {
            Self::LightDrizzle => 0.15,
            Self::ModerateDrizzle | Self::LightRain => 0.3,
            Self::DenseDrizzle => 0.45,
            Self::LightFreezingDrizzle => 0.2,
            Self::DenseFreezingDrizzle => 0.4,
            Self::ModerateRain => 0.6,
            Self::HeavyRain | Self::ViolentRainShowers => 1.0,
            Self::LightFreezingRain | Self::LightRainShowers => 0.35,
            Self::HeavyFreezingRain => 0.9,
            Self::ModerateRainShowers => 0.65,
            Self::Thunderstorm => 0.7,
            Self::ThunderstormLightHail => 0.75,
            Self::ThunderstormHeavyHail => 0.85,
            _ => 0.0,
        };
        let snow = match self {
            Self::SnowGrains => 0.25,
            Self::LightSnow => 0.3,
            Self::LightSnowShowers => 0.35,
            Self::ModerateSnow => 0.6,
            Self::HeavySnowShowers => 0.9,
            Self::HeavySnow => 1.0,
            _ => 0.0,
        };
        let lightning = matches!(
            self,
            Self::Thunderstorm | Self::ThunderstormLightHail | Self::ThunderstormHeavyHail
        ) as u8 as f32;
        let hail = match self {
            Self::ThunderstormHeavyHail => 1.0,
            Self::ThunderstormLightHail => 0.6,
            _ => 0.0,
        };
        [fog, cloud, rain, snow, lightning, hail]
    }
}

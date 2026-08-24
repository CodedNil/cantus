use crate::render::{
    Fragment, GAP, Globals, PANEL_START, TEXT_COLOR, TextFragment, UNIT,
    sdf::{PillSample, VISIBLE_ALPHA, cloud_mass, fbm, hash, pill_margin, pill_sheen, ripple_light, sample_pill, sd_rounded_box},
};
use core::f32::consts::PI;
use isthmus::{
    FloatExt, Quad, ShaderData,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv_std::arch::kill,
    text::TextStyle,
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

const WEEKDAY_COUNT: usize = 7;
const GRID_CELLS: usize = WEEKDAY_COUNT * 6;
const GRID_ROW_HEIGHT: f32 = UNIT * 6.0;
const GRID_TOP_Y: f32 = UNIT * 24.0;
const WEEKDAY_Y: f32 = UNIT * 17.0;
const TITLE: Vec2 = Vec2::new(WIDTH * 0.5, UNIT * 10.0);
const WEEKDAYS: [&str; WEEKDAY_COUNT] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const ORDINALS: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];
const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);
const WEATHER_STYLE: TextStyle = TextStyle::new(24.0, 600.0);
const TITLE_STYLE: TextStyle = TextStyle::new(20.0, 750.0);
const CLOCK_STYLE: TextStyle = TextStyle::new(12.0, 700.0);

#[repr(C)]
#[derive(Clone, Copy, Default, ShaderData)]
pub struct WeatherCondition {
    pub fog: f32,
    pub cloud: f32,
    pub rain: f32,
    pub snow: f32,
    pub lightning: f32,
    pub hail: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Default, ShaderData)]
pub struct StatusSky {
    pub sun_height: f32,
    pub conditions: WeatherCondition,
}

impl WeatherCondition {
    fn lerp(self, to: Self, amount: f32) -> Self {
        Self {
            fog: self.fog.lerp(to.fog, amount),
            cloud: self.cloud.lerp(to.cloud, amount),
            rain: self.rain.lerp(to.rain, amount),
            snow: self.snow.lerp(to.snow, amount),
            lightning: self.lightning.lerp(to.lightning, amount),
            hail: self.hail.lerp(to.hail, amount),
        }
    }
}

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

fn sample_weather_panel(pill: Quad, expansion: f32, pixel: Vec2, globals: Globals, time: f32) -> PillSample {
    let body = sample_pill(pill, pixel, globals, time);
    let pill_min = pill.center - pill.size * 0.5;
    let popup_size = vec2(WIDTH + FORECAST_X * expansion, ((EXTENSION - GAP) * expansion).max(0.001));
    let popup_center = vec2(expanded_x(pill_min.x, expansion), pill_min.y + pill.size.y + GAP * expansion) + popup_size * 0.5;
    let radius = (popup_size.y * 0.5).min(18.0);
    body.union(
        sd_rounded_box(pixel - popup_center, popup_size * 0.5, radius),
        sd_rounded_box(globals.pointer - popup_center, popup_size * 0.5, radius),
        56.0,
        expansion,
    )
}

fn forecast_center(height: f32, row: f32) -> f32 {
    UNIT * 14.0 + height * 0.5 + row * (height + GAP)
}

fn forecast_row(height: f32, row: f32) -> (Vec2, Vec2) {
    let size = Vec2::new(WIDTH - GAP * 2.0, height);
    let center = Vec2::new(FORECAST_X + WIDTH * 0.5, forecast_center(height, row));
    (center - size * 0.5, size)
}

fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let delay = 0.5 + (y / EXTENSION) * 0.18;
    expansion.smoothstep(delay, delay + 0.24)
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
        let phase = if hour < sunrise { (hour + 24.0 - sunset) / night } else { (hour - sunset) / night };
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
    let particle = distance.smoothstep(radius + 0.45, radius - 0.15) * hash(cell + 19.3).x.smoothstep(1.0 - density, 1.0);
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
pub fn sky_phase(sun_y: f32) -> Vec3 {
    let daylight = sun_y.smoothstep(-0.04, 0.2);
    vec3(
        daylight,
        sun_y.smoothstep(-0.32, -0.08) * (1.0 - daylight),
        sun_y.smoothstep(-0.18, 0.0) * sun_y.smoothstep(0.2, 0.02),
    )
}

pub fn scene(time: f32, cloud_scale: f32, p: Vec2, width: f32, dist: f32, phase: Vec3, weather: WeatherCondition) -> Vec3 {
    let sky_y = p.y / cloud_scale;
    let vertical = sky_y.smoothstep(1.0, 0.0);
    let mut color = vec3(0.006, 0.012, 0.035)
        .lerp(vec3(0.025, 0.04, 0.095), vertical)
        .lerp(vec3(0.08, 0.34, 0.62).lerp(vec3(0.32, 0.67, 0.87), vertical), phase.x)
        .lerp(vec3(0.10, 0.16, 0.30).lerp(vec3(0.22, 0.25, 0.45), vertical), phase.y * 0.8)
        .lerp(vec3(0.78, 0.30, 0.20).lerp(vec3(0.38, 0.22, 0.42), vertical), phase.z * 0.9);

    let star_cell = (p / 18.0).floor();
    let star_center = (star_cell + 0.2 + hash(star_cell) * 0.6) * 18.0;
    let stars = p.distance(star_center).smoothstep(1.0, 0.4) * hash(star_cell + 31.7).x.smoothstep(0.75, 1.0) * (1.0 - phase.x);
    color += Vec3::splat(stars * (1.0 - weather.cloud) * (0.3 + vertical * 0.7));

    if weather.cloud > VISIBLE_ALPHA {
        let mass = cloud_mass(p, cloud_scale, time);
        let billows = fbm(p / cloud_scale * 0.287 + vec2(time * 0.018, -3.7));
        let cloud_shape = (mass + (billows - 0.5) * 0.24).smoothstep(0.35, 0.6);
        let cloud_light = billows.smoothstep(0.42, 0.72) * 0.55 + mass.smoothstep(0.48, 0.7) * 0.45;
        let cloud_color = vec3(0.16, 0.2, 0.28)
            .lerp(vec3(0.32, 0.36, 0.43), cloud_light)
            .lerp(vec3(0.62, 0.7, 0.78).lerp(vec3(0.92, 0.94, 0.96), cloud_light), phase.x)
            .lerp(vec3(0.5, 0.36, 0.4).lerp(vec3(0.76, 0.59, 0.56), cloud_light), phase.z * 0.45);
        color = color.lerp(cloud_color, weather.cloud * (0.12 + cloud_shape * 0.7));
    }

    color = color.lerp(vec3(0.1, 0.17, 0.25), weather.rain * 0.2);
    if weather.rain > VISIBLE_ALPHA {
        let particle = precipitation(p, time, 0, weather.rain);
        color = color.lerp(particle.truncate(), particle.w);
    }
    if weather.snow > VISIBLE_ALPHA {
        let particle = precipitation(p, time, 1, weather.snow);
        color = color.lerp(particle.truncate(), particle.w);
    }
    if weather.hail > VISIBLE_ALPHA {
        let particle = precipitation(p, time, 2, weather.hail);
        color = color.lerp(particle.truncate(), particle.w);
    }

    let flash = (time * 2.7).sin().smoothstep(0.92, 1.0) * weather.lightning;
    color = color.lerp(vec3(0.65, 0.74, 0.96), flash * 0.55);

    if weather.fog > VISIBLE_ALPHA {
        let fog = fbm(vec2(p.x / width * 0.9 + time * 0.008, sky_y * 0.32 + 12.0));
        color = color.lerp(vec3(0.63, 0.69, 0.73), weather.fog * (0.58 + fog.smoothstep(0.35, 0.7) * 0.18));
    }
    color + pill_sheen(dist)
}

fn sun_layer(color: Vec3, point: Vec2, size: Vec2, [sun_x, sun_y]: [f32; 2], cloud: f32, time: f32) -> Vec3 {
    let sun = vec2(16.0 + sun_x * (size.x - 32.0), size.y * (0.72 - sun_y.saturate() * 0.45));
    let sun_color = vec3(0.96, 0.98, 1.0).lerp(vec3(0.98, 0.74, 0.66), sun_y.smoothstep(0.55, 0.02));
    let obstruction = if cloud > VISIBLE_ALPHA {
        cloud_mass(sun, size.y, time).smoothstep(0.43, 0.69) * cloud * 0.82
    } else {
        0.0
    };
    let clear = sun_y.smoothstep(-0.02, 0.04) * (1.0 - obstruction);
    let distance = point.distance(sun);
    color.lerp(sun_color, (distance.smoothstep(62.0, 4.0) * 0.24 + distance.smoothstep(11.0, 1.0) * 0.7) * clear)
}

#[isthmus::paint]
mod host {
    use super::*;

    use crate::interaction::Rect;
    use crate::{app::Background, render::RenderContext};
    use arrayvec::{ArrayString, ArrayVec};
    use jiff::{
        Span, Timestamp, Zoned,
        civil::{DateTime, Time},
        tz::{Offset, TimeZone},
    };
    use parking_lot::Mutex;
    use reqwest::Client;
    use std::{array::from_fn, fmt::Write, sync::Arc};
    use tracing::warn;

    #[derive(Default)]
    struct ForecastItem {
        text: [String; 2],
        hover_text: String,
        conditions: WeatherCondition,
        hour: f32,
    }

    struct WorldClock {
        label: String,
        timezone: TimeZone,
        weather: String,
    }

    pub struct WeatherPanel {
        expansion: f32,
        sun_hours: [f32; 2],
        temperature: String,
        utc_offset: Option<Offset>,
        details: String,
        hourly: [ForecastItem; HOURLY_FORECASTS],
        daily: [ForecastItem; DAILY_FORECASTS],
        timezones: ArrayVec<WorldClock, MAX_WORLD_CLOCKS>,
        month_offset: i32,
        month_hover: f32,
        previous_month_hover: f32,
        next_month_hover: f32,
        latest_update: Arc<Mutex<Option<monitor::WeatherUpdate>>>,
    }

    mod monitor {
        use super::{ForecastItem, HOURLY_STEP_HOURS, ORDINALS, WeatherCondition, WeatherPanel};
        use crate::app::Background;
        use futures_util::StreamExt;
        use jiff::{
            civil::DateTime,
            tz::{Offset, TimeZone},
        };
        use parking_lot::Mutex;
        use reqwest::Client;
        use serde::{Deserialize, de::DeserializeOwned};
        use std::{array::from_fn, collections::HashMap, error::Error, sync::Weak, time::Duration};
        use tokio::{
            sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
            time::{sleep, timeout},
        };
        use tracing::warn;
        use zbus::{
            Connection, Proxy,
            proxy::{Builder as ProxyBuilder, CacheProperties},
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

        pub(super) struct WeatherUpdate(Vec<(usize, Forecast)>);

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

        pub(super) fn start(timezones: Vec<String>, background: &Background, latest_update: Weak<Mutex<Option<WeatherUpdate>>>, http: Client) {
            let (location_tx, locations) = mpsc::unbounded_channel();
            background.spawn(async move {
                if let Err(error) = stream_location(&location_tx).await {
                    warn!(%error, "Location portal unavailable");
                }
            });
            background.spawn(async move {
                refresh_loop(&http, &timezones, &latest_update, locations).await;
            });
        }

        async fn stream_location(sender: &UnboundedSender<[f32; 2]>) -> Result<(), Box<dyn Error + Send + Sync>> {
            const DESTINATION: &str = "org.freedesktop.portal.Desktop";
            let connection = Connection::session().await?;
            let location = ProxyBuilder::<Proxy>::new(&connection)
                .destination(DESTINATION)?
                .path("/org/freedesktop/portal/desktop")?
                .interface("org.freedesktop.portal.Location")?
                .cache_properties(CacheProperties::No)
                .build()
                .await?;
            let session_token = format!("cantus_{:x}", fastrand::u64(..));
            let session: OwnedObjectPath = location
                .call(
                    "CreateSession",
                    &HashMap::from([("session_handle_token", Value::from(session_token)), ("accuracy", Value::from(2u32))]),
                )
                .await?;
            let mut updates = location.receive_signal("LocationUpdated").await?;

            let request_token = format!("cantus_{:x}", fastrand::u64(..));
            let sender_name = connection.unique_name().unwrap().trim_start_matches(':').replace('.', "_");
            let request = ProxyBuilder::<Proxy>::new(&connection)
                .destination(DESTINATION)?
                .path(format!("/org/freedesktop/portal/desktop/request/{sender_name}/{request_token}"))?
                .interface("org.freedesktop.portal.Request")?
                .cache_properties(CacheProperties::No)
                .build()
                .await?;
            let mut response = request.receive_signal("Response").await?;
            let _: OwnedObjectPath = location
                .call("Start", &(&session, "", HashMap::from([("handle_token", Value::from(request_token))])))
                .await?;
            let (status, _): (u32, HashMap<String, OwnedValue>) = response.next().await.ok_or("Location portal returned no response")?.body().deserialize()?;
            if status != 0 {
                return Err(format!("Location request failed with status {status}").into());
            }

            while let Some(update) = updates.next().await {
                let (_, location): (OwnedObjectPath, HashMap<String, OwnedValue>) = update.body().deserialize()?;
                if sender
                    .send([f64::try_from(&location["Latitude"])? as f32, f64::try_from(&location["Longitude"])? as f32])
                    .is_err()
                {
                    break;
                }
            }
            ProxyBuilder::<Proxy>::new(&connection)
                .destination(DESTINATION)?
                .path(session)?
                .interface("org.freedesktop.portal.Session")?
                .cache_properties(CacheProperties::No)
                .build()
                .await?
                .call::<_, _, ()>("Close", &())
                .await?;
            Ok(())
        }

        async fn refresh_loop(http: &Client, timezones: &[String], latest_update: &Weak<Mutex<Option<WeatherUpdate>>>, mut locations_rx: UnboundedReceiver<[f32; 2]>) {
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
                    Ok(results) => ready.into_iter().zip(results).map(|((index, _), forecast)| (index, forecast)).collect(),
                    Err(error) => {
                        retry = true;
                        warn!(%error, "Failed to refresh weather");
                        Vec::new()
                    }
                };
                if !forecasts.is_empty() {
                    let Some(latest_update) = latest_update.upgrade() else {
                        break;
                    };
                    *latest_update.lock() = Some(WeatherUpdate(forecasts));
                }
                let interval = if retry { RETRY_INTERVAL } else { REFRESH_INTERVAL };
                match timeout(interval, locations_rx.recv()).await {
                    Ok(Some(location)) => locations[0] = Some(location),
                    Ok(None) => sleep(interval).await,
                    Err(_) => {}
                }
            }
        }

        pub(super) fn apply(weather: &mut WeatherPanel, update: WeatherUpdate) {
            for (index, forecast) in update.0 {
                apply_forecast(weather, index, &forecast);
            }
        }

        fn apply_forecast(weather_model: &mut WeatherPanel, index: usize, forecast: &Forecast) {
            // Index 0 is the local forecast; the rest fill in each configured world clock.
            if let Some(timezone) = index.checked_sub(1).and_then(|index| weather_model.timezones.get_mut(index)) {
                timezone.weather = format!(
                    "{} · {:.0}°/{:.0}°",
                    weather(forecast.daily.weather_code[0]).0,
                    forecast.daily.temperature_2m_max[0],
                    forecast.daily.temperature_2m_min[0],
                );
                return;
            }
            if index != 0 {
                return;
            }
            weather_model.utc_offset = Offset::from_seconds(forecast.utc_offset_seconds).ok();
            weather_model.temperature = format!("{:.1}°C", forecast.current.temperature_2m);
            weather_model.sun_hours = [forecast.daily.sunrise[0], forecast.daily.sunset[0]].map(WeatherPanel::hour_of_day);
            weather_model.hourly = from_fn(|index| {
                let source = index * HOURLY_STEP_HOURS;
                let (description, conditions) = weather(forecast.hourly.weather_code[source]);
                let (time, degrees) = (
                    forecast.hourly.time[source].strftime("%H:%M").to_string(),
                    format!("{:.0}°", forecast.hourly.temperature_2m[source]),
                );
                ForecastItem {
                    hover_text: format!("{time} {description} {degrees}"),
                    text: [time, degrees],
                    conditions,
                    hour: WeatherPanel::hour_of_day(forecast.hourly.time[source]),
                }
            });
            weather_model.hourly[0].conditions = weather(forecast.current.weather_code).1;
            weather_model.daily = from_fn(|index| {
                let day = index + 1;
                let date = forecast.daily.sunrise[day];
                let number = date.day();
                let suffix = match number % 100 {
                    11..=13 => "th",
                    _ => ORDINALS[(number % 10) as usize],
                };
                let (description, conditions) = weather(forecast.daily.weather_code[day]);
                let range = format!("{:.0}°/{:.0}°", forecast.daily.temperature_2m_max[day], forecast.daily.temperature_2m_min[day]);
                ForecastItem {
                    hover_text: format!("{}{suffix} {description} {range}", date.strftime("%A %-d")),
                    text: [date.strftime("%a").to_string(), range],
                    conditions,
                    hour: -1.0,
                }
            });
            weather_model.details = format!(
                "{} · Humidity {}% · Wind {:.0} km/h",
                weather(forecast.current.weather_code).0,
                forecast.current.relative_humidity_2m,
                forecast.current.wind_speed_10m
            );
        }

        async fn geocode(http: &Client, timezone: &str) -> Result<[f32; 2], String> {
            let city = timezone.rsplit('/').next().unwrap_or(timezone).replace('_', " ");
            let query: String = form_urlencoded::byte_serialize(city.as_bytes()).collect();
            let results: SearchResults = get_json(http, format!("https://geocoding-api.open-meteo.com/v1/search?name={query}&count=10")).await?;
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
            let latitude = locations.iter().map(|(_, [latitude, _])| latitude.to_string()).collect::<Vec<_>>().join(",");
            let longitude = locations.iter().map(|(_, [_, longitude])| longitude.to_string()).collect::<Vec<_>>().join(",");
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
    impl WeatherPanel {
        pub(crate) fn new(timezones: &[String], background: &Background, http: Client) -> Self {
            let mut forecast_timezones = Vec::with_capacity(timezones.len());
            let timezones: ArrayVec<_, MAX_WORLD_CLOCKS> = timezones
                .iter()
                .filter_map(|name| {
                    let timezone = TimeZone::get(name).inspect_err(|error| warn!(timezone = name, %error, "Ignoring invalid timezone")).ok()?;
                    forecast_timezones.push(name.clone());
                    Some(WorldClock {
                        label: name.rsplit('/').next().unwrap_or(name).replace('_', " "),
                        timezone,
                        weather: String::from("Weather unavailable"),
                    })
                })
                .collect();
            let latest_update = Arc::new(Mutex::new(None));
            monitor::start(forecast_timezones, background, Arc::downgrade(&latest_update), http);
            Self {
                expansion: 0.0,
                sun_hours: [6.0, 18.0],
                temperature: String::new(),
                utc_offset: None,
                details: "Weather unavailable".into(),
                hourly: Default::default(),
                daily: Default::default(),
                timezones,
                month_offset: 0,
                month_hover: 0.0,
                previous_month_hover: 0.0,
                next_month_hover: 0.0,
                latest_update,
            }
        }

        pub fn show(&mut self, frame: &mut RenderContext, status_width: f32) -> StatusSky {
            let update = self.latest_update.try_lock().and_then(|mut latest| latest.take());
            if let Some(update) = update {
                monitor::apply(self, update);
            }
            let height = frame.config.height;
            let x = Self::pill_x(frame.paint.screen_size.x, status_width);
            let hovered = Self::visible_rects(x, height, self.expansion).into_iter().any(|rect| frame.interaction.pointer_in(rect));
            self.expansion = self.expansion.move_towards(f32::from(hovered), frame.paint.delta_time.min(1.0 / 30.0) * 3.0);
            let (weather_label, hour) = self.collapsed_label();
            let status_sky = StatusSky {
                sun_height: sun_position(hour, self.sun_hours)[1],
                conditions: self.hourly[0].conditions,
            };
            let current = self.hourly[0].conditions;
            let next = self.hourly[1].conditions;
            let sun = Vec2::from(sun_position(hour, self.sun_hours));
            let pill = Quad::from_min_max(vec2(x, PANEL_START), vec2(x + WIDTH, PANEL_START + height));
            let expansion = self.expansion.smoothstep(0.0, 1.0);
            let margin = pill_margin(frame.globals(), frame.paint.time);
            let render_min = vec2(expanded_x(x, expansion) - margin, PANEL_START - margin);
            let render_max = vec2(
                expanded_x(x, expansion) + WIDTH + FORECAST_X * expansion + margin,
                PANEL_START + height + EXTENSION * expansion + margin,
            );
            // GPU: Current weather pill unified with its expanded panel.
            frame.paint.paint_quad(
                Quad::from_min_max(render_min, render_max),
                |fragment: Fragment, pill: Quad, current: WeatherCondition, next: WeatherCondition, sun: Vec2, expansion: f32| {
                    let surface = sample_weather_panel(pill, expansion, fragment.pixel, fragment.globals, fragment.time);
                    let pill_min = pill.center - pill.size * 0.5;
                    if surface.alpha <= VISIBLE_ALPHA {
                        kill();
                    }
                    let body_local = pill.local(fragment.pixel) + pill.size * 0.5;
                    let edge = ((body_local.x / pill.size.x).clamp(0.0, 1.0) - 0.5).abs();
                    let body_conditions = current.lerp(next, edge.smoothstep(0.05, 0.25));
                    let conditions = body_conditions.lerp(current, expansion);
                    let in_body = fragment.pixel.y <= pill_min.y + pill.size.y;
                    let mut color = scene(
                        fragment.time,
                        fragment.globals.bar_height,
                        surface.refracted,
                        pill.size.x,
                        surface.distance,
                        sky_phase(sun.y),
                        conditions,
                    );
                    if in_body {
                        color = sun_layer(color, body_local, pill.size, sun.into(), body_conditions.cloud, fragment.time);
                    }
                    color = ripple_light(color, surface.ripple_flash);
                    surface.color(color)
                },
            );
            Self::label(
                frame,
                &weather_label,
                WEATHER_STYLE,
                vec2(x + WIDTH * 0.5, PANEL_START + height * 0.46),
                TEXT_COLOR,
                1.0,
                pill,
                expansion,
            );
            self.show_calendar(frame, x, height);
            status_sky
        }

        fn label(frame: &mut RenderContext, content: &str, style: TextStyle, center: Vec2, color: Vec3, alpha: f32, pill: Quad, expansion: f32) {
            let line = frame.paint.text().line(content, style).centered(center).with_color(color.extend(alpha));
            // GPU: Weather text.
            frame.paint.paint_text(line, |text: TextFragment, pill: Quad, expansion: f32| {
                let panel = sample_weather_panel(pill, expansion, text.pixel, text.globals, text.time);
                text.color(text.alpha() * panel.mask)
            });
        }

        fn pair(frame: &mut RenderContext, content: [&str; 2], style: TextStyle, center: Vec2, spacing: f32, color: Vec3, alpha: f32, pill: Quad, expansion: f32) {
            for (index, content) in content.into_iter().enumerate() {
                Self::label(
                    frame,
                    content,
                    style,
                    center + vec2(0.0, (index as f32 * 2.0 - 1.0) * spacing),
                    color,
                    alpha * if index == 0 { 1.0 } else { 0.75 },
                    pill,
                    expansion,
                );
            }
        }

        fn collapsed_label(&self) -> (ArrayString<64>, f32) {
            let time = Zoned::now();
            let hour = self
                .utc_offset
                .map_or_else(|| Self::hour_of_day(time.datetime()), |offset| Self::hour_of_day(offset.to_datetime(time.timestamp())));
            let clock = time.strftime("%a %d %b  %H:%M:%S");
            let mut label = ArrayString::new();
            if self.temperature.is_empty() {
                write!(label, "{clock}").unwrap();
            } else {
                write!(label, "{}   {clock}", self.temperature).unwrap();
            }
            (label, hour)
        }

        fn show_calendar(&mut self, frame: &mut RenderContext, x: f32, height: f32) {
            let bounds = Self::pill_rect(x, height);
            if self.expansion <= 0.0 {
                return;
            }
            let origin = Vec2::new(expanded_x(bounds.min.x, 1.0), bounds.max.y);
            let expansion = self.expansion.smoothstep(0.0, 1.0);
            let pill: Quad = bounds.into();
            let reveal = reveal_progress(expansion, TITLE.y);

            let title = frame.interaction.interact(Rect::from_center(origin + TITLE, Vec2::new(UNIT * 26.0, UNIT * 4.0)));
            if title.clicked() {
                self.month_offset = 0;
            }
            self.month_hover = self.month_hover.move_towards(f32::from(title.hovered()), frame.paint.delta_time / 0.12);

            let today = Zoned::now().date();
            let month = today.first_of_month().saturating_add(Span::new().months(self.month_offset));
            Self::label(
                frame,
                &month.strftime("%B %Y").to_string(),
                TITLE_STYLE.scaled(1.0 + self.month_hover * 0.2).with_weight_mix(0.5 + self.month_hover * 0.5),
                origin + TITLE,
                TEXT_COLOR,
                reveal_progress(expansion, TITLE.y),
                pill,
                expansion,
            );

            for (index, (side, glyph)) in [(-1.0f32, "<"), (1.0, ">")].into_iter().enumerate() {
                let position = Vec2::new(WIDTH * 0.5 + side * (WIDTH * 0.5 - UNIT * 7.0) * reveal, TITLE.y - (1.0 - reveal) * UNIT * 3.0);
                let response = frame.interaction.interact(Rect::from_center(origin + position, Vec2::splat(UNIT * 5.0)));
                if response.clicked() {
                    self.month_offset = (self.month_offset + side as i32).clamp(-1200, 1200);
                }
                let hover = if index == 0 { &mut self.previous_month_hover } else { &mut self.next_month_hover };
                *hover = hover.move_towards(f32::from(response.hovered()), frame.paint.delta_time / 0.12);
                let style = TITLE_STYLE.scaled(1.0 + *hover * 0.35).with_weight_mix(0.5 + *hover * 0.5);
                Self::label(frame, glyph, style, origin + position, TEXT_COLOR, reveal_progress(expansion, position.y), pill, expansion);
            }

            let mut hovered_detail = None;
            for (row, items) in [&self.hourly[..], &self.daily[..]].into_iter().enumerate() {
                let conditions = from_fn(|index| items[index.min(items.len() - 1)].conditions);
                let start_hour = items[0].hour;
                let count = items.len() as u32;
                let (row_origin, size) = forecast_row(height, row as f32);
                let step = size.x / items.len() as f32;
                let alpha = reveal_progress(expansion, row_origin.y + size.y * 0.5);
                let forecast_pill = Quad::from_min_max(origin + row_origin, origin + row_origin + size);
                // GPU: Hourly or daily forecast row.
                let margin = pill_margin(frame.globals(), frame.paint.time);
                frame.paint.paint_quad(
                    forecast_pill.expanded(margin),
                    |fragment: Fragment,
                     forecast_pill: Quad,
                     pill: Quad,
                     conditions: [WeatherCondition; HOURLY_FORECASTS],
                     count: u32,
                     start_hour: f32,
                     sun_hours: [f32; 2],
                     alpha: f32,
                     expansion: f32| {
                        let surface = sample_pill(forecast_pill, fragment.pixel, fragment.globals, fragment.time);
                        let panel = sample_weather_panel(pill, expansion, fragment.pixel, fragment.globals, fragment.time);
                        let position = (surface.uv().x * count as f32 - 0.5).clamp(0.0, count as f32 - 1.0);
                        let index = position.floor() as usize;
                        let conditions = conditions[index].lerp(conditions[(index + 1).min(count as usize - 1)], position.fract().smoothstep(0.0, 1.0));
                        let hour = if start_hour < 0.0 {
                            12.0
                        } else {
                            (start_hour + position * HOURLY_STEP_HOURS as f32) % 24.0
                        };
                        let coverage = surface.alpha * alpha;
                        if coverage <= VISIBLE_ALPHA {
                            kill();
                        }
                        let color = scene(
                            fragment.time,
                            fragment.globals.bar_height,
                            surface.refracted,
                            surface.size.x,
                            surface.distance,
                            sky_phase(sun_position(hour, sun_hours)[1]),
                            conditions,
                        )
                        .lerp(vec3(0.18, 0.22, 0.34), surface.ripple_flash);
                        surface.color(color) * (alpha * panel.mask)
                    },
                );
                for (column, forecast) in items.iter().enumerate() {
                    let center = origin + row_origin + vec2(step * (column as f32 + 0.5), size.y * 0.5);
                    if frame.interaction.pointer_in(Rect::from_center(center, vec2(step, size.y) * 0.5)) {
                        hovered_detail = Some(forecast.hover_text.as_str());
                    }
                    let [primary, secondary] = &forecast.text;
                    Self::pair(frame, [primary, secondary], DETAILS_STYLE, center, GAP, TEXT_COLOR, alpha, pill, expansion);
                }
            }
            Self::label(
                frame,
                hovered_detail.unwrap_or(&self.details),
                DETAILS_STYLE,
                origin + vec2(FORECAST_X + WIDTH * 0.5, TITLE.y),
                TEXT_COLOR,
                reveal_progress(expansion, TITLE.y),
                pill,
                expansion,
            );

            for (column, weekday) in WEEKDAYS.iter().enumerate() {
                let position = Vec2::new(grid_cell(column).x, WEEKDAY_Y);
                Self::label(
                    frame,
                    weekday,
                    DETAILS_STYLE,
                    origin + position,
                    TEXT_COLOR,
                    reveal_progress(expansion, position.y) * 0.75,
                    pill,
                    expansion,
                );
            }

            let grid_start = month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
            for index in 0..GRID_CELLS {
                let date = grid_start.saturating_add(Span::new().days(index as i64));
                let mut label = ArrayString::<2>::new();
                write!(label, "{}", date.day()).unwrap();
                let is_today = date == today;
                Self::label(
                    frame,
                    &label,
                    TextStyle::new(16.0, if is_today { 900.0 } else { 700.0 }),
                    origin + grid_cell(index),
                    if is_today { vec3(1.0, 0.68, 0.68) } else { TEXT_COLOR },
                    reveal_progress(expansion, grid_cell(index).y) * if date.month() == month.month() { 1.0 } else { 0.32 },
                    pill,
                    expansion,
                );
            }

            let now = Timestamp::now();
            for (index, timezone) in self.timezones.iter().enumerate() {
                let local = now.to_zoned(timezone.timezone.clone());
                let mut clock = ArrayString::<64>::new();
                write!(clock, "{} · {}", timezone.label, local.strftime("%H:%M")).unwrap();
                let center = vec2(FORECAST_X + WIDTH * 0.5, forecast_center(height, 1.0) + height * 0.5 + UNIT * (3.5 + index as f32 * 7.0));
                Self::pair(
                    frame,
                    [&clock, &timezone.weather],
                    CLOCK_STYLE,
                    origin + center,
                    GAP * 0.7,
                    TEXT_COLOR,
                    reveal_progress(expansion, center.y),
                    pill,
                    expansion,
                );
            }
        }

        fn hour_of_day(time: DateTime) -> f32 {
            time.time().duration_since(Time::midnight()).as_secs_f32() / 3600.0
        }

        fn pill_x(screen_width: f32, status_width: f32) -> f32 {
            screen_width - WIDTH - GAP - status_width
        }

        const fn pill_rect(x: f32, height: f32) -> Rect {
            Rect::new(x, PANEL_START, x + WIDTH, PANEL_START + height)
        }

        fn visible_rects(x: f32, height: f32, expansion: f32) -> [Rect; 2] {
            let pill = Self::pill_rect(x, height);
            let expansion = expansion.smoothstep(0.0, 1.0);
            let size = Vec2::new(WIDTH + FORECAST_X * expansion, EXTENSION * expansion);
            let x = expanded_x(pill.min.x, expansion);
            [pill, Rect::new(x, pill.max.y, x + size.x, pill.max.y + size.y)]
        }
    }
}

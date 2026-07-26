use crate::{
    AppUpdater,
    interaction::{InteractionState, Rect},
    openmeteo::{self, Forecast, WeatherCode},
    render::{approach, text::TextStyle},
};
use arrayvec::ArrayString;
use cantus_shared::{
    UNIT, smoothstep,
    tempo::{self, WIDTH, WeatherCondition, WeatherPill},
};
use glam::Vec2;
use jiff::{Span, Zoned, civil::DateTime};
use std::{array::from_fn, fmt::Write, time::UNIX_EPOCH};

const GRID_ROWS: usize = 6;
const GRID_ROW_HEIGHT: f32 = UNIT * 6.0;
const GRID_TOP_Y: f32 = UNIT * 24.0;
const WEEKDAY_Y: f32 = UNIT * 17.0;
const TITLE: Vec2 = Vec2::new(WIDTH * 0.5, UNIT * 10.0);
const DETAILS: Vec2 = Vec2::new(tempo::FORECAST_X + WIDTH * 0.5, TITLE.y);
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const ARROWS: [(&str, i32); 2] = [("<", -1), (">", 1)];

fn cell(index: usize) -> Vec2 {
    let column_width = WIDTH / WEEKDAYS.len() as f32;
    Vec2::new(
        (index % WEEKDAYS.len()) as f32 * column_width + column_width * 0.5,
        GRID_TOP_Y + (index / WEEKDAYS.len()) as f32 * GRID_ROW_HEIGHT,
    )
}

#[derive(Default)]
struct ForecastItem {
    text: [String; 2],
    hover_text: String,
    conditions: WeatherCondition,
}

fn pill_rect(screen_width: f32, height: f32) -> Rect {
    Rect::pill(tempo::pill_x(screen_width), WIDTH, height)
}

fn visible_rects(screen_width: f32, height: f32, expansion: f32) -> [Rect; 2] {
    let pill = pill_rect(screen_width, height);
    let expansion = smoothstep(0.0, 1.0, expansion);
    let size = tempo::popup_size(expansion);
    let x = tempo::expanded_x(pill.x0, expansion);
    [pill, Rect::new(x, pill.y1, x + size.x, pill.y1 + size.y)]
}

pub struct Weather {
    temperature: String,
    sun_hours: [f32; 2],
    utc_offset_seconds: Option<i32>,
    hourly_start: f32,
    details: String,
    hourly: [ForecastItem; tempo::HOURLY_FORECASTS],
    daily: [ForecastItem; 5],
    calendar_expansion: f32,
    month_offset: i32,
}

impl Weather {
    pub fn new([latitude, longitude]: [f32; 2], updater: AppUpdater) -> Self {
        openmeteo::spawn_refresh_loop(latitude, longitude, updater);
        Self {
            temperature: "--.-°C".into(),
            sun_hours: [6.0, 18.0],
            utc_offset_seconds: None,
            hourly_start: 0.0,
            details: "Weather unavailable".into(),
            hourly: Default::default(),
            daily: Default::default(),
            calendar_expansion: 0.0,
            month_offset: 0,
        }
    }

    pub fn scene(
        &mut self,
        screen_width: f32,
        height: f32,
        ui: &InteractionState,
        dt: f32,
    ) -> (WeatherPill, ArrayString<64>, f32) {
        let hovered = visible_rects(screen_width, height, self.calendar_expansion)
            .into_iter()
            .any(|rect| ui.contains(rect));
        approach(
            &mut self.calendar_expansion,
            f32::from(hovered),
            dt.min(1.0 / 30.0) * 3.0,
        );
        let time = Zoned::now();
        let hour = self.utc_offset_seconds.map_or_else(
            || hour_of_day(time.datetime()),
            |offset| {
                let utc_hours = UNIX_EPOCH.elapsed().unwrap_or_default().as_secs_f64() / 3600.0;
                (utc_hours + f64::from(offset) / 3600.0).rem_euclid(24.0) as f32
            },
        );
        let pill = WeatherPill {
            sun_hours: self.sun_hours,
            hourly_start: self.hourly_start,
            calendar_expansion: self.calendar_expansion,
            hourly: self.hourly.each_ref().map(|item| item.conditions),
            daily: self.daily.each_ref().map(|item| item.conditions),
        };
        let clock = time.strftime("%a %d %b  %H:%M:%S");
        let mut label = ArrayString::new();
        write!(label, "{}   {clock}", self.temperature).unwrap();
        (pill, label, hour)
    }

    pub fn calendar_labels(
        &mut self,
        screen_width: f32,
        height: f32,
        ui: &mut InteractionState,
        mut draw: impl FnMut(&str, Vec2, f32, TextStyle),
    ) {
        if self.calendar_expansion <= 0.0 {
            ui.regions.push(pill_rect(screen_width, height));
            return;
        }
        let pill = pill_rect(screen_width, height);
        let origin = Vec2::new(tempo::expanded_x(pill.x0, 1.0), pill.y1);
        let reveal = tempo::reveal_progress(self.calendar_expansion, TITLE.y);
        let title = ui.interact(origin + TITLE, Vec2::new(UNIT * 15.0, UNIT * 4.0));
        if title.clicked {
            self.month_offset = 0;
        }
        let arrows = ARROWS.map(|(text, side)| {
            let position = Vec2::new(
                WIDTH * 0.5 + side as f32 * (WIDTH * 0.5 - UNIT * 7.0) * reveal,
                TITLE.y - (1.0 - reveal) * UNIT * 3.0,
            );
            let response = ui.interact(origin + position, Vec2::splat(UNIT * 5.0));
            if response.clicked {
                self.month_offset = (self.month_offset + side).clamp(-1200, 1200);
            }
            (text, position, response.hovered)
        });

        let today = Zoned::now().date();
        let month = today
            .first_of_month()
            .saturating_add(Span::new().months(self.month_offset));
        let mut label = |text: &str, target: Vec2, alpha, style| {
            let eased = tempo::reveal_progress(self.calendar_expansion, target.y);
            draw(text, origin + target, alpha * eased, style);
        };
        let hovered_forecast = if self.calendar_expansion > 0.01 {
            let popup_x = origin.x;
            [self.hourly.len(), self.daily.len()]
                .into_iter()
                .enumerate()
                .find_map(|(row, count)| {
                    let (row_origin, size) = tempo::forecast_row(height, row as f32);
                    (0..count).find_map(|column| {
                        let rect = Rect::new(
                            popup_x + row_origin.x + column as f32 * size.x / count as f32,
                            pill.y1 + row_origin.y,
                            popup_x + row_origin.x + (column + 1) as f32 * size.x / count as f32,
                            pill.y1 + row_origin.y + size.y,
                        );
                        ui.contains(rect)
                            .then(|| self.forecast(row)[column].hover_text.as_str())
                    })
                })
        } else {
            None
        };
        label(
            hovered_forecast.unwrap_or(&self.details),
            DETAILS,
            1.0,
            TextStyle::DETAILS,
        );
        for (row, forecasts) in [&self.hourly[..], &self.daily[..]].into_iter().enumerate() {
            for (column, forecast) in forecasts.iter().enumerate() {
                for (line, text) in forecast.text.iter().enumerate() {
                    let position = Vec2::new(
                        tempo::FORECAST_X + (column as f32 + 0.5) * WIDTH / forecasts.len() as f32,
                        tempo::forecast_center(height, row as f32)
                            + (line as f32 * 2.0 - 1.0) * tempo::TOP_GAP,
                    );
                    label(text, position, 1.0, TextStyle::DETAILS);
                }
            }
        }

        label(
            &month.strftime("%B %Y").to_string(),
            TITLE,
            1.0,
            if title.hovered {
                TextStyle::CALENDAR_TITLE_HOVER
            } else {
                TextStyle::CALENDAR_TITLE
            },
        );
        for (text, position, hovered) in arrows {
            label(
                text,
                position,
                1.0,
                if hovered {
                    TextStyle::CALENDAR_ARROW_HOVER
                } else {
                    TextStyle::CALENDAR_TITLE
                },
            );
        }

        let grid_start = month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
        for (column, weekday) in WEEKDAYS.iter().enumerate() {
            label(
                weekday,
                Vec2::new(cell(column).x, WEEKDAY_Y),
                0.75,
                TextStyle::DETAILS,
            );
        }
        for index in 0..WEEKDAYS.len() * GRID_ROWS {
            let date = grid_start.saturating_add(Span::new().days(index as i64));
            let mut text = ArrayString::<2>::new();
            write!(text, "{}", date.day()).unwrap();
            let alpha = 0.32 + f32::from(date.month() == month.month()) * 0.68;
            let style = if date == today {
                TextStyle::TODAY
            } else {
                TextStyle::PRIMARY
            };
            label(&text, cell(index), alpha, style);
        }

        for rect in visible_rects(screen_width, height, self.calendar_expansion) {
            ui.regions.push(rect);
        }
    }

    const fn forecast(&self, row: usize) -> &[ForecastItem] {
        if row == 0 { &self.hourly } else { &self.daily }
    }

    pub fn apply_forecast(&mut self, forecast: &Forecast) {
        let raw = &forecast.current;
        self.utc_offset_seconds = Some(forecast.utc_offset_seconds);
        self.temperature = format!("{:.1}°C", raw.temperature_2m);
        self.sun_hours = [forecast.daily.sunrise[0], forecast.daily.sunset[0]].map(hour_of_day);
        self.hourly_start = hour_of_day(forecast.hourly.time[0]);
        self.hourly = from_fn(|index| {
            let source = index * tempo::HOURLY_STEP_HOURS;
            let time = forecast.hourly.time[source];
            ForecastItem {
                text: [
                    time.strftime("%H:%M").to_string(),
                    format!("{:.0}°", forecast.hourly.temperature_2m[source]),
                ],
                hover_text: format!(
                    "{} {} {:.0}°",
                    time.strftime("%H:%M"),
                    forecast.hourly.name(source),
                    forecast.hourly.temperature_2m[source]
                ),
                conditions: forecast.hourly.condition(source),
            }
        });
        self.hourly[0].conditions = openmeteo::coded_conditions(raw.weather_code);
        self.daily = from_fn(|day| {
            let day = day + 1;
            ForecastItem {
                text: [
                    forecast.daily.sunrise[day].strftime("%a").to_string(),
                    format!(
                        "{:.0}°/{:.0}°",
                        forecast.daily.temperature_2m_max[day], forecast.daily.temperature_2m_min[day]
                    ),
                ],
                hover_text: format!(
                    "{}{} {} {:.0}°/{:.0}°",
                    forecast.daily.sunrise[day].strftime("%A %-d"),
                    ordinal(forecast.daily.sunrise[day].day()),
                    WeatherCode::name(forecast.daily.weather_code[day]),
                    forecast.daily.temperature_2m_max[day],
                    forecast.daily.temperature_2m_min[day]
                ),
                conditions: openmeteo::coded_conditions(forecast.daily.weather_code[day]),
            }
        });
        self.details = format!(
            "{} · Humidity {}% · Wind {:.0} km/h",
            WeatherCode::name(raw.weather_code),
            raw.relative_humidity_2m,
            raw.wind_speed_10m
        );
    }
}

const fn ordinal(day: i8) -> &'static str {
    match day % 100 {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

fn hour_of_day(time: DateTime) -> f32 {
    f32::from(time.hour()) + f32::from(time.minute()) / 60.0 + f32::from(time.second()) / 3600.0
}

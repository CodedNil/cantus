use crate::{
    AppUpdater,
    interaction::Rect,
    openmeteo::{self, Forecast, WeatherCode},
    render::{status::GAP, text::TextStyle},
};
use arrayvec::ArrayString;
use cantus_shared::{
    UNIT, approach,
    status::StatusPill,
    tempo::{self, WeatherCondition, WeatherPill},
};
use glam::Vec2;
use jiff::{Span, Zoned, civil::DateTime};
use std::{array::from_fn, fmt::Write, time::UNIX_EPOCH};

pub const WIDTH: f32 = tempo::WIDTH;
const HOURS_PER_DAY: f64 = 24.0;
const DAYS_PER_WEEK: usize = 7;
const GRID_ROWS: usize = 6;
const GRID_ROW_HEIGHT: f32 = UNIT * 6.0;
const GRID_TOP_Y: f32 = UNIT * 24.0;
const WEEKDAY_Y: f32 = UNIT * 17.0;
const TITLE: Vec2 = Vec2::new(WIDTH * 0.5, UNIT * 10.0);
const TITLE_HALF_SIZE: Vec2 = Vec2::new(UNIT * 15.0, UNIT * 4.0);
const DETAILS: Vec2 = Vec2::new(tempo::FORECAST_X + WIDTH * 0.5, TITLE.y);
const ARROW_RADIUS: f32 = UNIT * 5.0;
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const ARROWS: [(&str, i32); 2] = [("<", -1), (">", 1)];

fn cell(index: usize) -> Vec2 {
    let column_width = WIDTH / DAYS_PER_WEEK as f32;
    Vec2::new(
        (index % DAYS_PER_WEEK) as f32 * column_width + column_width * 0.5,
        GRID_TOP_Y + (index / DAYS_PER_WEEK) as f32 * GRID_ROW_HEIGHT,
    )
}

fn arrow(side: i32, reveal: f32) -> Vec2 {
    Vec2::new(
        WIDTH * 0.5 + side as f32 * (WIDTH * 0.5 - UNIT * 7.0) * reveal,
        TITLE.y - (1.0 - reveal) * UNIT * 3.0,
    )
}

/// Zero selects the month title; -1 and 1 select the previous/next arrows.
fn header_action(point: Vec2, reveal: f32) -> Option<i32> {
    if Rect::from_center(TITLE, TITLE_HALF_SIZE).contains(point) {
        return Some(0);
    }
    ARROWS.into_iter().find_map(|(_, side)| {
        Rect::from_center(arrow(side, reveal), Vec2::splat(ARROW_RADIUS))
            .contains(point)
            .then_some(side)
    })
}

fn forecast_item(height: f32, row: f32, column: usize, count: usize, line: usize) -> Vec2 {
    Vec2::new(
        tempo::FORECAST_X + (column as f32 + 0.5) * WIDTH / count as f32,
        tempo::forecast_center(height, row) + (line as f32 * 2.0 - 1.0) * tempo::TOP_GAP,
    )
}

#[derive(Default)]
struct ForecastItem {
    text: [String; 2],
    conditions: WeatherCondition,
}

fn pill_rect(status: &StatusPill, height: f32) -> Rect {
    Rect::pill(status.x - WIDTH - GAP, WIDTH, height)
}

fn calendar_rect(status: &StatusPill, height: f32, popup: bool) -> Rect {
    let mut area = pill_rect(status, height);
    area.x0 = tempo::expanded_x(area.x0, 1.0);
    area.x1 = area.x0
        + if popup {
            tempo::popup_size(1.0).x
        } else {
            WIDTH
        };
    area.y1 += f32::from(popup) * tempo::EXTENSION;
    area
}

fn calendar_origin(status: &StatusPill, height: f32) -> Vec2 {
    let pill = calendar_rect(status, height, false);
    Vec2::new(pill.x0, pill.y1)
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
        status: &StatusPill,
        height: f32,
        mouse: Vec2,
        mouse_active: bool,
        dt: f32,
    ) -> (WeatherPill, ArrayString<64>, f32) {
        let hovered = self.hovered(status, height, mouse);
        approach(
            &mut self.calendar_expansion,
            f32::from(mouse_active && hovered),
            dt.min(1.0 / 30.0) * 3.0,
        );
        let time = Zoned::now();
        let hour = self
            .utc_offset_seconds
            .map_or_else(|| hour_of_day(time.datetime()), hour_at_offset);
        let pill = WeatherPill {
            x: pill_rect(status, height).x0,
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

    /// Whether `point` is over the pill's collapsed or fully-expanded position, or the popup.
    fn hovered(&self, status: &StatusPill, height: f32, point: Vec2) -> bool {
        pill_rect(status, height).contains(point)
            || calendar_rect(status, height, false).contains(point)
            || self.calendar_expansion > 0.25 && calendar_rect(status, height, true).contains(point)
    }

    pub fn interaction_rect(&self, status: &StatusPill, height: f32) -> Rect {
        if self.calendar_expansion > 0.25 {
            calendar_rect(status, height, true)
        } else {
            pill_rect(status, height)
        }
    }

    pub fn navigate_calendar(&mut self, position: Vec2, status: &StatusPill, height: f32) -> bool {
        if self.calendar_expansion < 0.5 {
            return false;
        }
        let local = position - calendar_origin(status, height);
        let reveal = tempo::reveal_progress(self.calendar_expansion, TITLE.y);
        let Some(step) = header_action(local, reveal) else {
            return false;
        };
        let new_offset = if step == 0 {
            0
        } else {
            (self.month_offset + step).clamp(-1200, 1200)
        };
        self.month_offset = new_offset;
        true
    }

    pub fn calendar_labels(
        &self,
        status: &StatusPill,
        height: f32,
        mouse: Vec2,
        mouse_active: bool,
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
        let header_reveal = tempo::reveal_progress(self.calendar_expansion, TITLE.y);
        let hovered = mouse_active
            .then(|| header_action(mouse - origin, header_reveal))
            .flatten();
        let mut label = |text: &str, target: Vec2, alpha, style| {
            let eased = tempo::reveal_progress(self.calendar_expansion, target.y);
            draw(text, origin + target, alpha * eased, style);
        };
        label(&self.details, DETAILS, 1.0, TextStyle::DETAILS);

        for (row, forecasts) in [&self.hourly[..], &self.daily[..]].into_iter().enumerate() {
            for (column, forecast) in forecasts.iter().enumerate() {
                for (line, text) in forecast.text.iter().enumerate() {
                    let target = forecast_item(height, row as f32, column, forecasts.len(), line);
                    label(text, target, 1.0, TextStyle::DETAILS);
                }
            }
        }

        label(
            &month.strftime("%B %Y").to_string(),
            TITLE,
            1.0,
            if hovered == Some(0) {
                TextStyle::CALENDAR_TITLE_HOVER
            } else {
                TextStyle::CALENDAR_TITLE
            },
        );
        for (text, side) in ARROWS {
            let position = arrow(side, header_reveal);
            label(
                text,
                position,
                1.0,
                if hovered == Some(side) {
                    TextStyle::CALENDAR_ARROW_HOVER
                } else {
                    TextStyle::CALENDAR_TITLE
                },
            );
        }

        let grid_start =
            month.saturating_sub(Span::new().days(month.weekday().to_monday_zero_offset()));
        for (column, weekday) in WEEKDAYS.iter().enumerate() {
            label(
                weekday,
                Vec2::new(cell(column).x, WEEKDAY_Y),
                0.75,
                TextStyle::DETAILS,
            );
        }

        for index in 0..DAYS_PER_WEEK * GRID_ROWS {
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
                        forecast.daily.temperature_2m_max[day],
                        forecast.daily.temperature_2m_min[day]
                    ),
                ],
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

fn hour_of_day(time: DateTime) -> f32 {
    f32::from(time.hour()) + f32::from(time.minute()) / 60.0 + f32::from(time.second()) / 3600.0
}

fn hour_at_offset(utc_offset_seconds: i32) -> f32 {
    let utc_hours = UNIX_EPOCH.elapsed().unwrap_or_default().as_secs_f64() / 3600.0;
    (utc_hours + f64::from(utc_offset_seconds) / 3600.0).rem_euclid(HOURS_PER_DAY) as f32
}

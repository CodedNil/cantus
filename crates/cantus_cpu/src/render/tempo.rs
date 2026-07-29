use crate::{
    AppUpdater,
    interaction::{InteractionState, Rect},
    openmeteo::{self, Forecast},
    render::{approach, text::TextStyle},
};
use arrayvec::ArrayString;
use cantus_gpu::{
    GAP, UNIT, smoothstep,
    tempo::{self, Data, WIDTH, WeatherCondition},
};
use glam::Vec2;
use jiff::{
    Span, Zoned,
    civil::{DateTime, Time},
    tz::Offset,
};
use std::{array::from_fn, fmt::Write};

const GRID_ROWS: usize = 6;
const GRID_ROW_HEIGHT: f32 = UNIT * 6.0;
const GRID_TOP_Y: f32 = UNIT * 24.0;
const WEEKDAY_Y: f32 = UNIT * 17.0;
const TITLE: Vec2 = Vec2::new(WIDTH * 0.5, UNIT * 10.0);
const DETAILS: Vec2 = Vec2::new(tempo::FORECAST_X + WIDTH * 0.5, TITLE.y);
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const ORDINALS: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];

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
}

const fn pill_rect(x: f32, height: f32) -> Rect {
    Rect::pill(x, WIDTH, height)
}

fn visible_rects(x: f32, height: f32, expansion: f32) -> [Rect; 2] {
    let pill = pill_rect(x, height);
    let expansion = smoothstep(0.0, 1.0, expansion);
    let size = tempo::popup_size(expansion);
    let x = tempo::expanded_x(pill.x0, expansion);
    [pill, Rect::new(x, pill.y1, x + size.x, pill.y1 + size.y)]
}

pub struct Weather {
    temperature: String,
    utc_offset: Option<Offset>,
    details: String,
    hourly: [ForecastItem; tempo::HOURLY_FORECASTS],
    daily: [ForecastItem; 5],
    data: Data,
    month_offset: i32,
}

impl Weather {
    pub fn new([latitude, longitude]: [f32; 2], updater: AppUpdater) -> Self {
        openmeteo::spawn_refresh_loop(latitude, longitude, updater);
        Self {
            temperature: "--.-°C".into(),
            utc_offset: None,
            details: "Weather unavailable".into(),
            hourly: Default::default(),
            daily: Default::default(),
            data: Data {
                sun_hours: [6.0, 18.0],
                ..Data::default()
            },
            month_offset: 0,
        }
    }

    pub fn scene(
        &mut self,
        x: f32,
        height: f32,
        ui: &InteractionState,
        dt: f32,
    ) -> (Data, ArrayString<64>, f32) {
        let hovered = visible_rects(x, height, self.data.calendar_expansion)
            .into_iter()
            .any(|rect| ui.contains(rect));
        approach(
            &mut self.data.calendar_expansion,
            f32::from(hovered),
            dt.min(1.0 / 30.0) * 3.0,
        );
        let time = Zoned::now();
        let hour = self.utc_offset.map_or_else(
            || hour_of_day(time.datetime()),
            |offset| hour_of_day(offset.to_datetime(time.timestamp())),
        );
        self.data.x = x;
        let clock = time.strftime("%a %d %b  %H:%M:%S");
        let mut label = ArrayString::new();
        write!(label, "{}   {clock}", self.temperature).unwrap();
        (self.data, label, hour)
    }

    pub fn calendar_labels(
        &mut self,
        x: f32,
        height: f32,
        ui: &mut InteractionState,
        mut draw: impl FnMut(&str, Vec2, f32, TextStyle),
    ) {
        let pill = pill_rect(x, height);
        if self.data.calendar_expansion <= 0.0 {
            ui.regions.push(pill);
            return;
        }
        let origin = Vec2::new(tempo::expanded_x(pill.x0, 1.0), pill.y1);
        let reveal = tempo::reveal_progress(self.data.calendar_expansion, TITLE.y);
        let title = ui.surface(Rect::from_center(
            origin + TITLE,
            Vec2::new(UNIT * 15.0, UNIT * 4.0),
        ));
        if title.clicked {
            self.month_offset = 0;
        }
        let arrows = [("<", -1), (">", 1)].map(|(text, side)| {
            let position = Vec2::new(
                WIDTH * 0.5 + side as f32 * (WIDTH * 0.5 - UNIT * 7.0) * reveal,
                TITLE.y - (1.0 - reveal) * UNIT * 3.0,
            );
            let response = ui.surface(Rect::from_center(origin + position, Vec2::splat(UNIT * 5.0)));
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
            let eased = tempo::reveal_progress(self.data.calendar_expansion, target.y);
            draw(text, origin + target, alpha * eased, style);
        };
        let forecasts = [&self.hourly[..], &self.daily[..]];
        let hovered_forecast = if self.data.calendar_expansion > 0.01 {
            forecasts.into_iter().enumerate().find_map(|(row, forecasts)| {
                let (row_origin, size) = tempo::forecast_row(height, row as f32);
                let column_width = size.x / forecasts.len() as f32;
                (0..forecasts.len()).find_map(|column| {
                    let x = origin.x + row_origin.x + column as f32 * column_width;
                    let rect = Rect::new(
                        x,
                        pill.y1 + row_origin.y,
                        x + column_width,
                        pill.y1 + row_origin.y + size.y,
                    );
                    ui.contains(rect).then(|| forecasts[column].hover_text.as_str())
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
        for (row, forecasts) in forecasts.into_iter().enumerate() {
            for (column, forecast) in forecasts.iter().enumerate() {
                for (line, text) in forecast.text.iter().enumerate() {
                    let position = Vec2::new(
                        tempo::FORECAST_X + (column as f32 + 0.5) * WIDTH / forecasts.len() as f32,
                        tempo::forecast_center(height, row as f32) + (line as f32 * 2.0 - 1.0) * GAP,
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

        for rect in visible_rects(x, height, self.data.calendar_expansion) {
            ui.regions.push(rect);
        }
    }

    pub fn apply_forecast(&mut self, forecast: &Forecast) {
        let raw = &forecast.current;
        self.utc_offset = Offset::from_seconds(forecast.utc_offset_seconds).ok();
        self.temperature = format!("{:.1}°C", raw.temperature_2m);
        self.data.sun_hours = [forecast.daily.sunrise[0], forecast.daily.sunset[0]].map(hour_of_day);
        self.data.hourly_start = hour_of_day(forecast.hourly.time[0]);
        let mut hourly_conditions = [WeatherCondition::default(); tempo::HOURLY_FORECASTS];
        self.hourly = from_fn(|index| {
            let source = index * tempo::HOURLY_STEP_HOURS;
            let time = forecast.hourly.time[source];
            hourly_conditions[index] = openmeteo::coded_conditions(forecast.hourly.weather_code[source]);
            ForecastItem {
                text: [
                    time.strftime("%H:%M").to_string(),
                    format!("{:.0}°", forecast.hourly.temperature_2m[source]),
                ],
                hover_text: format!(
                    "{} {} {:.0}°",
                    time.strftime("%H:%M"),
                    openmeteo::weather_code(forecast.hourly.weather_code[source]),
                    forecast.hourly.temperature_2m[source]
                ),
            }
        });
        hourly_conditions[0] = openmeteo::coded_conditions(raw.weather_code);
        self.data.hourly = hourly_conditions;
        let mut daily_conditions = [WeatherCondition::default(); 5];
        self.daily = from_fn(|day| {
            let day = day + 1;
            let date = forecast.daily.sunrise[day];
            let date_day = date.day();
            let suffix = if (11..=13).contains(&(date_day % 100)) {
                "th"
            } else {
                ORDINALS[(date_day % 10) as usize]
            };
            daily_conditions[day - 1] = openmeteo::coded_conditions(forecast.daily.weather_code[day]);
            ForecastItem {
                text: [
                    date.strftime("%a").to_string(),
                    format!(
                        "{:.0}°/{:.0}°",
                        forecast.daily.temperature_2m_max[day], forecast.daily.temperature_2m_min[day]
                    ),
                ],
                hover_text: format!(
                    "{}{} {} {:.0}°/{:.0}°",
                    date.strftime("%A %-d"),
                    suffix,
                    openmeteo::weather_code(forecast.daily.weather_code[day]),
                    forecast.daily.temperature_2m_max[day],
                    forecast.daily.temperature_2m_min[day]
                ),
            }
        });
        self.data.daily = daily_conditions;
        self.details = format!(
            "{} · Humidity {}% · Wind {:.0} km/h",
            openmeteo::weather_code(raw.weather_code),
            raw.relative_humidity_2m,
            raw.wind_speed_10m
        );
    }
}

fn hour_of_day(time: DateTime) -> f32 {
    time.time().duration_since(Time::midnight()).as_secs_f32() / 3600.0
}

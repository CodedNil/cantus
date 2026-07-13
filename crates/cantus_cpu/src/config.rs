use arrayvec::ArrayVec;
use cantus_shared::MAX_PILL_PLAYLIST_ICONS;
use serde::Deserialize;
use std::fs;
use tracing::warn;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    /// Spotify client ID to use for authentication.
    pub spotify_client_id: Option<String>,

    /// The monitor to display on.
    pub monitor: Option<String>,

    /// The width of the timeline in logical pixels.
    pub width: f32,
    /// The height of the timeline in logical pixels.
    pub height: f32,

    /// The layer the app should be on.
    pub layer: Layer,
    /// The corner/edge the application should anchor to.
    pub layer_anchor: LayerAnchor,

    /// How many minutes in the future to display in the timeline.
    pub timeline_future_minutes: f32,
    /// How many minutes before the current time to display in the timeline.
    pub timeline_past_minutes: f32,
    /// The width in logical pixels on the left where previous tracks are displayed.
    pub history_width: f32,

    /// Favourite playlists to display as buttons.
    pub playlists: ArrayVec<String, MAX_PILL_PLAYLIST_ICONS>,
    /// Whether star ratings should be enabled.
    pub ratings_enabled: bool,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerAnchor {
    Top,
    Bottom,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            spotify_client_id: None,
            monitor: None,
            width: 1050.0,
            height: 50.0,
            layer: Layer::Top,
            layer_anchor: LayerAnchor::Top,
            timeline_future_minutes: 12.0,
            timeline_past_minutes: 1.5,
            history_width: 100.0,
            playlists: ArrayVec::new(),
            ratings_enabled: false,
        }
    }
}

pub fn load() -> Config {
    let Some(config_dir) = dirs::config_dir() else {
        warn!("Falling back to default config, user config directory is unavailable");
        return Config::default();
    };
    let path = config_dir.join("cantus").join("cantus.toml");

    fs::read_to_string(&path)
        .inspect_err(|err| warn!("Falling back to default config, unable to read {path:?}: {err}"))
        .ok()
        .and_then(|contents| {
            toml::from_str::<Config>(&contents)
                .inspect_err(|err| {
                    warn!("Falling back to default config, failed to parse {path:?}: {err}");
                })
                .ok()
        })
        .unwrap_or_default()
}

impl Config {
    pub fn timeline_width(&self) -> f32 {
        self.width - self.history_width - 16.0
    }

    pub fn timeline_duration_ms(&self) -> f32 {
        self.timeline_future_minutes * 60_000.0
    }

    pub fn timeline_start_ms(&self) -> f32 {
        -self.timeline_past_minutes * 60_000.0
    }

    pub fn px_per_ms(&self) -> f32 {
        self.timeline_width() / self.timeline_duration_ms()
    }

    pub fn playhead_x(&self) -> f32 {
        self.history_width - self.timeline_start_ms() * self.px_per_ms()
    }
}

use serde::Deserialize;
use std::{fs, sync::LazyLock};
use tracing::warn;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    // Spotify client ID
    pub spotify_client_id: Option<String>,

    /// The monitor to display on.
    pub monitor: Option<String>,

    /// The width of the timeline in pixels.
    pub width: f32,
    /// The height of the timeline in pixels.
    pub height: f32,

    /// The layer the app should be on.
    ///
    /// Can be one of 'background', 'bottom', 'top', or 'overlay'.
    pub layer: String,
    /// The corner/edge the application should anchor to.
    ///
    /// Can be one of 'top' or 'bottom'.
    pub layer_anchor: String,

    /// How many minutes in the future to display in the timeline.
    pub timeline_future_minutes: f32,
    /// How many minutes before the current time to display in the timeline.
    pub timeline_past_minutes: f32,
    /// The width in pixels on the left where previous tracks are displayed.
    pub history_width: f32,

    /// Array of favourite playlists to display as buttons.
    pub playlists: Vec<String>,
    /// Should star ratings be enabled
    pub ratings_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            spotify_client_id: None,
            monitor: None,
            width: 1050.0,
            height: 50.0,
            layer: "top".into(),
            layer_anchor: "top".into(),
            timeline_future_minutes: 12.0,
            timeline_past_minutes: 1.5,
            history_width: 100.0,
            playlists: Vec::new(),
            ratings_enabled: false,
        }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(load_config);

fn load_config() -> Config {
    let path = dirs::config_dir()
        .expect("config directory unavailable")
        .join("cantus")
        .join("cantus.toml");

    match fs::read_to_string(&path) {
        Ok(contents) => match toml::from_str::<Config>(&contents) {
            Ok(config) => config,
            Err(err) => {
                warn!("Falling back to default config, failed to parse {path:?}: {err}");
                Config::default()
            }
        },
        Err(err) => {
            warn!("Falling back to default config, unable to read {path:?}: {err}");
            Config::default()
        }
    }
}

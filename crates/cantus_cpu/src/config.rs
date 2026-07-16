use arrayvec::ArrayVec;
use cantus_shared::MAX_PILL_PLAYLIST_ICONS;
use serde::Deserialize;
use std::{env, fs, path::PathBuf};
use tracing::warn;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    /// Spotify client ID to use for authentication.
    pub spotify_client_id: Option<String>,

    /// The monitor to display on.
    pub monitor: Option<String>,

    /// The height of the timeline in logical pixels.
    pub height: f32,

    /// Latitude and longitude used for the weather pill.
    pub location: [f32; 2],

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
            height: 50.0,
            location: [51.5074, -0.1278],
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

pub fn directory() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_default()
        .join("cantus")
}

pub fn load() -> Config {
    let path = directory().join("cantus.toml");
    fs::read_to_string(&path)
        .map_err(|error| error.to_string())
        .and_then(|contents| toml::from_str(&contents).map_err(|error| error.to_string()))
        .unwrap_or_else(|error| {
            warn!("Falling back to default config for {path:?}: {error}");
            Config::default()
        })
}

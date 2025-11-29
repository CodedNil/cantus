//! All objects related to context
use super::{custom_serde::option_duration_ms, device::Device, track::Track};
use chrono::Duration;
use serde::Deserialize;

/// Context object
#[derive(Deserialize)]
pub struct Context {
    /// The URI may be of any type, so it's not parsed into a [`crate::Id`]
    pub uri: String,
}

#[derive(Deserialize)]
pub struct CurrentPlaybackContext {
    pub device: Device,
    pub context: Option<Context>,
    #[serde(default)]
    #[serde(with = "option_duration_ms", rename = "progress_ms")]
    pub progress: Option<Duration>,
    pub is_playing: bool,
    pub item: Option<Track>,
}

#[derive(Deserialize)]
pub struct CurrentUserQueue {
    pub currently_playing: Option<Track>,
    pub queue: Vec<Track>,
}

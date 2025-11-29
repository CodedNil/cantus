//! All objects related to context
use super::{custom_serde::option_duration_ms, device::Device, track::FullTrack};
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use std::collections::HashMap;

/// Context object
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Context {
    /// The URI may be of any type, so it's not parsed into a [`crate::Id`]
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CurrentPlaybackContext {
    pub device: Device,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    #[serde(with = "option_duration_ms", rename = "progress_ms")]
    pub progress: Option<Duration>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CurrentUserQueue {
    pub currently_playing: Option<FullTrack>,
    pub queue: Vec<FullTrack>,
}

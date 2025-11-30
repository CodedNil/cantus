use super::custom_serde::{duration_ms, option_duration_ms, tracks_total};
use arrayvec::ArrayString;
use chrono::Duration;
use serde::{Deserialize, de::DeserializeOwned};
use thiserror::Error;

// Albums
pub type AlbumId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Album {
    pub id: AlbumId,
    #[serde(default)]
    pub images: Vec<Image>,
}

// Artists
pub type ArtistId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    #[serde(default)]
    pub images: Vec<Image>,
}

#[derive(Deserialize)]
pub struct Artists {
    pub artists: Vec<Artist>,
}

// Track
pub type TrackId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
}

#[derive(Deserialize)]
pub struct PartialTrack {
    pub id: TrackId,
}

// Playlist
pub type PlaylistId = ArrayString<22>;

/// Simplified playlist object
#[derive(Deserialize)]
pub struct Playlist {
    pub id: PlaylistId,
    #[serde(default)]
    pub images: Vec<Image>,
    pub name: String,
    pub snapshot_id: ArrayString<32>,
    #[serde(rename = "tracks", with = "tracks_total")]
    pub total_tracks: u32,
}

/// Playlist track object
#[derive(Deserialize, Default)]
pub struct PlaylistItem {
    pub track: Option<PartialTrack>,
}

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

/// Device object
#[derive(Deserialize)]
pub struct Device {
    pub volume_percent: Option<u32>,
}

/// Image object
#[derive(Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<u32>,
}

/// Custom deserializer to handle `Vec<Option<T>>` and filter out `None` values
/// This is useful for deserializing lists that may contain null values that are not relevants
fn vec_without_nulls<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let v = Vec::<Option<T>>::deserialize(deserializer)?;
    Ok(v.into_iter().flatten().collect())
}

#[derive(Deserialize)]
pub struct Page<T: DeserializeOwned> {
    #[serde(deserialize_with = "vec_without_nulls")]
    pub items: Vec<T>,
    pub total: u32,
}

/// Groups up the kinds of errors that may happen in this crate.
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
}

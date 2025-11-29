//! All kinds of playlists objects
use super::{Image, PlaylistId, track::FullTrack};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Playlist result object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlaylistResult {
    pub snapshot_id: String,
}

/// Playlist Track Reference Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlaylistTracksRef {
    pub href: String,
    pub total: u32,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

/// Simplified playlist object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedPlaylist {
    pub href: String,
    pub id: PlaylistId,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub images: Vec<Image>,
    pub name: String,
    pub snapshot_id: String,
    pub tracks: PlaylistTracksRef,
}

/// Playlist track object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlaylistItem {
    pub added_at: Option<DateTime<Utc>>,
    pub is_local: bool,
    pub track: Option<FullTrack>,
}

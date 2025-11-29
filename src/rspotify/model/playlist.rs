//! All kinds of playlists objects
use super::{image::Image, track::PartialTrack};
use arrayvec::ArrayString;
use serde::Deserialize;

pub type PlaylistId = ArrayString<22>;

#[derive(Deserialize, Default)]
pub struct PlaylistTracksRef {
    pub total: u32,
}

/// Simplified playlist object
#[derive(Deserialize)]
pub struct Playlist {
    pub id: PlaylistId,
    #[serde(default)]
    pub images: Vec<Image>,
    pub name: String,
    pub snapshot_id: String,
    pub tracks: PlaylistTracksRef,
}

/// Playlist track object
#[derive(Deserialize, Default)]
pub struct PlaylistItem {
    pub track: Option<PartialTrack>,
}

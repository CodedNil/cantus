//! All kinds of tracks object
use super::{
    album::SimplifiedAlbum, artist::SimplifiedArtist, custom_serde::duration_ms, idtypes::TrackId,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};

/// Full track object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullTrack {
    pub id: TrackId,
    pub name: String,
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
}

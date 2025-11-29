use super::{album::Album, artist::Artist, custom_serde::duration_ms};
use arrayvec::ArrayString;
use chrono::Duration;
use serde::Deserialize;

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

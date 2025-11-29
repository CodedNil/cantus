//! All Spotify API endpoint response objects. Please refer to the endpoints
//! where they are used for a link to their reference in the Spotify API
//! documentation.
pub mod album;
pub mod artist;
pub mod auth;
pub mod category;
pub mod context;
pub(crate) mod custom_serde;
pub mod device;
pub mod enums;
pub mod error;
pub mod idtypes;
pub mod image;
pub mod offset;
pub mod page;
pub mod playing;
pub mod playlist;
pub mod recommend;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

pub use {
    album::*, artist::*, context::*, device::*, enums::*, error::*, idtypes::*, image::*, page::*,
    playlist::*, show::*, track::*, user::*,
};

use serde::{Deserialize, Serialize};

/// Followers object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

/// A full track object or a full episode object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayableItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
    // The fallback variant to store the raw JSON for anything that doesn't parse
    // see https://github.com/ramsayleung/rspotify/issues/525 for
    // detail
    Unknown(serde_json::Value),
}

impl PlayableItem {
    /// Check if this is an unknown/malformed item that couldn't be parsed as Track or Episode.
    ///
    /// Returns `true` if the item was captured as raw JSON due to schema mismatch.
    pub fn is_unknown(&self) -> bool {
        matches!(self, PlayableItem::Unknown(_))
    }

    /// Utility to get the ID from either variant in the enum.
    ///
    /// Note that if it's a track and if it's local, it may not have an ID, in
    /// which case this function will return `None`.
    #[must_use]
    pub fn id(&self) -> Option<PlayableId<'_>> {
        match self {
            PlayableItem::Track(t) => t.id.as_ref().map(|t| PlayableId::Track(t.as_ref())),
            PlayableItem::Episode(e) => Some(PlayableId::Episode(e.id.as_ref())),
            PlayableItem::Unknown(value) => {
                let id_str = value.get("id")?.as_str()?;
                if let Some(type_str) = value.get("type").and_then(|v| v.as_str()) {
                    match type_str {
                        "episode" => Some(PlayableId::Episode(EpisodeId::from_id(id_str).ok()?)),
                        _ => Some(PlayableId::Track(TrackId::from_id(id_str).ok()?)),
                    }
                } else {
                    // Default to track if type is unclear
                    Some(PlayableId::Track(TrackId::from_id(id_str).ok()?))
                }
            }
        }
    }
}

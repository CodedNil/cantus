//! All Spotify API endpoint response objects. Please refer to the endpoints
//! where they are used for a link to their reference in the Spotify API
//! documentation.
pub mod album;
pub mod artist;
pub mod auth;
pub mod context;
pub mod custom_serde;
pub mod device;
pub mod error;
pub mod idtypes;
pub mod image;
pub mod page;
pub mod playlist;
pub mod track;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// Followers object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

/// Type: `artist`, `album`, `track`, `playlist`, `show` or `episode`
#[derive(
    Clone, Serialize, Deserialize, PartialEq, Eq, Debug, Display, EnumString, IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Type {
    Artist,
    Album,
    Track,
    Playlist,
    User,
    Show,
    Episode,
    Collection,
    // Fallback variant to capture unknown variants introduced by
    // Spotify rather than raise a deserialization error.
    #[serde(untagged)]
    Unknown(String),
}

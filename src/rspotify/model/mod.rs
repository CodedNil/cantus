//! All Spotify API endpoint response objects. Please refer to the endpoints
//! where they are used for a link to their reference in the Spotify API
//! documentation.
pub mod album;
pub mod artist;
pub mod auth;
pub mod context;
pub mod custom_serde;
pub mod device;
pub mod enums;
pub mod error;
pub mod idtypes;
pub mod image;
pub mod page;
pub mod playlist;
pub mod track;

pub use {
    album::*, artist::*, context::*, device::*, enums::*, error::*, idtypes::*, image::*, page::*,
    playlist::*,
};

use serde::{Deserialize, Serialize};

/// Followers object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

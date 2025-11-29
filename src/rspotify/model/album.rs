//! All objects related to album defined by Spotify API
use super::{AlbumId, Image, RestrictionReason, SimplifiedArtist};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Simplified Album Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SimplifiedAlbum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_group: Option<String>,
    pub album_type: Option<String>,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<AlbumId>,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restriction>,
}

/// Album restriction object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub reason: RestrictionReason,
}

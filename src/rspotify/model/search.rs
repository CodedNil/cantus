//! All object related to search
use super::{
    FullArtist, FullTrack, Page, SimplifiedAlbum, SimplifiedEpisode, SimplifiedPlaylist,
    SimplifiedShow,
};
use serde::{Deserialize, Serialize};

/// Search result of any kind
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchResult {
    #[serde(rename = "playlists")]
    Playlists(Page<SimplifiedPlaylist>),
    #[serde(rename = "albums")]
    Albums(Page<SimplifiedAlbum>),
    #[serde(rename = "artists")]
    Artists(Page<FullArtist>),
    #[serde(rename = "tracks")]
    Tracks(Page<FullTrack>),
    #[serde(rename = "shows")]
    Shows(Page<SimplifiedShow>),
    #[serde(rename = "episodes")]
    Episodes(Page<SimplifiedEpisode>),
}

/// Search result of any multiple kinds
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchMultipleResult {
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub albums: Option<Page<SimplifiedAlbum>>,
    pub artists: Option<Page<FullArtist>>,
    pub tracks: Option<Page<FullTrack>>,
    pub shows: Option<Page<SimplifiedShow>>,
    pub episodes: Option<Page<SimplifiedEpisode>>,
}

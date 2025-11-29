use crate::rspotify::model::Type;
use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};
use strum::Display;
use thiserror::Error;

/// Spotify ID or URI parsing error
///
/// See also [`Id`] for details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, Error)]
pub enum IdError {
    /// Spotify URI prefix is not `spotify:` or `spotify/`.
    Prefix,
    /// Spotify URI can't be split into type and id parts (e.g., it has invalid
    /// separator).
    Format,
    /// Spotify URI has invalid type name, or id has invalid type in a given
    /// context (e.g. a method expects a track id, but artist id is provided).
    Type,
    /// Spotify id is invalid (empty or contains invalid characters).
    Id,
}

/// A lower level function to parse a URI into both its type and its actual ID.
/// Note that this function doesn't check the validity of the returned ID (e.g.,
/// whether it's alphanumeric; that should be done in `Id::from_id`).
///
/// This is only useful for advanced use-cases, such as implementing your own ID
/// type.
pub fn parse_uri(uri: &str) -> Result<(Type, &str), IdError> {
    let mut chars = uri.strip_prefix("spotify").ok_or(IdError::Prefix)?.chars();
    let sep = match chars.next() {
        Some(ch) if ch == '/' || ch == ':' => ch,
        _ => return Err(IdError::Prefix),
    };
    let rest = chars.as_str();

    let (tpe, id) = rest
        .rfind(sep)
        .map(|mid| rest.split_at(mid))
        .ok_or(IdError::Format)?;

    // Note that in case the type isn't known at compile time,
    // any type will be accepted.
    tpe.parse::<Type>()
        .map_or(Err(IdError::Type), |tpe| Ok((tpe, &id[1..])))
}

/// ID of type `Artist`. Requires alphanumeric characters only.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize, Eq, Hash)]
#[serde(try_from = "String")]
pub struct ArtistId(ArrayString<22>);

impl ArtistId {
    /// Parse Spotify ID from string slice.
    pub fn from_id(id: &str) -> Result<Self, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            let mut string = ArrayString::<22>::new();
            string.push_str(id);
            Ok(Self(string))
        } else {
            Err(IdError::Id)
        }
    }

    /// Parse Spotify URI from string slice
    pub fn from_uri(uri: &str) -> Result<Self, IdError> {
        let (tpe, id) = parse_uri(uri)?;
        if tpe == Type::Artist {
            Self::from_id(id)
        } else {
            Err(IdError::Type)
        }
    }

    /// Returns the inner id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Returns a Spotify object URI in a well-known format: `spotify:track:id`.
    pub fn uri(&self) -> String {
        format!("spotify:artist:{}", self.id())
    }

    pub fn join_ids(ids: impl IntoIterator<Item = Self>) -> String {
        ids.into_iter().map(|id| id.0).collect::<Vec<_>>().join(",")
    }
}

/// Deserialize from string
impl TryFrom<String> for ArtistId {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id_or_uri: &str = &value;
        match Self::from_uri(id_or_uri) {
            Ok(id) => Ok(id),
            Err(IdError::Prefix) => Self::from_id(id_or_uri),
            Err(error) => Err(error),
        }
        .map_err(|e| format!("Invalid ArtistId: {e}"))
    }
}

/// Displaying the ID shows its URI
impl std::fmt::Display for ArtistId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.uri())
    }
}

/// ID of type `Album`. Requires alphanumeric characters only.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize, Eq, Hash)]
#[serde(try_from = "String")]
pub struct AlbumId(ArrayString<22>);

impl AlbumId {
    /// Parse Spotify ID from string slice.
    pub fn from_id(id: &str) -> Result<Self, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            let mut string = ArrayString::<22>::new();
            string.push_str(id);
            Ok(Self(string))
        } else {
            Err(IdError::Id)
        }
    }

    /// Parse Spotify URI from string slice
    pub fn from_uri(uri: &str) -> Result<Self, IdError> {
        let (tpe, id) = parse_uri(uri)?;
        if tpe == Type::Album {
            Self::from_id(id)
        } else {
            Err(IdError::Type)
        }
    }

    /// Returns the inner id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Returns a Spotify object URI in a well-known format: `spotify:track:id`.
    pub fn uri(&self) -> String {
        format!("spotify:album:{}", self.id())
    }
}

/// Deserialize from string
impl TryFrom<String> for AlbumId {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id_or_uri: &str = &value;
        match Self::from_uri(id_or_uri) {
            Ok(id) => Ok(id),
            Err(IdError::Prefix) => Self::from_id(id_or_uri),
            Err(error) => Err(error),
        }
        .map_err(|e| format!("Invalid AlbumId: {e}"))
    }
}

/// Displaying the ID shows its URI
impl std::fmt::Display for AlbumId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.uri())
    }
}

/// ID of type `Track`. Requires alphanumeric characters only.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize, Eq, Hash)]
#[serde(try_from = "String")]
pub struct TrackId(ArrayString<22>);

impl TrackId {
    /// Parse Spotify ID from string slice.
    pub fn from_id(id: &str) -> Result<Self, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            let mut string = ArrayString::<22>::new();
            string.push_str(id);
            Ok(Self(string))
        } else {
            Err(IdError::Id)
        }
    }

    /// Parse Spotify URI from string slice
    pub fn from_uri(uri: &str) -> Result<Self, IdError> {
        let (tpe, id) = parse_uri(uri)?;
        if tpe == Type::Track {
            Self::from_id(id)
        } else {
            Err(IdError::Type)
        }
    }

    /// Returns the inner id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Returns a Spotify object URI in a well-known format: `spotify:track:id`.
    pub fn uri(&self) -> String {
        format!("spotify:track:{}", self.id())
    }

    pub fn join_ids(ids: impl IntoIterator<Item = Self>) -> String {
        ids.into_iter().map(|id| id.0).collect::<Vec<_>>().join(",")
    }
}

/// Deserialize from string
impl TryFrom<String> for TrackId {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id_or_uri: &str = &value;
        match Self::from_uri(id_or_uri) {
            Ok(id) => Ok(id),
            Err(IdError::Prefix) => Self::from_id(id_or_uri),
            Err(error) => Err(error),
        }
        .map_err(|e| format!("Invalid TrackId: {e}"))
    }
}

/// Displaying the ID shows its URI
impl std::fmt::Display for TrackId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.uri())
    }
}

/// ID of type `Playlist`. Requires alphanumeric characters only.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize, Eq, Hash)]
#[serde(try_from = "String")]
pub struct PlaylistId(ArrayString<22>);

impl PlaylistId {
    /// Parse Spotify ID from string slice.
    pub fn from_id(id: &str) -> Result<Self, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            let mut string = ArrayString::<22>::new();
            string.push_str(id);
            Ok(Self(string))
        } else {
            Err(IdError::Id)
        }
    }

    /// Parse Spotify URI from string slice
    pub fn from_uri(uri: &str) -> Result<Self, IdError> {
        let (tpe, id) = parse_uri(uri)?;
        if tpe == Type::Playlist {
            Self::from_id(id)
        } else {
            Err(IdError::Type)
        }
    }

    /// Returns the inner id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Returns a Spotify object URI in a well-known format: `spotify:track:id`.
    pub fn uri(&self) -> String {
        format!("spotify:playlist:{}", self.id())
    }
}

/// Deserialize from string
impl TryFrom<String> for PlaylistId {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id_or_uri: &str = &value;
        match Self::from_uri(id_or_uri) {
            Ok(id) => Ok(id),
            Err(IdError::Prefix) => Self::from_id(id_or_uri),
            Err(error) => Err(error),
        }
        .map_err(|e| format!("Invalid PlaylistId: {e}"))
    }
}

/// Displaying the ID shows its URI
impl std::fmt::Display for PlaylistId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.uri())
    }
}

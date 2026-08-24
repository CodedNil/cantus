use super::{ClientResult, PLAYLIST_TRACKS_CACHE, PlaylistTracks, RATING_PLAYLISTS, SpotifyWorker, config_path, write_cache};
use crate::app::{
    music::{ArtState, CondensedPlaylist, PlaylistId, TrackId},
    send_update,
};
use librespot_core::{FileId, Session, SpotifyId};
use librespot_protocol::playlist4_external::{Add, Delta, Item, ListAttributes, ListChanges, Op, Rem, SelectedListContent, op};
use protobuf::{Message as _, MessageField};
use reqwest::Method;
use std::{
    str,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{error, warn};

impl SpotifyWorker {
    pub(super) async fn update_library(&mut self, track_id: TrackId, changes: &[(PlaylistId, bool)], liked: Option<bool>) {
        let uri = format!("spotify:track:{track_id}");
        for &(playlist_id, add) in changes {
            let Some(revision) = self.playlist_cache.get(&playlist_id).map(|(revision, _)| revision.clone()) else {
                warn!(%playlist_id, "Spotify playlist is not loaded");
                continue;
            };
            let item = Item {
                uri: Some(uri.clone()),
                ..Default::default()
            };
            let operation = if add {
                Op {
                    kind: Some(op::Kind::ADD.into()),
                    add: MessageField::some(Add {
                        items: vec![item],
                        add_last: Some(true),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            } else {
                Op {
                    kind: Some(op::Kind::REM.into()),
                    rem: MessageField::some(Rem {
                        items: vec![item],
                        items_as_key: Some(true),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            };
            let request = ListChanges {
                base_revision: Some(revision.clone()),
                deltas: vec![Delta {
                    base_version: Some(revision),
                    ops: vec![operation],
                    ..Default::default()
                }],
                want_resulting_revisions: Some(true),
                ..Default::default()
            };
            let result = self
                .request_connected_proto(&Method::POST, &format!("/playlist/v2/playlist/{playlist_id}/changes"), &request)
                .await;
            if let Err(error) = result {
                error!(%error, %playlist_id, "Failed to update Spotify playlist");
            } else if let Some((_, tracks)) = self.playlist_cache.get_mut(&playlist_id) {
                let tracks = Arc::make_mut(tracks);
                if add {
                    tracks.insert(track_id);
                } else {
                    tracks.remove(&track_id);
                }
            }
        }
        let Some(should_like) = liked else {
            return;
        };
        let username = self.session.username();
        let body = match collection_write(track_id, !should_like) {
            Ok(body) => body,
            Err(error) => {
                error!(%error, %track_id, "Failed to encode Spotify library update");
                return;
            }
        };
        if let Err(error) = self.request_connected(&Method::POST, &format!("/collection/collection/{username}"), body).await {
            error!(%error, %track_id, "Failed to update Spotify library");
        }
    }

    pub(super) async fn refresh_playlists(&mut self) {
        if let Err(error) = self.load_playlists().await {
            warn!(%error, "Failed to refresh Spotify playlists");
        }
    }

    async fn load_playlists(&mut self) -> ClientResult<()> {
        let root = SelectedListContent::parse_from_bytes(&self.session.spclient().get_rootlist(0, Some(10_000)).await?)?;
        let mut cache_changed = false;
        let mut updates = Vec::new();
        for (item, metadata) in root.contents.get_or_default().items.iter().zip(&root.contents.get_or_default().meta_items) {
            let Some(id) = item.uri().strip_prefix("spotify:playlist:").and_then(|id| id.parse::<PlaylistId>().ok()) else {
                continue;
            };
            let attributes = metadata.attributes.get_or_default();
            let name = attributes.name();
            let rating_index = RATING_PLAYLISTS
                .iter()
                .position(|rating| *rating == name)
                .filter(|_| self.ratings_enabled)
                .map(|index| index as u8);
            if !self.playlist_targets.iter().any(|target| target == name) && rating_index.is_none() {
                continue;
            }
            let tracks = if let Some((_, tracks)) = self.playlist_cache.get(&id).filter(|(revision, _)| revision.as_slice() == metadata.revision()) {
                Arc::clone(tracks)
            } else {
                let tracks = fetch_playlist_tracks(&self.session, id).await?;
                self.playlist_cache.insert(id, (metadata.revision().to_vec(), Arc::clone(&tracks)));
                cache_changed = true;
                tracks
            };
            updates.push(CondensedPlaylist {
                id,
                name: name.to_owned(),
                image_url: playlist_image(attributes),
                art: ArtState::default(),
                tracks,
                rating_index,
            });
        }
        let cached = self.playlist_cache.len();
        self.playlist_cache.retain(|id, _| updates.iter().any(|playlist| playlist.id == *id));
        cache_changed |= cached != self.playlist_cache.len();
        send_update(&self.updater, move |app| {
            for playlist in &mut updates {
                if let Some(old) = app.music.playlists.iter().find(|old| old.id == playlist.id && old.image_url == playlist.image_url) {
                    playlist.art = old.art.clone();
                }
            }
            updates.sort_unstable_by(|a, b| a.name.cmp(&b.name));
            app.music.playlists = updates;
            app.refresh_enrichment(false);
        });
        if cache_changed && let Err(err) = write_cache(&config_path(PLAYLIST_TRACKS_CACHE), &self.playlist_cache) {
            warn!("Failed to persist playlist cache: {err}");
        }
        Ok(())
    }
}

async fn fetch_playlist_tracks(session: &Session, id: PlaylistId) -> ClientResult<PlaylistTracks> {
    let spotify_id = SpotifyId::from_base62(&id)?;
    let playlist = SelectedListContent::parse_from_bytes(&session.spclient().get_playlist(&spotify_id).await?)?;
    Ok(Arc::new(
        playlist
            .contents
            .get_or_default()
            .items
            .iter()
            .filter_map(|item| item.uri().strip_prefix("spotify:track:")?.parse().ok())
            .collect(),
    ))
}

fn playlist_image(attributes: &ListAttributes) -> Option<String> {
    attributes
        .picture_size
        .iter()
        .rev()
        .find_map(|picture| {
            let value = picture.url();
            (!value.is_empty()).then(|| {
                value
                    .strip_prefix("spotify:image:")
                    .map_or_else(|| value.to_owned(), |id| format!("https://i.scdn.co/image/{id}"))
            })
        })
        .or_else(|| {
            let picture = attributes.picture();
            let id = str::from_utf8(picture)
                .ok()
                .and_then(|picture| picture.strip_prefix("spotify:image:"))
                .map(str::to_owned)
                .or_else(|| (picture.len() == 20).then(|| FileId::from_raw(picture).to_string()))?;
            Some(format!("https://i.scdn.co/image/{id}"))
        })
}

fn collection_write(track_id: TrackId, removed: bool) -> ClientResult<Vec<u8>> {
    let mut item = vec![0x12, 0x10];
    item.extend_from_slice(&SpotifyId::from_base62(&track_id)?.to_raw());
    if removed {
        item.extend_from_slice(&[0x30, 1]);
    } else {
        item.push(0x28);
        write_varint(&mut item, SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
    }
    let mut collection = vec![0x0a];
    write_varint(&mut collection, item.len() as u64);
    collection.extend(item);
    Ok(collection)
}

fn write_varint(output: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        output.push(value as u8 | 0x80);
        value >>= 7;
    }
    output.push(value as u8);
}

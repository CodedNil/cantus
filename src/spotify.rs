use crate::{
    background::update_color_palettes,
    config::CONFIG,
    rspotify::{
        AlbumId, ArtistId, Artists, CurrentPlaybackContext, CurrentUserQueue, Page, Playlist,
        PlaylistId, PlaylistItem, SpotifyClient, Track, TrackId,
    },
};
use arrayvec::ArrayString;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::LazyLock,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};
use tracing::{error, info, warn};
use vello::peniko::{
    Blob, Extend, ImageAlphaType, ImageBrush, ImageData, ImageFormat, ImageQuality, ImageSampler,
};

pub static PLAYBACK_STATE: LazyLock<RwLock<PlaybackState>> = LazyLock::new(|| {
    RwLock::new(PlaybackState {
        playing: false,
        progress: 0,
        volume: None,
        queue: Vec::new(),
        queue_index: 0,
        playlists: HashMap::new(),

        current_context: None,
        context_updated: false,

        interaction: false,
        last_interaction: Instant::now(),
        last_progress_update: Instant::now(),
        last_grabbed_playback: Instant::now().checked_sub(Duration::from_secs(60)).unwrap(),
        last_grabbed_queue: Instant::now().checked_sub(Duration::from_secs(60)).unwrap(),
    })
});
pub static IMAGES_CACHE: LazyLock<DashMap<String, Option<ImageBrush>>> =
    LazyLock::new(DashMap::new);
pub static ALBUM_DATA_CACHE: LazyLock<DashMap<AlbumId, Option<AlbumData>>> =
    LazyLock::new(DashMap::new);
pub static ARTIST_DATA_CACHE: LazyLock<DashMap<ArtistId, Option<String>>> =
    LazyLock::new(DashMap::new);

const RATING_PLAYLISTS: [&str; 10] = [
    "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

pub static SPOTIFY_CLIENT: LazyLock<SpotifyClient> = LazyLock::new(|| {
    // Initialize Spotify client with credentials and OAuth scopes
    let mut scopes = HashSet::new();
    scopes.insert("user-read-playback-state".to_owned());
    scopes.insert("user-modify-playback-state".to_owned());
    scopes.insert("user-read-currently-playing".to_owned());
    scopes.insert("playlist-read-private".to_owned());
    scopes.insert("playlist-read-collaborative".to_owned());
    scopes.insert("playlist-modify-private".to_owned());
    scopes.insert("playlist-modify-public".to_owned());
    scopes.insert("user-library-read".to_owned());
    scopes.insert("user-library-modify".to_owned());
    SpotifyClient::new(
        CONFIG.spotify_client_id.clone().expect(
            "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
        ),
        &scopes,
        dirs::config_dir()
            .unwrap()
            .join("cantus")
            .join("spotify_cache.json"),
    )
});

pub struct PlaybackState {
    pub playing: bool,
    pub progress: u32,
    pub volume: Option<u8>,
    pub queue: Vec<Track>,
    pub queue_index: usize,
    pub playlists: HashMap<PlaylistId, CondensedPlaylist>,

    current_context: Option<String>,
    context_updated: bool,

    pub interaction: bool,
    pub last_interaction: Instant,
    pub last_progress_update: Instant,
    last_grabbed_playback: Instant,
    last_grabbed_queue: Instant,
}

pub struct AlbumData {
    /// Simplified color palette (RGBA, alpha = percentage 0-100).
    pub primary_colors: Vec<[u8; 4]>,
}

pub struct CondensedPlaylist {
    pub id: PlaylistId,
    pub name: String,
    pub image_url: String,
    pub tracks: HashSet<TrackId>,
    pub tracks_total: u32,
    snapshot_id: ArrayString<32>,
    pub rating_index: Option<u8>,
}

/// Mutably updates the global playback state.
pub fn update_playback_state<F>(update: F)
where
    F: FnOnce(&mut PlaybackState),
{
    let mut state = PLAYBACK_STATE.write();
    update(&mut state);
}

type PlaylistCache = HashMap<PlaylistId, (ArrayString<32>, HashSet<TrackId>)>;

fn load_cached_playlist_tracks() -> PlaylistCache {
    let cache_path = dirs::config_dir()
        .unwrap()
        .join("cantus")
        .join("cantus_playlist_tracks.json");
    let bytes = match fs::read(&cache_path) {
        Ok(bytes) => bytes,
        Err(err) => {
            warn!("Failed to read playlist cache at {cache_path:?}: {err}");
            return HashMap::new();
        }
    };

    match serde_json::from_slice::<PlaylistCache>(&bytes) {
        Ok(map) => map,
        Err(err) => {
            warn!("Failed to parse playlist cache at {cache_path:?}: {err}",);
            HashMap::new()
        }
    }
}

fn persist_playlist_cache() {
    let cache_payload: PlaylistCache = PLAYBACK_STATE
        .read()
        .playlists
        .values()
        .map(|playlist| {
            (
                playlist.id,
                (
                    playlist.snapshot_id,
                    playlist.tracks.iter().copied().collect(),
                ),
            )
        })
        .collect();
    if cache_payload.is_empty() {
        return;
    }

    let cache_path = dirs::config_dir()
        .unwrap()
        .join("cantus")
        .join("cantus_playlist_tracks.json");
    match serde_json::to_vec(&cache_payload) {
        Ok(serialized) => {
            if let Err(err) = fs::write(cache_path.clone(), serialized) {
                warn!("Failed to write playlist cache at {cache_path:?}: {err}");
            }
        }
        Err(err) => {
            warn!(
                "Failed to serialise playlist cache for {} playlists: {err}",
                cache_payload.len(),
            );
        }
    }
}

/// Init the spotify client
pub fn init() {
    // Make sure the cantus directory exists
    let cantus_dir = dirs::config_dir().unwrap().join("cantus");
    if !cantus_dir.exists() {
        std::fs::create_dir(&cantus_dir).unwrap();
    }

    // Ensure SPOTIFY_CLIENT is initialized
    let _ = &*SPOTIFY_CLIENT;

    // Begin polling
    spawn(poll_playlists);
    spawn(|| {
        loop {
            get_spotify_playback();
            get_spotify_queue();
            sleep(Duration::from_millis(500));
        }
    });
}

/// Pulls the current playback queue and status from the Spotify Web API and updates shared state.
fn get_spotify_playback() {
    // Wait if we have recently interacted with spotify
    let now = Instant::now();
    {
        let state = PLAYBACK_STATE.read();
        if now < state.last_interaction
            || now < state.last_grabbed_playback + Duration::from_secs(1)
        {
            return;
        }
    }

    // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-information-about-the-users-current-playback
    let result = match SPOTIFY_CLIENT.api_get("me/player") {
        Ok(result) => {
            if result.is_empty() {
                // Spotify is not playing anything
                update_playback_state(|state| {
                    state.last_grabbed_playback = Instant::now();
                });
                return;
            }
            result
        }
        Err(err) => {
            update_playback_state(|state| {
                state.last_grabbed_playback = Instant::now();
            });
            error!("Failed to fetch current playback: {err}");
            return;
        }
    };
    let current_playback = match serde_json::from_str::<CurrentPlaybackContext>(&result) {
        Ok(playback) => playback,
        Err(err) => {
            update_playback_state(|state| {
                state.last_grabbed_playback = Instant::now();
            });
            error!("Failed to parse current playback: {err}");
            return;
        }
    };
    let request_duration = now.elapsed();

    // Update the playback state
    update_playback_state(|state| {
        let new_context = current_playback.context.as_ref().map(|c| &c.uri);
        let now = Instant::now();
        let queue_deadline = now.checked_sub(Duration::from_secs(60)).unwrap();

        if state.current_context.as_ref() != new_context {
            state.context_updated = true;
            state.current_context = new_context.map(String::from);
            state.last_grabbed_queue = queue_deadline;
        }

        // Song has changed, lets update to the new index and force a queue refresh
        if let Some(track) = current_playback.item {
            state.queue_index = state
                .queue
                .iter()
                .position(|t| t.name == track.name)
                .unwrap_or_else(|| {
                    state.last_grabbed_queue = queue_deadline;
                    0
                });
        }

        state.volume = current_playback.device.volume_percent.map(|v| v as u8);
        state.playing = current_playback.is_playing;
        state.progress = current_playback.progress_ms
            + if current_playback.is_playing {
                (request_duration.as_millis() / 2) as u32
            } else {
                0
            };
        state.last_progress_update = now;
        state.last_grabbed_playback = now;
    });
}

/// Pulls the current playback queue and status from the Spotify Web API and updates shared state.
fn get_spotify_queue() {
    // Wait if we have recently interacted with spotify
    let now = Instant::now();
    {
        let state = PLAYBACK_STATE.read();
        if now < state.last_interaction || now < state.last_grabbed_queue + Duration::from_secs(15)
        {
            return;
        }
    }

    // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-queue
    let result = match SPOTIFY_CLIENT.api_get("me/player/queue") {
        Ok(result) => result,
        Err(err) => {
            update_playback_state(|state| {
                state.last_grabbed_queue = Instant::now();
            });
            error!("Failed to fetch current queue: {err}");
            return;
        }
    };
    let queue = match serde_json::from_str::<CurrentUserQueue>(&result) {
        Ok(queue) => queue,
        Err(err) => {
            update_playback_state(|state| {
                state.last_grabbed_playback = Instant::now();
            });
            error!("Failed to parse current queue: {err}");
            return;
        }
    };

    // Get current track and the upcoming queue
    let Some(currently_playing) = queue.currently_playing else {
        return;
    };
    let new_queue: Vec<Track> = std::iter::once(currently_playing)
        .chain(queue.queue)
        .collect();
    let current_title = new_queue.first().unwrap().name.clone();

    // Start a task to fetch missing artists & images
    let mut missing_urls = HashSet::new();
    let mut missing_artists = HashSet::new();
    for track in &new_queue {
        if !IMAGES_CACHE.contains_key(&track.album.image) {
            missing_urls.insert(track.album.image.clone());
        }
        if !ARTIST_DATA_CACHE.contains_key(&track.artist.id) {
            missing_artists.insert(track.artist.id);
        }
    }
    // Start downloading missing album images
    for url in missing_urls {
        ensure_image_cached(url.as_str());
    }

    // Cache artists, and download images
    if !missing_artists.is_empty() {
        let artist_query = missing_artists
            .into_iter()
            .map(|artist| artist.as_str().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        spawn(move || {
            // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-artists
            let result = match SPOTIFY_CLIENT.api_get(&format!("artists/?ids={artist_query}")) {
                Ok(result) => result,
                Err(err) => {
                    error!("Failed to fetch artists: {err}");
                    return;
                }
            };
            let Ok(artists) = serde_json::from_str::<Artists>(&result) else {
                return;
            };
            for artist in artists.artists {
                ARTIST_DATA_CACHE.insert(artist.id, Some(artist.image.clone()));
                ensure_image_cached(artist.image.as_str());
            }
        });
    }

    // Update the playback state
    update_playback_state(|state| {
        if !state.context_updated
            && let Some(new_index) = state.queue.iter().position(|t| t.name == current_title)
        {
            // Delete everything past the new_index, and append the new tracks at the end
            state.queue_index = new_index;
            state.queue.truncate(new_index);
            state.queue.extend(new_queue);
        } else {
            // Context switched - reset queue entirely
            state.context_updated = false;
            state.queue = new_queue;
            state.queue_index = 0;
        }

        state.last_grabbed_queue = Instant::now();
    });
}

/// Downloads and caches an image from the given URL.
fn ensure_image_cached(url: &str) {
    if IMAGES_CACHE.contains_key(url) {
        return;
    }
    IMAGES_CACHE.insert(url.to_owned(), None);

    let url = url.to_owned();
    spawn(move || {
        let mut response = match SPOTIFY_CLIENT.http.get(&url).call() {
            Ok(response) => response,
            Err(err) => {
                warn!("Failed to cache image {url}: {err}");
                return;
            }
        };
        let Ok(dynamic_image) =
            image::load_from_memory(&response.body_mut().read_to_vec().unwrap())
        else {
            warn!("Failed to cache image {url}: failed to read image");
            return;
        };
        // Resize the image to exactly 64x64 to ensure consistent buffer sizes for WGPU
        let dynamic_image = if dynamic_image.width() != 64 || dynamic_image.height() != 64 {
            dynamic_image.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3)
        } else {
            dynamic_image
        };
        IMAGES_CACHE.insert(
            url,
            Some(ImageBrush {
                image: ImageData {
                    data: Blob::from(dynamic_image.to_rgba8().into_raw()),
                    format: ImageFormat::Rgba8,
                    alpha_type: ImageAlphaType::Alpha,
                    width: dynamic_image.width(),
                    height: dynamic_image.height(),
                },
                sampler: ImageSampler {
                    x_extend: Extend::Pad,
                    y_extend: Extend::Pad,
                    quality: ImageQuality::Medium,
                    alpha: 1.0,
                },
            }),
        );
        update_color_palettes();
    });
}

fn poll_playlists() {
    // Get initial playlist data
    let target_playlists = CONFIG
        .playlists
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let include_ratings = CONFIG.ratings_enabled;
    let cached_playlist_tracks = load_cached_playlist_tracks();

    // Spawn loop to collect spotify playlist tracks
    loop {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-list-of-current-users-playlists
        let result = match SPOTIFY_CLIENT.api_get_payload("me/playlists", &[("limit", "50")]) {
            Ok(result) => result,
            Err(err) => {
                error!("Failed to fetch users playlists: {err}");
                String::new()
            }
        };
        let playlists = match serde_json::from_str::<Page<Playlist>>(&result) {
            Ok(playlists) => playlists.items,
            Err(err) => {
                error!("Failed to parse users playlists: {err}");
                Vec::new()
            }
        };

        for playlist in playlists {
            // Check if the playlist is on the contained list
            if !(target_playlists.contains(playlist.name.as_str())
                || (include_ratings && RATING_PLAYLISTS.contains(&playlist.name.as_str())))
            {
                continue;
            }
            // Download the playlist image
            ensure_image_cached(&playlist.image);

            let rating_index = if CONFIG.ratings_enabled {
                RATING_PLAYLISTS
                    .iter()
                    .enumerate()
                    .find(|(_, p)| *p == &playlist.name)
                    .map(|(i, _)| i as u8)
            } else {
                None
            };

            // Load from cache if available
            if let Some(cached) = cached_playlist_tracks.get(&playlist.id)
                && playlist.snapshot_id == cached.0
            {
                update_playback_state(|state| {
                    state.playlists.insert(
                        playlist.id,
                        CondensedPlaylist {
                            id: playlist.id,
                            name: playlist.name,
                            image_url: playlist.image,
                            tracks: cached.1.clone(),
                            tracks_total: playlist.total_tracks,
                            snapshot_id: cached.0,
                            rating_index,
                        },
                    );
                });
                continue;
            }
            // Make sure the playlist has changed if it's already in the state
            if Some(&playlist.snapshot_id)
                == PLAYBACK_STATE
                    .read()
                    .playlists
                    .get(&playlist.id)
                    .map(|p| &p.snapshot_id)
            {
                continue;
            }

            // Fetch all new tracks in one go for changed playlists
            let chunk_size = 50;
            let num_pages = playlist.total_tracks.div_ceil(chunk_size) as usize;
            info!("Fetching {num_pages} pages from playlist {}", playlist.name);
            let mut pages = Vec::new();
            for page in 0..num_pages {
                // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlists-tracks
                let result = match SPOTIFY_CLIENT.api_get_payload(
                    &format!("playlists/{}/tracks", playlist.id),
                    &[
                        (
                            "fields",
                            "href,limit,offset,total,items(is_local,track(id))",
                        ),
                        ("limit", &chunk_size.to_string()),
                        ("offset", &((page as u32) * chunk_size).to_string()),
                    ],
                ) {
                    Ok(result) => result,
                    Err(err) => {
                        error!("Failed to fetch playlist page: {err}");
                        return;
                    }
                };
                match serde_json::from_str::<Page<PlaylistItem>>(&result) {
                    Ok(page) => pages.push(page),
                    Err(err) => {
                        error!("Failed to parse playlist page: {err}");
                        return;
                    }
                }
            }

            // Process the collected pages into a single track ID set and get the total
            let new_total = pages.first().map_or(0, |p| p.total);
            let playlist_track_ids: HashSet<TrackId> = pages
                .into_iter()
                .flat_map(|page| page.items)
                .map(|item| item.track.id)
                .collect();

            update_playback_state(|state| {
                state
                    .playlists
                    .entry(playlist.id)
                    .and_modify(|state_playlist| {
                        state_playlist.tracks.clone_from(&playlist_track_ids);
                        state_playlist.tracks_total = new_total;
                        state_playlist.snapshot_id = playlist.snapshot_id;
                    })
                    .or_insert_with(|| CondensedPlaylist {
                        id: playlist.id,
                        name: playlist.name,
                        image_url: playlist.image,
                        tracks: playlist_track_ids,
                        tracks_total: new_total,
                        snapshot_id: playlist.snapshot_id,
                        rating_index,
                    });
            });
            persist_playlist_cache();
        }

        sleep(Duration::from_secs(12));
    }
}

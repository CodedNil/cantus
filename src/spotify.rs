use crate::{background::update_color_palettes, config::CONFIG};
use anyhow::Result;
use dashmap::DashMap;
use parking_lot::RwLock;
use rspotify::{
    AuthCodePkceSpotify, Config, Credentials, OAuth,
    model::{
        AdditionalType, AlbumId, ArtistId, PlayableItem, PlaylistId, SimplifiedPlaylist, TrackId,
    },
    prelude::{BaseClient, OAuthClient},
    scopes,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::{LazyLock, OnceLock},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};
use tracing::{error, info, warn};
use ureq::Agent;
use vello::peniko::{
    Blob, Extend, ImageAlphaType, ImageBrush, ImageData, ImageFormat, ImageQuality, ImageSampler,
};

pub static PLAYBACK_STATE: LazyLock<RwLock<PlaybackState>> = LazyLock::new(|| {
    RwLock::new(PlaybackState {
        last_updated: Instant::now(),
        last_interaction: Instant::now(),
        playing: false,
        progress: 0,
        volume: None,
        queue: Vec::new(),
        queue_index: 0,
        current_context: None,
        playlists: HashMap::new(),
    })
});
pub static IMAGES_CACHE: LazyLock<DashMap<String, ImageBrush>> = LazyLock::new(DashMap::new);
pub static ALBUM_DATA_CACHE: LazyLock<DashMap<AlbumId<'static>, Option<AlbumData>>> =
    LazyLock::new(DashMap::new);
pub static ARTIST_DATA_CACHE: LazyLock<DashMap<ArtistId<'static>, Option<String>>> =
    LazyLock::new(DashMap::new);

pub const RATING_PLAYLISTS: [&str; 10] = [
    "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

static HTTP_CLIENT: LazyLock<Agent> = LazyLock::new(Agent::new_with_defaults);
pub static SPOTIFY_CLIENT: OnceLock<AuthCodePkceSpotify> = OnceLock::new();

pub struct PlaybackState {
    pub last_updated: Instant,
    pub last_interaction: Instant,
    pub playing: bool,
    pub progress: u32,
    pub volume: Option<u8>,
    pub queue: Vec<Track>,
    pub queue_index: usize,
    pub current_context: Option<String>,
    pub playlists: HashMap<String, Playlist>,
}

pub struct Track {
    pub id: TrackId<'static>,
    pub title: String,
    pub artist_id: ArtistId<'static>,
    pub artist_name: String,
    pub album_id: AlbumId<'static>,
    pub image_url: String,
    pub milliseconds: u32,
}

pub struct AlbumData {
    /// Simplified color palette (RGBA, alpha = percentage 0-100).
    pub primary_colors: Vec<[u8; 4]>,
    /// Generated texture derived from the palette for shader backgrounds.
    pub palette_image: ImageBrush,
}

pub struct Playlist {
    pub id: PlaylistId<'static>,
    pub name: String,
    pub image_url: String,
    pub tracks: HashSet<TrackId<'static>>,
    pub tracks_total: u32,
    snapshot_id: String,
}

/// Mutably updates the global playback state.
pub fn update_playback_state<F>(update: F)
where
    F: FnOnce(&mut PlaybackState),
{
    let mut state = PLAYBACK_STATE.write();
    update(&mut state);
}

type PlaylistCache = HashMap<PlaylistId<'static>, (String, HashSet<TrackId<'static>>)>;

fn load_cached_playlist_tracks() -> PlaylistCache {
    let cache_path = dirs::config_dir()
        .unwrap()
        .join("cantus")
        .join("cantus_playlist_tracks.json");
    let bytes = match fs::read(cache_path.clone()) {
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
                playlist.id.clone(),
                (
                    playlist.snapshot_id.clone(),
                    playlist.tracks.iter().cloned().collect(),
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

    // Initialize Spotify client with credentials and OAuth scopes
    let mut spotify = AuthCodePkceSpotify::with_config(
        Credentials {
            id: CONFIG.spotify_client_id.clone().expect(
                "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
            ),
            secret: None,
        },
        OAuth {
            redirect_uri: String::from("http://127.0.0.1:7474/callback"),
            scopes: scopes!(
                "user-read-playback-state",
                "user-modify-playback-state",
                "user-read-currently-playing",
                "playlist-read-private",
                "playlist-read-collaborative",
                "playlist-modify-private",
                "playlist-modify-public",
                "user-library-read",
                "user-library-modify"
            ),
            ..Default::default()
        },
        Config {
            token_cached: true,
            cache_path: dirs::config_dir()
                .unwrap()
                .join("cantus")
                .join("spotify_cache.json"),
            token_refreshing: true,
            ..Default::default()
        },
    );

    // Prompt user for authorization and get the token
    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).unwrap();
    SPOTIFY_CLIENT.set(spotify).unwrap();

    // Begin polling
    spawn(poll_playlists);
    spawn(|| {
        loop {
            update_state_from_spotify();
            sleep(Duration::from_millis(200));
        }
    });
}

/// Pulls the current playback queue and status from the Spotify Web API and updates shared state.
fn update_state_from_spotify() {
    // Wait if we have recently interacted with spotify
    let now = Instant::now();
    if now < PLAYBACK_STATE.read().last_interaction {
        return;
    }
    if now < PLAYBACK_STATE.read().last_updated + Duration::from_millis(2000) {
        return;
    }

    // Fetch current playback and queue concurrently
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();
    let (current_playback, queue) = match (
        spotify_client.current_playback(None, None::<Vec<&AdditionalType>>),
        spotify_client.current_user_queue(),
    ) {
        (Ok(Some(playback)), Ok(queue)) => (playback, queue),
        (Ok(None), _) => {
            // Spotify is not playing anything
            return;
        }
        (Err(err), _) => {
            error!("Failed to fetch current playback: {err}");
            return;
        }
        (_, Err(err)) => {
            error!("Failed to fetch current queue: {err}");
            return;
        }
    };
    let request_duration = now.elapsed();

    // Get current track and the upcoming queue
    let Some(currently_playing) = queue.currently_playing else {
        return;
    };
    let new_queue: Vec<Track> = std::iter::once(currently_playing)
        .chain(queue.queue)
        .filter_map(|item| match item {
            PlayableItem::Track(track) => Some({
                let artist = track.artists.first().unwrap();
                Track {
                    id: track.id.unwrap(),
                    title: track.name,
                    artist_id: artist.id.clone().unwrap(),
                    artist_name: artist.name.clone(),
                    album_id: track.album.id.unwrap(),
                    image_url: track
                        .album
                        .images
                        .into_iter()
                        .min_by_key(|img| img.width)
                        .map(|img| img.url)
                        .unwrap(),
                    milliseconds: track.duration.num_milliseconds() as u32,
                }
            }),
            PlayableItem::Episode(_) | PlayableItem::Unknown(_) => None,
        })
        .collect();
    let current_title = new_queue.first().unwrap().title.clone();

    // Start a task to fetch missing artists & images
    let mut missing_urls = HashSet::new();
    let mut missing_artists = HashSet::new();
    for track in &new_queue {
        if !IMAGES_CACHE.contains_key(&track.image_url) {
            missing_urls.insert(track.image_url.clone());
        }
        if !ARTIST_DATA_CACHE.contains_key(&track.artist_id) {
            missing_artists.insert(track.artist_id.clone());
        }
    }
    // Start downloading missing album images
    for url in missing_urls {
        spawn(move || {
            if let Err(err) = ensure_image_cached(url.as_str()) {
                warn!("failed to cache image {url}: {err}");
            }
        });
    }

    // Cache artists, and download images
    if !missing_artists.is_empty() {
        let Ok(artists) = spotify_client.artists(missing_artists) else {
            return;
        };
        for artist in artists {
            let artist_image = artist.images.into_iter().min_by_key(|img| img.width);
            ARTIST_DATA_CACHE.insert(artist.id, artist_image.as_ref().map(|a| a.url.clone()));
            spawn(move || {
                if let Some(artist_image) = artist_image
                    && let Err(err) = ensure_image_cached(artist_image.url.as_str())
                {
                    warn!("failed to cache image {}: {err}", artist_image.url);
                }
            });
        }
    }

    // Update the playback state
    update_playback_state(|state| {
        let new_context = current_playback.context.map(|c| c.uri);
        if state.current_context == new_context
            && let Some(new_index) = state.queue.iter().position(|t| t.title == current_title)
        {
            // Delete everything past the new_index, and append the new tracks at the end
            state.queue_index = new_index;
            state.queue.truncate(new_index);
            state.queue.extend(new_queue);
        } else {
            // Context switched - reset queue entirely
            info!("Context changed, resetting queue");
            state.queue = new_queue;
            state.queue_index = 0;
            state.current_context = new_context;
        }

        state.volume = current_playback.device.volume_percent.map(|v| v as u8);
        state.playing = current_playback.is_playing;
        let progress = current_playback
            .progress
            .map_or(0, |p| p.num_milliseconds()) as u32;
        state.progress = progress
            + if current_playback.is_playing {
                (request_duration.as_millis() / 2) as u32
            } else {
                0
            };
        state.last_updated = Instant::now();
    });
}

/// Downloads and caches an image from the given URL.
fn ensure_image_cached(url: &str) -> Result<()> {
    if IMAGES_CACHE.contains_key(url) {
        return Ok(());
    }
    let mut response = HTTP_CLIENT.get(url).call()?;
    let dynamic_image = image::load_from_memory(&response.body_mut().read_to_vec()?)?;
    // If width or height more thant 64 pixels, resize the image
    let dynamic_image = if dynamic_image.width() > 64 || dynamic_image.height() > 64 {
        dynamic_image.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3)
    } else {
        dynamic_image
    };
    IMAGES_CACHE.insert(
        url.to_owned(),
        ImageBrush {
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
        },
    );
    if let Err(err) = update_color_palettes() {
        warn!("failed to update color palettes: {err}");
    }
    Ok(())
}

fn poll_playlists() {
    // Get initial playlist data
    let config = &*CONFIG;
    let target_playlists = config
        .playlists
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let include_ratings = config.ratings_enabled;
    let mut cached_playlist_tracks = load_cached_playlist_tracks();

    // Grab the current users playlists from spotify
    let playlists: Vec<Playlist> = SPOTIFY_CLIENT
        .get()
        .unwrap()
        .current_user_playlists_manual(Some(50), None)
        .unwrap()
        .items
        .into_iter()
        .filter(|playlist| {
            target_playlists.contains(playlist.name.as_str())
                || (include_ratings && RATING_PLAYLISTS.contains(&playlist.name.as_str()))
        })
        .map(|playlist| {
            let cached = cached_playlist_tracks
                .remove(&playlist.id)
                .unwrap_or_default();
            Playlist {
                id: playlist.id.clone(),
                name: playlist.name,
                image_url: playlist
                    .images
                    .iter()
                    .min_by_key(|img| img.width)
                    .unwrap()
                    .url
                    .clone(),
                tracks: cached.1,
                tracks_total: playlist.tracks.total,
                snapshot_id: cached.0,
            }
        })
        .collect();

    // Download all the playlist images
    let image_urls = playlists
        .iter()
        .map(|p| p.image_url.clone())
        .collect::<Vec<_>>();
    for url in image_urls {
        spawn(move || {
            if let Err(err) = ensure_image_cached(&url) {
                warn!("failed to cache image {url}: {err}");
            }
        });
    }

    // Push the data to the global state
    update_playback_state(|state| {
        state.playlists = playlists.into_iter().map(|p| (p.name.clone(), p)).collect();
    });

    // Spawn loop to collect spotify playlist tracks
    loop {
        refresh_playlists();

        sleep(Duration::from_secs(12));
    }
}

fn refresh_playlists() {
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();

    let playlist_snapshots = {
        let state = PLAYBACK_STATE.read();
        state
            .playlists
            .values()
            .map(|playlist| (playlist.id.clone(), playlist.snapshot_id.clone()))
            .collect::<HashMap<_, _>>()
    };

    // Find playlists which have changed
    let changed_playlists: Vec<SimplifiedPlaylist> = spotify_client
        .current_user_playlists_manual(Some(50), None)
        .unwrap()
        .items
        .into_iter()
        .filter_map(|playlist| {
            playlist_snapshots
                .get(&playlist.id)
                .and_then(|state_snapshot| {
                    (playlist.snapshot_id != *state_snapshot).then_some(playlist)
                })
        })
        .collect();

    // Fetch all new tracks in one go for changed playlists
    for playlist in changed_playlists {
        let chunk_size = 50;
        let num_pages = playlist.tracks.total.div_ceil(chunk_size) as usize;
        info!("Fetching {num_pages} pages from playlist {}", playlist.name);
        let mut pages = Vec::new();
        for page in 0..num_pages {
            match spotify_client.playlist_items_manual(
                playlist.id.clone(),
                Some("href,limit,offset,total,items(is_local,track(id))"),
                None,
                Some(chunk_size),
                Some((page as u32) * chunk_size),
            ) {
                Ok(page) => pages.push(page),
                Err(err) => {
                    error!("Failed to fetch playlist page: {:?}", err);
                    return;
                }
            }
        }

        // Process the collected pages into a single track ID set and get the total
        let new_total = pages.first().map_or(0, |p| p.total);
        let playlist_track_ids: HashSet<TrackId> = pages
            .into_iter()
            .flat_map(|page| page.items)
            .filter_map(|item| {
                let Some(PlayableItem::Unknown(track)) = &item.track else {
                    return None;
                };
                TrackId::from_id(track.get("id")?.as_str()?)
                    .ok()
                    .map(TrackId::into_static)
            })
            .collect();

        update_playback_state(|state| {
            let state_playlist = state.playlists.get_mut(&playlist.name).unwrap();
            state_playlist.tracks = playlist_track_ids;
            state_playlist.tracks_total = new_total;
            state_playlist.snapshot_id = playlist.snapshot_id;
        });
        persist_playlist_cache();
    }
}

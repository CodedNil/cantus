use crate::PANEL_HEIGHT_BASE;
use anyhow::Result;
use auto_palette::Palette;
use chrono::TimeDelta;
use dashmap::DashMap;
use futures::future::try_join_all;
use image::{GenericImageView, RgbaImage};
use itertools::Itertools;
use parking_lot::Mutex;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;
use reqwest::Client;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{AdditionalType, ArtistId, Context, FullTrack, PlayableItem, PlaylistId, TrackId},
    prelude::{BaseClient, OAuthClient},
    scopes,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, hash_map::DefaultHasher},
    convert::TryInto,
    env, fs,
    hash::{Hash, Hasher},
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering as AtomicOrdering},
    },
    time::Instant,
};
use tokio::{
    sync::OnceCell,
    task::JoinSet,
    time::{Duration, sleep},
};
use tracing::{error, info, warn};
use vello::{
    kurbo::{Point, Rect},
    peniko::{Blob, ImageAlphaType, ImageData, ImageFormat},
};
use zbus::{
    Connection,
    fdo::{DBusProxy, PropertiesProxy},
    names::InterfaceName,
    zvariant::OwnedValue,
};

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

/// Maximum number of historical tracks to keep before trimming.
const MAX_HISTORY: usize = 20;

/// Dimensions of the generated palette-based textures.
const PALETTE_IMAGE_HEIGHT: u32 = PANEL_HEIGHT_BASE as u32;
const PALETTE_IMAGE_WIDTH: u32 = PALETTE_IMAGE_HEIGHT * 3;

/// Number of refinement passes when synthesising the background texture.
const PALETTE_PASS_COUNT: usize = 8;
/// Maximum number of brush placements per pass.
const PALETTE_STROKES_PER_PASS: usize = 16;

/// MPRIS interface identifier used for playback control.
const PLAYER_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2.Player");
/// Root MPRIS interface that exposes metadata and identity.
const ROOT_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2");
/// Object path for the Spotify MPRIS instance on D-Bus.
const MPRIS_OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(PlaybackState {
        last_updated: Instant::now(),
        playing: false,
        shuffle: false,
        progress: 0,
        queue: Vec::new(),
        queue_index: 0,
        current_context: None,
        playlists: Vec::new(),
        current_playlist_idx: 0,
    }))
});
pub static IMAGES_CACHE: LazyLock<DashMap<String, ImageData>> = LazyLock::new(DashMap::new);
pub static TRACK_DATA_CACHE: LazyLock<DashMap<TrackId<'static>, TrackData>> =
    LazyLock::new(DashMap::new);
pub static ARTIST_DATA_CACHE: LazyLock<DashMap<ArtistId<'static>, ArtistData>> =
    LazyLock::new(DashMap::new);
const PLAYLIST_CACHE_PATH: &str = "/tmp/cantus_playlist_tracks.json";
static BRUSHES: LazyLock<[RgbaImage; 5]> = LazyLock::new(|| {
    let bytes = (
        include_bytes!("../assets/brushes/brush1.png"),
        include_bytes!("../assets/brushes/brush2.png"),
        include_bytes!("../assets/brushes/brush3.png"),
        include_bytes!("../assets/brushes/brush4.png"),
        include_bytes!("../assets/brushes/brush5.png"),
    );
    [
        image::load_from_memory(bytes.0).unwrap().to_rgba8(),
        image::load_from_memory(bytes.1).unwrap().to_rgba8(),
        image::load_from_memory(bytes.2).unwrap().to_rgba8(),
        image::load_from_memory(bytes.3).unwrap().to_rgba8(),
        image::load_from_memory(bytes.4).unwrap().to_rgba8(),
    ]
});

pub const RATING_PLAYLISTS: [&str; 11] = [
    "0.0", "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
static SPOTIFY_CLIENT: OnceCell<AuthCodeSpotify> = OnceCell::const_new();
static SPOTIFY_INTERACTION_ACTIVE: AtomicBool = AtomicBool::new(false);

struct SpotifyInteractionGuard;
impl SpotifyInteractionGuard {
    fn try_acquire() -> Option<Self> {
        SPOTIFY_INTERACTION_ACTIVE
            .compare_exchange(
                false,
                true,
                AtomicOrdering::Acquire,
                AtomicOrdering::Relaxed,
            )
            .is_ok()
            .then(|| Self)
    }
}
impl Drop for SpotifyInteractionGuard {
    fn drop(&mut self) {
        SPOTIFY_INTERACTION_ACTIVE.store(false, AtomicOrdering::Release);
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackState {
    pub last_updated: Instant,
    pub playing: bool,
    pub shuffle: bool,
    pub progress: u32,
    pub queue: Vec<Track>,
    pub queue_index: usize,
    pub current_context: Option<Context>,
    pub playlists: Vec<Playlist>,
    pub current_playlist_idx: usize,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: TrackId<'static>,
    pub title: String,
    pub artist_id: ArtistId<'static>,
    pub artist_name: String,
    pub album_name: String,
    pub image_url: String,
    pub release_date: String,
    pub milliseconds: u32,
}

#[derive(Debug, Clone)]
pub struct TrackData {
    /// Simplified color palette (RGBA, alpha = percentage 0-100).
    pub primary_colors: Vec<[u8; 4]>,
    /// Generated texture derived from the palette for shader backgrounds.
    pub palette_image: ImageData,
}

pub struct ArtistData {
    pub name: String,
    pub genres: Vec<String>,
    pub popularity: u8,
    pub image_url: String,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: PlaylistId<'static>,
    pub name: String,
    pub image_url: String,
    pub tracks: HashSet<TrackId<'static>>,
    pub tracks_total: u32,
}

impl Track {
    fn from_rspotify(track: FullTrack) -> Self {
        let artist = track.artists.first().unwrap();
        Self {
            id: track.id.unwrap(),
            title: track.name,
            artist_id: artist.id.clone().unwrap(),
            artist_name: artist.name.clone(),
            album_name: track.album.name,
            image_url: track
                .album
                .images
                .into_iter()
                .min_by_key(|img| img.width.unwrap())
                .map(|img| img.url)
                .unwrap(),
            release_date: track.album.release_date.unwrap(),
            milliseconds: track.duration.num_milliseconds() as u32,
        }
    }
}

/// Mutably updates the global playback state inside the mutex.
fn update_playback_state<F>(update: F)
where
    F: FnOnce(&mut PlaybackState),
{
    let mut state = PLAYBACK_STATE.lock();
    update(&mut state);
}

fn load_cached_playlist_tracks() -> HashMap<PlaylistId<'static>, HashSet<TrackId<'static>>> {
    let bytes = match fs::read(PLAYLIST_CACHE_PATH) {
        Ok(bytes) => bytes,
        Err(err) => {
            warn!("Failed to read playlist cache at {PLAYLIST_CACHE_PATH}: {err}");
            return HashMap::new();
        }
    };

    match serde_json::from_slice::<HashMap<PlaylistId<'static>, HashSet<TrackId<'static>>>>(&bytes)
    {
        Ok(map) => map,
        Err(err) => {
            warn!("Failed to parse playlist cache at {PLAYLIST_CACHE_PATH}: {err}");
            HashMap::new()
        }
    }
}

fn persist_playlist_cache() {
    let state = PLAYBACK_STATE.lock();
    if state.playlists.is_empty() {
        return;
    }
    let cache_payload: HashMap<PlaylistId<'static>, HashSet<TrackId<'static>>> = state
        .playlists
        .iter()
        .map(|playlist| {
            (
                playlist.id.clone_static(),
                playlist.tracks.iter().map(TrackId::clone_static).collect(),
            )
        })
        .collect();
    drop(state);

    match serde_json::to_vec(&cache_payload) {
        Ok(serialized) => {
            if let Err(err) = fs::write(PLAYLIST_CACHE_PATH, serialized) {
                warn!("Failed to write playlist cache at {PLAYLIST_CACHE_PATH}: {err}");
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
pub async fn init() {
    // Initialize Spotify client with credentials and OAuth scopes
    let spotify = AuthCodeSpotify::with_config(
        Credentials::from_env()
            .expect("Missing env credentials RSPOTIFY_CLIENT_ID RSPOTIFY_CLIENT_SECRET"),
        OAuth {
            redirect_uri: String::from("http://127.0.0.1:7474/callback"),
            scopes: scopes!(
                "user-read-playback-state",
                "user-modify-playback-state",
                "user-read-currently-playing",
                "playlist-read-private",
                "playlist-read-collaborative",
                "playlist-modify-private",
                "playlist-modify-public"
            ),
            ..Default::default()
        },
        Config {
            token_cached: true,
            ..Default::default()
        },
    );

    // Prompt user for authorization and get the token
    let url = spotify.get_authorize_url(true).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();
    SPOTIFY_CLIENT.set(spotify).unwrap();

    // Begin polling
    tokio::spawn(polling_task());
}

/// Asynchronous task to poll MPRIS every 500ms and Spotify API every X seconds or on song change.
async fn polling_task() {
    let mut last_mpris_track_id: Option<String> = None; // Local state for track ID
    let connection = match Connection::session().await {
        Ok(conn) => conn,
        Err(err) => panic!("Failed to connect to D-Bus session: {err}"),
    };
    let dbus_proxy = match DBusProxy::new(&connection).await {
        Ok(proxy) => proxy,
        Err(err) => panic!("Failed creating D-Bus proxy: {err}"),
    };

    // Get initial playlist data
    let playlists_env = env::var("PLAYLISTS").unwrap_or_default();
    let target_playlists = playlists_env.split(',').collect::<Vec<_>>();
    let mut cached_playlist_tracks = load_cached_playlist_tracks();
    let new_playlists: Vec<Playlist> = SPOTIFY_CLIENT
        .get()
        .unwrap()
        .current_user_playlists_manual(Some(50), None)
        .await
        .unwrap()
        .items
        .into_iter()
        .filter(|playlist| {
            target_playlists.contains(&playlist.name.as_str())
                || RATING_PLAYLISTS.contains(&playlist.name.as_str())
        })
        .map(|playlist| Playlist {
            id: playlist.id.clone(),
            name: playlist.name,
            image_url: playlist
                .images
                .iter()
                .min_by_key(|img| img.width.unwrap_or_default())
                .unwrap()
                .url
                .clone(),
            tracks: cached_playlist_tracks
                .remove(&playlist.id)
                .unwrap_or_default(),
            tracks_total: playlist.tracks.total,
        })
        .collect();
    update_playback_state(|state| {
        state.playlists.clone_from(&new_playlists);
    });

    // Download all the playlist images
    tokio::spawn(async move {
        let mut set = JoinSet::new();
        for url in new_playlists.iter().map(|p| p.image_url.clone()) {
            set.spawn(async move {
                if let Err(err) = ensure_image_cached(url.as_str()).await {
                    warn!("failed to cache image {url}: {err}");
                }
            });
        }
        // Concurrently run all tasks
        while set.join_next().await.is_some() {}
    });

    // Spawn loop to collect spotify playlist tracks
    tokio::spawn(async move {
        loop {
            let spotify_client = SPOTIFY_CLIENT.get().unwrap();
            let (current_playlist_id, current_playlist_total, current_playlist_idx) = {
                let mut state = PLAYBACK_STATE.lock();
                let idx = state.current_playlist_idx;
                state.current_playlist_idx =
                    (state.current_playlist_idx + 1) % state.playlists.len();
                (
                    state.playlists[idx].id.clone(),
                    state.playlists[idx].tracks_total,
                    idx,
                )
            };
            let chunk_size = 50;
            let num_pages = current_playlist_total.div_ceil(chunk_size) as usize;
            let mut fetches = Vec::with_capacity(num_pages);
            for p in 0..num_pages {
                let id = current_playlist_id.clone();
                fetches.push(async move {
                    spotify_client
                        .playlist_items_manual(
                            id,
                            Some("href,limit,offset,total,items(is_local,track(id))"),
                            None,
                            Some(chunk_size),
                            Some((p as u32) * chunk_size),
                        )
                        .await
                });
            }

            // Grab all data concurrently
            match try_join_all(fetches).await {
                Ok(pages) => {
                    // Sort through playlist items and push them into the dat
                    let mut playlist_track_ids =
                        HashSet::with_capacity(current_playlist_total as usize);
                    for item in pages.iter().flat_map(|page| &page.items) {
                        // Track returned will be unknown since we add the filter
                        if let Some(PlayableItem::Unknown(track)) = &item.track {
                            let id = track.get("id").unwrap().as_str().unwrap();
                            playlist_track_ids.insert(TrackId::from_id(id).unwrap().into_static());
                        }
                    }
                    let new_total = pages
                        .first()
                        .map_or(current_playlist_total, |page| page.total);
                    update_playback_state(move |state| {
                        state.playlists[current_playlist_idx]
                            .tracks
                            .clone_from(&playlist_track_ids);
                        state.playlists[current_playlist_idx].tracks_total = new_total;
                    });
                    persist_playlist_cache();
                }
                Err(err) => {
                    error!("Failed to fetch current playback and queue: {}", err);
                }
            }

            sleep(Duration::from_secs(12)).await;
        }
    });

    let mut spotify_poll_counter = 100; // Counter for Spotify API polling
    loop {
        // --- MPRIS Polling Logic ---
        let (should_refresh_spotify, used_mpris_progress) =
            update_state_from_mpris(&connection, &dbus_proxy, &mut last_mpris_track_id).await;

        // --- Spotify API Polling Logic ---
        spotify_poll_counter += 1;
        if spotify_poll_counter >= 8 || should_refresh_spotify {
            spotify_poll_counter = 0; // Reset counter
            update_state_from_spotify(used_mpris_progress).await;
        }

        sleep(Duration::from_millis(500)).await;
    }
}

/// Synchronizes playback information with the MPRIS interface and returns whether Spotify data should refresh, and whether MPRIS data was fetched.
async fn update_state_from_mpris(
    connection: &Connection,
    dbus_proxy: &DBusProxy<'_>,
    last_track_id: &mut Option<String>,
) -> (bool, bool) {
    let Ok(names) = dbus_proxy.list_names().await else {
        return (false, false);
    };

    let mut properties_proxy = None;
    for name in names {
        if !name.starts_with("org.mpris.MediaPlayer2.") {
            continue;
        }

        let Ok(builder) = PropertiesProxy::builder(connection)
            .destination(name)
            .and_then(|builder| builder.path(MPRIS_OBJECT_PATH))
        else {
            continue;
        };
        let Ok(proxy) = builder.build().await else {
            continue;
        };
        let Ok(identity) = proxy.get(ROOT_INTERFACE, "Identity").await else {
            continue;
        };
        if identity.to_string() == "\"Spotify\"" {
            properties_proxy = Some(proxy);
            break;
        }
    }
    let Some(properties_proxy) = properties_proxy else {
        return (false, false);
    };

    let new_track_id = properties_proxy
        .get(PLAYER_INTERFACE, "Metadata")
        .await
        .ok()
        .and_then(|metadata| {
            Some(
                HashMap::<String, OwnedValue>::try_from(metadata)
                    .ok()?
                    .get("mpris:trackid")?
                    .to_string(),
            )
        });
    let mut should_refresh = new_track_id
        .filter(|track_id| last_track_id.as_ref() != Some(track_id))
        .is_some_and(|track_id| {
            *last_track_id = Some(track_id);
            true
        });

    let playing = properties_proxy
        .get(PLAYER_INTERFACE, "PlaybackStatus")
        .await
        .ok()
        .and_then(|value| value.try_into().ok())
        .map(|status: String| status == "Playing");
    let progress = properties_proxy
        .get(PLAYER_INTERFACE, "Position")
        .await
        .ok()
        .and_then(|value| value.try_into().ok())
        .map(|position: i64| position / 1_000);

    let mut updated_progress = false;
    if playing.is_some() || progress.is_some() {
        update_playback_state(|state| {
            if let Some(playing) = playing
                && playing != state.playing
            {
                should_refresh = true;
                state.playing = playing;
                if let Some(progress) = progress {
                    state.progress = progress as u32;
                    state.last_updated = Instant::now();
                    updated_progress = true;
                }
            }
        });
    }

    (should_refresh, updated_progress)
}

/// Pulls the current playback queue and status from the Spotify Web API and updates shared state.
async fn update_state_from_spotify(used_mpris_progress: bool) {
    // Fetch current playback and queue concurrently
    let request_start = Instant::now();
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();

    // Get one playlists tracks per loop to keep them fresh
    let (current_playback, queue) = match tokio::try_join!(
        spotify_client.current_playback(None, None::<Vec<&AdditionalType>>),
        spotify_client.current_user_queue(),
    ) {
        Ok((Some(playback), queue)) => (playback, queue),
        Ok((None, _)) => {
            error!("Failed to fetch current playback from Spotify API: Returned None");
            return;
        }
        Err(err) => {
            error!("Failed to fetch current playback and queue: {}", err);
            return;
        }
    };

    // Get current track and the upcoming queue
    let current_track = if let Some(PlayableItem::Track(track)) = queue.currently_playing {
        Track::from_rspotify(track)
    } else {
        return;
    };
    let new_queue: Vec<Track> = std::iter::once(current_track.clone())
        .chain(queue.queue.into_iter().filter_map(|item| match item {
            PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
            PlayableItem::Episode(_) | PlayableItem::Unknown(_) => None,
        }))
        .collect();

    // Start a task to fetch missing artists & images
    let missing_urls = new_queue
        .iter()
        .filter(|track| !IMAGES_CACHE.contains_key(&track.image_url))
        .map(|track| track.image_url.clone())
        .collect::<HashSet<_>>();
    let missing_artists = new_queue
        .iter()
        .filter_map(|track| {
            if ARTIST_DATA_CACHE.contains_key(&track.artist_id) {
                None
            } else {
                Some(track.artist_id.clone())
            }
        })
        .collect::<HashSet<_>>();
    if !missing_urls.is_empty() || !missing_artists.is_empty() {
        tokio::spawn(async move {
            // Grab artists in one go from spotify
            let Ok(artists) = spotify_client.artists(missing_artists).await else {
                return;
            };

            // Start downloading missing album images
            let mut set = JoinSet::new();
            for url in missing_urls {
                set.spawn(async move {
                    if let Err(err) = ensure_image_cached(url.as_str()).await {
                        warn!("failed to cache image {url}: {err}");
                    }
                });
            }

            // Cache artists, and download images
            for artist in artists {
                let artist_image = artist
                    .images
                    .into_iter()
                    .min_by_key(|img| img.width.unwrap())
                    .unwrap();
                ARTIST_DATA_CACHE.insert(
                    artist.id,
                    ArtistData {
                        name: artist.name.clone(),
                        genres: artist.genres.clone(),
                        popularity: artist.popularity as u8,
                        image_url: artist_image.url.clone(),
                    },
                );
                set.spawn(async move {
                    let url = artist_image.url.clone();
                    if let Err(err) = ensure_image_cached(url.as_str()).await {
                        warn!("failed to cache image {url}: {err}");
                    }
                });
            }

            // Concurrently run all tasks
            while set.join_next().await.is_some() {}

            // Now that we have both the album and artist images downloaded, update track color palette
            if let Err(err) = update_color_palettes() {
                warn!("failed to update color palettes: {err}");
            }
        });
    }

    // Update the playback state
    update_playback_state(move |state| {
        if state.current_context == current_playback.context
            && let Some(new_index) = state
                .queue
                .iter()
                .position(|track| track.title == current_track.title)
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
            state.current_context = current_playback.context;
        }

        // Trim old history to prevent unbounded growth
        if state.queue_index > MAX_HISTORY {
            state.queue.drain(0..(state.queue_index - MAX_HISTORY));
            state.queue_index = MAX_HISTORY;
        }

        state.playing = current_playback.is_playing;
        state.shuffle = current_playback.shuffle_state;
        if !used_mpris_progress {
            let progress = current_playback
                .progress
                .map_or(0, |p| p.num_milliseconds()) as u32;
            let http_delay = (request_start.elapsed().as_millis() / 2) as u32;
            state.progress = progress + http_delay;
            state.last_updated = Instant::now();
        }
    });
}

/// Downloads and caches an image from the given URL.
async fn ensure_image_cached(url: &str) -> Result<()> {
    if IMAGES_CACHE.contains_key(url) {
        return Ok(());
    }
    let response = HTTP_CLIENT.get(url).send().await?.error_for_status()?;
    let dynamic_image = image::load_from_memory(&response.bytes().await?)?;
    let (width, height) = dynamic_image.dimensions();
    let rgba = dynamic_image.to_rgba8();
    IMAGES_CACHE.insert(
        url.to_owned(),
        ImageData {
            data: Blob::from(rgba.into_raw()),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width,
            height,
        },
    );
    Ok(())
}

/// Downloads and caches an image from the given URL.
fn update_color_palettes() -> Result<()> {
    let state = PLAYBACK_STATE.lock().clone();
    let mut pending_palettes = Vec::new();
    for track in &state.queue {
        if !TRACK_DATA_CACHE.contains_key(&track.id)
            && let Some(image) = IMAGES_CACHE.get(&track.image_url)
            && let Some(artist_image_ref) = ARTIST_DATA_CACHE.get(&track.artist_id)
            && let Some(artist_image) = IMAGES_CACHE.get(&artist_image_ref.image_url)
        {
            // Merge the images side by side
            let width = image.width;
            let height = image.height;
            let album_image =
                RgbaImage::from_raw(width, height, image.data.data().to_vec()).unwrap();

            // Get palette, try on the album image or if that doesn't get enough colours include the artist image
            let swatches = {
                let palette: Palette<f64> = Palette::builder()
                    .algorithm(auto_palette::Algorithm::SNIC)
                    .filter(ChromaFilter { threshold: 20 })
                    .build(&auto_palette::ImageData::new(width, height, &album_image)?)?;
                let swatches = palette
                    .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                    .or_else(|_| palette.find_swatches(NUM_SWATCHES))?;
                if swatches.len() < NUM_SWATCHES {
                    // Generate a new image with the artist image
                    let artist_new_width = (width as f32 * 0.1).round() as u32;
                    let mut new_img = RgbaImage::new(width + artist_new_width, height);
                    image::imageops::overlay(&mut new_img, &album_image, 0, 0);
                    let artist_img_resized = image::imageops::resize(
                        &image::RgbaImage::from_raw(
                            artist_image.width,
                            artist_image.height,
                            artist_image.data.data().to_vec(),
                        )
                        .unwrap(),
                        artist_new_width,
                        height,
                        image::imageops::FilterType::Triangle,
                    );
                    image::imageops::overlay(
                        &mut new_img,
                        &artist_img_resized,
                        i64::from(width),
                        0,
                    );

                    let palette: Palette<f64> = Palette::builder()
                        .algorithm(auto_palette::Algorithm::SLIC)
                        .filter(ChromaFilter { threshold: 20 })
                        .build(&auto_palette::ImageData::new(
                            width + artist_new_width,
                            height,
                            &new_img,
                        )?)?;
                    palette
                        .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                        .or_else(|_| palette.find_swatches(NUM_SWATCHES))?
                } else {
                    swatches
                }
            };

            // Sort out the ratios
            let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
            let primary_colors = swatches
                .iter()
                .map(|s| {
                    let rgb = s.color().to_rgb();
                    // Sometimes ratios can be tiny like 0.05%, this brings them a little closer to even
                    let lerped_ratio = lerp(
                        0.5,
                        (s.ratio() / total_ratio_sum) as f32,
                        1.0 / swatches.len() as f32,
                    );
                    [rgb.r, rgb.g, rgb.b, (lerped_ratio * 255.0).round() as u8]
                })
                .sorted_by(|a, b| b[3].cmp(&a[3]))
                .collect::<Vec<_>>();

            let palette_seed = {
                let mut hasher = DefaultHasher::new();
                track.id.hash(&mut hasher);
                hasher.finish()
            };
            pending_palettes.push((track.id.clone(), primary_colors, palette_seed));
        }
    }
    drop(state);

    let generated_data: Vec<_> = pending_palettes
        .into_par_iter()
        .map(|(track_id, primary_colors, palette_seed)| {
            let palette_image = ImageData {
                data: Blob::from(generate_palette_image(&primary_colors, palette_seed)),
                format: ImageFormat::Rgba8,
                alpha_type: ImageAlphaType::Alpha,
                width: PALETTE_IMAGE_WIDTH,
                height: PALETTE_IMAGE_HEIGHT,
            };
            (
                track_id,
                TrackData {
                    primary_colors,
                    palette_image,
                },
            )
        })
        .collect();
    for (track_id, track_data) in generated_data {
        TRACK_DATA_CACHE.insert(track_id, track_data);
    }

    Ok(())
}

/// Skip to the specified track in the queue.
pub async fn skip_to_track(track_id: TrackId<'static>, point: Point, rect: Rect) {
    let Some(_interaction_guard) = SpotifyInteractionGuard::try_acquire() else {
        warn!("Spotify interaction already in progress; skip_to_track returning early");
        return;
    };

    let playback_state = PLAYBACK_STATE.lock().clone();
    let queue_index = playback_state.queue_index;
    let Some(position_in_queue) = playback_state.queue.iter().position(|t| t.id == track_id) else {
        error!("Track not found in queue");
        return;
    };
    match queue_index.cmp(&position_in_queue) {
        Ordering::Equal => {
            let position = (point.x - rect.x0) / rect.width();
            let song_ms = playback_state.queue[position_in_queue].milliseconds;
            // If click is near the very left, reset to the start of the song, else seek to clicked position
            let milliseconds = if point.x < 20.0 || position < 0.05 {
                0.0
            } else {
                f64::from(song_ms) * position
            };
            info!(
                "Seeking track {track_id} to {}%",
                (milliseconds / f64::from(song_ms) * 100.0).round()
            );
            if let Err(err) = SPOTIFY_CLIENT
                .get()
                .unwrap()
                .seek_track(TimeDelta::milliseconds(milliseconds as i64), None)
                .await
            {
                error!("Failed to seek track: {err}");
            }
        }
        Ordering::Greater => {
            let position_difference = queue_index - position_in_queue;
            info!("Rewinding to track {track_id}, {position_difference} skips");
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().previous_track(None).await {
                    error!("Failed to skip to track: {err}");
                }
            }
        }
        Ordering::Less => {
            let position_difference = position_in_queue - queue_index;
            info!("Skipping to track {track_id}, {position_difference} skips");
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().next_track(None).await {
                    error!("Failed to skip to track: {err}");
                }
            }
        }
    }

    update_state_from_spotify(false).await;
}

/// A filter that filters chroma values.
#[derive(Debug)]
pub struct ChromaFilter {
    threshold: u8,
}
impl auto_palette::Filter for ChromaFilter {
    fn test(&self, pixel: &auto_palette::Rgba) -> bool {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max - min) > self.threshold
    }
}

fn generate_palette_image(colors: &[[u8; 4]], seed: u64) -> Vec<u8> {
    let mut canvas = RgbaImage::from_pixel(
        PALETTE_IMAGE_WIDTH,
        PALETTE_IMAGE_HEIGHT,
        image::Rgba([12, 14, 18, 255]),
    );

    if colors.is_empty() {
        return canvas.into_raw();
    }

    let mut rng = SmallRng::seed_from_u64(seed);

    let mut targets = colors
        .iter()
        .map(|c| f32::from(c[3]).max(1.0))
        .collect::<Vec<_>>();
    let total_target = targets.iter().copied().sum::<f32>().max(1.0);
    for weight in &mut targets {
        *weight /= total_target;
    }

    let color_vectors = colors
        .iter()
        .map(|[r, g, b, _]| [f32::from(*r), f32::from(*g), f32::from(*b)])
        .collect::<Vec<_>>();

    let base = colors[0];
    for pixel in canvas.pixels_mut() {
        pixel.0 = [base[0], base[1], base[2], 255];
    }

    // Fill with the first colour; refinement passes will rebalance ratios.
    let total_pixels = (PALETTE_IMAGE_WIDTH * PALETTE_IMAGE_HEIGHT) as f32;

    let mut counts = vec![0u32; colors.len()];
    let mut coverage = vec![0.0f32; colors.len()];
    let mut per_color_strokes = vec![0u8; colors.len()];
    let mut available_indices = Vec::with_capacity(colors.len());

    for pass in 0..PALETTE_PASS_COUNT {
        let base_height = lerp(
            pass as f32 / PALETTE_PASS_COUNT as f32,
            PALETTE_IMAGE_HEIGHT as f32 * 0.7,
            PALETTE_IMAGE_HEIGHT as f32 * 0.3,
        );

        // Count pixels for each color, to get ratios
        counts.fill(0);
        for pixel in canvas.pixels() {
            let pr = f32::from(pixel[0]);
            let pg = f32::from(pixel[1]);
            let pb = f32::from(pixel[2]);
            let mut best_index = 0usize;
            let mut best_distance = f32::MAX;
            for (index, color) in color_vectors.iter().enumerate() {
                let dr = pr - color[0];
                let dg = pg - color[1];
                let db = pb - color[2];
                let distance = dr * dr + dg * dg + db * db;
                if distance < best_distance {
                    best_distance = distance;
                    best_index = index;
                }
            }
            counts[best_index] += 1;
        }
        for (index, ratio) in coverage.iter_mut().enumerate() {
            *ratio = counts[index] as f32 / total_pixels;
        }

        // Get how far we are off in total
        let total_coverage_diff = coverage
            .iter()
            .zip(targets.iter())
            .map(|(&c, &t)| (c - t).abs())
            .sum::<f32>()
            .abs();
        if total_coverage_diff <= f32::EPSILON {
            per_color_strokes.fill(0);
        } else {
            for (index, strokes) in per_color_strokes.iter_mut().enumerate() {
                *strokes = (((coverage[index] - targets[index]).abs() / total_coverage_diff)
                    * PALETTE_STROKES_PER_PASS as f32)
                    .floor() as u8;
            }
        }

        available_indices.clear();
        for (index, &count) in per_color_strokes.iter().enumerate() {
            if count > 0 {
                available_indices.push(index);
            }
        }

        for _ in 0..PALETTE_STROKES_PER_PASS {
            if available_indices.is_empty() {
                break;
            }

            // Randomly select an index from the available candidates
            let index_to_pick = rng.random_range(0..available_indices.len());
            let color_index = available_indices[index_to_pick];
            let strokes_left = &mut per_color_strokes[color_index];
            *strokes_left = strokes_left.saturating_sub(1);
            if *strokes_left == 0 {
                available_indices.swap_remove(index_to_pick);
            }
            let color = colors[color_index];

            // Pick a random brush
            let template = &BRUSHES[rng.random_range(0..BRUSHES.len())];
            let brush_factor = rng.random_range(0.75..1.2);
            let brush_size = (base_height * brush_factor)
                .round()
                .clamp(6.0, PALETTE_IMAGE_HEIGHT as f32) as u32;

            let mut stamp = image::imageops::resize(
                template,
                brush_size,
                brush_size,
                image::imageops::FilterType::Triangle,
            );

            let fade_factor = rng.random_range(0.55..0.9);
            for pixel in stamp.pixels_mut() {
                let alpha = pixel[3];
                if alpha == 0 {
                    continue;
                }
                let faded_alpha =
                    ((f32::from(alpha) * fade_factor).round().clamp(1.0, 255.0)) as u8;
                pixel[0] = color[0];
                pixel[1] = color[1];
                pixel[2] = color[2];
                pixel[3] = faded_alpha;
            }

            let offset_x =
                i64::from(rng.random_range(0..=PALETTE_IMAGE_WIDTH)) - i64::from(brush_size / 2);
            let offset_y =
                i64::from(rng.random_range(0..=PALETTE_IMAGE_HEIGHT)) - i64::from(brush_size / 2);

            image::imageops::overlay(&mut canvas, &stamp, offset_x, offset_y);
        }
    }

    // Blur the image
    image::imageops::blur(&canvas, 10.0).into_raw()
}

fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

use anyhow::Result;
use auto_palette::Palette;
use dashmap::DashMap;
use image::{GenericImageView, RgbaImage, imageops};
use parking_lot::Mutex;
use reqwest::Client;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{AdditionalType, ArtistId, Context, FullTrack, Id, PlayableItem, artist},
    prelude::{BaseClient, OAuthClient},
    scopes,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::TryInto,
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
use vello::peniko::{Blob, ImageAlphaType, ImageData, ImageFormat};
use zbus::{
    Connection,
    fdo::{DBusProxy, PropertiesProxy},
    names::InterfaceName,
    zvariant::OwnedValue,
};

/// Blur sigma applied to album artwork when generating the shader background.
const BACKGROUND_BLUR_SIGMA: f32 = 0.1;
/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

/// Maximum number of historical tracks to keep before trimming.
const MAX_HISTORY: usize = 20;

/// MPRIS interface identifier used for playback control.
const PLAYER_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2.Player");
/// Root MPRIS interface that exposes metadata and identity.
const ROOT_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2");
/// Object path for the Spotify MPRIS instance on D-Bus.
const MPRIS_OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PlaybackState::default())));
pub static IMAGES_CACHE: LazyLock<DashMap<String, CachedImage>> = LazyLock::new(DashMap::new);
pub static ARTIST_IMAGES_CACHE: LazyLock<DashMap<String, Image>> = LazyLock::new(DashMap::new);
pub static TRACK_DATA_CACHE: LazyLock<DashMap<String, TrackData>> = LazyLock::new(DashMap::new);
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
static SPOTIFY_CLIENT: OnceCell<AuthCodeSpotify> = OnceCell::const_new();
static SPOTIFY_INTERACTION_ACTIVE: AtomicBool = AtomicBool::new(false);

struct SpotifyInteractionGuard;

impl SpotifyInteractionGuard {
    fn try_acquire() -> Option<Self> {
        if SPOTIFY_INTERACTION_ACTIVE
            .compare_exchange(
                false,
                true,
                AtomicOrdering::Acquire,
                AtomicOrdering::Relaxed,
            )
            .is_ok()
        {
            Some(Self)
        } else {
            None
        }
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
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            last_updated: Instant::now(),
            playing: false,
            shuffle: false,
            progress: 0,
            queue: Vec::new(),
            queue_index: 0,
            current_context: None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub artist_name: String,
    pub album_name: String,
    pub image: Image,
    pub release_date: String,
    pub milliseconds: u32,
}

#[derive(Debug, Default, Clone)]
pub struct TrackData {
    /// Simplified color palette (RGBA, alpha = percentage 0-100).
    pub primary_colors: Vec<[u8; 4]>,
}

#[derive(Debug, Default, Clone)]
pub struct Image {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Clone)]
pub struct CachedImage {
    pub original: ImageData,
    pub blurred: ImageData,
}

impl Track {
    fn from_rspotify(track: FullTrack) -> Self {
        let FullTrack {
            id,
            name,
            artists,
            album,
            duration,
            ..
        } = track;
        let album_image = album
            .images
            .into_iter()
            .min_by_key(|img| img.width.unwrap())
            .unwrap();
        let image = Image {
            url: album_image.url,
            width: album_image.width.unwrap() as u16,
            height: album_image.height.unwrap() as u16,
        };
        let artist = artists.first().unwrap();
        Self {
            id: id.unwrap().id().to_string(),
            title: name,
            artist_id: artist.id.clone().unwrap().id().to_string(),
            artist_name: artist.name.clone(),
            album_name: album.name,
            image,
            release_date: album.release_date.unwrap(),
            milliseconds: duration.num_milliseconds() as u32,
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
    let mut spotify_poll_counter = 100; // Counter for Spotify API polling

    loop {
        // --- MPRIS Polling Logic ---
        let (should_refresh_spotify, used_mpris_progress) =
            update_state_from_mpris(&connection, &dbus_proxy, &mut last_mpris_track_id).await;

        // --- Spotify API Polling Logic ---
        spotify_poll_counter += 1;
        if spotify_poll_counter >= 3 || should_refresh_spotify {
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

    let current_track = if let Some(PlayableItem::Track(track)) = queue.currently_playing {
        Track::from_rspotify(track)
    } else {
        return;
    };

    let new_queue: Vec<Track> = std::iter::once(current_track.clone())
        .chain(queue.queue.into_iter().filter_map(|item| match item {
            PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
            _ => None,
        }))
        .collect();

    // Start a task to fetch missing artists & images
    let missing_urls = new_queue
        .iter()
        .filter(|track| !IMAGES_CACHE.contains_key(&track.image.url))
        .map(|track| track.image.url.clone())
        .collect::<HashSet<_>>();
    let missing_artists = new_queue
        .iter()
        .filter_map(|track| {
            if ARTIST_IMAGES_CACHE.contains_key(&track.artist_id) {
                None
            } else {
                Some(track.artist_id.clone())
            }
        })
        .collect::<HashSet<_>>();
    if !missing_urls.is_empty() || !missing_artists.is_empty() {
        tokio::spawn(async move {
            // Grab artists in one go from spotify
            let Ok(artists) = spotify_client
                .artists(
                    missing_artists
                        .iter()
                        .map(|id| ArtistId::from_id(id).unwrap()),
                )
                .await
            else {
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
                ARTIST_IMAGES_CACHE.insert(
                    artist.id.id().to_string(),
                    Image {
                        url: artist_image.url.clone(),
                        width: artist_image.width.unwrap() as u16,
                        height: artist_image.height.unwrap() as u16,
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
                .position(|t| t.title == current_track.title)
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
    let blurred_rgba = imageops::blur(&rgba, BACKGROUND_BLUR_SIGMA * width as f32);

    let original = ImageData {
        data: Blob::from(rgba.into_raw()),
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::Alpha,
        width,
        height,
    };
    let blurred = ImageData {
        data: Blob::from(blurred_rgba.into_raw()),
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::Alpha,
        width,
        height,
    };

    IMAGES_CACHE.insert(url.to_string(), CachedImage { original, blurred });
    Ok(())
}

/// Downloads and caches an image from the given URL.
fn update_color_palettes() -> Result<()> {
    let state = PLAYBACK_STATE.lock().clone();
    for track in &state.queue {
        if !TRACK_DATA_CACHE.contains_key(&track.id)
            && let Some(image) = IMAGES_CACHE.get(&track.image.url)
            && let Some(artist_image_ref) = ARTIST_IMAGES_CACHE.get(&track.artist_id)
            && let Some(artist_image) = IMAGES_CACHE.get(&artist_image_ref.url)
        {
            // Merge the images side by side
            let width = image.original.width;
            let height = image.original.height;
            let artist_new_width = (width as f32 * 0.25).round() as u32;
            let mut new_img = RgbaImage::new(width + artist_new_width, height);
            image::imageops::overlay(
                &mut new_img,
                &RgbaImage::from_raw(width, height, image.original.data.data().to_vec()).unwrap(),
                0,
                0,
            );
            let artist_img_resized = image::imageops::resize(
                &image::RgbaImage::from_raw(
                    artist_image.original.width,
                    artist_image.original.height,
                    artist_image.original.data.data().to_vec(),
                )
                .unwrap(),
                artist_new_width,
                height,
                image::imageops::FilterType::Triangle,
            );
            image::imageops::overlay(&mut new_img, &artist_img_resized, i64::from(width), 0);

            // Get palette
            let palette: Palette<f64> = Palette::builder()
                .algorithm(auto_palette::Algorithm::KMeans)
                .filter(ChromaFilter::new(30))
                .build(&auto_palette::ImageData::new(
                    width + artist_new_width,
                    height,
                    &new_img.into_vec(),
                )?)?;
            let swatches = palette
                .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Colorful)
                .or_else(|_| {
                    palette.find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                })
                .or_else(|_| palette.find_swatches(NUM_SWATCHES))?;
            let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
            let mut primary_colors = swatches
                .iter()
                .map(|s| {
                    let rgb = s.color().to_rgb();
                    [
                        rgb.r,
                        rgb.g,
                        rgb.b,
                        ((s.ratio() / total_ratio_sum) * 255.0).round() as u8,
                    ]
                })
                .collect::<Vec<_>>();
            primary_colors.sort_by(|a, b| b[3].cmp(&a[3]));

            TRACK_DATA_CACHE.insert(track.id.clone(), TrackData { primary_colors });
        }
    }
    drop(state);

    Ok(())
}

/// Skip to the specified track in the queue.
pub async fn skip_to_track(track_id: &str) {
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
            info!("Already playing track {track_id}");
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

impl ChromaFilter {
    const fn new(threshold: u8) -> Self {
        Self { threshold }
    }
}

impl auto_palette::Filter for ChromaFilter {
    fn test(&self, pixel: &auto_palette::Rgba) -> bool {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max - min) > self.threshold
    }
}

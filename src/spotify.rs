use anyhow::Result;
use dashmap::DashMap;
use image::{GenericImageView, imageops};
use parking_lot::Mutex;
use reqwest::Client;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{AdditionalType, Context, FullTrack, Id, PlayableItem},
    prelude::OAuthClient,
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
/// Minimum channel spread (max - min RGB) required for a sample to count as colorful; raise to allow muddier tones.
const PRIMARY_COLOR_MIN_CHROMA: u8 = 60;
/// Minimum brightness (max RGB channel) required before accepting a sample; lower to admit darker hues.
const PRIMARY_COLOR_MIN_BRIGHTNESS: u8 = 50;
/// RGB distance threshold—pixels within this radius join the nearest cluster; higher values merge more aggressively.
const PRIMARY_COLOR_DISTANCE_THRESHOLD: f32 = 110.0;

/// MPRIS interface identifier used for playback control.
const PLAYER_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2.Player");
/// Root MPRIS interface that exposes metadata and identity.
const ROOT_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2");
/// Object path for the Spotify MPRIS instance on D-Bus.
const MPRIS_OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

/// Maximum number of historical tracks to keep before trimming.
const MAX_HISTORY: usize = 20;
/// Shared playback state guarded by a mutex for renderer access.
pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PlaybackState::default())));
/// Cache of album art (original, blurred, palette) keyed by URL.
pub static IMAGES_CACHE: LazyLock<DashMap<String, CachedImage>> = LazyLock::new(DashMap::new);
/// Shared HTTP client reused across Spotify API and image download requests.
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
/// Lazily initialised Spotify Web API client authenticated via OAuth.
static SPOTIFY_CLIENT: OnceCell<AuthCodeSpotify> = OnceCell::const_new();
/// Flag preventing concurrent Spotify interaction calls that would conflict.
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
    pub name: String,
    pub artists: Vec<String>,
    pub album_name: String,
    pub image: Image,
    pub release_date: String,
    pub milliseconds: u32,
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
    /// Simplified color palette (RGBA, alpha = percentage 0-100).
    pub primary_colors: Vec<[u8; 4]>,
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
        let (image_url, image_width, image_height) = (
            album_image.url,
            album_image.width.unwrap() as u16,
            album_image.height.unwrap() as u16,
        );
        let image = Image {
            url: image_url,
            width: image_width,
            height: image_height,
        };
        Self {
            id: id.unwrap().id().to_string(),
            name,
            artists: artists.into_iter().map(|a| a.name).collect(),
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
    let future_tracks: Vec<Track> = queue
        .queue
        .into_iter()
        .filter_map(|item| match item {
            PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
            _ => None,
        })
        .collect();

    // Start a task to fetch missing images
    let mut missing_urls = HashSet::new();
    if !IMAGES_CACHE.contains_key(&current_track.image.url) {
        missing_urls.insert(current_track.image.url.clone());
    }
    for track in &future_tracks {
        if !IMAGES_CACHE.contains_key(&track.image.url) {
            missing_urls.insert(track.image.url.clone());
        }
    }
    if !missing_urls.is_empty() {
        tokio::spawn(async move {
            let mut set = JoinSet::new();
            for url in missing_urls {
                set.spawn(async move {
                    if let Err(err) = ensure_image_cached(url.as_str()).await {
                        warn!("failed to cache image {url}: {err}");
                    }
                });
            }
            while set.join_next().await.is_some() {}
        });
    }

    // Update the playback state
    update_playback_state(move |state| {
        let new_queue: Vec<Track> = std::iter::once(current_track.clone())
            .chain(future_tracks.into_iter())
            .collect();

        if state.current_context == current_playback.context
            && let Some(new_index) = state
                .queue
                .iter()
                .position(|t| t.name == current_track.name)
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

/// Extracts up to four vivid colors from an RGBA image and encodes each color's share (0-100) in its alpha channel.
fn compute_primary_colors(pixels: &[u8]) -> Vec<[u8; 4]> {
    /// Incrementally tracks a cluster's running RGB average and size.
    struct Cluster {
        avg: [f32; 3],
        count: u32,
    }

    impl Cluster {
        fn new(r: u8, g: u8, b: u8) -> Self {
            Self {
                avg: [f32::from(r), f32::from(g), f32::from(b)],
                count: 1,
            }
        }

        fn add(&mut self, r: u8, g: u8, b: u8) {
            let count_f = self.count as f32;
            self.avg[0] = (self.avg[0] * count_f + f32::from(r)) / (count_f + 1.0);
            self.avg[1] = (self.avg[1] * count_f + f32::from(g)) / (count_f + 1.0);
            self.avg[2] = (self.avg[2] * count_f + f32::from(b)) / (count_f + 1.0);
            self.count += 1;
        }

        fn distance_sq(&self, r: u8, g: u8, b: u8) -> f32 {
            let dr = self.avg[0] - f32::from(r);
            let dg = self.avg[1] - f32::from(g);
            let db = self.avg[2] - f32::from(b);
            dr * dr + dg * dg + db * db
        }

        fn rgb(&self) -> (u8, u8, u8) {
            let clamp = |value: f32| value.round().clamp(0.0, 255.0) as u8;
            (clamp(self.avg[0]), clamp(self.avg[1]), clamp(self.avg[2]))
        }
    }

    // Accumulate candidate clusters as we walk the image once.
    let mut clusters: Vec<Cluster> = Vec::new();
    let mut considered: u32 = 0;
    let distance_threshold_sq = PRIMARY_COLOR_DISTANCE_THRESHOLD * PRIMARY_COLOR_DISTANCE_THRESHOLD;

    for pixel in pixels.chunks_exact(4) {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3];

        if a < 32 {
            continue;
        }

        let max_channel = r.max(g).max(b);
        let min_channel = r.min(g).min(b);
        if max_channel < PRIMARY_COLOR_MIN_BRIGHTNESS {
            continue;
        }
        if max_channel.saturating_sub(min_channel) < PRIMARY_COLOR_MIN_CHROMA {
            continue;
        }

        considered += 1;

        if let Some((best_idx, best_dist)) = clusters
            .iter()
            .enumerate()
            .map(|(idx, cluster)| (idx, cluster.distance_sq(r, g, b)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
            && best_dist <= distance_threshold_sq
        {
            clusters[best_idx].add(r, g, b);
            continue;
        }

        // No cluster close enough, so start a new one anchored at this color.
        clusters.push(Cluster::new(r, g, b));
    }

    if considered == 0 {
        return Vec::new();
    }

    // Convert surviving clusters into a list that keeps RGB alongside coverage share.
    let mut buckets: Vec<([u8; 3], f32)> = clusters
        .into_iter()
        .filter_map(|cluster| {
            if cluster.count == 0 {
                return None;
            }
            let (r, g, b) = cluster.rgb();
            let max_channel = r.max(g).max(b);
            let min_channel = r.min(g).min(b);
            if max_channel < PRIMARY_COLOR_MIN_BRIGHTNESS {
                return None;
            }
            if max_channel.saturating_sub(min_channel) < PRIMARY_COLOR_MIN_CHROMA {
                return None;
            }
            let coverage = cluster.count as f32 / considered as f32;
            Some((cluster.rgb().into(), coverage))
        })
        .collect();

    // Sort by coverage so we can slice down to the most visually significant colors.
    buckets.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    let dominant: Vec<([u8; 3], f32)> = buckets.into_iter().take(4).collect();

    if dominant.is_empty() {
        return Vec::new();
    }

    // Renormalize coverage so the selected colors sum to exactly 100%.
    let total_share: f32 = dominant.iter().map(|(_, share)| share).sum();
    if total_share <= f32::EPSILON {
        return dominant
            .into_iter()
            .map(|(rgb, _)| [rgb[0], rgb[1], rgb[2], 0])
            .collect();
    }

    // Distribute integer percentages while preserving any rounding remainder for the last color.
    let mut remaining = 100i32;
    let last_index = dominant.len().saturating_sub(1);
    let mut result = Vec::with_capacity(dominant.len());
    for (index, (rgb, share)) in dominant.into_iter().enumerate() {
        let assigned = if index == last_index {
            remaining
        } else {
            let scaled = (share / total_share * 100.0).round() as i32;
            let clamped = scaled.clamp(0, remaining);
            remaining -= clamped;
            clamped
        };
        let alpha = assigned.clamp(0, 100) as u8;
        result.push([rgb[0], rgb[1], rgb[2], alpha]);
    }

    result
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
    let primary_colors = compute_primary_colors(rgba.as_raw());
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

    IMAGES_CACHE.insert(
        url.to_string(),
        CachedImage {
            original,
            blurred,
            primary_colors,
        },
    );
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

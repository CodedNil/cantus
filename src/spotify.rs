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
    sync::{Arc, LazyLock},
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

const PLAYER_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2.Player");
const ROOT_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2");
const MPRIS_OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";
const BACKGROUND_BLUR_SIGMA: f32 = 0.1;

/// Stores the current playback state
const MAX_HISTORY: usize = 20;
pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PlaybackState::default())));
pub static IMAGES_CACHE: LazyLock<DashMap<String, CachedImage>> = LazyLock::new(DashMap::new);
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
static SPOTIFY_CLIENT: OnceCell<AuthCodeSpotify> = OnceCell::const_new();

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
}

impl Track {
    fn from_rspotify(track: FullTrack) -> Self {
        let image = track
            .album
            .images
            .into_iter()
            .min_by_key(|img| img.width.unwrap())
            .map(|img| Image {
                url: img.url,
                width: img.width.unwrap() as u16,
                height: img.height.unwrap() as u16,
            })
            .unwrap();
        Self {
            id: track.id.unwrap().id().to_string(),
            name: track.name,
            artists: track.artists.into_iter().map(|a| a.name).collect(),
            album_name: track.album.name,
            image,
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
    let mut spotify_poll_counter = 100; // Counter for Spotify API polling

    let connection = match Connection::session().await {
        Ok(conn) => conn,
        Err(err) => panic!("Failed to connect to D-Bus session: {err}"),
    };

    let dbus_proxy = match DBusProxy::new(&connection).await {
        Ok(proxy) => proxy,
        Err(err) => panic!("Failed creating D-Bus proxy: {err}"),
    };

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

/// Skip to the specified track in the queue.
pub async fn skip_to_track(track_id: &str) {
    let playback_state = PLAYBACK_STATE.lock().clone();
    let queue_index = playback_state.queue_index;
    let Some(position_in_queue) = playback_state.queue.iter().position(|t| t.id == track_id) else {
        error!("Track not found in queue");
        return;
    };
    match queue_index.cmp(&position_in_queue) {
        Ordering::Equal => {
            info!("Already playing track {}", track_id);
        }
        Ordering::Greater => {
            let position_difference = queue_index - position_in_queue;
            info!(
                "Rewinding to track {}, {} skips",
                track_id, position_difference
            );
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().previous_track(None).await {
                    error!("Failed to skip to track: {}", err);
                }
            }
        }
        Ordering::Less => {
            let position_difference = position_in_queue - queue_index;
            info!(
                "Skipping to track {}, {} skips",
                track_id, position_difference
            );
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().next_track(None).await {
                    error!("Failed to skip to track: {}", err);
                }
            }
        }
    }
}

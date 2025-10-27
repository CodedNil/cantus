use anyhow::Result;
use dashmap::DashMap;
use futures::future::join_all;
use image::{GenericImageView, imageops};
use parking_lot::Mutex;
use reqwest::Client;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{AdditionalType, FullTrack, PlayableItem},
    prelude::OAuthClient,
    scopes,
};
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    sync::{Arc, LazyLock},
    time::Instant,
};
use tokio::time::{Duration, sleep};
use tracing::warn;
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
const BACKGROUND_BLUR_SIGMA: f32 = 3.0;

/// Stores the current playback state
pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PlaybackState::default())));
pub static IMAGES_CACHE: LazyLock<DashMap<String, CachedImage>> = LazyLock::new(DashMap::new);
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

#[derive(Clone)]
pub struct PlaybackState {
    pub last_updated: Instant,
    pub playing: bool,
    pub shuffle: bool,
    pub progress: u64,
    pub currently_playing: Option<Track>,
    pub queue: Vec<Track>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            last_updated: Instant::now(),
            playing: false,
            shuffle: false,
            progress: 0,
            currently_playing: None,
            queue: Vec::new(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Track {
    pub name: String,
    pub artists: Vec<String>,
    pub album_name: String,
    pub image: Image,
    pub release_date: String,
    pub milliseconds: u64,
}

#[derive(Default, Clone)]
pub struct Image {
    pub url: String,
    pub width: u32,
    pub height: u32,
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
            .max_by_key(|img| img.width.unwrap())
            .map(|img| Image {
                url: img.url,
                width: img.width.unwrap(),
                height: img.height.unwrap(),
            })
            .unwrap();
        Self {
            name: track.name,
            artists: track.artists.into_iter().map(|a| a.name).collect(),
            album_name: track.album.name,
            image,
            release_date: track.album.release_date.unwrap(),
            milliseconds: track.duration.num_milliseconds() as u64,
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

/// Initializes the Spotify client and spawns the combined MPRIS and Spotify polling task.
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
                "playlist-modify-public",
                "user-read-playback-position",
                "user-read-recently-played"
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

    // Spawn the combined polling task
    tokio::spawn(polling_task(spotify));
}

/// Asynchronous task to poll MPRIS every 500ms and Spotify API every 4 seconds or on song change.
async fn polling_task(spotify_client: AuthCodeSpotify) {
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
        let should_refresh_spotify =
            update_state_from_mpris(&connection, &dbus_proxy, &mut last_mpris_track_id).await;

        // --- Spotify API Polling Logic ---
        spotify_poll_counter += 1;
        if spotify_poll_counter >= 8 || should_refresh_spotify {
            spotify_poll_counter = 0; // Reset counter
            update_state_from_spotify(&spotify_client).await;
        }

        sleep(Duration::from_millis(500)).await;
    }
}

/// Synchronizes playback information with the MPRIS interface and returns whether Spotify data should refresh.
async fn update_state_from_mpris(
    connection: &Connection,
    dbus_proxy: &DBusProxy<'_>,
    last_track_id: &mut Option<String>,
) -> bool {
    let Ok(names) = dbus_proxy.list_names().await else {
        return false;
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
        return false;
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
    let should_refresh = new_track_id
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

    if playing.is_some() || progress.is_some() {
        update_playback_state(|state| {
            if let Some(playing) = playing {
                state.playing = playing;
            }
            if let Some(progress) = progress {
                state.progress = progress as u64;
                state.last_updated = Instant::now();
            }
        });
    }

    should_refresh
}

/// Pulls the current playback queue and status from the Spotify Web API and updates shared state.
async fn update_state_from_spotify(spotify_client: &AuthCodeSpotify) {
    if let Ok(queue) = spotify_client.current_user_queue().await {
        let currently_playing_track = queue.currently_playing.and_then(|item| match item {
            PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
            _ => None,
        });
        let queued_tracks: Vec<Track> = queue
            .queue
            .into_iter()
            .filter_map(|item| match item {
                PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
                _ => None,
            })
            .collect();

        // Start a task to fetch missing images
        let mut missing_urls = HashSet::new();
        if let Some(track) = currently_playing_track.as_ref()
            && !IMAGES_CACHE.contains_key(&track.image.url)
        {
            missing_urls.insert(track.image.url.clone());
        }
        for track in &queued_tracks {
            if !IMAGES_CACHE.contains_key(&track.image.url) {
                missing_urls.insert(track.image.url.clone());
            }
        }
        if !missing_urls.is_empty() {
            tokio::spawn(async move {
                join_all(missing_urls.into_iter().map(|url| async move {
                    if let Err(err) = ensure_image_cached(url.as_str()).await {
                        warn!("failed to cache image {url}: {err}");
                    }
                }))
                .await;
            });
        }

        // Update the playback state
        update_playback_state(move |state| {
            state.currently_playing = currently_playing_track;
            state.queue = queued_tracks;
        });
    }

    if let Ok(Some(playback)) = spotify_client
        .current_playback(None, None::<Vec<&AdditionalType>>)
        .await
    {
        let is_playing = playback.is_playing;
        let shuffle = playback.shuffle_state;
        let progress = playback.progress.map_or(0, |p| p.num_milliseconds());

        update_playback_state(|state| {
            state.playing = is_playing;
            state.shuffle = shuffle;
            state.progress = progress as u64;
            state.last_updated = Instant::now();
        });
    }
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
    let blurred_rgba = dynamic_image.to_rgba8(); //imageops::blur(&rgba, BACKGROUND_BLUR_SIGMA);

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

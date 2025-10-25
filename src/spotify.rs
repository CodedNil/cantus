use dashmap::DashMap;
use image::DynamicImage;
use parking_lot::Mutex;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{AdditionalType, FullTrack, PlayableItem},
    prelude::OAuthClient,
    scopes,
};
use std::{
    collections::HashMap,
    convert::TryInto,
    sync::{Arc, LazyLock},
};
use tokio::time::{Duration, sleep};
use zbus::{
    Connection,
    fdo::{DBusProxy, PropertiesProxy},
    names::InterfaceName,
    zvariant::{OwnedObjectPath, OwnedValue},
};

const PLAYER_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2.Player");
const ROOT_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2");
const MPRIS_OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

/// Stores the current playback state
pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PlaybackState::default())));
pub static IMAGES_CACHE: LazyLock<DashMap<String, DynamicImage>> = LazyLock::new(DashMap::new);

#[derive(Default, Clone, Debug)]
pub struct PlaybackState {
    pub playing: bool,
    pub shuffle: bool,
    pub progress: i64,
    pub currently_playing: Option<Track>,
    pub queue: Vec<Track>,
}

#[derive(Default, Clone, Debug)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
    pub album_id: String,
    pub album_name: String,
    pub image: Image,
    pub release_date: String,
    pub release_date_precision: String,
    pub milliseconds: u64,
}

#[derive(Default, Clone, Debug)]
pub struct Image {
    pub url: String,
    pub width: u32,
    pub height: u32,
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
                width: img.width.unwrap(),
                height: img.height.unwrap(),
            })
            .unwrap();
        Self {
            id: track.id.unwrap().to_string(),
            name: track.name,
            artists: track.artists.into_iter().map(|a| a.name).collect(),
            album_id: track.album.id.unwrap().to_string(),
            album_name: track.album.name,
            image,
            release_date: track.album.release_date.unwrap(),
            release_date_precision: track.album.release_date_precision.unwrap(),
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
        if proxy
            .get(ROOT_INTERFACE, "Identity")
            .await
            .ok()
            .map(|value| value.to_string())
            .as_deref()
            == Some("Spotify")
        {
            properties_proxy = Some(proxy);
            break;
        }
    }
    let Some(properties_proxy) = properties_proxy else {
        return false;
    };

    let mut should_refresh = false;

    if let Some(track_id) = properties_proxy
        .get(PLAYER_INTERFACE, "Metadata")
        .await
        .ok()
        .and_then(|metadata| -> Option<String> {
            let metadata = HashMap::<String, OwnedValue>::try_from(metadata).ok()?;
            if let Some(track_id_value) = metadata.get("mpris:trackid") {
                if let Ok(path) = OwnedObjectPath::try_from(track_id_value.clone()) {
                    return Some(path.to_string());
                }
                if let Ok(track_id) = track_id_value.clone().try_into() {
                    return Some(track_id);
                }
            }
            metadata
                .get("xesam:url")
                .and_then(|value| value.clone().try_into().ok())
        })
        && last_track_id.as_ref() != Some(&track_id)
    {
        *last_track_id = Some(track_id);
        should_refresh = true;
    }

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
                state.progress = progress;
            }
        });
    }

    should_refresh
}

/// Pulls the current playback queue and status from the Spotify Web API and updates shared state.
async fn update_state_from_spotify(spotify_client: &AuthCodeSpotify) {
    if let Ok(queue) = spotify_client.current_user_queue().await {
        update_playback_state(move |state| {
            state.currently_playing = queue.currently_playing.and_then(|item| match item {
                PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
                _ => None,
            });
            tracing::info!(
                "Updated currently playing track {:?}",
                state.currently_playing
            );
            state.queue = queue
                .queue
                .into_iter()
                .filter_map(|item| match item {
                    PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
                    _ => None,
                })
                .collect();
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
            state.progress = progress;
        });
    }
}

use crate::{
    AppUpdater, MAX_HISTORY_TRACKS, Update,
    art::{self, ArtState},
    config::{self, Config},
    model::{CondensedPlaylist, PlaylistId, PlaylistTracks, Track, TrackId, deserialize_images},
    send_update,
};
use arrayvec::{ArrayString, ArrayVec};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use cantus_shared::{MAX_PILL_PLAYLIST_ICONS, PackedAudioFeatures};
use jiff::Timestamp;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufRead, BufReader, Write},
    mem,
    net::TcpListener,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc,
        mpsc::{self, Receiver, RecvTimeoutError, Sender},
    },
    thread::spawn,
    time::{Duration, Instant},
};
use thiserror::Error;
use tracing::{error, info, warn};
use ureq::{
    Agent, Body,
    http::{Error as HttpRequestError, Method, Request, Response},
};

const API_BASE: &str = "https://api.spotify.com/v1";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const PLAYLIST_TRACKS_CACHE: &str = "cantus_playlist_tracks.json";
const SPOTIFY_TOKEN_CACHE: &str = "spotify_cache.json";
const RECCO_TRACK_URL: &str = "https://api.reccobeats.com/v1/track";
const SCOPES: &str = "\
user-read-playback-state user-modify-playback-state user-read-currently-playing \
playlist-read-private playlist-read-collaborative playlist-modify-private playlist-modify-public \
user-library-read user-library-modify";

struct SpotifyState {
    current_context: Option<String>,
    context_updated: bool,
    last_grabbed_queue: Instant,
}

#[derive(Deserialize)]
struct ReccoTrack {
    id: String,
    href: String,
}

#[derive(Deserialize)]
struct ReccoPage<T> {
    content: Vec<T>,
}

const VERIFIER_BYTES: usize = 43;
const REDIRECT_HOST: &str = "127.0.0.1";
const REDIRECT_PORT: u16 = 7474;
const REDIRECT_URI: &str = "http://127.0.0.1:7474/callback";

struct SpotifyClient {
    client_id: String,
    cache_path: PathBuf,
    token: OAuthCredentials,
    http: Agent,
}

#[derive(Deserialize)]
struct PartialTrack {
    id: Option<TrackId>,
}

#[derive(Deserialize)]
struct Playlist {
    id: PlaylistId,
    name: String,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    image: Option<String>,
    snapshot_id: ArrayString<32>,
    tracks: TracksRef,
}

#[derive(Deserialize)]
struct PlaylistItem {
    track: Option<PartialTrack>,
}

#[derive(Deserialize)]
struct Context {
    uri: String,
}

#[derive(Deserialize)]
struct CurrentPlaybackContext {
    device: Device,
    context: Option<Context>,
    #[serde(default)]
    progress_ms: u32,
    is_playing: bool,
    item: Option<Track>,
}

#[derive(Deserialize)]
struct CurrentUserQueue {
    currently_playing: Option<Track>,
    queue: Vec<Track>,
}

#[derive(Deserialize)]
struct Device {
    volume_percent: Option<u8>,
}

#[derive(Serialize, Deserialize)]
struct OAuthCredentials {
    access_token: String,
    expires_at: i64,
    refresh_token: Option<String>,
    #[serde(default)]
    scope: String,
}

impl OAuthCredentials {
    fn is_expired(&self) -> bool {
        Timestamp::now().as_second().saturating_add(10) >= self.expires_at
    }
}

fn parse_credentials(json: &str) -> serde_json::Result<OAuthCredentials> {
    let mut value = serde_json::from_str::<serde_json::Value>(json)?;
    if let Some(ttl) = value.get("expires_in").and_then(serde_json::Value::as_i64) {
        value["expires_at"] = Timestamp::now().as_second().saturating_add(ttl).into();
    }
    serde_json::from_value(value)
}

fn response_json<T: DeserializeOwned>(mut response: Response<Body>) -> ClientResult<T> {
    serde_json::from_reader(response.body_mut().as_reader()).map_err(Into::into)
}

fn read_token_cache(cache_path: &Path, scopes: &str) -> ClientResult<Option<OAuthCredentials>> {
    let token = match fs::read_to_string(cache_path) {
        Ok(cache) => parse_credentials(&cache)
            .inspect_err(|err| warn!("Failed to parse Spotify token cache: {err}"))
            .ok(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(err) => return Err(err.into()),
    };
    Ok(token.filter(|token| {
        scopes.split_whitespace().all(|required| {
            token
                .scope
                .split_whitespace()
                .any(|scope| scope == required)
        })
    }))
}

fn prompt_for_token(
    url: &str,
    client_id: &str,
    verifier: &str,
    http: &Agent,
    expected_state: &str,
) -> ClientResult<OAuthCredentials> {
    match Command::new("xdg-open").arg(url).spawn() {
        Ok(_) => println!("Opened {url} in your browser."),
        Err(err) => eprintln!(
            "Error when trying to open an URL in your browser: {err:?}. Please navigate here manually: {url}"
        ),
    }

    let listener = TcpListener::bind((REDIRECT_HOST, REDIRECT_PORT))?;
    let (mut stream, _) = listener.accept()?;
    let mut request_line = String::new();
    BufReader::new(&stream).read_line(&mut request_line)?;

    let request_target = request_line
        .split_whitespace()
        .nth(1)
        .ok_or(ClientError::InvalidAuthorizationResponse)?;
    let query = request_target
        .split_once('?')
        .map(|(_, query)| query)
        .ok_or(ClientError::InvalidAuthorizationResponse)?;
    let mut params = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<HashMap<_, _>>();
    let code = params
        .remove("code")
        .ok_or(ClientError::InvalidAuthorizationResponse)?;
    let actual_state = params
        .remove("state")
        .ok_or(ClientError::InvalidAuthorizationResponse)?;
    if actual_state != expected_state {
        return Err(ClientError::InvalidAuthorizationState);
    }

    let message = "Cantus connected successfully, this tab can be closed.";
    write!(
        stream,
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{message}",
        message.len(),
    )?;

    let response = http
        .post(TOKEN_URL)
        .send_form([
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", REDIRECT_URI),
            ("client_id", client_id),
            ("code_verifier", verifier),
        ])?
        .into_body()
        .read_to_string()?;
    Ok(parse_credentials(&response)?)
}

impl SpotifyClient {
    fn authorization(&mut self) -> ClientResult<String> {
        if self.token.is_expired() {
            self.refresh_token()?;
        }
        Ok(format!("Bearer {}", self.token.access_token))
    }

    fn refresh_token(&mut self) -> ClientResult<()> {
        self.token = self.refetch_token()?;
        self.write_token_cache()
    }

    fn retry_unauthorized<T>(
        &mut self,
        mut request: impl FnMut(&mut Self) -> ClientResult<T>,
    ) -> ClientResult<T> {
        match request(self) {
            Err(ClientError::Http(ureq::Error::StatusCode(401))) => {
                self.refresh_token()?;
                request(self)
            }
            result => result,
        }
    }

    fn write_token_cache(&self) -> ClientResult<()> {
        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.cache_path, serde_json::to_vec(&self.token)?)?;
        Ok(())
    }

    fn refetch_token(&self) -> ClientResult<OAuthCredentials> {
        let Some(refresh_token) = &self.token.refresh_token else {
            warn!("No refresh token available");
            return Err(ClientError::InvalidToken);
        };
        let response = self
            .http
            .post(TOKEN_URL)
            .send_form([
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &self.client_id),
            ])?
            .into_body()
            .read_to_string()?;
        let mut token = parse_credentials(&response)?;
        if token.refresh_token.is_none() {
            token.refresh_token.clone_from(&self.token.refresh_token);
        }
        if token.scope.is_empty() {
            token.scope.clone_from(&self.token.scope);
        }
        info!("Refetched Spotify token");
        Ok(token)
    }
    fn api_json<T: DeserializeOwned>(&mut self, url: &str, label: &str) -> Option<T> {
        self.api_json_payload(url, &[], label)
    }

    fn api_json_payload<T: DeserializeOwned>(
        &mut self,
        url: &str,
        payload: &[(&str, &str)],
        label: &str,
    ) -> Option<T> {
        self.retry_unauthorized(|client| {
            let authorization = client.authorization()?;
            client
                .http
                .get(format!("{API_BASE}/{url}"))
                .header("authorization", authorization)
                .query_pairs(payload.iter().copied())
                .call()
                .map_err(Into::into)
                .and_then(response_json)
        })
        .inspect_err(|e| error!("Failed to fetch {label}: {e}"))
        .ok()
    }

    fn api_request(&mut self, method: &Method, path: &str, json: Option<&str>) -> ClientResult<()> {
        self.retry_unauthorized(|client| {
            let request = Request::builder()
                .method(method.clone())
                .uri(format!("{API_BASE}/{path}"))
                .header("authorization", client.authorization()?);
            let request = if let Some(json) = json {
                request
                    .header("content-type", "application/json; charset=utf-8")
                    .body(json)?
            } else {
                request.body("")?
            };
            client.http.run(request)?;
            Ok(())
        })
    }

    fn new(client_id: String, scopes: &str, cache_path: PathBuf) -> ClientResult<Self> {
        let state = generate_random_string(
            16,
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        );
        let (verifier, url) = get_authorize_url(&client_id, scopes, &state);
        let agent = Agent::new_with_defaults();
        let token = match read_token_cache(&cache_path, scopes)? {
            Some(token) => token,
            None => prompt_for_token(&url, &client_id, &verifier, &agent, &state)?,
        };
        let spotify_client = Self {
            client_id,
            cache_path,
            token,
            http: agent,
        };
        spotify_client.write_token_cache()?;
        Ok(spotify_client)
    }
}

fn get_authorize_url(client_id: &str, scopes: &str, state: &str) -> (String, String) {
    let verifier = generate_random_string(
        VERIFIER_BYTES,
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~",
    );

    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));

    let query = form_urlencoded::Serializer::new(String::new())
        .extend_pairs([
            ("client_id", client_id),
            ("response_type", "code"),
            ("redirect_uri", REDIRECT_URI),
            ("code_challenge_method", "S256"),
            ("code_challenge", &challenge),
            ("state", state),
            ("scope", scopes),
        ])
        .finish();
    (
        verifier,
        format!("https://accounts.spotify.com/authorize?{query}"),
    )
}

fn generate_random_string(length: usize, alphabet: &[u8]) -> String {
    (0..length)
        .map(|_| alphabet[fastrand::usize(..alphabet.len())] as char)
        .collect()
}

#[derive(Debug, Error)]
enum ClientError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("http error: {0}")]
    Http(#[from] ureq::Error),
    #[error("invalid HTTP request: {0}")]
    BuildRequest(#[from] HttpRequestError),
    #[error("input/output error: {0}")]
    Io(#[from] io::Error),
    #[error("image decode error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Token is not valid")]
    InvalidToken,
    #[error("Spotify authorization response was invalid")]
    InvalidAuthorizationResponse,
    #[error("Spotify authorization state did not match the original request")]
    InvalidAuthorizationState,
}

type ClientResult<T> = Result<T, ClientError>;

fn config_path(file: &str) -> PathBuf {
    config::directory().join(file)
}

#[derive(Deserialize)]
struct TracksRef {
    total: u32,
}

#[derive(Deserialize)]
struct Page<T> {
    items: Vec<Option<T>>,
}

const RATING_PLAYLISTS: [&str; 10] = [
    "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

type PlaylistCache = HashMap<PlaylistId, (ArrayString<32>, PlaylistTracks)>;

fn load_cached_playlist_tracks() -> PlaylistCache {
    fs::read(config_path(PLAYLIST_TRACKS_CACHE))
        .ok()
        .and_then(|bytes| {
            serde_json::from_slice(&bytes)
                .inspect_err(|e| warn!("Failed to parse playlist cache: {e}"))
                .ok()
        })
        .unwrap_or_default()
}

pub struct SpotifyBackend {
    commands: Sender<Update<SpotifyClient>>,
    updater: AppUpdater,
}

impl SpotifyBackend {
    pub fn new(config: &Config, updater: AppUpdater) -> Self {
        let client_id = config.spotify_client_id.clone().expect(
            "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
        );
        let client = SpotifyClient::new(client_id, SCOPES, config_path(SPOTIFY_TOKEN_CACHE))
            .expect("Failed to initialize Spotify client");
        let (commands, receiver) = mpsc::channel();
        let features = audio_features_backend(client.http.clone(), updater.clone());
        let now = Instant::now();
        let worker = SpotifyWorker {
            client,
            updater: updater.clone(),
            playback: SpotifyState {
                current_context: None,
                context_updated: false,
                last_grabbed_queue: now.checked_sub(Duration::from_mins(1)).unwrap_or(now),
            },
            playlists: PlaylistPollState::new(config),
            features,
        };
        spawn(move || worker.run(&receiver));
        Self { commands, updater }
    }

    pub fn set_volume(&self, volume: u8) {
        self.request(
            Method::PUT,
            format!("me/player/volume?volume_percent={volume}"),
        );
    }

    pub fn seek(&self, position_ms: u32) {
        self.request(
            Method::PUT,
            format!("me/player/seek?position_ms={position_ms}"),
        );
    }

    pub fn skip(&self, forward: bool, count: usize) {
        send_update(&self.commands, move |client| {
            let path = if forward {
                "me/player/next"
            } else {
                "me/player/previous"
            };
            for _ in 0..count {
                run_request(client, &Method::POST, path);
            }
        });
    }

    pub fn set_playing(&self, playing: bool) {
        let action = if playing { "play" } else { "pause" };
        self.request(Method::PUT, format!("me/player/{action}"));
    }

    pub fn set_playlist_membership(&self, playlist_id: PlaylistId, track_id: TrackId, add: bool) {
        send_update(&self.commands, move |client| {
            set_playlist_membership(client, playlist_id, track_id, add);
        });
    }

    pub fn set_rating(
        &self,
        track_id: TrackId,
        changes: Vec<(PlaylistId, bool)>,
        should_like: bool,
    ) {
        send_update(&self.commands, move |client| {
            for (playlist_id, add) in changes {
                set_playlist_membership(client, playlist_id, track_id, add);
            }
            set_liked_state(client, track_id, should_like);
        });
    }

    fn request(&self, method: Method, path: impl Into<String>) {
        let path = path.into();
        send_update(&self.commands, move |client| {
            run_request(client, &method, &path);
        });
    }
}

struct SpotifyWorker {
    client: SpotifyClient,
    updater: AppUpdater,
    playback: SpotifyState,
    playlists: PlaylistPollState,
    features: Sender<Vec<TrackId>>,
}

impl SpotifyWorker {
    fn run(mut self, commands: &Receiver<Update<SpotifyClient>>) {
        let mut next_playback = Instant::now();
        let mut next_playlists = Instant::now() + Duration::from_secs(1);
        loop {
            let now = Instant::now();
            if now >= next_playback {
                get_spotify_playback(&mut self.client, &self.updater, &mut self.playback);
                get_spotify_queue(
                    &mut self.client,
                    &self.updater,
                    &mut self.playback,
                    &self.features,
                );
                next_playback = Instant::now() + Duration::from_secs(1);
            }
            if now >= next_playlists {
                self.playlists.poll(&mut self.client, &self.updater);
                next_playlists = Instant::now() + Duration::from_secs(20);
            }
            let timeout = next_playback
                .min(next_playlists)
                .saturating_duration_since(Instant::now());
            match commands.recv_timeout(timeout) {
                Ok(command) => command(&mut self.client),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    }
}

fn run_request(client: &mut SpotifyClient, method: &Method, path: &str) {
    if let Err(err) = client.api_request(method, path, None) {
        error!("Spotify request {path} failed: {err}");
    }
}

fn set_playlist_membership(
    client: &mut SpotifyClient,
    playlist_id: PlaylistId,
    track_id: TrackId,
    add: bool,
) {
    let track_uri = format!("spotify:track:{track_id}");
    let (method, body) = if add {
        (Method::POST, json!({ "uris": [track_uri] }))
    } else {
        (Method::DELETE, json!({ "items": [{ "uri": track_uri }] }))
    };
    let path = format!("playlists/{playlist_id}/items");
    if let Err(err) = client.api_request(&method, &path, Some(&body.to_string())) {
        error!("Failed to update playlist {playlist_id} for track {track_id}: {err}");
    }
}

fn set_liked_state(client: &mut SpotifyClient, track_id: TrackId, should_like: bool) {
    let track_uri = format!("spotify:track:{track_id}");
    if let Some([liked]) = client.api_json::<[bool; 1]>(
        &format!("me/library/contains/?uris={track_uri}"),
        "liked state",
    ) && liked != should_like
    {
        let method = if should_like {
            Method::PUT
        } else {
            Method::DELETE
        };
        if let Err(err) =
            client.api_request(&method, &format!("me/library/?uris={track_uri}"), None)
        {
            error!("Failed to update liked state for track {track_id}: {err}");
        }
    }
}

fn track_index(queue: &[Track], id: Option<TrackId>, name: &str) -> Option<usize> {
    id.and_then(|track_id| queue.iter().position(|t| t.id == Some(track_id)))
        .or_else(|| queue.iter().position(|t| t.name == name))
}

fn get_spotify_playback(
    client: &mut SpotifyClient,
    updater: &AppUpdater,
    spotify_state: &mut SpotifyState,
) {
    // https://developer.spotify.com/documentation/web-api/reference/get-information-about-the-users-current-playback
    let Some(current_playback) = client.api_json::<CurrentPlaybackContext>("me/player", "playback")
    else {
        return;
    };

    let now = Instant::now();
    let new_context = current_playback.context.as_ref().map(|c| &c.uri);
    let queue_deadline = now.checked_sub(Duration::from_mins(1)).unwrap_or(now);
    if spotify_state.current_context.as_ref() != new_context {
        spotify_state.context_updated = true;
        spotify_state.current_context = new_context.map(String::from);
        spotify_state.last_grabbed_queue = queue_deadline;
    }
    send_update(updater, move |app| {
        let state = &mut app.playback;
        if let Some(track) = current_playback.item {
            state.queue_index = track_index(&state.queue, track.id, &track.name).unwrap_or(0);
        }

        state.volume = current_playback.device.volume_percent;
        if now >= state.last_interaction {
            if current_playback.is_playing && !state.playing {
                app.render.last_toggle_playing = now;
            }
            state.playing = current_playback.is_playing;
            state.update_progress(current_playback.progress_ms, now);
        } else {
            state.last_progress_update = now;
        }
    });
}

fn get_spotify_queue(
    client: &mut SpotifyClient,
    updater: &AppUpdater,
    spotify_state: &mut SpotifyState,
    features: &Sender<Vec<TrackId>>,
) {
    let now = Instant::now();
    if now < spotify_state.last_grabbed_queue + Duration::from_secs(15) {
        return;
    }

    // https://developer.spotify.com/documentation/web-api/reference/get-queue
    let Some(q) = client.api_json::<CurrentUserQueue>("me/player/queue", "queue") else {
        return;
    };
    let Some(currently_playing) = q.currently_playing else {
        // Nothing is currently playing
        return;
    };
    let current_track_id = currently_playing.id;
    let mut new_queue = q.queue;
    new_queue.insert(0, currently_playing);
    let feature_ids = new_queue
        .iter()
        .filter_map(|track| track.id)
        .collect::<Vec<_>>();

    let context_updated = spotify_state.context_updated;
    spotify_state.context_updated = false;
    spotify_state.last_grabbed_queue = Instant::now();
    send_update(updater, move |app| {
        let state = &mut app.playback;
        let current_title = &new_queue[0].name;
        let art_by_url: HashMap<_, _> = state
            .queue
            .iter()
            .filter_map(|track| match (&track.album.image, &track.art) {
                (Some(url), ArtState::Ready(art)) => Some((url.clone(), Arc::clone(art))),
                _ => None,
            })
            .collect();
        let mut old_queue = mem::take(&mut state.queue);
        let history_len = if context_updated {
            0
        } else {
            track_index(&old_queue, current_track_id, current_title).unwrap_or(0)
        };
        let history_start = history_len.saturating_sub(MAX_HISTORY_TRACKS);
        let mut remaining = old_queue.split_off(history_len);
        let mut reconciled = old_queue.split_off(history_start);
        let retained_history_len = reconciled.len();
        for mut track in new_queue {
            if let Some(index) = remaining.iter().position(|old| old.id == track.id) {
                let old = remaining.swap_remove(index);
                track.art = old.art;
                track.runtime = old.runtime;
                track.audio_features = old.audio_features;
            } else if let Some(art) = track
                .album
                .image
                .as_ref()
                .and_then(|url| art_by_url.get(url))
            {
                track.art = ArtState::Ready(Arc::clone(art));
            }
            reconciled.push(track);
        }
        state.queue = reconciled;
        state.queue_index = retained_history_len;
    });

    if features.send(feature_ids).is_err() {
        warn!("Discarded audio-feature request after its worker stopped");
    }
}

fn audio_features_backend(http: Agent, updater: AppUpdater) -> Sender<Vec<TrackId>> {
    let (sender, receiver) = mpsc::channel::<Vec<TrackId>>();
    spawn(move || {
        let mut cache = HashMap::new();
        for ids in receiver {
            resolve_audio_features(&http, &ids, &mut cache);
            let features = ids
                .into_iter()
                .filter_map(|id| cache.get(&id).copied().flatten().map(|value| (id, value)))
                .collect::<HashMap<_, _>>();
            send_update(&updater, move |app| {
                for track in &mut app.playback.queue {
                    track.audio_features = track.id.and_then(|id| features.get(&id).copied());
                }
            });
        }
    });
    sender
}

fn resolve_audio_features(
    http: &Agent,
    track_ids: &[TrackId],
    cache: &mut HashMap<TrackId, Option<PackedAudioFeatures>>,
) {
    let mut missing = track_ids
        .iter()
        .copied()
        .filter(|id| !cache.contains_key(id))
        .collect::<HashSet<_>>();

    if missing.is_empty() {
        return;
    }
    let result: ClientResult<ReccoPage<ReccoTrack>> = http
        .get(RECCO_TRACK_URL)
        .query("size", "50")
        .query_pairs(missing.iter().map(|id| ("ids", id.as_str())))
        .call()
        .map_err(ClientError::from)
        .and_then(response_json);
    let Ok(page) = result.inspect_err(|err| {
        warn!("Failed to resolve Spotify tracks with ReccoBeats: {err}");
    }) else {
        return;
    };
    for recco_track in page.content {
        let Some(spotify_id) = recco_track
            .href
            .rsplit('/')
            .next()
            .and_then(|id| id.parse::<TrackId>().ok())
            .filter(|id| missing.remove(id))
        else {
            continue;
        };
        let url = format!("{RECCO_TRACK_URL}/{}/audio-features", recco_track.id);
        if let Ok(features) = http
            .get(url)
            .call()
            .map_err(ClientError::from)
            .and_then(response_json)
            .inspect_err(|err| {
                warn!("Failed to fetch ReccoBeats features for {spotify_id}: {err}");
            })
        {
            cache.insert(spotify_id, Some(features));
        }
    }
    cache.extend(missing.drain().map(|id| (id, None)));
}

pub fn download_image(backend: &SpotifyBackend, url: String) {
    let updater = backend.updater.clone();
    send_update(&backend.commands, move |client| {
        let http = client.http.clone();
        spawn(move || {
            let result = http
                .get(&url)
                .call()
                .map_err(ClientError::from)
                .and_then(|mut response| Ok(response.body_mut().read_to_vec()?))
                .and_then(|bytes| image::load_from_memory(&bytes).map_err(Into::into))
                .map(|image| Arc::new(art::prepare(&image)));
            let state = match result {
                Ok(art) => ArtState::Ready(art),
                Err(err) => {
                    warn!("Failed to load image {url}: {err}");
                    ArtState::RetryAt(Instant::now() + Duration::from_secs(30))
                }
            };
            send_update(&updater, move |app| app.set_art_state(&url, &state));
        });
    });
}

struct PlaylistPollState {
    targets: ArrayVec<String, MAX_PILL_PLAYLIST_ICONS>,
    persistent_cache: PlaylistCache,
    loaded_playlists: HashSet<PlaylistId>,
    ratings_enabled: bool,
}

impl PlaylistPollState {
    fn new(config: &Config) -> Self {
        Self {
            targets: config.playlists.clone(),
            persistent_cache: load_cached_playlist_tracks(),
            loaded_playlists: HashSet::new(),
            ratings_enabled: config.ratings_enabled,
        }
    }

    fn poll(&mut self, client: &mut SpotifyClient, updater: &AppUpdater) {
        // https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists
        let Some(playlists) = client.api_json_payload::<Page<Playlist>>(
            "me/playlists",
            &[("limit", "50")],
            "playlists",
        ) else {
            return;
        };

        let mut cache_changed = false;
        let mut changed = Vec::new();
        for playlist in playlists.items.into_iter().flatten() {
            let rating_index = rating_index(self.ratings_enabled, &playlist.name);
            if !self.targets.contains(&playlist.name) && rating_index.is_none() {
                continue;
            }
            let cached_tracks = self
                .persistent_cache
                .get(&playlist.id)
                .filter(|(snapshot, _)| *snapshot == playlist.snapshot_id)
                .map(|(_, tracks)| Arc::clone(tracks));
            let tracks = if let Some(tracks) = cached_tracks {
                if !self.loaded_playlists.insert(playlist.id) {
                    continue;
                }
                tracks
            } else if let Some(tracks) = fetch_playlist_tracks(client, &playlist) {
                self.loaded_playlists.insert(playlist.id);
                self.persistent_cache
                    .insert(playlist.id, (playlist.snapshot_id, Arc::clone(&tracks)));
                cache_changed = true;
                tracks
            } else {
                continue;
            };
            changed.push(CondensedPlaylist {
                id: playlist.id,
                name: playlist.name,
                image_url: playlist.image,
                tracks,
                rating_index,
                art: ArtState::Missing,
            });
        }
        if cache_changed
            && let Ok(serialized) = serde_json::to_vec(&self.persistent_cache)
            && let Err(err) = fs::write(config_path(PLAYLIST_TRACKS_CACHE), serialized)
        {
            warn!("Failed to persist playlist cache: {err}");
        }
        apply_playlist_updates(updater, changed);
    }
}

fn rating_index(enabled: bool, name: &str) -> Option<u8> {
    enabled.then_some(())?;
    RATING_PLAYLISTS
        .iter()
        .position(|&playlist| playlist == name)
        .map(|index| index as u8)
}

fn fetch_playlist_tracks(
    client: &mut SpotifyClient,
    playlist: &Playlist,
) -> Option<PlaylistTracks> {
    let chunk_size = 50;
    let num_pages = playlist.tracks.total.div_ceil(chunk_size);
    let mut tracks = HashSet::with_capacity(playlist.tracks.total as usize);
    info!("Fetching {num_pages} pages from playlist {}", playlist.name);

    for page in 0..num_pages {
        let limit = chunk_size.to_string();
        let offset = (page * chunk_size).to_string();
        let page = client.api_json_payload::<Page<PlaylistItem>>(
            &format!("playlists/{}/items", playlist.id),
            &[
                (
                    "fields",
                    "href,limit,offset,total,items(is_local,track(id))",
                ),
                ("limit", &limit),
                ("offset", &offset),
            ],
            "playlist page",
        )?;
        tracks.extend(
            page.items
                .into_iter()
                .flatten()
                .filter_map(|item| item.track?.id),
        );
    }
    Some(Arc::new(tracks))
}

fn apply_playlist_updates(updater: &AppUpdater, changed: Vec<CondensedPlaylist>) {
    if changed.is_empty() {
        return;
    }
    send_update(updater, move |app| {
        let playlists = &mut app.playback.playlists;
        for mut update in changed {
            if let Some(previous) = playlists
                .iter_mut()
                .find(|playlist| playlist.id == update.id)
            {
                if previous.image_url == update.image_url {
                    update.art = mem::take(&mut previous.art);
                }
                *previous = update;
            } else {
                playlists.push(update);
            }
        }
        playlists.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    });
}

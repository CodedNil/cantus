use crate::{
    MAX_HISTORY_TRACKS,
    art::{self, ArtState},
    config::{self, Config},
    model::{
        AppUpdater, CondensedPlaylist, PlaylistId, PlaylistTracks, Track, TrackId,
        deserialize_images,
    },
};
use arrayvec::{ArrayString, ArrayVec};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use cantus_shared::MAX_PILL_PLAYLIST_ICONS;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufRead, BufReader, Write},
    mem,
    net::TcpListener,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    thread::{JoinHandle, spawn},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use thiserror::Error;
use tracing::{error, info, warn};
use ureq::{
    Agent,
    http::{Error as HttpRequestError, Method, Request, request::Builder as RequestBuilder},
};

const API_BASE: &str = "https://api.spotify.com/v1";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const PLAYLIST_TRACKS_CACHE: &str = "cantus_playlist_tracks.json";
const SPOTIFY_TOKEN_CACHE: &str = "spotify_cache.json";
const SCOPES: &str = "\
user-read-playback-state user-modify-playback-state user-read-currently-playing \
playlist-read-private playlist-read-collaborative playlist-modify-private playlist-modify-public \
user-library-read user-library-modify";

struct SpotifyState {
    current_context: Option<String>,
    context_updated: bool,
    last_grabbed_queue: Instant,
}

const VERIFIER_BYTES: usize = 43;
const REDIRECT_HOST: &str = "127.0.0.1";
const REDIRECT_PORT: u16 = 7474;
const REDIRECT_URI: &str = "http://127.0.0.1:7474/callback";

pub struct SpotifyClient {
    client_id: String,
    cache_path: PathBuf,
    token: RwLock<Token>,
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
struct Token {
    #[serde(rename = "access_token")]
    access: String,
    expires_in: u32,
    #[serde(default, deserialize_with = "deserialize_expiration")]
    expires_at: Option<u64>,
    #[serde(rename = "refresh_token")]
    refresh: Option<String>,
    #[serde(default, rename = "scope")]
    scopes: String,
}

impl Token {
    fn is_expired(&self) -> bool {
        self.expires_at
            .is_none_or(|expiration| unix_time().saturating_add(10) >= expiration)
    }

    fn set_expiration(&mut self) {
        self.expires_at = unix_time().checked_add(u64::from(self.expires_in));
    }
}

fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn deserialize_expiration<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u64>, D::Error> {
    Ok(serde_json::Value::deserialize(deserializer)?.as_u64())
}

fn read_token_cache(cache_path: &Path, scopes: &str) -> ClientResult<Option<Token>> {
    let token = match fs::read_to_string(cache_path) {
        Ok(cache) => serde_json::from_str::<Token>(&cache)
            .inspect_err(|err| warn!("Failed to parse Spotify token cache: {err}"))
            .ok(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(err) => return Err(err.into()),
    };
    Ok(token.filter(|token| {
        scopes.split_whitespace().all(|required| {
            token
                .scopes
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
) -> ClientResult<Token> {
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
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
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
    let mut token = serde_json::from_str::<Token>(&response)?;
    token.set_expiration();
    Ok(token)
}

impl SpotifyClient {
    fn auth_headers(&self) -> ClientResult<String> {
        {
            let token = self.token.read();
            if !token.is_expired() {
                let header = format!("Bearer {}", token.access);
                drop(token);
                return Ok(header);
            }
        }

        let mut token = self.token.write();
        if token.is_expired() {
            *token = self.refetch_token(&token)?;
            self.write_token_cache(&token)?;
        }
        let header = format!("Bearer {}", token.access);
        drop(token);
        Ok(header)
    }

    fn api_url(path: &str) -> String {
        format!("{API_BASE}/{path}")
    }

    fn request(&self, method: Method, path: &str) -> ClientResult<RequestBuilder> {
        Ok(Request::builder()
            .method(method)
            .uri(Self::api_url(path))
            .header("authorization", self.auth_headers()?))
    }

    pub fn api_json<T: DeserializeOwned>(&self, url: &str, label: &str) -> Option<T> {
        self.api_json_payload(url, &[], label)
    }

    fn api_json_payload<T: DeserializeOwned>(
        &self,
        url: &str,
        payload: &[(&str, &str)],
        label: &str,
    ) -> Option<T> {
        self.auth_headers()
            .and_then(|authorization| {
                self.http
                    .get(Self::api_url(url))
                    .header("authorization", authorization)
                    .query_pairs(payload.iter().copied())
                    .call()
                    .map_err(Into::into)
                    .and_then(|mut response| {
                        serde_json::from_reader(response.body_mut().as_reader()).map_err(Into::into)
                    })
            })
            .inspect_err(|e| error!("Failed to fetch {label}: {e}"))
            .ok()
    }

    pub fn api_request(&self, method: Method, path: &str, json: Option<&str>) -> ClientResult<()> {
        let request = self.request(method, path)?;
        let request = if let Some(json) = json {
            request
                .header("content-type", "application/json; charset=utf-8")
                .body(json)?
        } else {
            request.body("")?
        };
        self.http.run(request)?;
        Ok(())
    }

    fn write_token_cache(&self, token: &Token) -> ClientResult<()> {
        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.cache_path, serde_json::to_string(token)?)?;
        Ok(())
    }

    fn refetch_token(&self, current: &Token) -> ClientResult<Token> {
        let Some(refresh_token) = &current.refresh else {
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
        let mut token = serde_json::from_str::<Token>(&response)?;
        if token.refresh.is_none() {
            token.refresh.clone_from(&current.refresh);
        }
        if token.scopes.is_empty() {
            token.scopes.clone_from(&current.scopes);
        }
        info!("Refetched token: {}", token.expires_in);
        token.set_expiration();
        Ok(token)
    }

    pub fn new(client_id: String, scopes: &str, cache_path: PathBuf) -> ClientResult<Self> {
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
            token: RwLock::new(token),
            http: agent,
        };
        spotify_client.write_token_cache(&spotify_client.token.read())?;
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
pub enum ClientError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("http error: {0}")]
    Http(String),
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

impl From<ureq::Error> for ClientError {
    fn from(err: ureq::Error) -> Self {
        match &err {
            ureq::Error::StatusCode(code) => {
                let hint = match *code {
                    401 => " - check that your Spotify token is still valid",
                    403 => " - you may not have permission for this resource",
                    404 => {
                        " - make sure a Spotify device is active (start playback on a client first)"
                    }
                    429 => " - rate limited, try again later",
                    _ => "",
                };
                Self::Http(format!("Spotify API returned HTTP {code}{hint}"))
            }
            _ => Self::Http(err.to_string()),
        }
    }
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

fn create_client(config: &Config) -> SpotifyClient {
    SpotifyClient::new(
        config.spotify_client_id.clone().expect(
            "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
        ),
        SCOPES,
        config_path(SPOTIFY_TOKEN_CACHE),
    )
    .expect("Failed to initialize Spotify client")
}

type PlaylistCache = HashMap<PlaylistId, (ArrayString<32>, PlaylistTracks)>;

fn load_cached_playlist_tracks() -> PlaylistCache {
    fs::read(config_path(PLAYLIST_TRACKS_CACHE))
        .ok()
        .and_then(|b| {
            serde_json::from_slice(&b)
                .inspect_err(|e| warn!("Failed to parse playlist cache: {e}"))
                .ok()
        })
        .unwrap_or_default()
}

fn persist_playlist_cache(cache: &PlaylistCache) {
    if !cache.is_empty()
        && let Ok(ser) = serde_json::to_vec(cache)
    {
        let _ = fs::write(config_path(PLAYLIST_TRACKS_CACHE), ser);
    }
}

pub struct SpotifyBackend {
    pub client: Arc<SpotifyClient>,
    pub updater: AppUpdater,
    playback: PollTask<SpotifyState>,
    playlists: PollTask<PlaylistPollState>,
}

impl SpotifyBackend {
    pub fn new(config: &Config, updater: AppUpdater) -> Self {
        let cantus_dir = config_path("");
        fs::create_dir_all(cantus_dir).expect("Failed to create Cantus config directory");
        let client = Arc::new(create_client(config));
        let now = Instant::now();
        let initial_poll = now.checked_sub(Duration::from_mins(1)).unwrap_or(now);
        Self {
            updater: updater.clone(),
            playback: PollTask::new(
                SpotifyState {
                    current_context: None,
                    context_updated: false,
                    last_grabbed_queue: initial_poll,
                },
                Duration::from_secs(1),
            ),
            playlists: PollTask::new(
                PlaylistPollState::new(Arc::clone(&client), config, updater),
                Duration::from_secs(20),
            ),
            client,
        }
    }

    pub fn tick(&mut self) {
        let client = Arc::clone(&self.client);
        let updater = self.updater.clone();
        self.playback.run(move |state| {
            get_spotify_playback(&client, &updater, state);
            get_spotify_queue(&client, &updater, state);
        });
        self.playlists.run(PlaylistPollState::poll);
    }
}

struct PollTask<T> {
    state: Option<T>,
    task: Option<JoinHandle<T>>,
    next_poll: Instant,
    interval: Duration,
}

impl<T: Send + 'static> PollTask<T> {
    fn new(state: T, interval: Duration) -> Self {
        Self {
            state: Some(state),
            task: None,
            next_poll: Instant::now(),
            interval,
        }
    }

    fn run(&mut self, poll: impl FnOnce(&mut T) + Send + 'static) {
        if self.task.as_ref().is_some_and(JoinHandle::is_finished) {
            self.state = self.task.take().and_then(|task| task.join().ok());
        }
        let now = Instant::now();
        if now >= self.next_poll
            && let Some(mut state) = self.state.take()
        {
            self.next_poll = now + self.interval;
            self.task = Some(spawn(move || {
                poll(&mut state);
                state
            }));
        }
    }
}

fn track_index(queue: &[Track], id: Option<TrackId>, name: &str) -> Option<usize> {
    id.and_then(|track_id| queue.iter().position(|t| t.id == Some(track_id)))
        .or_else(|| queue.iter().position(|t| t.name == name))
}

fn get_spotify_playback(
    client: &SpotifyClient,
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
    updater.send(move |app| {
        let state = &mut app.playback_state;
        if let Some(track) = current_playback.item {
            state.queue_index = track_index(&state.queue, track.id, &track.name).unwrap_or(0);
        }

        state.volume = current_playback.device.volume_percent;
        if now >= state.last_interaction {
            if current_playback.is_playing && !state.playing {
                app.last_toggle_playing = now;
            }
            state.playing = current_playback.is_playing;
            state.progress = current_playback.progress_ms;
        }
        state.last_progress_update = now;
    });
}

fn get_spotify_queue(
    client: &Arc<SpotifyClient>,
    updater: &AppUpdater,
    spotify_state: &mut SpotifyState,
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

    let context_updated = spotify_state.context_updated;
    spotify_state.context_updated = false;
    spotify_state.last_grabbed_queue = Instant::now();
    updater.send(move |app| {
        let state = &mut app.playback_state;
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
}

pub fn download_image(backend: &SpotifyBackend, url: String) {
    let client = Arc::clone(&backend.client);
    let updater = backend.updater.clone();
    spawn(move || {
        let result = client
            .http
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
        updater.send(move |app| app.set_art_state(&url, &state));
    });
}

struct PlaylistPollState {
    client: Arc<SpotifyClient>,
    updater: AppUpdater,
    targets: ArrayVec<String, MAX_PILL_PLAYLIST_ICONS>,
    persistent_cache: PlaylistCache,
    loaded_playlists: HashSet<PlaylistId>,
    ratings_enabled: bool,
}

impl PlaylistPollState {
    fn new(client: Arc<SpotifyClient>, config: &Config, updater: AppUpdater) -> Self {
        let playlist_cache = load_cached_playlist_tracks();
        Self {
            client,
            updater,
            targets: config.playlists.clone(),
            persistent_cache: playlist_cache,
            loaded_playlists: HashSet::new(),
            ratings_enabled: config.ratings_enabled,
        }
    }

    fn poll(&mut self) {
        // https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists
        let Some(playlists) = self.client.api_json_payload::<Page<Playlist>>(
            "me/playlists",
            &[("limit", "50")],
            "playlists",
        ) else {
            return;
        };

        let mut cache_changed = false;
        let mut updates = Vec::new();
        for playlist in playlists.items.into_iter().flatten() {
            let rating_index = rating_index(self.ratings_enabled, &playlist.name);
            if !self.targets.contains(&playlist.name) && rating_index.is_none() {
                continue;
            }
            if let Some(tracks) = self
                .persistent_cache
                .get(&playlist.id)
                .filter(|(snapshot, _)| *snapshot == playlist.snapshot_id)
                .map(|(_, tracks)| Arc::clone(tracks))
            {
                if self.loaded_playlists.insert(playlist.id) {
                    queue_playlist(&mut updates, playlist, tracks, rating_index);
                }
                continue;
            }

            if let Some(tracks) = fetch_playlist_tracks(&self.client, &playlist) {
                self.loaded_playlists.insert(playlist.id);
                self.persistent_cache
                    .insert(playlist.id, (playlist.snapshot_id, tracks.clone()));
                queue_playlist(&mut updates, playlist, tracks, rating_index);
                cache_changed = true;
            }
        }
        if cache_changed {
            persist_playlist_cache(&self.persistent_cache);
        }
        apply_playlist_updates(&self.updater, updates);
    }
}

fn rating_index(enabled: bool, name: &str) -> Option<u8> {
    enabled.then_some(())?;
    RATING_PLAYLISTS
        .iter()
        .position(|&playlist| playlist == name)
        .map(|index| index as u8)
}

fn fetch_playlist_tracks(client: &SpotifyClient, playlist: &Playlist) -> Option<PlaylistTracks> {
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

fn queue_playlist(
    updates: &mut Vec<CondensedPlaylist>,
    playlist: Playlist,
    tracks: PlaylistTracks,
    rating_index: Option<u8>,
) {
    updates.push(CondensedPlaylist {
        id: playlist.id,
        name: playlist.name,
        image_url: playlist.image,
        tracks,
        rating_index,
        art: ArtState::Missing,
    });
}

fn apply_playlist_updates(updater: &AppUpdater, changed: Vec<CondensedPlaylist>) {
    if changed.is_empty() {
        return;
    }
    updater.send(move |app| {
        let playlists = &mut app.playback_state.playlists;
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

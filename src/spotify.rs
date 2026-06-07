use crate::{
    ARTIST_DATA_CACHE, Artist, ArtistId, CondensedPlaylist, IMAGES_CACHE, PLAYBACK_STATE,
    PlaylistId, Track, TrackId, config::CONFIG, deserialize_images, render::update_color_palettes,
    update_playback_state,
};
use arrayvec::ArrayString;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};
use tap::{Pipe, Tap};
use thiserror::Error;
use time::{Duration as TimeDuration, OffsetDateTime};
use tracing::{error, info, warn};
use ureq::Agent;
use url::Url;

const API_BASE: &str = "https://api.spotify.com/v1";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const PLAYLIST_TRACKS_CACHE: &str = "cantus_playlist_tracks.json";
const SPOTIFY_TOKEN_CACHE: &str = "spotify_cache.json";

struct SpotifyState {
    current_context: Option<String>,
    context_updated: bool,
    last_grabbed_playback: Instant,
    last_grabbed_queue: Instant,
}

static SPOTIFY_STATE: LazyLock<RwLock<SpotifyState>> = LazyLock::new(|| {
    let one_min_ago = Instant::now().checked_sub(Duration::from_secs(60)).unwrap();
    RwLock::new(SpotifyState {
        current_context: None,
        context_updated: false,
        last_grabbed_playback: one_min_ago,
        last_grabbed_queue: one_min_ago,
    })
});

// --- RSPOTIFY LOGIC ---
const VERIFIER_BYTES: usize = 43;
const REDIRECT_HOST: &str = "127.0.0.1";
const REDIRECT_PORT: u16 = 7474;

#[derive(Debug)]
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
    #[serde(deserialize_with = "deserialize_tracks_total", rename = "tracks")]
    total_tracks: u32,
}

#[derive(Deserialize)]
struct PlaylistItem {
    track: PartialTrack,
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
    volume_percent: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Token {
    #[serde(rename = "access_token")]
    access: String,
    expires_in: u32,
    expires_at: Option<OffsetDateTime>,
    #[serde(rename = "refresh_token")]
    refresh: Option<String>,
    #[serde(
        default,
        serialize_with = "serialize_scopes",
        deserialize_with = "deserialize_scopes",
        rename = "scope"
    )]
    scopes: HashSet<String>,
}

impl Token {
    fn is_expired(&self) -> bool {
        self.expires_at.is_none_or(|expiration| {
            OffsetDateTime::now_utc() + TimeDuration::seconds(10) >= expiration
        })
    }

    fn set_expiration(&mut self) {
        self.expires_at = OffsetDateTime::now_utc()
            .checked_add(TimeDuration::seconds(i64::from(self.expires_in)));
    }
}

fn read_token_cache(
    allow_expired: bool,
    cache_path: &Path,
    scopes: &HashSet<String>,
) -> ClientResult<Option<Token>> {
    let token = match fs::read_to_string(cache_path) {
        Ok(cache) => serde_json::from_str::<Token>(&cache)
            .inspect_err(|err| warn!("Failed to parse Spotify token cache: {err}"))
            .ok(),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(err) => return Err(err.into()),
    };
    Ok(token.filter(|t| scopes.is_subset(&t.scopes) && (allow_expired || !t.is_expired())))
}

fn prompt_for_token(
    url: &str,
    client_id: &str,
    verifier: &str,
    http: &Agent,
    expected_state: &str,
) -> ClientResult<Token> {
    match webbrowser::open(url) {
        Ok(()) => println!("Opened {url} in your browser."),
        Err(err) => eprintln!(
            "Error when trying to open an URL in your browser: {err:?}. Please navigate here manually: {url}"
        ),
    }

    let listener = TcpListener::bind((REDIRECT_HOST, REDIRECT_PORT))?;
    let (mut stream, _) = listener.accept()?;
    let mut request_line = String::new();
    BufReader::new(&stream).read_line(&mut request_line)?;

    let auth_response = Url::parse(&format!(
        "http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback{}",
        request_line
            .split_whitespace()
            .nth(1)
            .ok_or(ClientError::InvalidAuthorizationResponse)?
    ))?;
    let code = auth_response
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.into_owned())
        .ok_or(ClientError::InvalidAuthorizationResponse)?;
    let actual_state = auth_response
        .query_pairs()
        .find(|(key, _)| key == "state")
        .map(|(_, value)| value.into_owned())
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
            (
                "redirect_uri",
                &format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback"),
            ),
            ("client_id", client_id),
            ("code_verifier", verifier),
        ])?
        .into_body()
        .read_to_string()?;
    serde_json::from_str::<Token>(&response)?
        .tap_mut(Token::set_expiration)
        .pipe(Ok)
}

impl SpotifyClient {
    fn auth_headers(&self) -> ClientResult<String> {
        let cached_access = {
            let token = self.token.read();
            (!token.is_expired()).then(|| token.access.clone())
        };
        if let Some(access) = cached_access {
            return Ok(format!("Bearer {access}"));
        }

        let access = {
            let mut token = self.token.write();
            if token.is_expired() {
                *token = self.refetch_token(&token)?;
                self.write_token_cache(&token)?;
            }
            token.access.clone()
        };
        Ok(format!("Bearer {access}"))
    }

    fn api_url(path: &str) -> String {
        format!("{API_BASE}/{path}")
    }

    fn api_json<T: DeserializeOwned>(&self, url: &str, label: &str) -> Option<T> {
        self.api_get(url)
            .inspect_err(|e| error!("Failed to fetch {label}: {e}"))
            .ok()
            .filter(|res| !res.is_empty())
            .and_then(|res| parse_json(&res, label))
    }

    fn api_json_payload<T: DeserializeOwned>(
        &self,
        url: &str,
        payload: &[(&str, &str)],
        label: &str,
    ) -> Option<T> {
        self.api_get_payload(url, payload)
            .inspect_err(|e| error!("Failed to fetch {label}: {e}"))
            .ok()
            .and_then(|res| parse_json(&res, label))
    }

    pub fn api_get(&self, url: &str) -> ClientResult<String> {
        let response = self
            .http
            .get(Self::api_url(url))
            .header("authorization", self.auth_headers()?)
            .call()?;
        Ok(response.into_body().read_to_string()?)
    }

    pub fn api_get_payload(&self, url: &str, payload: &[(&str, &str)]) -> ClientResult<String> {
        let response = self
            .http
            .get(Self::api_url(url))
            .header("authorization", self.auth_headers()?)
            .query_pairs(payload.iter().copied())
            .call()?;
        Ok(response.into_body().read_to_string()?)
    }

    pub fn api_post(&self, url: &str) -> ClientResult<()> {
        self.http
            .post(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_empty()?;
        Ok(())
    }

    pub fn api_post_payload(&self, url: &str, payload: &str) -> ClientResult<()> {
        self.http
            .post(Self::api_url(url))
            .header("Content-Type", "application/json; charset=utf-8")
            .header("authorization", self.auth_headers()?)
            .send(payload)?;
        Ok(())
    }

    pub fn api_put(&self, url: &str) -> ClientResult<()> {
        self.http
            .put(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_empty()?;
        Ok(())
    }

    pub fn api_delete(&self, url: &str) -> ClientResult<()> {
        self.http
            .delete(Self::api_url(url))
            .header("authorization", self.auth_headers()?)
            .call()?;
        Ok(())
    }

    pub fn api_delete_payload(&self, url: &str, payload: &str) -> ClientResult<()> {
        self.http
            .delete(Self::api_url(url))
            .header("Content-Type", "application/json; charset=utf-8")
            .header("authorization", self.auth_headers()?)
            .force_send_body()
            .send(payload)?;
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
        token
            .tap(|t| info!("Refetched token: {}", t.expires_in))
            .tap_mut(Token::set_expiration)
            .pipe(Ok)
    }

    pub fn new(
        client_id: String,
        scopes: &HashSet<String>,
        cache_path: PathBuf,
    ) -> ClientResult<Self> {
        let state = generate_random_string(
            16,
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        );
        let (verifier, url) = get_authorize_url(&client_id, scopes, &state)?;
        let agent = Agent::new_with_defaults();
        let token = match read_token_cache(true, &cache_path, scopes)? {
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

fn get_authorize_url(
    client_id: &str,
    scopes: &HashSet<String>,
    state: &str,
) -> ClientResult<(String, String)> {
    let verifier = generate_random_string(
        VERIFIER_BYTES,
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~",
    );

    let challenge = URL_SAFE_NO_PAD.encode(
        Sha256::new()
            .tap_mut(|h| h.update(verifier.as_bytes()))
            .finalize(),
    );

    let parsed = Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("client_id", client_id),
            ("response_type", "code"),
            (
                "redirect_uri",
                &format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback"),
            ),
            ("code_challenge_method", "S256"),
            ("code_challenge", &challenge),
            ("state", state),
            (
                "scope",
                scopes
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(" ")
                    .as_str(),
            ),
        ],
    )?;
    Ok((verifier, parsed.into()))
}

fn generate_random_string(length: usize, alphabet: &[u8]) -> String {
    let range = alphabet.len();
    (0..length)
        .map(|_| alphabet[fastrand::usize(..range)] as char)
        .collect()
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),
    #[error("http error: {0}")]
    Http(String),
    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
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
        Self::Http(err.to_string())
    }
}

type ClientResult<T> = Result<T, ClientError>;

fn config_path(file: &str) -> PathBuf {
    dirs::config_dir().unwrap().join("cantus").join(file)
}

fn parse_json<T: DeserializeOwned>(input: &str, label: &str) -> Option<T> {
    serde_json::from_str(input)
        .inspect_err(|e| error!("Failed to parse {label}: {e}"))
        .ok()
}

fn deserialize_scopes<'de, D>(d: D) -> Result<HashSet<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let scopes: String = Deserialize::deserialize(d)?;
    Ok(scopes.split_whitespace().map(ToOwned::to_owned).collect())
}

fn serialize_scopes<S>(scopes: &HashSet<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut scopes = scopes.iter().map(String::as_str).collect::<Vec<_>>();
    scopes.sort_unstable();
    s.serialize_str(&scopes.join(" "))
}

#[derive(Deserialize)]
struct TracksRef {
    total: u32,
}

fn deserialize_tracks_total<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(TracksRef::deserialize(deserializer)?.total)
}

fn vec_without_nulls<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let v = Vec::<Option<T>>::deserialize(deserializer)?;
    Ok(v.into_iter().flatten().collect())
}

#[derive(Deserialize)]
struct Page<T: DeserializeOwned> {
    #[serde(deserialize_with = "vec_without_nulls")]
    items: Vec<T>,
    total: u32,
}

// --- SPOTIFY LOGIC ---
const RATING_PLAYLISTS: [&str; 10] = [
    "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

pub static SPOTIFY_CLIENT: LazyLock<SpotifyClient> = LazyLock::new(|| {
    let scopes = [
        "user-read-playback-state",
        "user-modify-playback-state",
        "user-read-currently-playing",
        "playlist-read-private",
        "playlist-read-collaborative",
        "playlist-modify-private",
        "playlist-modify-public",
        "user-library-read",
        "user-library-modify",
    ]
    .iter()
    .map(std::string::ToString::to_string)
    .collect();

    SpotifyClient::new(
        CONFIG.spotify_client_id.clone().expect(
            "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
        ),
        &scopes,
        config_path(SPOTIFY_TOKEN_CACHE),
    )
    .expect("Failed to initialize Spotify client")
});

type PlaylistCache = HashMap<PlaylistId, (ArrayString<32>, HashSet<TrackId>)>;

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

fn persist_playlist_cache() {
    let cache_payload: PlaylistCache = PLAYBACK_STATE
        .read()
        .playlists
        .values()
        .map(|p| {
            if p.tracks.len() as u32 > p.tracks_total {
                warn!(
                    "Playlist {} has more cached tracks than Spotify reported",
                    p.name
                );
            }
            (p.id, (p.snapshot_id, p.tracks.clone()))
        })
        .collect();
    if !cache_payload.is_empty()
        && let Ok(ser) = serde_json::to_vec(&cache_payload)
    {
        let _ = fs::write(config_path(PLAYLIST_TRACKS_CACHE), ser);
    }
}

pub fn init() {
    let cantus_dir = config_path("");
    if !cantus_dir.exists() {
        fs::create_dir(&cantus_dir).unwrap();
    }
    let _ = &*SPOTIFY_CLIENT;
    spawn(poll_playlists);
    spawn(|| {
        loop {
            get_spotify_playback();
            get_spotify_queue();
            sleep(Duration::from_millis(500));
        }
    });
}

fn track_index(queue: &[Track], id: Option<TrackId>, name: &str) -> Option<usize> {
    id.and_then(|track_id| queue.iter().position(|t| t.id == Some(track_id)))
        .or_else(|| queue.iter().position(|t| t.name == name))
}

fn get_spotify_playback() {
    let now = Instant::now();
    if now < PLAYBACK_STATE.read().last_interaction
        || now < SPOTIFY_STATE.read().last_grabbed_playback + Duration::from_secs(1)
    {
        return;
    }

    // https://developer.spotify.com/documentation/web-api/reference/get-information-about-the-users-current-playback
    let Some(current_playback) =
        SPOTIFY_CLIENT.api_json::<CurrentPlaybackContext>("me/player", "playback")
    else {
        return;
    };

    let now = Instant::now();
    let mut spotify_state = SPOTIFY_STATE.write();
    update_playback_state(|state| {
        let new_context = current_playback.context.as_ref().map(|c| &c.uri);
        let queue_deadline = now.checked_sub(Duration::from_secs(60)).unwrap();

        if spotify_state.current_context.as_ref() != new_context {
            spotify_state.context_updated = true;
            spotify_state.current_context = new_context.map(String::from);
            spotify_state.last_grabbed_queue = queue_deadline;
        }

        if let Some(track) = current_playback.item {
            state.queue_index =
                track_index(&state.queue, track.id, &track.name).unwrap_or_else(|| {
                    spotify_state.last_grabbed_queue = queue_deadline;
                    0
                });
        }

        state.volume = current_playback.device.volume_percent.map(|v| v as u8);
        if now >= state.last_interaction {
            state.playing = current_playback.is_playing;
            state.progress = current_playback.progress_ms;
        }
        state.last_progress_update = now;
        spotify_state.last_grabbed_playback = now;
    });
}

fn get_spotify_queue() {
    let now = Instant::now();
    if now < PLAYBACK_STATE.read().last_interaction
        || now < SPOTIFY_STATE.read().last_grabbed_queue + Duration::from_secs(15)
    {
        return;
    }

    // https://developer.spotify.com/documentation/web-api/reference/get-queue
    let Some(q) = SPOTIFY_CLIENT.api_json::<CurrentUserQueue>("me/player/queue", "queue") else {
        return;
    };
    let Some(currently_playing) = q.currently_playing else {
        error!("Failed to get queue data: Nothing is currently playing");
        return;
    };
    let current_track_id = currently_playing.id;
    let current_title = currently_playing.name.clone();
    let new_queue: Vec<Track> = std::iter::once(currently_playing).chain(q.queue).collect();

    cache_queue_images(&new_queue);

    let mut spotify_state = SPOTIFY_STATE.write();
    update_playback_state(|state| {
        if !spotify_state.context_updated
            && let Some(new_index) = track_index(&state.queue, current_track_id, &current_title)
        {
            state.queue_index = new_index;
            state.queue.truncate(new_index);
            state.queue.extend(new_queue);
        } else {
            spotify_state.context_updated = false;
            state.queue = new_queue;
            state.queue_index = 0;
        }
        spotify_state.last_grabbed_queue = Instant::now();
    });
}

fn cache_queue_images(queue: &[Track]) {
    let mut missing_artists = HashSet::new();
    for track in queue {
        if let Some(key) = &track.album.image {
            ensure_image_cached(key);
        }
        if let Some(artist_id) = track.artist.id
            && !ARTIST_DATA_CACHE.contains_key(&artist_id)
            && missing_artists.insert(artist_id)
        {
            spawn(move || fetch_artist_image(artist_id));
        }
    }
}

fn fetch_artist_image(artist_id: ArtistId) {
    let Some(artist) = SPOTIFY_CLIENT.api_json::<Artist>(&format!("artists/{artist_id}"), "artist")
    else {
        return;
    };
    if let Some(actual_id) = artist.id {
        ARTIST_DATA_CACHE.insert(actual_id, artist.image.clone());
        if let Some(image) = artist.image.as_deref() {
            ensure_image_cached(image);
        }
    }
}

fn ensure_image_cached(url: &str) {
    if IMAGES_CACHE.contains_key(url) {
        return;
    }
    IMAGES_CACHE.insert(url.to_owned(), None);

    let url = url.to_owned();
    spawn(move || {
        let result = SPOTIFY_CLIENT
            .http
            .get(&url)
            .call()
            .map_err(ClientError::from)
            .and_then(|mut resp| Ok(resp.body_mut().read_to_vec()?))
            .and_then(|bytes| Ok(image::load_from_memory(&bytes)?));
        let Ok(img) = result.inspect_err(|err| warn!("Failed to cache image {url}: {err}")) else {
            IMAGES_CACHE.remove(&url);
            return;
        };
        let img = if img.width() != 64 || img.height() != 64 {
            img.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3)
        } else {
            img
        };
        IMAGES_CACHE.insert(url, Some(Arc::new(img.to_rgba8())));
        update_color_palettes();
    });
}

fn poll_playlists() {
    let targets = CONFIG
        .playlists
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let mut cached = load_cached_playlist_tracks();

    loop {
        // https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists
        let playlists = SPOTIFY_CLIENT
            .api_json_payload::<Page<Playlist>>("me/playlists", &[("limit", "50")], "playlists")
            .map(|p| p.items)
            .unwrap_or_default();

        for playlist in playlists {
            let rating_index = rating_index(&playlist.name);
            if !targets.contains(playlist.name.as_str()) && rating_index.is_none() {
                continue;
            }
            if let Some(image) = &playlist.image {
                ensure_image_cached(image);
            }

            if let Some((snapshot_id, tracks)) = cached.remove(&playlist.id)
                && snapshot_id == playlist.snapshot_id
            {
                insert_playlist(&playlist, tracks, playlist.total_tracks, rating_index);
                continue;
            }

            if Some(&playlist.snapshot_id)
                != PLAYBACK_STATE
                    .read()
                    .playlists
                    .get(&playlist.id)
                    .map(|p| &p.snapshot_id)
                && let Some((total, tracks)) = fetch_playlist_tracks(&playlist)
            {
                insert_playlist(&playlist, tracks, total, rating_index);
                persist_playlist_cache();
            }
        }

        sleep(Duration::from_secs(20));
    }
}

fn rating_index(name: &str) -> Option<u8> {
    CONFIG.ratings_enabled.then_some(())?;
    RATING_PLAYLISTS
        .iter()
        .position(|&p| p == name)
        .map(|i| i as u8)
}

fn fetch_playlist_tracks(playlist: &Playlist) -> Option<(u32, HashSet<TrackId>)> {
    let chunk_size = 50;
    let num_pages = playlist.total_tracks.div_ceil(chunk_size);
    let mut total = 0;
    let mut tracks = HashSet::new();
    info!("Fetching {num_pages} pages from playlist {}", playlist.name);

    for page in 0..num_pages {
        let page = fetch_playlist_page(playlist.id, chunk_size, page)?;
        total = page.total;
        tracks.extend(page.items.iter().filter_map(|item| item.track.id));
    }
    Some((total, tracks))
}

fn fetch_playlist_page(
    playlist_id: PlaylistId,
    chunk_size: u32,
    page: u32,
) -> Option<Page<PlaylistItem>> {
    let limit = chunk_size.to_string();
    let offset = (page * chunk_size).to_string();
    SPOTIFY_CLIENT.api_json_payload(
        &format!("playlists/{playlist_id}/items"),
        &[
            (
                "fields",
                "href,limit,offset,total,items(is_local,track(id))",
            ),
            ("limit", &limit),
            ("offset", &offset),
        ],
        "playlist page",
    )
}

fn insert_playlist(
    playlist: &Playlist,
    tracks: HashSet<TrackId>,
    tracks_total: u32,
    rating_index: Option<u8>,
) {
    PLAYBACK_STATE.write().playlists.insert(
        playlist.id,
        CondensedPlaylist {
            id: playlist.id,
            name: playlist.name.clone(),
            image_url: playlist.image.clone(),
            tracks,
            tracks_total,
            snapshot_id: playlist.snapshot_id,
            rating_index,
        },
    );
}

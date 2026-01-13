use crate::config::CONFIG;
use arrayvec::ArrayString;
use auto_palette::Palette;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use dashmap::DashMap;
use image::RgbaImage;
use itertools::Itertools;
use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener},
    path::PathBuf,
    sync::{Arc, LazyLock},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};
use thiserror::Error;
use time::{Duration as TimeDuration, OffsetDateTime};
use tracing::{error, info, warn};
use ureq::Agent;
use url::Url;

// --- RSPOTIFY LOGIC ---
const VERIFIER_BYTES: usize = 43;
const REDIRECT_HOST: &str = "127.0.0.1";
const REDIRECT_PORT: u16 = 7474;

#[derive(Debug)]
pub struct SpotifyClient {
    client_id: String,
    cache_path: PathBuf,
    token: RwLock<Token>,
    pub http: Agent,
}

pub type AlbumId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Album {
    pub id: AlbumId,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: String,
}

pub type ArtistId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: String,
}

#[derive(Deserialize)]
pub struct Artists {
    pub artists: Vec<Artist>,
}

pub type TrackId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub album: Album,
    #[serde(deserialize_with = "deserialize_first_artist", rename = "artists")]
    pub artist: Artist,
    pub duration_ms: u32,
}

#[derive(Deserialize)]
pub struct PartialTrack {
    pub id: TrackId,
}

pub type PlaylistId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Playlist {
    pub id: PlaylistId,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: String,
    pub snapshot_id: ArrayString<32>,
    #[serde(deserialize_with = "deserialize_tracks_total", rename = "tracks")]
    pub total_tracks: u32,
}

#[derive(Deserialize)]
pub struct PlaylistItem {
    pub track: PartialTrack,
}

#[derive(Deserialize)]
pub struct Context {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct CurrentPlaybackContext {
    pub device: Device,
    pub context: Option<Context>,
    #[serde(default)]
    pub progress_ms: u32,
    pub is_playing: bool,
    pub item: Option<Track>,
}

#[derive(Deserialize)]
pub struct CurrentUserQueue {
    pub currently_playing: Option<Track>,
    pub queue: Vec<Track>,
}

#[derive(Deserialize)]
pub struct Device {
    pub volume_percent: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Token {
    #[serde(rename = "access_token")]
    access: String,
    expires_in: u32,
    expires_at: Option<OffsetDateTime>,
    #[serde(rename = "refresh_token")]
    refresh: Option<String>,
    #[serde(
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
}

fn read_token_cache(
    allow_expired: bool,
    cache_path: &PathBuf,
    scopes: &HashSet<String>,
) -> Result<Option<Token>, std::io::Error> {
    let token: Token = serde_json::from_str(&fs::read_to_string(cache_path)?)?;
    if !scopes.is_subset(&token.scopes) || (!allow_expired && token.is_expired()) {
        Ok(None)
    } else {
        Ok(Some(token))
    }
}

fn prompt_for_token(
    url: &str,
    cache_path: &PathBuf,
    scopes: &HashSet<String>,
    client_id: &str,
    verifier: &str,
    http: &Agent,
) -> Token {
    if let Ok(Some(cached)) = read_token_cache(true, cache_path, scopes) {
        return cached;
    }
    match webbrowser::open(url) {
        Ok(()) => println!("Opened {url} in your browser."),
        Err(why) => eprintln!(
            "Error when trying to open an URL in your browser: {why:?}. \
             Please navigate here manually: {url}"
        ),
    }

    let listener = TcpListener::bind(SocketAddr::new(
        REDIRECT_HOST.parse::<IpAddr>().unwrap(),
        REDIRECT_PORT,
    ))
    .unwrap();

    let mut stream = listener.incoming().flatten().next().unwrap();
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    let request_path = request_line.split_whitespace().nth(1).unwrap();
    let redirect_full_url =
        format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback{request_path}");
    let code = Url::parse(&redirect_full_url)
        .unwrap()
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.into_owned())
        .unwrap();
    let message = "Cantus connected successfully, this tab can be closed.";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap();

    let response = http
        .post("https://accounts.spotify.com/api/token")
        .send_form([
            ("grant_type", "authorization_code"),
            ("code", &code),
            (
                "redirect_uri",
                &format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback"),
            ),
            ("client_id", client_id),
            ("code_verifier", verifier),
        ])
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();
    let mut token = serde_json::from_str::<Token>(&response).unwrap();
    token.expires_at =
        OffsetDateTime::now_utc().checked_add(TimeDuration::seconds(i64::from(token.expires_in)));
    token
}

impl SpotifyClient {
    fn auth_headers(&self) -> ClientResult<String> {
        if self.token.read().is_expired() {
            let token = self.refetch_token()?;
            *self.token.write() = token;
            self.write_token_cache();
        }
        Ok(format!("Bearer {}", self.token.read().access))
    }

    pub fn api_get(&self, url: &str) -> ClientResult<String> {
        let response = self
            .http
            .get(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .call()?;
        Ok(response.into_body().read_to_string()?)
    }

    pub fn api_get_payload(&self, url: &str, payload: &[(&str, &str)]) -> ClientResult<String> {
        let response = self
            .http
            .get(format!("https://api.spotify.com/v1/{url}"))
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
            .post(format!("https://api.spotify.com/v1/{url}"))
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
            .delete(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .call()?;
        Ok(())
    }

    pub fn api_delete_payload(&self, url: &str, payload: &str) -> ClientResult<()> {
        self.http
            .delete(format!("https://api.spotify.com/v1/{url}"))
            .header("Content-Type", "application/json; charset=utf-8")
            .header("authorization", self.auth_headers()?)
            .force_send_body()
            .send(payload)?;
        Ok(())
    }

    fn write_token_cache(&self) {
        fs::write(
            &self.cache_path,
            serde_json::to_string(&*self.token.read()).unwrap(),
        )
        .unwrap();
    }

    fn refetch_token(&self) -> ClientResult<Token> {
        let Some(refresh_token) = &self.token.read().refresh else {
            return Err(ClientError::InvalidToken);
        };
        let response = self
            .http
            .post("https://accounts.spotify.com/api/token")
            .send_form([
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &self.client_id),
            ])?
            .into_body()
            .read_to_string()?;
        let mut token = serde_json::from_str::<Token>(&response)?;
        token.expires_at = OffsetDateTime::now_utc()
            .checked_add(TimeDuration::seconds(i64::from(token.expires_in)));
        Ok(token)
    }

    pub fn new(client_id: String, scopes: &HashSet<String>, cache_path: PathBuf) -> Self {
        let state = generate_random_string(
            16,
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        );
        let (verifier, url) = get_authorize_url(&client_id, scopes, &state).unwrap();
        let agent = Agent::new_with_defaults();
        let token = prompt_for_token(&url, &cache_path, scopes, &client_id, &verifier, &agent);
        let spotify_client = Self {
            client_id,
            cache_path,
            token: RwLock::new(token),
            http: Agent::new_with_defaults(),
        };
        spotify_client.write_token_cache();
        spotify_client
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

    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

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

    #[error("Token is not valid")]
    InvalidToken,
}

impl From<ureq::Error> for ClientError {
    fn from(err: ureq::Error) -> Self {
        Self::Http(err.to_string())
    }
}

pub type ClientResult<T> = Result<T, ClientError>;

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
    let scopes = scopes.clone().into_iter().collect::<Vec<_>>().join(" ");
    s.serialize_str(&scopes)
}

#[derive(Deserialize)]
struct TracksRef {
    total: u32,
}

fn deserialize_tracks_total<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let tracks_ref = TracksRef::deserialize(deserializer)?;
    Ok(tracks_ref.total)
}

#[derive(Deserialize)]
struct Image {
    url: String,
    width: Option<u32>,
}
fn deserialize_images<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let images: Vec<Image> = Vec::deserialize(deserializer)?;
    Ok(images
        .into_iter()
        .min_by_key(|img| img.width)
        .map(|img| img.url)
        .unwrap())
}

fn deserialize_first_artist<'de, D>(deserializer: D) -> Result<Artist, D::Error>
where
    D: Deserializer<'de>,
{
    let artists: Vec<Artist> = Vec::deserialize(deserializer)?;
    Ok(artists.into_iter().next().unwrap())
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
pub struct Page<T: DeserializeOwned> {
    #[serde(deserialize_with = "vec_without_nulls")]
    pub items: Vec<T>,
    pub total: u32,
}

// --- SPOTIFY LOGIC ---
pub static PLAYBACK_STATE: LazyLock<RwLock<PlaybackState>> = LazyLock::new(|| {
    RwLock::new(PlaybackState {
        playing: false,
        progress: 0,
        volume: None,
        queue: Vec::new(),
        queue_index: 0,
        playlists: HashMap::new(),

        current_context: None,
        context_updated: false,

        interaction: false,
        last_interaction: Instant::now(),
        last_progress_update: Instant::now(),
        last_grabbed_playback: Instant::now().checked_sub(Duration::from_secs(60)).unwrap(),
        last_grabbed_queue: Instant::now().checked_sub(Duration::from_secs(60)).unwrap(),
    })
});
pub static IMAGES_CACHE: LazyLock<DashMap<String, Option<Arc<RgbaImage>>>> =
    LazyLock::new(DashMap::new);
pub static ALBUM_DATA_CACHE: LazyLock<DashMap<AlbumId, Option<AlbumData>>> =
    LazyLock::new(DashMap::new);
pub static ARTIST_DATA_CACHE: LazyLock<DashMap<ArtistId, Option<String>>> =
    LazyLock::new(DashMap::new);

const RATING_PLAYLISTS: [&str; 10] = [
    "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

pub static SPOTIFY_CLIENT: LazyLock<SpotifyClient> = LazyLock::new(|| {
    let mut scopes = HashSet::new();
    scopes.insert("user-read-playback-state".to_owned());
    scopes.insert("user-modify-playback-state".to_owned());
    scopes.insert("user-read-currently-playing".to_owned());
    scopes.insert("playlist-read-private".to_owned());
    scopes.insert("playlist-read-collaborative".to_owned());
    scopes.insert("playlist-modify-private".to_owned());
    scopes.insert("playlist-modify-public".to_owned());
    scopes.insert("user-library-read".to_owned());
    scopes.insert("user-library-modify".to_owned());
    SpotifyClient::new(
        CONFIG.spotify_client_id.clone().expect(
            "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
        ),
        &scopes,
        dirs::config_dir()
            .unwrap()
            .join("cantus")
            .join("spotify_cache.json"),
    )
});

pub struct PlaybackState {
    pub playing: bool,
    pub progress: u32,
    pub volume: Option<u8>,
    pub queue: Vec<Track>,
    pub queue_index: usize,
    pub playlists: HashMap<PlaylistId, CondensedPlaylist>,

    current_context: Option<String>,
    context_updated: bool,

    pub interaction: bool,
    pub last_interaction: Instant,
    pub last_progress_update: Instant,
    last_grabbed_playback: Instant,
    last_grabbed_queue: Instant,
}

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

pub struct AlbumData {
    pub primary_colors: Vec<[u8; NUM_SWATCHES]>,
}

pub struct CondensedPlaylist {
    pub id: PlaylistId,
    pub name: String,
    pub image_url: String,
    pub tracks: HashSet<TrackId>,
    pub tracks_total: u32,
    snapshot_id: ArrayString<32>,
    pub rating_index: Option<u8>,
}

pub fn update_playback_state<F>(update: F)
where
    F: FnOnce(&mut PlaybackState),
{
    let mut state = PLAYBACK_STATE.write();
    update(&mut state);
}

type PlaylistCache = HashMap<PlaylistId, (ArrayString<32>, HashSet<TrackId>)>;

fn load_cached_playlist_tracks() -> PlaylistCache {
    let cache_path = dirs::config_dir()
        .unwrap()
        .join("cantus")
        .join("cantus_playlist_tracks.json");
    let bytes = match fs::read(&cache_path) {
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
                playlist.id,
                (
                    playlist.snapshot_id,
                    playlist.tracks.iter().copied().collect(),
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

pub fn init() {
    let cantus_dir = dirs::config_dir().unwrap().join("cantus");
    if !cantus_dir.exists() {
        std::fs::create_dir(&cantus_dir).unwrap();
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

fn get_spotify_playback() {
    let now = Instant::now();
    {
        let state = PLAYBACK_STATE.read();
        if now < state.last_interaction
            || now < state.last_grabbed_playback + Duration::from_secs(1)
        {
            return;
        }
    }

    let current_playback_opt = SPOTIFY_CLIENT
        .api_get("me/player")
        .ok()
        .filter(|res| !res.is_empty())
        .and_then(|res| {
            serde_json::from_str::<CurrentPlaybackContext>(&res)
                .map_err(|e| error!("Failed to parse playback: {e}"))
                .ok()
        });
    update_playback_state(|state| state.last_grabbed_playback = Instant::now());
    let Some(current_playback) = current_playback_opt else {
        return;
    };

    update_playback_state(|state| {
        let new_context = current_playback.context.as_ref().map(|c| &c.uri);
        let now = Instant::now();
        let queue_deadline = now.checked_sub(Duration::from_secs(60)).unwrap();

        if state.current_context.as_ref() != new_context {
            state.context_updated = true;
            state.current_context = new_context.map(String::from);
            state.last_grabbed_queue = queue_deadline;
        }

        if let Some(track) = current_playback.item {
            state.queue_index = state
                .queue
                .iter()
                .position(|t| t.name == track.name)
                .unwrap_or_else(|| {
                    state.last_grabbed_queue = queue_deadline;
                    0
                });
        }

        state.volume = current_playback.device.volume_percent.map(|v| v as u8);
        state.playing = current_playback.is_playing;
        state.progress = current_playback.progress_ms;
        state.last_progress_update = now;
        state.last_grabbed_playback = now;
    });
}

fn get_spotify_queue() {
    let now = Instant::now();
    {
        let state = PLAYBACK_STATE.read();
        if now < state.last_interaction || now < state.last_grabbed_queue + Duration::from_secs(15)
        {
            return;
        }
    }

    let queue_opt = SPOTIFY_CLIENT
        .api_get("me/player/queue")
        .map_err(|e| error!("Failed to fetch queue: {e}"))
        .ok()
        .and_then(|res| {
            serde_json::from_str::<CurrentUserQueue>(&res)
                .map_err(|e| error!("Failed to parse queue: {e}"))
                .ok()
        });
    update_playback_state(|state| state.last_grabbed_queue = Instant::now());

    let Some(queue) = queue_opt else {
        return;
    };

    let Some(currently_playing) = queue.currently_playing else {
        return;
    };
    let new_queue: Vec<Track> = std::iter::once(currently_playing)
        .chain(queue.queue)
        .collect();
    let current_title = new_queue.first().unwrap().name.clone();

    let mut missing_urls = HashSet::new();
    let mut missing_artists = HashSet::new();
    for track in &new_queue {
        if !IMAGES_CACHE.contains_key(&track.album.image) {
            missing_urls.insert(track.album.image.clone());
        }
        if !ARTIST_DATA_CACHE.contains_key(&track.artist.id) {
            missing_artists.insert(track.artist.id);
        }
    }
    for url in missing_urls {
        ensure_image_cached(url.as_str());
    }

    if !missing_artists.is_empty() {
        let artist_query = missing_artists
            .into_iter()
            .map(|artist| artist.as_str().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        spawn(move || {
            let Some(artists) = SPOTIFY_CLIENT
                .api_get(&format!("artists/?ids={artist_query}"))
                .map_err(|e| error!("Failed to fetch artists: {e}"))
                .ok()
                .and_then(|res| serde_json::from_str::<Artists>(&res).ok())
            else {
                return;
            };
            for artist in artists.artists {
                ARTIST_DATA_CACHE.insert(artist.id, Some(artist.image.clone()));
                ensure_image_cached(artist.image.as_str());
            }
        });
    }

    update_playback_state(|state| {
        if !state.context_updated
            && let Some(new_index) = state.queue.iter().position(|t| t.name == current_title)
        {
            state.queue_index = new_index;
            state.queue.truncate(new_index);
            state.queue.extend(new_queue);
        } else {
            state.context_updated = false;
            state.queue = new_queue;
            state.queue_index = 0;
        }

        state.last_grabbed_queue = Instant::now();
    });
}

fn ensure_image_cached(url: &str) {
    if IMAGES_CACHE.contains_key(url) {
        return;
    }
    IMAGES_CACHE.insert(url.to_owned(), None);

    let url = url.to_owned();
    spawn(move || {
        let mut response = match SPOTIFY_CLIENT.http.get(&url).call() {
            Ok(response) => response,
            Err(err) => {
                warn!("Failed to cache image {url}: {err}");
                return;
            }
        };
        let Ok(dynamic_image) =
            image::load_from_memory(&response.body_mut().read_to_vec().unwrap())
        else {
            warn!("Failed to cache image {url}: failed to read image");
            return;
        };
        let dynamic_image = if dynamic_image.width() != 64 || dynamic_image.height() != 64 {
            dynamic_image.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3)
        } else {
            dynamic_image
        };
        IMAGES_CACHE.insert(url, Some(Arc::new(dynamic_image.to_rgba8())));
        update_color_palettes();
    });
}

fn poll_playlists() {
    let target_playlists = CONFIG
        .playlists
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let include_ratings = CONFIG.ratings_enabled;
    let cached_playlist_tracks = load_cached_playlist_tracks();

    loop {
        let playlists = SPOTIFY_CLIENT
            .api_get_payload("me/playlists", &[("limit", "50")])
            .map_err(|err| error!("Failed to fetch users playlists: {err}"))
            .and_then(|res| {
                serde_json::from_str::<Page<Playlist>>(&res)
                    .map_err(|err| error!("Failed to parse users playlists: {err}"))
            })
            .map(|page| page.items)
            .unwrap_or_default();

        for playlist in playlists {
            if !(target_playlists.contains(playlist.name.as_str())
                || (include_ratings && RATING_PLAYLISTS.contains(&playlist.name.as_str())))
            {
                continue;
            }
            ensure_image_cached(&playlist.image);

            let rating_index = if CONFIG.ratings_enabled {
                RATING_PLAYLISTS
                    .iter()
                    .enumerate()
                    .find(|(_, p)| *p == &playlist.name)
                    .map(|(i, _)| i as u8)
            } else {
                None
            };

            if let Some(cached) = cached_playlist_tracks.get(&playlist.id)
                && playlist.snapshot_id == cached.0
            {
                update_playback_state(|state| {
                    state.playlists.insert(
                        playlist.id,
                        CondensedPlaylist {
                            id: playlist.id,
                            name: playlist.name,
                            image_url: playlist.image,
                            tracks: cached.1.clone(),
                            tracks_total: playlist.total_tracks,
                            snapshot_id: cached.0,
                            rating_index,
                        },
                    );
                });
                continue;
            }
            if Some(&playlist.snapshot_id)
                == PLAYBACK_STATE
                    .read()
                    .playlists
                    .get(&playlist.id)
                    .map(|p| &p.snapshot_id)
            {
                continue;
            }

            let chunk_size = 50;
            let num_pages = playlist.total_tracks.div_ceil(chunk_size) as usize;
            info!("Fetching {num_pages} pages from playlist {}", playlist.name);
            let mut pages = Vec::new();
            for page in 0..num_pages {
                let page_data = SPOTIFY_CLIENT
                    .api_get_payload(
                        &format!("playlists/{}/tracks", playlist.id),
                        &[
                            (
                                "fields",
                                "href,limit,offset,total,items(is_local,track(id))",
                            ),
                            ("limit", &chunk_size.to_string()),
                            ("offset", &((page as u32) * chunk_size).to_string()),
                        ],
                    )
                    .map_err(|e| error!("Failed to fetch playlist page: {e}"))
                    .ok()
                    .and_then(|res| {
                        serde_json::from_str::<Page<PlaylistItem>>(&res)
                            .map_err(|e| error!("Failed to parse playlist page: {e}"))
                            .ok()
                    });

                if let Some(p) = page_data {
                    pages.push(p);
                } else {
                    return;
                }
            }

            let new_total = pages.first().map_or(0, |p| p.total);
            let playlist_track_ids: HashSet<TrackId> = pages
                .into_iter()
                .flat_map(|page| page.items)
                .map(|item| item.track.id)
                .collect();

            update_playback_state(|state| {
                state
                    .playlists
                    .entry(playlist.id)
                    .and_modify(|state_playlist| {
                        state_playlist.tracks.clone_from(&playlist_track_ids);
                        state_playlist.tracks_total = new_total;
                        state_playlist.snapshot_id = playlist.snapshot_id;
                    })
                    .or_insert_with(|| CondensedPlaylist {
                        id: playlist.id,
                        name: playlist.name,
                        image_url: playlist.image,
                        tracks: playlist_track_ids,
                        tracks_total: new_total,
                        snapshot_id: playlist.snapshot_id,
                        rating_index,
                    });
            });
            persist_playlist_cache();
        }

        sleep(Duration::from_secs(12));
    }
}

/// Downloads and caches an image from the given URL.
fn update_color_palettes() {
    let state = PLAYBACK_STATE.read();
    for track in &state.queue {
        if ALBUM_DATA_CACHE.contains_key(&track.album.id) {
            continue;
        }
        let Some(image_ref) = IMAGES_CACHE.get(&track.album.image) else {
            continue;
        };
        let Some(album_image) = image_ref.as_ref() else {
            continue;
        };
        let Some(artist_image_url_ref) = ARTIST_DATA_CACHE
            .get(&track.artist.id)
            .map(|entry| entry.value().clone())
        else {
            continue;
        };
        ALBUM_DATA_CACHE.insert(track.album.id, None);

        let width = album_image.width();
        let height = album_image.height();

        let get_swatches = |img_data| {
            let palette: Palette<f64> = Palette::builder()
                .algorithm(auto_palette::Algorithm::SLIC)
                .filter(ChromaFilter { threshold: 30 })
                .build(&img_data)
                .unwrap();
            palette
                .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                .or_else(|_| palette.find_swatches(NUM_SWATCHES))
                .unwrap()
        };

        let mut swatches = get_swatches(
            auto_palette::ImageData::new(width, height, album_image.as_ref()).unwrap(),
        );
        if swatches.len() < NUM_SWATCHES
            && let Some(artist_image_url) = artist_image_url_ref.as_ref()
        {
            let Some(artist_image_ref) = IMAGES_CACHE.get(artist_image_url) else {
                ALBUM_DATA_CACHE.remove(&track.album.id);
                continue;
            };
            let Some(artist_image) = artist_image_ref.as_ref() else {
                ALBUM_DATA_CACHE.remove(&track.album.id);
                continue;
            };
            let artist_new_width = (width as f32 * 0.1).round() as u32;
            let mut new_img = RgbaImage::new(width + artist_new_width, height);
            image::imageops::overlay(&mut new_img, album_image.as_ref(), 0, 0);
            let artist_img_resized = image::imageops::resize(
                artist_image.as_ref(),
                artist_new_width,
                height,
                image::imageops::FilterType::Nearest,
            );
            image::imageops::overlay(&mut new_img, &artist_img_resized, i64::from(width), 0);

            swatches = get_swatches(
                auto_palette::ImageData::new(new_img.width(), new_img.height(), &new_img).unwrap(),
            );
        }

        let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
        let primary_colors = swatches
            .iter()
            .map(|s| {
                let rgb = s.color().to_rgb();
                let ratio = ((s.ratio() / total_ratio_sum) as f32 * 255.0).round() as u8;
                [rgb.r, rgb.g, rgb.b, ratio]
            })
            .sorted_by(|a, b| b[3].cmp(&a[3]))
            .collect::<Vec<_>>();

        ALBUM_DATA_CACHE.insert(track.album.id, Some(AlbumData { primary_colors }));
    }
    drop(state);
}

/// A filter that filters chroma values.
#[derive(Debug)]
struct ChromaFilter {
    threshold: u8,
}
impl auto_palette::Filter for ChromaFilter {
    fn test(&self, pixel: &auto_palette::Rgba) -> bool {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max - min) > self.threshold
    }
}

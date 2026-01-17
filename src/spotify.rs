use crate::{
    ARTIST_DATA_CACHE, Artist, CondensedPlaylist, IMAGES_CACHE, PLAYBACK_STATE, PlaylistId, Track,
    TrackId, config::CONFIG, deserialize_images, render::update_color_palettes,
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
    id: TrackId,
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

    fn set_expiration(&mut self) {
        self.expires_at = OffsetDateTime::now_utc()
            .checked_add(TimeDuration::seconds(i64::from(self.expires_in)));
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
        Err(err) => eprintln!(
            "Error when trying to open an URL in your browser: {err:?}. Please navigate here manually: {url}"
        ),
    }

    let listener = TcpListener::bind((REDIRECT_HOST, REDIRECT_PORT)).unwrap();
    let mut stream = listener.incoming().flatten().next().unwrap();
    let mut request_line = String::new();
    BufReader::new(&stream)
        .read_line(&mut request_line)
        .unwrap();

    let code = Url::parse(&format!(
        "http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback{}",
        request_line.split_whitespace().nth(1).unwrap()
    ))
    .unwrap()
    .query_pairs()
    .find(|(key, _)| key == "code")
    .map(|(_, value)| value.into_owned())
    .unwrap();

    let message = "Cantus connected successfully, this tab can be closed.";
    write!(
        stream,
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    )
    .unwrap();

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
    token.set_expiration();
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
        token.set_expiration();
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
            http: agent,
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

type ClientResult<T> = Result<T, ClientError>;

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
    s.serialize_str(&scopes.iter().cloned().collect::<Vec<_>>().join(" "))
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
        dirs::config_dir()
            .unwrap()
            .join("cantus")
            .join("spotify_cache.json"),
    )
});

type PlaylistCache = HashMap<PlaylistId, (ArrayString<32>, HashSet<TrackId>)>;

fn load_cached_playlist_tracks() -> PlaylistCache {
    let path = dirs::config_dir()
        .unwrap()
        .join("cantus")
        .join("cantus_playlist_tracks.json");
    fs::read(&path)
        .ok()
        .and_then(|b| {
            serde_json::from_slice(&b)
                .map_err(|e| warn!("Failed to parse playlist cache: {e}"))
                .ok()
        })
        .unwrap_or_default()
}

fn persist_playlist_cache() {
    let cache_payload: PlaylistCache = PLAYBACK_STATE
        .read()
        .playlists
        .values()
        .map(|p| (p.id, (p.snapshot_id, p.tracks.iter().copied().collect())))
        .collect();
    if !cache_payload.is_empty() {
        let path = dirs::config_dir()
            .unwrap()
            .join("cantus")
            .join("cantus_playlist_tracks.json");
        if let Ok(ser) = serde_json::to_vec(&cache_payload) {
            let _ = fs::write(path, ser);
        }
    }
}

pub fn init() {
    let cantus_dir = dirs::config_dir().unwrap().join("cantus");
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

fn get_spotify_playback() {
    let now = Instant::now();
    if now < PLAYBACK_STATE.read().last_interaction
        || now < SPOTIFY_STATE.read().last_grabbed_playback + Duration::from_secs(1)
    {
        return;
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
    let Some(current_playback) = current_playback_opt else {
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
            state.queue_index = state
                .queue
                .iter()
                .position(|t| t.name == track.name)
                .unwrap_or_else(|| {
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

    let queue_data = SPOTIFY_CLIENT
        .api_get("me/player/queue")
        .map_err(|e| error!("Failed to fetch queue: {e}"))
        .ok()
        .and_then(|res| {
            serde_json::from_str::<CurrentUserQueue>(&res)
                .map_err(|e| error!("Failed to parse queue: {e}"))
                .ok()
        });
    let Some(queue) = queue_data.and_then(|q| q.currently_playing.map(|cp| (cp, q.queue))) else {
        return;
    };

    let new_queue: Vec<Track> = std::iter::once(queue.0).chain(queue.1).collect();
    let current_title = new_queue[0].name.clone();

    let mut missing_artists = HashSet::new();
    for track in &new_queue {
        if let Some(key) = &track.album.image {
            ensure_image_cached(key);
        }
        if !ARTIST_DATA_CACHE.contains_key(&track.artist.id) {
            missing_artists.insert(track.artist.id);
        }
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
                .and_then(|res| {
                    let result = serde_json::from_str::<HashMap<String, Vec<Artist>>>(&res);
                    match result {
                        Ok(mut map) => map.remove("artists"),
                        Err(err) => {
                            error!("Deserialization error: {err:?}");
                            None
                        }
                    }
                })
            else {
                return;
            };
            for artist in artists {
                ARTIST_DATA_CACHE.insert(artist.id, artist.image.clone());
                if let Some(image) = artist.image.as_deref() {
                    ensure_image_cached(image);
                }
            }
        });
    }

    let mut spotify_state = SPOTIFY_STATE.write();
    update_playback_state(|state| {
        if !spotify_state.context_updated
            && let Some(new_index) = state.queue.iter().position(|t| t.name == current_title)
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

fn ensure_image_cached(url: &str) {
    if IMAGES_CACHE.contains_key(url) {
        return;
    }
    IMAGES_CACHE.insert(url.to_owned(), None);

    let url = url.to_owned();
    spawn(move || {
        if let Ok(mut resp) = SPOTIFY_CLIENT.http.get(&url).call()
            && let Ok(img) = image::load_from_memory(&resp.body_mut().read_to_vec().unwrap())
        {
            let img = if img.width() != 64 || img.height() != 64 {
                img.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3)
            } else {
                img
            };
            IMAGES_CACHE.insert(url, Some(Arc::new(img.to_rgba8())));
            update_color_palettes();
        }
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
        let playlists = SPOTIFY_CLIENT
            .api_get_payload("me/playlists", &[("limit", "50")])
            .ok()
            .and_then(|res| serde_json::from_str::<Page<Playlist>>(&res).ok())
            .map(|p| p.items)
            .unwrap_or_default();

        for playlist in playlists {
            let is_rating =
                CONFIG.ratings_enabled && RATING_PLAYLISTS.contains(&playlist.name.as_str());
            if !targets.contains(playlist.name.as_str()) && !is_rating {
                continue;
            }
            if let Some(image) = &playlist.image {
                ensure_image_cached(image);
            }

            let rating_index = if CONFIG.ratings_enabled {
                RATING_PLAYLISTS
                    .iter()
                    .position(|&p| p == playlist.name)
                    .map(|i| i as u8)
            } else {
                None
            };

            // Take from cache if exists
            if let Some((snapshot_id, tracks)) = cached.remove(&playlist.id)
                && snapshot_id == playlist.snapshot_id
            {
                PLAYBACK_STATE.write().playlists.insert(
                    playlist.id,
                    CondensedPlaylist {
                        id: playlist.id,
                        name: playlist.name.clone(),
                        image_url: playlist.image.clone(),
                        tracks,
                        tracks_total: playlist.total_tracks,
                        snapshot_id,
                        rating_index,
                    },
                );
                continue;
            }

            // State mismatched, fetch new
            if Some(&playlist.snapshot_id)
                != PLAYBACK_STATE
                    .read()
                    .playlists
                    .get(&playlist.id)
                    .map(|p| &p.snapshot_id)
            {
                // Fetch the fresh playlists as needed
                let chunk_size = 50;
                let num_pages = playlist.total_tracks.div_ceil(chunk_size);
                info!("Fetching {num_pages} pages from playlist {}", playlist.name);
                let mut total = 0;
                let mut playlist_track_ids = HashSet::new();
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
                                ("offset", &(page * chunk_size).to_string()),
                            ],
                        )
                        .ok()
                        .and_then(|res| {
                            serde_json::from_str::<Page<PlaylistItem>>(&res)
                                .map_err(|e| error!("Failed to parse playlist page: {e}"))
                                .ok()
                        });

                    if let Some(page) = page_data {
                        total = page.total;
                        playlist_track_ids.extend(page.items.iter().map(|item| item.track.id));
                    } else {
                        return;
                    }
                }

                update_playback_state(|state| {
                    state
                        .playlists
                        .entry(playlist.id)
                        .and_modify(|state_playlist| {
                            state_playlist.tracks.clone_from(&playlist_track_ids);
                            state_playlist.tracks_total = total;
                            state_playlist.snapshot_id = playlist.snapshot_id;
                        })
                        .or_insert_with(|| CondensedPlaylist {
                            id: playlist.id,
                            name: playlist.name,
                            image_url: playlist.image,
                            tracks: playlist_track_ids,
                            tracks_total: total,
                            snapshot_id: playlist.snapshot_id,
                            rating_index,
                        });
                });
                persist_playlist_cache();
            }
        }

        sleep(Duration::from_secs(20));
    }
}

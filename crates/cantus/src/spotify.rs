use crate::{
    app::{
        AppUpdater, Background, Fetch, TRACK_SPACING_MS, Update,
        config::{self, Config},
        enrichment::{ArtState, Enrichment},
        send_update,
    },
    render::{
        lyrics::Lyrics,
        track::{AudioFeatures, MAX_PILL_PLAYLIST_ICONS},
    },
};
use arrayvec::{ArrayString, ArrayVec};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use jiff::Timestamp;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    fs,
    io::{self, Read, Write},
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
use tracing::{error, info, warn};
use ureq::{
    Agent, Body, Error as HttpError, RequestBuilder,
    http::{Method, Request, Response, StatusCode},
    typestate::WithoutBody,
};

const API_BASE: &str = "https://api.spotify.com/v1";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const SPOTIFY_TOKEN_CACHE: &str = "spotify_cache.json";
const PLAYLIST_TRACKS_CACHE: &str = "cantus_playlist_tracks.json";
const RECCO_TRACK_URL: &str = "https://api.reccobeats.com/v1/track";
const SCOPES: &str = "user-read-playback-state user-modify-playback-state user-read-currently-playing \
playlist-read-private playlist-read-collaborative playlist-modify-private playlist-modify-public \
user-library-read user-library-modify";
const MAX_HISTORY_TRACKS: usize = 6;

pub type TrackId = ArrayString<22>;
pub type PlaylistId = ArrayString<22>;
type PlaylistTracks = Arc<HashSet<TrackId>>;

#[derive(Default)]
pub struct PlaybackState {
    pub playing: bool,
    pub volume: Option<u8>,
    pub queue: Vec<Track>,
    pub playlists: Vec<CondensedPlaylist>,
    pub timeline: Timeline,
}

/// The observed and visually smoothed position of the playback queue.
pub struct Timeline {
    pub index: usize,
    pub position_ms: f32,
    pub observed_at: Instant,
    rate: f32,
    /// Server updates are ignored until this time, so a local seek is not clobbered.
    pub hold_until: Instant,
    pub queue_start_ms: f32,
    pub movement: f32,
}

impl Default for Timeline {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            index: 0,
            position_ms: 0.0,
            observed_at: now,
            rate: 1.0,
            hold_until: now,
            queue_start_ms: 0.0,
            movement: 0.0,
        }
    }
}

impl Timeline {
    pub const fn reset_rate(&mut self) {
        self.rate = 1.0;
    }

    pub fn estimated_position(&self, playing: bool) -> f32 {
        self.position_ms + self.observed_at.elapsed().as_millis() as f32 * self.rate * f32::from(playing)
    }

    pub fn track_at_playhead(&self, queue: &[Track]) -> Option<(usize, f32)> {
        let mut start_ms = self.queue_start_ms;
        queue.iter().enumerate().find_map(|(index, track)| {
            let current = (start_ms <= 0.0 && start_ms + track.duration_ms as f32 >= 0.0)
                .then_some((index, -start_ms));
            start_ms += track.queue_span_ms();
            current
        })
    }
}

impl PlaybackState {
    pub fn update_timeline(&mut self, drag_offset_ms: f32, dragging: bool, delta_time: f32) {
        if self.queue.is_empty() {
            self.timeline.queue_start_ms = 0.0;
            self.timeline.movement = 0.0;
            return;
        }
        let index = self.timeline.index.min(self.queue.len() - 1);
        let target = -self.timeline.estimated_position(self.playing)
            - self.queue[..index].iter().map(Track::queue_span_ms).sum::<f32>()
            + drag_offset_ms;
        let difference = target - self.timeline.queue_start_ms;
        let next = if !dragging && difference.abs() > 200.0 {
            self.timeline.queue_start_ms + difference * 3.5 * delta_time
        } else {
            target
        };
        let target_movement = (next - self.timeline.queue_start_ms) * delta_time;
        self.timeline.movement +=
            (target_movement - self.timeline.movement) * (delta_time * 10.0).min(1.0);
        self.timeline.queue_start_ms = next;
    }

    fn replace_queue(
        &mut self,
        new_queue: Vec<Track>,
        current_id: Option<TrackId>,
        context_changed: bool,
    ) {
        let mut old_queue = mem::take(&mut self.queue);
        let history_len = track_index(&old_queue, current_id, &new_queue[0].name)
            .filter(|_| !context_changed)
            .unwrap_or(0);
        let mut remaining = old_queue.split_off(history_len);
        old_queue.drain(..history_len.saturating_sub(MAX_HISTORY_TRACKS));
        self.timeline.index = old_queue.len();

        for mut track in new_queue {
            if let Some(index) = remaining.iter().position(|old| old.id == track.id) {
                track.runtime = remaining.swap_remove(index).runtime;
            }
            old_queue.push(track);
        }
        self.queue = old_queue;
    }
}

#[derive(Deserialize)]
pub struct Track {
    pub id: Option<TrackId>,
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub duration_ms: u32,
    #[serde(skip)]
    pub runtime: TrackRuntime,
}

#[derive(Default)]
pub struct TrackRuntime {
    /// Album art, shared with other slots on the same URL and freed with the track.
    pub art: ArtState,
    pub playlist_expansion: f32,
    pub detail_alpha: f32,
    pub primary_icon_alpha: f32,
    pub audio_features: Fetch<AudioFeatures>,
    pub(crate) lyrics: Fetch<Lyrics>,
}

impl Track {
    pub fn artist(&self) -> &str {
        self.artists.first().map_or("", |artist| &artist.name)
    }

    pub fn queue_span_ms(&self) -> f32 {
        self.duration_ms as f32 + TRACK_SPACING_MS
    }
}

pub struct CondensedPlaylist {
    pub id: PlaylistId,
    name: String,
    pub image_url: Option<String>,
    pub art: ArtState,
    pub tracks: PlaylistTracks,
    pub rating_index: Option<u8>,
}

impl CondensedPlaylist {
    pub fn set_membership(&mut self, track_id: TrackId, add: bool) -> bool {
        let tracks = Arc::make_mut(&mut self.tracks);
        if add {
            tracks.insert(track_id)
        } else {
            tracks.remove(&track_id)
        }
    }
}

pub fn playlist_icons(
    track_id: TrackId,
    playlists: &[CondensedPlaylist],
    contains_track: bool,
) -> impl Iterator<Item = &CondensedPlaylist> {
    playlists.iter().filter(move |playlist| {
        playlist.rating_index.is_none() && playlist.tracks.contains(&track_id) == contains_track
    })
}

fn deserialize_images<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Image {
        url: String,
        width: Option<u32>,
    }

    Ok(Vec::<Image>::deserialize(deserializer)?
        .into_iter()
        .min_by_key(|image| image.width.unwrap_or(u32::MAX))
        .map(|image| image.url))
}

#[derive(Deserialize)]
pub struct Album {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct Artist {
    pub name: String,
}

fn track_index(queue: &[Track], id: Option<TrackId>, name: &str) -> Option<usize> {
    id.and_then(|track_id| queue.iter().position(|track| track.id == Some(track_id)))
        .or_else(|| queue.iter().position(|track| track.name == name))
}

#[derive(Deserialize)]
struct ReccoTrack {
    id: String,
    href: String,
}

const REDIRECT_ADDR: &str = "127.0.0.1:7474";
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
    snapshot_id: String,
    items: TracksRef,
}

#[derive(Deserialize)]
struct PlaylistItem {
    item: Option<PartialTrack>,
}

#[derive(Deserialize)]
struct Context {
    uri: String,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum PlaybackItem {
    Track(Box<Track>),
    #[serde(other)]
    Other,
}

impl PlaybackItem {
    fn into_track(self) -> Option<Track> {
        match self {
            Self::Track(track) => Some(*track),
            Self::Other => None,
        }
    }
}

#[derive(Deserialize)]
struct CurrentPlaybackContext {
    device: Device,
    context: Option<Context>,
    progress_ms: Option<u32>,
    is_playing: bool,
    item: Option<PlaybackItem>,
}

#[derive(Deserialize)]
struct CurrentUserQueue {
    currently_playing: Option<PlaybackItem>,
    queue: Vec<PlaybackItem>,
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

fn parse_credentials(json: &str) -> serde_json::Result<OAuthCredentials> {
    let mut value = serde_json::from_str::<serde_json::Value>(json)?;
    if let Some(ttl) = value.get("expires_in").and_then(serde_json::Value::as_i64) {
        value["expires_at"] = Timestamp::now().as_second().saturating_add(ttl).into();
    }
    serde_json::from_value(value)
}

fn request_json<T: DeserializeOwned>(request: RequestBuilder<WithoutBody>) -> ClientResult<T> {
    request.call()?.body_mut().read_json().map_err(Into::into)
}

fn write_cache(path: &Path, value: &impl Serialize) -> ClientResult<()> {
    serde_json::to_writer(fs::File::create(path)?, value).map_err(Into::into)
}

fn read_cache<T: DeserializeOwned>(path: &Path) -> Option<T> {
    serde_json::from_slice(&fs::read(path).ok()?)
        .inspect_err(|err| warn!(%err, ?path, "Failed to parse cache"))
        .ok()
}

fn read_token_cache(cache_path: &Path) -> Option<OAuthCredentials> {
    let token: OAuthCredentials = read_cache(cache_path)?;
    let granted: HashSet<&str> = token.scope.split_whitespace().collect();
    SCOPES
        .split_whitespace()
        .all(|scope| granted.contains(scope))
        .then_some(token)
}

/// Runs the interactive PKCE authorization flow in the user's browser.
fn prompt_for_token(client_id: &str, http: &Agent) -> ClientResult<OAuthCredentials> {
    let verifier = random_token::<32>()?;
    let expected_state = random_token::<16>()?;
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let query = form_urlencoded::Serializer::new(String::new())
        .extend_pairs([
            ("client_id", client_id),
            ("response_type", "code"),
            ("redirect_uri", REDIRECT_URI),
            ("code_challenge_method", "S256"),
            ("code_challenge", &challenge),
            ("state", &expected_state),
            ("scope", SCOPES),
        ])
        .finish();
    let url = format!("https://accounts.spotify.com/authorize?{query}");
    match Command::new("xdg-open").arg(&url).spawn() {
        Ok(_) => info!(%url, "Opened Spotify authorization URL in browser"),
        Err(err) => warn!(%err, %url, "Failed to open Spotify authorization URL; open it manually"),
    }

    let listener = TcpListener::bind(REDIRECT_ADDR)?;
    let (mut stream, _) = listener.accept()?;
    let mut buffer = [0; 1024];
    let count = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..count]);

    // The request starts with "GET /callback?code=...&state=... HTTP/1.1".
    let query = request
        .split_whitespace()
        .nth(1)
        .and_then(|target| Some(target.split_once('?')?.1))
        .ok_or_else(|| client_error("invalid Spotify authorization response"))?;
    let params: HashMap<_, _> = form_urlencoded::parse(query.as_bytes()).collect();
    if params.get("state").is_none_or(|state| *state != expected_state) {
        return Err(client_error("Spotify authorization state did not match"));
    }
    let code = params
        .get("code")
        .ok_or_else(|| client_error("invalid Spotify authorization response"))?;

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
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
            ("client_id", client_id),
            ("code_verifier", &verifier),
        ])?
        .into_body()
        .read_to_string()?;
    Ok(parse_credentials(&response)?)
}

impl SpotifyClient {
    fn authorization(&mut self) -> ClientResult<String> {
        // Refresh 10 seconds early so the token stays valid for the request itself.
        if Timestamp::now().as_second().saturating_add(10) >= self.token.expires_at {
            self.token = self.refetch_token()?;
            write_cache(&self.cache_path, &self.token)?;
        }
        Ok(format!("Bearer {}", self.token.access_token))
    }

    fn refetch_token(&self) -> ClientResult<OAuthCredentials> {
        let Some(refresh_token) = &self.token.refresh_token else {
            return prompt_for_token(&self.client_id, &self.http);
        };
        let response = match self.http.post(TOKEN_URL).send_form([
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.client_id),
        ]) {
            // A rejected or expired refresh token requires the user to reauthorize.
            Err(HttpError::StatusCode(400 | 401)) => {
                warn!("Spotify rejected the refresh token, requesting new authorization");
                return prompt_for_token(&self.client_id, &self.http);
            }
            response => response?.into_body().read_to_string()?,
        };
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

    fn send(&mut self, method: &Method, path: &str, json: Option<&str>) -> ClientResult<Response<Body>> {
        let request = Request::builder()
            .method(method.clone())
            .uri(format!("{API_BASE}/{path}"))
            .header("authorization", self.authorization()?)
            .header("content-type", "application/json; charset=utf-8")
            .body(json.unwrap_or_default())?;
        let result = self.http.run(request);
        // A 401 means the token is no longer valid: refresh it for the next request.
        if matches!(result, Err(HttpError::StatusCode(401))) {
            self.token.expires_at = 0;
        }
        result.map_err(Into::into)
    }

    fn api_json_payload<T: DeserializeOwned>(
        &mut self,
        url: &str,
        payload: &[(&str, &str)],
    ) -> Option<T> {
        let url = if payload.is_empty() {
            url.into()
        } else {
            form_urlencoded::Serializer::new(format!("{url}?"))
                .extend_pairs(payload)
                .finish()
        };
        let mut response = self
            .send(&Method::GET, &url, None)
            .inspect_err(|error| error!(%error, %url, "Spotify request failed"))
            .ok()?;
        if response.status() == StatusCode::NO_CONTENT {
            return None;
        }
        response
            .body_mut()
            .read_json()
            .inspect_err(|error| error!(%error, %url, "Failed to decode Spotify response"))
            .ok()
    }

    fn run_request(&mut self, method: &Method, path: &str, json: Option<&str>) {
        if let Err(err) = self.send(method, path, json) {
            error!(%err, %path, "Spotify request failed");
        }
    }

    fn new(client_id: String, cache_path: PathBuf, agent: Agent) -> ClientResult<Self> {
        let token =
            read_token_cache(&cache_path).map_or_else(|| prompt_for_token(&client_id, &agent), Ok)?;
        write_cache(&cache_path, &token)?;
        Ok(Self {
            client_id,
            cache_path,
            token,
            http: agent,
        })
    }
}

fn random_token<const N: usize>() -> Result<String, getrandom::Error> {
    let mut bytes = [0; N];
    getrandom::fill(&mut bytes)?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

type ClientResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn client_error(message: &'static str) -> Box<dyn Error + Send + Sync> {
    io::Error::other(message).into()
}

fn config_path(file: &str) -> PathBuf {
    config::directory().join(file)
}

#[derive(Deserialize)]
struct TracksRef {
    total: u32,
}

#[derive(Deserialize)]
struct Page<T> {
    #[serde(alias = "content")]
    items: Vec<T>,
}

const RATING_PLAYLISTS: [&str; 10] = [
    "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

type PlaylistCache = HashMap<PlaylistId, (String, PlaylistTracks)>;

pub struct SpotifyBackend {
    commands: Sender<Update<SpotifyClient>>,
}

impl SpotifyBackend {
    /// Starts the authenticated Spotify client and its polling worker.
    ///
    /// # Panics
    ///
    /// Panics when configuration or Spotify authentication cannot be initialized.
    pub fn new(config: &mut Config, updater: &AppUpdater, background: Background) -> Self {
        fs::create_dir_all(config::directory()).expect("Failed to create Cantus config directory");
        let client_id = config.spotify_client_id.take().expect(
            "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
        );
        let client = SpotifyClient::new(
            client_id,
            config_path(SPOTIFY_TOKEN_CACHE),
            background.http.clone(),
        )
        .expect("Failed to initialize Spotify client");
        let (commands, receiver) = mpsc::channel();
        let worker = SpotifyWorker {
            client,
            updater: updater.clone(),
            background,
            current_context: None,
            playlist_targets: mem::take(&mut config.playlists),
            playlist_cache: read_cache(&config_path(PLAYLIST_TRACKS_CACHE)).unwrap_or_default(),
            ratings_enabled: config.ratings_enabled,
        };
        spawn(move || worker.run(&receiver));
        Self { commands }
    }

    pub fn skip(&self, forward: bool, count: usize) {
        send_update(&self.commands, move |client| {
            let path = if forward {
                "me/player/next"
            } else {
                "me/player/previous"
            };
            for _ in 0..count {
                client.run_request(&Method::POST, path, None);
            }
        });
    }

    pub fn set_playing(&self, playing: bool) {
        let action = if playing { "play" } else { "pause" };
        self.request(Method::PUT, format!("me/player/{action}"));
    }

    /// Adds or removes the track from the given playlists, and from the user's Liked Songs when its liked state differs from `liked`.
    pub fn update_library(
        &self,
        track_id: TrackId,
        changes: Vec<(PlaylistId, bool)>,
        liked: Option<bool>,
    ) {
        send_update(&self.commands, move |client| {
            let uri = format!("spotify:track:{track_id}");
            for (playlist_id, add) in changes {
                let (method, body) = if add {
                    (Method::POST, json!({ "uris": [&uri] }))
                } else {
                    (Method::DELETE, json!({ "items": [{ "uri": &uri }] }))
                };
                let path = format!("playlists/{playlist_id}/items");
                client.run_request(&method, &path, Some(&body.to_string()));
            }
            if let Some(should_like) = liked
                && let Some([liked]) =
                    client.api_json_payload::<[bool; 1]>("me/library/contains", &[("uris", &uri)])
                && liked != should_like
            {
                let method = if should_like { Method::PUT } else { Method::DELETE };
                let path = form_urlencoded::Serializer::new("me/library?".to_owned())
                    .append_pair("uris", &uri)
                    .finish();
                client.run_request(&method, &path, None);
            }
        });
    }

    fn request(&self, method: Method, path: impl Into<String>) {
        let path = path.into();
        send_update(&self.commands, move |client| {
            client.run_request(&method, &path, None);
        });
    }

    pub fn player_parameter(&self, action: &str, parameter: &str, value: impl Display) {
        self.request(Method::PUT, format!("me/player/{action}?{parameter}={value}"));
    }
}

struct SpotifyWorker {
    client: SpotifyClient,
    updater: AppUpdater,
    background: Background,
    current_context: Option<String>,
    playlist_targets: ArrayVec<String, MAX_PILL_PLAYLIST_ICONS>,
    playlist_cache: PlaylistCache,
    ratings_enabled: bool,
}

impl SpotifyWorker {
    fn run(mut self, commands: &Receiver<Update<SpotifyClient>>) {
        let mut next_playback = Instant::now();
        let mut next_queue = Instant::now();
        let mut next_playlists = Instant::now() + Duration::from_secs(1);
        let mut context_changed = false;
        loop {
            let now = Instant::now();
            if now >= next_playback {
                if self.poll_playback() {
                    context_changed = true;
                    next_queue = now;
                }
                next_playback = Instant::now() + Duration::from_secs(1);
            }
            if now >= next_queue {
                let succeeded = self.poll_queue(context_changed).is_some();
                context_changed &= !succeeded;
                let retry = if succeeded { 15 } else { 1 };
                next_queue = Instant::now() + Duration::from_secs(retry);
            }
            if now >= next_playlists {
                let _ = self.poll_playlists();
                next_playlists = Instant::now() + Duration::from_secs(20);
            }
            let timeout = next_playback
                .min(next_queue)
                .min(next_playlists)
                .saturating_duration_since(Instant::now());
            match commands.recv_timeout(timeout) {
                Ok(command) => command(&mut self.client),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    }

    fn poll_playback(&mut self) -> bool {
        let Some(mut current_playback) = self
            .client
            .api_json_payload::<CurrentPlaybackContext>("me/player", &[])
        else {
            return false;
        };

        let now = Instant::now();
        let new_context = current_playback.context.take().map(|context| context.uri);
        let context_changed = self.current_context != new_context;
        if context_changed {
            self.current_context = new_context;
        }
        send_update(&self.updater, move |app| {
            let state = &mut app.playback;
            state.volume = current_playback.device.volume_percent;
            if now < state.timeline.hold_until {
                return;
            }
            let previous_index = state.timeline.index;
            if let Some(track) = current_playback.item.and_then(PlaybackItem::into_track) {
                state.timeline.index = track_index(&state.queue, track.id, &track.name).unwrap_or(0);
            }
            if current_playback.is_playing && !state.playing {
                app.render.last_toggle_time = app.render.start_time.elapsed().as_secs_f32();
            }
            let server = current_playback.progress_ms.unwrap_or_default() as f32
                + now.elapsed().as_millis() as f32 * f32::from(current_playback.is_playing);
            let local = state.timeline.estimated_position(state.playing);
            let discontinuity = server - local;
            if previous_index != state.timeline.index
                || state.playing != current_playback.is_playing
                || discontinuity.abs() > 2_000.0
            {
                state.timeline.position_ms = server;
                state.timeline.rate = 1.0;
            } else {
                state.timeline.position_ms = local;
                state.timeline.rate = (1.0 + discontinuity / 5_000.0).clamp(0.95, 1.05);
            }
            state.timeline.observed_at = now;
            state.playing = current_playback.is_playing;
        });
        context_changed
    }

    fn poll_queue(&mut self, context_changed: bool) -> Option<()> {
        let q = self
            .client
            .api_json_payload::<CurrentUserQueue>("me/player/queue", &[])?;
        let currently_playing = q.currently_playing?.into_track()?;
        let current_track_id = currently_playing.id;
        let mut new_queue = vec![currently_playing];
        new_queue.extend(q.queue.into_iter().filter_map(PlaybackItem::into_track));
        let background = self.background.clone();

        send_update(&self.updater, move |app| {
            app.playback
                .replace_queue(new_queue, current_track_id, context_changed);
            let now = Instant::now();
            let feature_ids = app
                .playback
                .queue
                .iter_mut()
                .filter_map(|track| {
                    let id = track.id.filter(|_| track.runtime.audio_features.request(now))?;
                    Some(id)
                })
                .collect::<Vec<_>>();
            if !feature_ids.is_empty() && !fetch_audio_features(&background, feature_ids.clone()) {
                warn!("Discarded audio-feature request after workers stopped");
                for track in &mut app.playback.queue {
                    if track.id.is_some_and(|id| feature_ids.contains(&id)) {
                        track.runtime.audio_features = Fetch::Missing(now);
                    }
                }
            }
            app.refresh_art();
        });
        Some(())
    }

    fn poll_playlists(&mut self) -> Option<()> {
        let playlists = self
            .client
            .api_json_payload::<Page<Option<Playlist>>>("me/playlists", &[("limit", "50")])?;

        let mut cache_changed = false;
        let mut wanted = HashSet::new();
        let mut updates = Vec::new();
        for playlist in playlists.items.into_iter().flatten() {
            let rating_index = RATING_PLAYLISTS
                .iter()
                .position(|&name| name == playlist.name)
                .filter(|_| self.ratings_enabled)
                .map(|index| index as u8);
            if !self.playlist_targets.contains(&playlist.name) && rating_index.is_none() {
                continue;
            }
            wanted.insert(playlist.id);
            let tracks = if let Some((_, tracks)) = self
                .playlist_cache
                .get(&playlist.id)
                .filter(|(snapshot, _)| snapshot == &playlist.snapshot_id)
            {
                Arc::clone(tracks)
            } else {
                let Some(tracks) = fetch_playlist_tracks(&mut self.client, &playlist) else {
                    continue;
                };
                self.playlist_cache
                    .insert(playlist.id, (playlist.snapshot_id, Arc::clone(&tracks)));
                cache_changed = true;
                tracks
            };
            updates.push(CondensedPlaylist {
                id: playlist.id,
                name: playlist.name,
                image_url: playlist.image,
                art: ArtState::default(),
                tracks,
                rating_index,
            });
        }
        let cached = self.playlist_cache.len();
        self.playlist_cache.retain(|id, _| wanted.contains(id));
        cache_changed |= cached != self.playlist_cache.len();
        send_update(&self.updater, move |app| {
            let previous = mem::take(&mut app.playback.playlists);
            for playlist in &mut updates {
                if let Some(old) = previous
                    .iter()
                    .find(|old| old.id == playlist.id && old.image_url == playlist.image_url)
                {
                    playlist.art = old.art.clone();
                }
            }
            updates.sort_unstable_by(|a, b| a.name.cmp(&b.name));
            app.playback.playlists = updates;
            app.refresh_art();
        });
        if cache_changed
            && let Err(err) = write_cache(&config_path(PLAYLIST_TRACKS_CACHE), &self.playlist_cache)
        {
            warn!("Failed to persist playlist cache: {err}");
        }
        Some(())
    }
}

fn fetch_audio_features(background: &Background, ids: Vec<TrackId>) -> bool {
    let http = background.http.clone();
    background.submit(move || {
        let features = resolve_audio_features(&http, &ids);
        Enrichment::AudioFeatures(ids, features)
    })
}

fn resolve_audio_features(http: &Agent, track_ids: &[TrackId]) -> HashMap<TrackId, AudioFeatures> {
    let Ok(page) = request_json::<Page<ReccoTrack>>(
        http.get(RECCO_TRACK_URL)
            .query("size", "50")
            .query_pairs(track_ids.iter().map(|id| ("ids", id.as_str()))),
    )
    .inspect_err(|err| warn!("Failed to resolve Spotify tracks with ReccoBeats: {err}")) else {
        return HashMap::new();
    };
    let mut resolved = HashMap::new();
    for recco_track in page.items {
        let Some(spotify_id) = recco_track
            .href
            .rsplit('/')
            .next()
            .and_then(|id| id.parse::<TrackId>().ok())
        else {
            continue;
        };
        let url = format!("{RECCO_TRACK_URL}/{}/audio-features", recco_track.id);
        if let Ok(features) = request_json::<AudioFeatures>(http.get(url)).inspect_err(|err| {
            warn!("Failed to fetch ReccoBeats features for {spotify_id}: {err}");
        }) {
            resolved.insert(spotify_id, features.normalized());
        }
    }
    resolved
}

fn fetch_playlist_tracks(client: &mut SpotifyClient, playlist: &Playlist) -> Option<PlaylistTracks> {
    const FIELDS: &str = "href,limit,offset,total,items(is_local,item(id))";
    const PAGE_SIZE: u32 = 50;
    let total = playlist.items.total;
    let num_pages = total.div_ceil(PAGE_SIZE);
    let mut tracks = HashSet::with_capacity(total as usize);
    info!("Fetching {num_pages} pages from playlist {}", playlist.name);

    let limit = PAGE_SIZE.to_string();
    for page in 0..num_pages {
        let offset = (page * PAGE_SIZE).to_string();
        let page = client.api_json_payload::<Page<Option<PlaylistItem>>>(
            &format!("playlists/{}/items", playlist.id),
            &[("fields", FIELDS), ("limit", &limit), ("offset", &offset)],
        )?;
        tracks.extend(page.items.into_iter().flatten().filter_map(|item| item.item?.id));
    }
    Some(Arc::new(tracks))
}

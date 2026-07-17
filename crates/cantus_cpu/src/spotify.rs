use crate::{
    AppUpdater, MAX_HISTORY_TRACKS, Update,
    art::{self, ArtState},
    config::{self, Config},
    model::{CondensedPlaylist, PlaylistId, PlaylistTracks, Track, TrackId, deserialize_images},
    send_update,
};
use arrayvec::{ArrayString, ArrayVec};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use cantus_shared::{AudioFeatures, MAX_PILL_PLAYLIST_ICONS};
use jiff::Timestamp;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
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
use tracing::{error, info, warn};
use ureq::{
    Agent, Body,
    http::{Method, Request, Response, StatusCode},
};

const API_BASE: &str = "https://api.spotify.com/v1";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const SPOTIFY_TOKEN_CACHE: &str = "spotify_cache.json";
const PLAYLIST_TRACKS_CACHE: &str = "cantus_playlist_tracks.json";
const RECCO_TRACK_URL: &str = "https://api.reccobeats.com/v1/track";
const SCOPES: &str = "\
user-read-playback-state user-modify-playback-state user-read-currently-playing \
playlist-read-private playlist-read-collaborative playlist-modify-private playlist-modify-public \
user-library-read user-library-modify";

#[derive(Deserialize)]
struct ReccoTrack {
    id: String,
    href: String,
}

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
    items: Option<TracksRef>,
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
    Track(Track),
    #[serde(other)]
    Other,
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

fn write_json(path: &Path, value: &impl Serialize) -> ClientResult<()> {
    fs::write(path, serde_json::to_vec(value)?).map_err(Into::into)
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

    let query = request_line
        .split_whitespace()
        .nth(1)
        .and_then(|target| target.split_once('?'))
        .map(|pair| pair.1)
        .ok_or_else(|| client_error("invalid Spotify authorization response"))?;
    let mut params = form_urlencoded::parse(query.as_bytes());
    let mut param = |name| {
        params
            .find_map(|(key, value)| (key == name).then(|| value.into_owned()))
            .ok_or_else(|| client_error("invalid Spotify authorization response"))
    };
    let code = param("code")?;
    let actual_state = param("state")?;
    if actual_state != expected_state {
        return Err(client_error("Spotify authorization state did not match"));
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
            self.token = self.refetch_token()?;
            write_json(&self.cache_path, &self.token)?;
        }
        Ok(format!("Bearer {}", self.token.access_token))
    }

    fn refetch_token(&self) -> ClientResult<OAuthCredentials> {
        let Some(refresh_token) = &self.token.refresh_token else {
            warn!("No refresh token available");
            return Err(client_error("no Spotify refresh token available"));
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
    fn api_json_payload<T: DeserializeOwned>(
        &mut self,
        url: &str,
        payload: &[(&str, &str)],
        label: &str,
    ) -> Option<T> {
        let authorization = self.authorization().ok()?;
        let response = self
            .http
            .get(format!("{API_BASE}/{url}"))
            .header("authorization", authorization)
            .query_pairs(payload.iter().copied())
            .call()
            .inspect_err(|e| error!("Failed to fetch {label}: {e}"))
            .ok()?;
        if response.status() == StatusCode::NO_CONTENT {
            return None;
        }
        response_json(response)
            .inspect_err(|e| error!("Failed to decode {label}: {e}"))
            .ok()
    }

    fn run_request(&mut self, method: &Method, path: &str, json: Option<&str>) {
        let result: ClientResult<()> = (|| {
            let request = Request::builder()
                .method(method.clone())
                .uri(format!("{API_BASE}/{path}"))
                .header("authorization", self.authorization()?);
            let request = match json {
                Some(json) => request
                    .header("content-type", "application/json; charset=utf-8")
                    .body(json)?,
                None => request.body("")?,
            };
            self.http.run(request)?;
            Ok(())
        })();
        if let Err(err) = result {
            error!(%err, %path, "Spotify request failed");
        }
    }

    fn new(client_id: String, scopes: &str, cache_path: PathBuf) -> ClientResult<Self> {
        let agent = Agent::new_with_defaults();
        let token = if let Some(token) = read_token_cache(&cache_path, scopes)? {
            token
        } else {
            let state = random_token::<16>()?;
            let (verifier, url) = get_authorize_url(&client_id, scopes, &state)?;
            prompt_for_token(&url, &client_id, &verifier, &agent, &state)?
        };
        write_json(&cache_path, &token)?;
        Ok(Self {
            client_id,
            cache_path,
            token,
            http: agent,
        })
    }
}

fn get_authorize_url(client_id: &str, scopes: &str, state: &str) -> ClientResult<(String, String)> {
    let verifier = random_token::<32>()?;

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
    Ok((
        verifier,
        format!("https://accounts.spotify.com/authorize?{query}"),
    ))
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

fn boxed_error(error: impl Error + Send + Sync + 'static) -> Box<dyn Error + Send + Sync> {
    Box::new(error)
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

type PlaylistCache = HashMap<PlaylistId, (ArrayString<32>, PlaylistTracks)>;

pub struct SpotifyBackend {
    commands: Sender<Update<SpotifyClient>>,
    updater: AppUpdater,
    http: Agent,
}

impl SpotifyBackend {
    pub fn new(config: &Config, updater: AppUpdater) -> Self {
        fs::create_dir_all(config::directory()).expect("Failed to create Cantus config directory");
        let client_id = config.spotify_client_id.clone().expect(
            "Spotify client ID not set, set it in the config file under key `spotify_client_id`.",
        );
        let client = SpotifyClient::new(client_id, SCOPES, config_path(SPOTIFY_TOKEN_CACHE))
            .expect("Failed to initialize Spotify client");
        let (commands, receiver) = mpsc::channel();
        let http = client.http.clone();
        let features = audio_features_backend(http.clone(), updater.clone());
        let now = Instant::now();
        let worker = SpotifyWorker {
            client,
            updater: updater.clone(),
            features,
            current_context: None,
            context_updated: false,
            last_grabbed_queue: now.checked_sub(Duration::from_mins(1)).unwrap_or(now),
            playlist_targets: config.playlists.clone(),
            playlist_snapshots: HashMap::new(),
            playlist_cache: fs::read(config_path(PLAYLIST_TRACKS_CACHE))
                .ok()
                .and_then(|bytes| {
                    serde_json::from_slice(&bytes)
                        .inspect_err(|err| warn!("Failed to parse playlist cache: {err}"))
                        .ok()
                })
                .unwrap_or_default(),
            ratings_enabled: config.ratings_enabled,
        };
        spawn(move || worker.run(&receiver));
        Self {
            commands,
            updater,
            http,
        }
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

    pub fn update_library(
        &self,
        track_id: TrackId,
        changes: Vec<(PlaylistId, bool)>,
        liked: Option<bool>,
    ) {
        send_update(&self.commands, move |client| {
            for (playlist_id, add) in changes {
                set_playlist_membership(client, playlist_id, track_id, add);
            }
            if let Some(liked) = liked {
                set_liked_state(client, track_id, liked);
            }
        });
    }

    pub fn download_image(&self, url: String) {
        let updater = self.updater.clone();
        let http = self.http.clone();
        spawn(move || {
            let result = http
                .get(&url)
                .call()
                .map_err(boxed_error)
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
    }

    fn request(&self, method: Method, path: impl Into<String>) {
        let path = path.into();
        send_update(&self.commands, move |client| {
            client.run_request(&method, &path, None);
        });
    }

    pub fn player_parameter(&self, action: &str, parameter: &str, value: impl Display) {
        self.request(
            Method::PUT,
            format!("me/player/{action}?{parameter}={value}"),
        );
    }
}

struct SpotifyWorker {
    client: SpotifyClient,
    updater: AppUpdater,
    features: Sender<Vec<TrackId>>,
    current_context: Option<String>,
    context_updated: bool,
    last_grabbed_queue: Instant,
    playlist_targets: ArrayVec<String, MAX_PILL_PLAYLIST_ICONS>,
    playlist_snapshots: HashMap<PlaylistId, ArrayString<32>>,
    playlist_cache: PlaylistCache,
    ratings_enabled: bool,
}

impl SpotifyWorker {
    fn run(mut self, commands: &Receiver<Update<SpotifyClient>>) {
        let mut next_playback = Instant::now();
        let mut next_playlists = Instant::now() + Duration::from_secs(1);
        loop {
            let now = Instant::now();
            if now >= next_playback {
                self.poll_playback();
                self.poll_queue();
                next_playback = Instant::now() + Duration::from_secs(1);
            }
            if now >= next_playlists {
                self.poll_playlists();
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
    client.run_request(&method, &path, Some(&body.to_string()));
}

fn set_liked_state(client: &mut SpotifyClient, track_id: TrackId, should_like: bool) {
    let track_uri = format!("spotify:track:{track_id}");
    if let Some([liked]) = client.api_json_payload::<[bool; 1]>(
        "me/library/contains",
        &[("uris", &track_uri)],
        "liked state",
    ) && liked != should_like
    {
        let method = if should_like {
            Method::PUT
        } else {
            Method::DELETE
        };
        let path = form_urlencoded::Serializer::new(String::from("me/library?"))
            .append_pair("uris", &track_uri)
            .finish();
        client.run_request(&method, &path, None);
    }
}

fn track_index(queue: &[Track], id: Option<TrackId>, name: &str) -> Option<usize> {
    id.and_then(|track_id| queue.iter().position(|track| track.id == Some(track_id)))
        .or_else(|| queue.iter().position(|track| track.name == name))
}

impl SpotifyWorker {
    fn poll_playback(&mut self) {
        // https://developer.spotify.com/documentation/web-api/reference/get-information-about-the-users-current-playback
        let Some(current_playback) =
            self.client
                .api_json_payload::<CurrentPlaybackContext>("me/player", &[], "playback")
        else {
            return;
        };

        let now = Instant::now();
        let new_context = current_playback.context.as_ref().map(|c| &c.uri);
        let queue_deadline = now.checked_sub(Duration::from_mins(1)).unwrap_or(now);
        if self.current_context.as_ref() != new_context {
            self.context_updated = true;
            self.current_context = new_context.map(String::from);
            self.last_grabbed_queue = queue_deadline;
        }
        send_update(&self.updater, move |app| {
            let state = &mut app.playback;
            if let Some(PlaybackItem::Track(track)) = current_playback.item {
                state.queue_index = track_index(&state.queue, track.id, &track.name).unwrap_or(0);
            }

            state.volume = current_playback.device.volume_percent;
            if now >= state.last_interaction {
                if current_playback.is_playing && !state.playing {
                    app.render.last_toggle_playing = now;
                }
                state.playing = current_playback.is_playing;
                state.update_progress(current_playback.progress_ms.unwrap_or_default(), now);
            } else {
                state.last_progress_update = now;
            }
        });
    }
    fn poll_queue(&mut self) {
        let now = Instant::now();
        if now < self.last_grabbed_queue + Duration::from_secs(15) {
            return;
        }

        // https://developer.spotify.com/documentation/web-api/reference/get-queue
        let Some(q) =
            self.client
                .api_json_payload::<CurrentUserQueue>("me/player/queue", &[], "queue")
        else {
            return;
        };
        let Some(PlaybackItem::Track(currently_playing)) = q.currently_playing else {
            // Nothing is currently playing
            return;
        };
        let current_track_id = currently_playing.id;
        let mut new_queue = q
            .queue
            .into_iter()
            .filter_map(|item| match item {
                PlaybackItem::Track(track) => Some(track),
                PlaybackItem::Other => None,
            })
            .collect::<Vec<_>>();
        new_queue.insert(0, currently_playing);
        let feature_ids = new_queue
            .iter()
            .filter_map(|track| track.id)
            .collect::<Vec<_>>();

        let context_updated = self.context_updated;
        self.context_updated = false;
        self.last_grabbed_queue = Instant::now();
        send_update(&self.updater, move |app| {
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

        if self.features.send(feature_ids).is_err() {
            warn!("Discarded audio-feature request after its worker stopped");
        }
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
                .filter_map(|id| cache.get(&id).copied().map(|value| (id, value)))
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
    cache: &mut HashMap<TrackId, AudioFeatures>,
) {
    let mut missing = track_ids
        .iter()
        .copied()
        .filter(|id| !cache.contains_key(id))
        .collect::<HashSet<_>>();

    if missing.is_empty() {
        return;
    }
    let Ok(page) = http
        .get(RECCO_TRACK_URL)
        .query("size", "50")
        .query_pairs(missing.iter().map(|id| ("ids", id.as_str())))
        .call()
        .map_err(boxed_error)
        .and_then(response_json::<Page<ReccoTrack>>)
        .inspect_err(|err| warn!("Failed to resolve Spotify tracks with ReccoBeats: {err}"))
    else {
        return;
    };
    for recco_track in page.items {
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
            .map_err(boxed_error)
            .and_then(response_json)
            .inspect_err(|err| warn!("Failed to fetch ReccoBeats features for {spotify_id}: {err}"))
        {
            cache.insert(spotify_id, features);
        }
    }
}

impl SpotifyWorker {
    fn poll_playlists(&mut self) {
        // https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists
        let Some(playlists) = self.client.api_json_payload::<Page<Option<Playlist>>>(
            "me/playlists",
            &[("limit", "50")],
            "playlists",
        ) else {
            return;
        };

        for playlist in playlists.items.into_iter().flatten() {
            let rating_index = rating_index(self.ratings_enabled, &playlist.name);
            if !self.playlist_targets.contains(&playlist.name) && rating_index.is_none() {
                continue;
            }
            if self.playlist_snapshots.get(&playlist.id) == Some(&playlist.snapshot_id) {
                continue;
            }
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
                if let Err(err) =
                    write_json(&config_path(PLAYLIST_TRACKS_CACHE), &self.playlist_cache)
                {
                    warn!("Failed to persist playlist cache: {err}");
                }
                tracks
            };
            self.playlist_snapshots
                .insert(playlist.id, playlist.snapshot_id);
            send_update(&self.updater, move |app| {
                let mut update = CondensedPlaylist {
                    id: playlist.id,
                    name: playlist.name,
                    image_url: playlist.image,
                    tracks,
                    rating_index,
                    art: ArtState::Missing,
                };
                let playlists = &mut app.playback.playlists;
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
                playlists.sort_unstable_by(|a, b| a.name.cmp(&b.name));
            });
        }
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
    const FIELDS: &str = "href,limit,offset,total,items(is_local,item(id))";
    let chunk_size = 50;
    let total = playlist.items.as_ref()?.total;
    let num_pages = total.div_ceil(chunk_size);
    let mut tracks = HashSet::with_capacity(total as usize);
    info!("Fetching {num_pages} pages from playlist {}", playlist.name);

    for page in 0..num_pages {
        let limit = chunk_size.to_string();
        let offset = (page * chunk_size).to_string();
        let page = client.api_json_payload::<Page<Option<PlaylistItem>>>(
            &format!("playlists/{}/items", playlist.id),
            &[("fields", FIELDS), ("limit", &limit), ("offset", &offset)],
            "playlist page",
        )?;
        tracks.extend(
            page.items
                .into_iter()
                .flatten()
                .filter_map(|item| item.item?.id),
        );
    }
    Some(Arc::new(tracks))
}

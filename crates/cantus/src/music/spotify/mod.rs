use super::{
    CondensedPlaylist, LyricSegment, MusicResult, MusicService, PlaybackCommand, PlaylistId, Track,
    TrackId, TrackRuntime,
};
use crate::{
    app::{
        AppUpdater, Background, Fetch,
        config::{self, Config},
        enrichment::ArtState,
        send_update,
    },
    render::track::{AudioFeatures, MAX_PILL_PLAYLIST_ICONS},
};
use arrayvec::ArrayVec;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use flate2::{Compression, write::GzEncoder};
use librespot_protocol::{
    connect::{
        Capabilities, Cluster, Device as ConnectDevice, DeviceInfo, MemberType, PutStateReason,
        PutStateRequest, SetVolumeCommand,
    },
    devices::DeviceType,
    extended_metadata::{BatchedEntityRequest, BatchedExtensionResponse, EntityRequest, ExtensionQuery},
    extension_kind::ExtensionKind,
    metadata,
    player::{ContextPlayerOptions, PlayerState, ProvidedTrack, Suppressions},
    playlist4_external::{
        Add, Delta, Item, ListAttributes, ListChanges, Op, Rem, SelectedListContent, op,
    },
};
use protobuf::{EnumOrUnknown, Message as _, MessageField};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::Write as _,
    fs,
    io::{self, Read, Write},
    mem,
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
    str,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::spawn,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tracing::{error, info, warn};
use ureq::{
    Agent,
    http::{Method, Request},
};

mod dealer;
mod session;

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
const SPOTIFY_SESSION_CACHE: &str = "spotify_session.json";
const PLAYLIST_TRACKS_CACHE: &str = "cantus_playlist_tracks.json";
const RECCO_FEATURES_URL: &str = "https://api.reccobeats.com/v1/audio-features";
const SCOPES: &str = "streaming app-remote-control";
type PlaylistTracks = Arc<HashSet<TrackId>>;

#[derive(Deserialize)]
struct ReccoFeatures {
    href: String,
    #[serde(flatten)]
    features: AudioFeatures,
}

#[derive(Deserialize)]
struct OAuthToken {
    access_token: String,
}

const REDIRECT_ADDR: &str = "127.0.0.1:8898";
const REDIRECT_URI: &str = "http://127.0.0.1:8898/login";

#[derive(Clone)]
struct SpotifyClient {
    session: Arc<Mutex<session::Session>>,
    http: Agent,
    metadata_http: Agent,
}

fn write_cache(path: &Path, value: &impl Serialize) -> ClientResult<()> {
    serde_json::to_writer(fs::File::create(path)?, value)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}

fn read_cache<T: DeserializeOwned>(path: &Path) -> Option<T> {
    serde_json::from_slice(&fs::read(path).ok()?)
        .inspect_err(|err| warn!(%err, ?path, "Failed to parse cache"))
        .ok()
}

/// Runs the interactive PKCE authorization flow in the user's browser.
fn prompt_for_token(http: &Agent) -> ClientResult<String> {
    let verifier = random_token::<32>()?;
    let expected_state = random_token::<16>()?;
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let query = form_urlencoded::Serializer::new(String::new())
        .extend_pairs([
            ("client_id", CLIENT_ID),
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
            ("client_id", CLIENT_ID),
            ("code_verifier", &verifier),
        ])?
        .into_body()
        .read_to_string()?;
    Ok(serde_json::from_str::<OAuthToken>(&response)?.access_token)
}

impl SpotifyClient {
    fn new(http: Agent) -> ClientResult<Self> {
        let cache = config_path(SPOTIFY_SESSION_CACHE);
        let session = session::login(&http, "", &cache).or_else(|_| {
            let token = prompt_for_token(&http)?;
            session::login(&http, &token, &cache)
        })?;
        info!(
            username = %session.username,
            device_id = %session.device_id,
            spclient = %session.spclient,
            dealer = %session.dealer,
            "Authenticated Spotify session"
        );
        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            http,
            metadata_http: Agent::config_builder()
                .timeout_global(Some(Duration::from_secs(3)))
                .build()
                .into(),
        })
    }

    fn request(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        body: Vec<u8>,
    ) -> ClientResult<Vec<u8>> {
        self.request_on(&self.http, method, path, headers, body)
    }

    fn request_proto<T: protobuf::Message>(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        message: &T,
    ) -> ClientResult<Vec<u8>> {
        self.request(method, path, headers, message.write_to_bytes()?)
    }

    fn request_metadata<T: protobuf::Message>(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        message: &T,
    ) -> ClientResult<Vec<u8>> {
        self.request_on(
            &self.metadata_http,
            method,
            path,
            headers,
            message.write_to_bytes()?,
        )
    }

    fn request_on(
        &self,
        http: &Agent,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        body: Vec<u8>,
    ) -> ClientResult<Vec<u8>> {
        let (token, endpoint, client_token) = self.with_session(|session| {
            Ok((
                session.authorization(http)?.to_owned(),
                session.spclient.clone(),
                session.client_token.clone(),
            ))
        })?;
        let mut request = Request::builder()
            .method(method)
            .uri(format!("https://{endpoint}/{}", path.trim_start_matches('/')))
            .header("authorization", format!("Bearer {token}"))
            .header("client-token", client_token);
        for &(name, value) in headers {
            request = request.header(name, value);
        }
        Ok(http.run(request.body(body)?)?.body_mut().read_to_vec()?)
    }

    fn with_session<T>(
        &self,
        work: impl FnOnce(&mut session::Session) -> ClientResult<T>,
    ) -> ClientResult<T> {
        let mut session = self
            .session
            .lock()
            .map_err(|_| io::Error::other("Spotify session lock poisoned"))?;
        work(&mut session)
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
struct ReccoResponse {
    #[serde(alias = "content")]
    items: Vec<ReccoFeatures>,
}

const RATING_PLAYLISTS: [&str; 10] = [
    "0.5", "1.0", "1.5", "2.0", "2.5", "3.0", "3.5", "4.0", "4.5", "5.0",
];

type PlaylistCache = HashMap<PlaylistId, (Vec<u8>, PlaylistTracks)>;

#[derive(Deserialize)]
struct SpotifyLyrics {
    lyrics: SpotifyLyricsInner,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpotifyLyricsInner {
    lines: Vec<SpotifyLyricLine>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpotifyLyricLine {
    start_time_ms: String,
    words: String,
}

pub struct SpotifyBackend {
    events: Sender<WorkerEvent>,
    client: SpotifyClient,
}

enum WorkerEvent {
    Command(PlaybackCommand),
    Connected(String),
    Cluster(Cluster),
    Metadata(HashMap<String, metadata::Track>),
    PlaylistsChanged,
}

#[derive(Clone, Copy)]
enum PlayerCommand {
    Playing(bool),
    Seek(u32),
    Skip(bool),
}

impl SpotifyBackend {
    /// Starts the authenticated Spotify client and its event worker.
    ///
    /// # Panics
    ///
    /// Panics when configuration or Spotify authentication cannot be initialized.
    pub fn new(config: &mut Config, updater: &AppUpdater, background: Background) -> Self {
        fs::create_dir_all(config::directory()).expect("Failed to create Cantus config directory");
        let client =
            SpotifyClient::new(background.http.clone()).expect("Failed to initialize Spotify client");
        let (events, receiver) = mpsc::channel();
        dealer::connect(client.clone(), events.clone());
        let worker = SpotifyWorker {
            client: client.clone(),
            events: events.clone(),
            updater: updater.clone(),
            background,
            connection_id: None,
            active_device: None,
            playlist_targets: mem::take(&mut config.playlists),
            playlist_cache: read_cache(&config_path(PLAYLIST_TRACKS_CACHE)).unwrap_or_default(),
            track_metadata: HashMap::new(),
            metadata_pending: HashSet::new(),
            queue_state: None,
            ratings_enabled: config.ratings_enabled,
        };
        spawn(move || worker.run(&receiver));
        Self { events, client }
    }
}

impl MusicService for SpotifyBackend {
    fn command(&self, command: PlaybackCommand) {
        if self.events.send(WorkerEvent::Command(command)).is_err() {
            warn!("Discarded music command after Spotify worker stopped");
        }
    }

    fn lyrics(&self, track_id: TrackId) -> MusicResult<Vec<LyricSegment>> {
        let response = match self.client.request(
            Method::GET,
            &format!("color-lyrics/v2/track/{track_id}?format=json&market=from_token"),
            &[("accept", "application/json"), ("app-platform", "WebPlayer")],
            Vec::new(),
        ) {
            Ok(response) => response,
            Err(error)
                if error
                    .downcast_ref::<ureq::Error>()
                    .is_some_and(|error| matches!(error, ureq::Error::StatusCode(404))) =>
            {
                return Ok(Vec::new());
            }
            Err(error) => return Err(error),
        };
        let lines = serde_json::from_slice::<SpotifyLyrics>(&response)?.lyrics.lines;
        Ok(lines
            .iter()
            .enumerate()
            .filter_map(|(index, line)| {
                let start_ms = line.start_time_ms.parse().ok()?;
                Some(LyricSegment {
                    start_ms,
                    end_ms: lines
                        .get(index + 1)
                        .and_then(|next| next.start_time_ms.parse().ok())
                        .unwrap_or(start_ms + 1_000.0),
                    text: line.words.clone(),
                    lane: 0,
                })
            })
            .collect())
    }
}

struct SpotifyWorker {
    client: SpotifyClient,
    events: Sender<WorkerEvent>,
    updater: AppUpdater,
    background: Background,
    connection_id: Option<String>,
    active_device: Option<String>,
    playlist_targets: ArrayVec<String, MAX_PILL_PLAYLIST_ICONS>,
    playlist_cache: PlaylistCache,
    track_metadata: HashMap<String, metadata::Track>,
    metadata_pending: HashSet<String>,
    queue_state: Option<QueueState>,
    ratings_enabled: bool,
}

struct QueueState {
    tracks: Vec<ProvidedTrack>,
    current: usize,
    duration_ms: Option<u32>,
}

struct PlaybackUpdate {
    playing: bool,
    position_ms: f32,
    rate: f32,
    volume: Option<u8>,
    observed_at: Instant,
}

impl SpotifyWorker {
    fn run(mut self, events: &Receiver<WorkerEvent>) {
        while let Ok(event) = events.recv() {
            self.event(event);
        }
    }

    fn command(&mut self, command: PlaybackCommand) {
        match command {
            PlaybackCommand::SetPlaying(playing) => {
                self.player_command(PlayerCommand::Playing(playing));
            }
            PlaybackCommand::SetVolume(volume) => self.set_volume(volume),
            PlaybackCommand::Seek(position_ms) => {
                self.player_command(PlayerCommand::Seek(position_ms));
            }
            PlaybackCommand::Skip { forward, count } => {
                for _ in 0..count {
                    self.player_command(PlayerCommand::Skip(forward));
                }
            }
            PlaybackCommand::UpdateLibrary {
                track_id,
                playlists,
                liked,
            } => self.update_library(track_id, &playlists, liked),
        }
    }

    fn update_library(
        &mut self,
        track_id: TrackId,
        changes: &[(PlaylistId, bool)],
        liked: Option<bool>,
    ) {
        let uri = format!("spotify:track:{track_id}");
        for &(playlist_id, add) in changes {
            let Some(revision) = self
                .playlist_cache
                .get(&playlist_id)
                .map(|(revision, _)| revision.clone())
            else {
                warn!(%playlist_id, "Spotify playlist is not loaded");
                continue;
            };
            let item = Item {
                uri: Some(uri.clone()),
                ..Default::default()
            };
            let operation = if add {
                Op {
                    kind: Some(op::Kind::ADD.into()),
                    add: MessageField::some(Add {
                        items: vec![item],
                        add_last: Some(true),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            } else {
                Op {
                    kind: Some(op::Kind::REM.into()),
                    rem: MessageField::some(Rem {
                        items: vec![item],
                        items_as_key: Some(true),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            };
            let request = ListChanges {
                base_revision: Some(revision.clone()),
                deltas: vec![Delta {
                    base_version: Some(revision),
                    ops: vec![operation],
                    ..Default::default()
                }],
                want_resulting_revisions: Some(true),
                ..Default::default()
            };
            let result = self.client.request_proto(
                Method::POST,
                &format!("playlist/v2/playlist/{playlist_id}"),
                &[
                    ("content-type", "application/x-protobuf"),
                    (
                        "x-spotify-connection-id",
                        self.connection_id.as_deref().unwrap_or_default(),
                    ),
                ],
                &request,
            );
            if let Err(error) = result {
                error!(%error, %playlist_id, "Failed to update Spotify playlist");
            } else if let Some((_, tracks)) = self.playlist_cache.get_mut(&playlist_id) {
                let tracks = Arc::make_mut(tracks);
                if add {
                    tracks.insert(track_id);
                } else {
                    tracks.remove(&track_id);
                }
            }
        }
        let Some(should_like) = liked else {
            return;
        };
        let username = self
            .client
            .session
            .lock()
            .map(|session| session.username.clone())
            .unwrap_or_default();
        let body = match collection_write(track_id, !should_like) {
            Ok(body) => body,
            Err(error) => {
                error!(%error, %track_id, "Failed to encode Spotify library update");
                return;
            }
        };
        if let Err(error) = self.client.request(
            Method::POST,
            &format!("collection/collection/{username}"),
            &[
                ("content-type", "application/x-protobuf"),
                (
                    "x-spotify-connection-id",
                    self.connection_id.as_deref().unwrap_or_default(),
                ),
            ],
            body,
        ) {
            error!(%error, %track_id, "Failed to update Spotify library");
        }
    }

    fn event(&mut self, event: WorkerEvent) {
        match event {
            WorkerEvent::Command(command) => self.command(command),
            WorkerEvent::Connected(connection_id) => {
                self.connection_id = Some(connection_id);
                match self.register() {
                    Ok(cluster) => self.update_cluster(cluster),
                    Err(error) => error!(%error, "Failed to register Spotify observer"),
                }
                self.refresh_playlists();
            }
            WorkerEvent::Cluster(cluster) => self.update_cluster(cluster),
            WorkerEvent::Metadata(metadata) => {
                if metadata.is_empty() {
                    self.metadata_pending.clear();
                } else {
                    self.metadata_pending.retain(|uri| !metadata.contains_key(uri));
                }
                self.track_metadata.extend(metadata);
                self.publish_queue(None);
            }
            WorkerEvent::PlaylistsChanged => self.refresh_playlists(),
        }
    }

    fn register(&self) -> ClientResult<Cluster> {
        let (device_id, client_id) = {
            let session = self
                .client
                .session
                .lock()
                .map_err(|_| io::Error::other("Spotify session lock poisoned"))?;
            (session.device_id.clone(), CLIENT_ID.to_owned())
        };
        let request = PutStateRequest {
            device: MessageField::some(ConnectDevice {
                device_info: MessageField::some(DeviceInfo {
                    can_play: false,
                    name: "Cantus".into(),
                    capabilities: MessageField::some(Capabilities {
                        can_be_player: false,
                        is_observable: true,
                        needs_full_player_state: true,
                        hidden: true,
                        supports_gzip_pushes: true,
                        supports_playlist_v2: true,
                        supported_types: vec!["audio/track".into(), "audio/episode".into()],
                        ..Default::default()
                    }),
                    device_type: EnumOrUnknown::new(DeviceType::OBSERVER),
                    device_id: device_id.clone(),
                    client_id,
                    ..Default::default()
                }),
                player_state: MessageField::some(PlayerState {
                    session_id: random_token::<16>()?,
                    playback_speed: 1.0,
                    options: MessageField::some(ContextPlayerOptions::default()),
                    suppressions: MessageField::some(Suppressions::default()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            member_type: EnumOrUnknown::new(MemberType::CONNECT_STATE),
            put_state_reason: EnumOrUnknown::new(PutStateReason::NEW_DEVICE),
            client_side_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            ..Default::default()
        };
        let connection = self.connection_id.as_deref().unwrap_or_default();
        let bytes = self.client.request_proto(
            Method::PUT,
            &format!("connect-state/v1/devices/{device_id}"),
            &[
                ("x-spotify-connection-id", connection),
                ("content-type", "application/x-protobuf"),
            ],
            &request,
        )?;
        let cluster = Cluster::parse_from_bytes(&bytes)?;
        Ok(cluster)
    }

    fn update_cluster(&mut self, cluster: Cluster) {
        let Some(player) = cluster.player_state.into_option() else {
            warn!("Spotify cluster update contained no player state");
            return;
        };
        self.active_device =
            (!cluster.active_device_id.is_empty()).then(|| cluster.active_device_id.clone());
        let playing = player.is_playing && !player.is_paused;
        let observed_at = Instant::now();
        let rate = player.playback_speed.max(0.0) as f32;
        let current_position = player.prev_tracks.len();
        let position = player_position(&player, rate);
        let mut provided = player.prev_tracks;
        if let Some(current) = player.track.into_option() {
            provided.push(current);
        }
        provided.extend(player.next_tracks);
        let active_uris = provided
            .iter()
            .map(|track| track.uri.as_str())
            .collect::<HashSet<_>>();
        self.track_metadata
            .retain(|uri, _| active_uris.contains(uri.as_str()));
        self.schedule_metadata(&provided);
        self.queue_state = Some(QueueState {
            tracks: provided,
            current: current_position,
            duration_ms: u32::try_from(player.duration).ok(),
        });
        let volume = self
            .active_device
            .as_ref()
            .and_then(|id| cluster.device.get(id))
            .map(|device| (device.volume.saturating_mul(100) / 65_535) as u8);
        self.publish_queue(Some(PlaybackUpdate {
            playing,
            position_ms: position,
            rate,
            volume,
            observed_at,
        }));
    }

    fn publish_queue(&self, playback: Option<PlaybackUpdate>) {
        let Some(queue_state) = &self.queue_state else {
            return;
        };
        let mut index = 0;
        let mut queue = Vec::with_capacity(queue_state.tracks.len());
        for (provided_index, track) in queue_state.tracks.iter().enumerate() {
            if provided_index == queue_state.current {
                index = queue.len();
            }
            if track.uri.ends_with(":delimiter") {
                continue;
            }
            if let Some(rendered) = track_from_provided(
                track,
                self.track_metadata.get(&track.uri),
                (provided_index == queue_state.current)
                    .then_some(queue_state.duration_ms)
                    .flatten(),
            ) {
                queue.push(rendered);
            }
        }
        let background = self.background.clone();
        send_update(&self.updater, move |app| {
            let state = &mut app.playback;
            let mut old = HashMap::<String, VecDeque<TrackRuntime>>::new();
            for track in mem::take(&mut state.queue) {
                old.entry(track.uri).or_default().push_back(track.runtime);
            }
            let mut queue = queue;
            for track in &mut queue {
                if let Some(runtime) = old.get_mut(&track.uri).and_then(VecDeque::pop_front) {
                    track.runtime = runtime;
                }
            }
            state.queue = queue;
            let index = index.min(state.queue.len().saturating_sub(1));
            if let Some(playback) = playback {
                state.volume = playback.volume;
                state.timeline.observe_server(
                    playback.position_ms,
                    index,
                    playback.rate,
                    playback.observed_at,
                );
                if playback.playing && !state.playing {
                    app.render.last_toggle_time = app.render.start_time.elapsed().as_secs_f32();
                }
                state.playing = playback.playing;
            } else {
                state.timeline.index = index;
            }
            let now = Instant::now();
            let mut feature_ids = app
                .playback
                .queue
                .iter_mut()
                .filter_map(|track| {
                    let id = track.id.filter(|_| track.runtime.audio_features.request(now))?;
                    Some(id)
                })
                .collect::<Vec<_>>();
            feature_ids.sort_unstable();
            feature_ids.dedup();
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
    }

    fn schedule_metadata(&mut self, tracks: &[ProvidedTrack]) {
        let requested = tracks
            .iter()
            .filter(|track| {
                track.uri.starts_with("spotify:track:")
                    && !track.metadata.contains_key("duration")
                    && !self.track_metadata.contains_key(&track.uri)
                    && self.metadata_pending.insert(track.uri.clone())
            })
            .cloned()
            .collect::<Vec<_>>();
        if requested.is_empty() {
            return;
        }
        let client = self.client.clone();
        let sender = self.events.clone();
        spawn(move || {
            let metadata = fetch_track_metadata(&client, &requested);
            let _ = sender.send(WorkerEvent::Metadata(metadata));
        });
    }

    fn player_command(&self, command: PlayerCommand) {
        let (Some(connection), Some(target)) = (&self.connection_id, &self.active_device) else {
            return;
        };
        let (endpoint, value) = match command {
            PlayerCommand::Playing(playing) => (if playing { "resume" } else { "pause" }, Value::Null),
            PlayerCommand::Seek(position) => ("seek_to", Value::from(position)),
            PlayerCommand::Skip(forward) => {
                (if forward { "skip_next" } else { "skip_prev" }, Value::Null)
            }
        };
        let mut command = json!({
            "endpoint": endpoint,
            "options": {
                "override_restrictions": false,
                "only_for_local_device": false,
                "system_initiated": false,
            },
        });
        if !value.is_null() {
            command["value"] = value;
        }
        let body = serde_json::to_vec(&json!({
            "command": command,
            "connection_type": "wlan",
            "intent_id": random_token::<16>().unwrap_or_default(),
        }))
        .unwrap_or_default();
        let mut compressed = GzEncoder::new(Vec::new(), Compression::fast());
        let result = compressed
            .write_all(&body)
            .and_then(|()| compressed.finish())
            .map_err(Into::into)
            .and_then(|body| {
                self.client.request(
                    Method::POST,
                    &format!(
                        "connect-state/v1/player/command/from/{}/to/{target}",
                        self.device_id()
                    ),
                    &[
                        ("x-spotify-connection-id", connection),
                        ("content-type", "application/json"),
                        ("content-encoding", "gzip"),
                    ],
                    body,
                )
            });
        if let Err(error) = result {
            error!(%error, %endpoint, "Spotify player command failed");
        }
    }

    fn set_volume(&self, percent: u8) {
        let (Some(connection), Some(target)) = (&self.connection_id, &self.active_device) else {
            return;
        };
        let command = SetVolumeCommand {
            volume: i32::from(percent) * 65_535 / 100,
            ..Default::default()
        };
        if let Err(error) = self.client.request_proto(
            Method::PUT,
            &format!(
                "connect-state/v1/connect/volume/from/{}/to/{target}",
                self.device_id()
            ),
            &[
                ("x-spotify-connection-id", connection),
                ("content-type", "application/x-protobuf"),
            ],
            &command,
        ) {
            error!(%error, "Spotify volume command failed");
        }
    }

    fn device_id(&self) -> String {
        self.client
            .session
            .lock()
            .map(|session| session.device_id.clone())
            .unwrap_or_default()
    }

    fn refresh_playlists(&mut self) {
        if let Err(error) = self.load_playlists() {
            warn!(%error, "Failed to refresh Spotify playlists");
        }
    }

    fn load_playlists(&mut self) -> ClientResult<()> {
        let username = self
            .client
            .session
            .lock()
            .map_err(|_| io::Error::other("Spotify session lock poisoned"))?
            .username
            .clone();
        let bytes = self.client.request(
            Method::GET,
            &format!(
                "playlist/v2/user/{username}/rootlist?decorate=revision,attributes,length,owner,capabilities,status_code&from=0&length=10000"
            ),
            &[],
            Vec::new(),
        )?;
        let root = SelectedListContent::parse_from_bytes(&bytes)?;
        let mut cache_changed = false;
        let mut wanted = HashSet::new();
        let mut updates = Vec::new();
        for (item, metadata) in root
            .contents
            .get_or_default()
            .items
            .iter()
            .zip(&root.contents.get_or_default().meta_items)
        {
            let Some(id) = item
                .uri()
                .strip_prefix("spotify:playlist:")
                .and_then(|id| id.parse::<PlaylistId>().ok())
            else {
                continue;
            };
            let attributes = metadata.attributes.get_or_default();
            let name = attributes.name();
            let rating_index = RATING_PLAYLISTS
                .iter()
                .position(|rating| *rating == name)
                .filter(|_| self.ratings_enabled)
                .map(|index| index as u8);
            if !self.playlist_targets.iter().any(|target| target == name) && rating_index.is_none() {
                continue;
            }
            wanted.insert(id);
            let tracks = if let Some((_, tracks)) = self
                .playlist_cache
                .get(&id)
                .filter(|(revision, _)| revision.as_slice() == metadata.revision())
            {
                Arc::clone(tracks)
            } else {
                let tracks = fetch_playlist_tracks(&self.client, id)?;
                self.playlist_cache
                    .insert(id, (metadata.revision().to_vec(), Arc::clone(&tracks)));
                cache_changed = true;
                tracks
            };
            updates.push(CondensedPlaylist {
                id,
                name: name.to_owned(),
                image_url: playlist_image(attributes),
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
        Ok(())
    }
}

fn player_position(player: &PlayerState, rate: f32) -> f32 {
    let position = player.position_as_of_timestamp.max(0) as f64;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let age_ms = now.saturating_sub(player.timestamp);
    (position + age_ms as f64 * f64::from(rate)) as f32
}

fn fetch_audio_features(background: &Background, ids: Vec<TrackId>) -> bool {
    let http = background.http.clone();
    background.submit(move || {
        let features = resolve_audio_features(&http, &ids);
        Box::new(move |app| {
            for track in &mut app.playback.queue {
                let Some(id) = track.id.filter(|id| ids.contains(id)) else {
                    continue;
                };
                track.runtime.audio_features = features
                    .get(&id)
                    .copied()
                    .map_or_else(Fetch::default, Fetch::Ready);
            }
        })
    })
}

fn resolve_audio_features(http: &Agent, track_ids: &[TrackId]) -> HashMap<TrackId, AudioFeatures> {
    let mut output = HashMap::new();
    for batch in track_ids.chunks(40) {
        let ids = batch.iter().map(TrackId::as_str).collect::<Vec<_>>().join(",");
        let Ok(features) = http
            .get(RECCO_FEATURES_URL)
            .query("ids", &ids)
            .call()
            .and_then(|response| response.into_body().read_json::<ReccoResponse>())
            .inspect_err(|err| warn!("Failed to fetch ReccoBeats audio features: {err}"))
        else {
            continue;
        };
        output.extend(features.items.into_iter().filter_map(|item| {
            Some((
                item.href.rsplit('/').next()?.parse().ok()?,
                item.features.normalized(),
            ))
        }));
    }
    output
}

fn fetch_playlist_tracks(client: &SpotifyClient, id: PlaylistId) -> ClientResult<PlaylistTracks> {
    let bytes = client.request(
        Method::GET,
        &format!("playlist/v2/playlist/{id}"),
        &[],
        Vec::new(),
    )?;
    let playlist = SelectedListContent::parse_from_bytes(&bytes)?;
    Ok(Arc::new(
        playlist
            .contents
            .get_or_default()
            .items
            .iter()
            .filter_map(|item| item.uri().strip_prefix("spotify:track:")?.parse().ok())
            .collect(),
    ))
}

fn playlist_image(attributes: &ListAttributes) -> Option<String> {
    attributes
        .picture_size
        .iter()
        .rev()
        .find_map(|picture| image_url(picture.url()))
        .or_else(|| {
            let picture = attributes.picture();
            let id = str::from_utf8(picture)
                .ok()
                .and_then(|picture| picture.strip_prefix("spotify:image:"))
                .map(str::to_owned)
                .or_else(|| {
                    (picture.len() == 20).then(|| {
                        picture.iter().fold(String::with_capacity(40), |mut id, byte| {
                            write!(id, "{byte:02x}").ok();
                            id
                        })
                    })
                })?;
            Some(format!("https://i.scdn.co/image/{id}"))
        })
}

fn image_url(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    Some(
        value
            .strip_prefix("spotify:image:")
            .map_or_else(|| value.to_owned(), |id| format!("https://i.scdn.co/image/{id}")),
    )
}

fn collection_write(track_id: TrackId, removed: bool) -> ClientResult<Vec<u8>> {
    let mut item = vec![0x12, 0x10];
    item.extend_from_slice(&base62(track_id.as_str())?);
    if removed {
        item.extend_from_slice(&[0x30, 1]);
    } else {
        item.push(0x28);
        write_varint(&mut item, SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
    }
    let mut collection = vec![0x0a];
    write_varint(&mut collection, item.len() as u64);
    collection.extend(item);
    Ok(collection)
}

fn write_varint(output: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        output.push(value as u8 | 0x80);
        value >>= 7;
    }
    output.push(value as u8);
}

fn base62(id: &str) -> ClientResult<[u8; 16]> {
    let mut value = num_bigint::BigUint::from(0u8);
    for byte in id.bytes() {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'z' => byte - b'a' + 10,
            b'A'..=b'Z' => byte - b'A' + 36,
            _ => return Err(client_error("invalid Spotify ID")),
        };
        value = value * 62u8 + digit;
    }
    let encoded = value.to_bytes_be();
    if encoded.len() > 16 {
        return Err(client_error("Spotify ID exceeds 128 bits"));
    }
    let mut bytes = [0; 16];
    bytes[16 - encoded.len()..].copy_from_slice(&encoded);
    Ok(bytes)
}

fn fetch_track_metadata(
    client: &SpotifyClient,
    tracks: &[ProvidedTrack],
) -> HashMap<String, metadata::Track> {
    let entity_request = tracks
        .iter()
        .filter(|track| {
            track.uri.starts_with("spotify:track:") && !track.metadata.contains_key("duration")
        })
        .map(|track| EntityRequest {
            entity_uri: track.uri.clone(),
            query: vec![ExtensionQuery {
                extension_kind: EnumOrUnknown::new(ExtensionKind::TRACK_V4),
                ..Default::default()
            }],
            ..Default::default()
        })
        .collect::<Vec<_>>();
    if entity_request.is_empty() {
        return HashMap::new();
    }

    let request = BatchedEntityRequest {
        entity_request,
        ..Default::default()
    };
    let result = client
        .request_metadata(
            Method::POST,
            "extended-metadata/v0/extended-metadata",
            &[("content-type", "application/x-protobuf")],
            &request,
        )
        .and_then(|bytes| Ok(BatchedExtensionResponse::parse_from_bytes(&bytes)?));
    let Ok(response) = result else {
        warn!("Failed to fetch Spotify track metadata");
        return HashMap::new();
    };

    response
        .extended_metadata
        .into_iter()
        .filter(|array| array.extension_kind == EnumOrUnknown::new(ExtensionKind::TRACK_V4))
        .flat_map(|array| array.extension_data)
        .filter_map(|data| {
            let bytes = data.extension_data.into_option()?.value;
            Some((data.entity_uri, metadata::Track::parse_from_bytes(&bytes).ok()?))
        })
        .collect()
}

fn track_from_provided(
    track: &ProvidedTrack,
    track_metadata: Option<&metadata::Track>,
    fallback_duration_ms: Option<u32>,
) -> Option<Track> {
    let metadata = &track.metadata;
    let duration_ms = metadata
        .get("duration")
        .and_then(|duration| duration.parse().ok())
        .or_else(|| track_metadata.and_then(|track| u32::try_from(track.duration()).ok()))
        .or(fallback_duration_ms)?;
    Some(Track {
        id: track
            .uri
            .strip_prefix("spotify:track:")
            .and_then(|id| id.parse().ok()),
        uri: track.uri.clone(),
        name: metadata
            .get("title")
            .cloned()
            .or_else(|| track_metadata.map(|track| track.name().to_owned()))
            .unwrap_or_default(),
        artist: metadata
            .get("artist_name")
            .cloned()
            .or_else(|| {
                track_metadata
                    .and_then(|track| track.artist.first().map(|artist| artist.name().to_owned()))
            })
            .unwrap_or_default(),
        album: metadata
            .get("album_title")
            .cloned()
            .or_else(|| track_metadata.map(|track| track.album.get_or_default().name().to_owned()))
            .unwrap_or_default(),
        image: ["image_xlarge_url", "image_large_url", "image_url"]
            .into_iter()
            .find_map(|key| metadata.get(key))
            .map(|url| {
                url.strip_prefix("spotify:image:")
                    .map_or_else(|| url.clone(), |id| format!("https://i.scdn.co/image/{id}"))
            })
            .or_else(|| track_metadata.and_then(track_image_url)),
        duration_ms,
        runtime: TrackRuntime::default(),
    })
}

fn track_image_url(track: &metadata::Track) -> Option<String> {
    let album = track.album.as_ref()?;
    let image = album
        .cover_group
        .as_ref()
        .into_iter()
        .flat_map(|group| &group.image)
        .chain(&album.cover)
        .max_by_key(|image| image.width());
    let id = image?.file_id();
    (!id.is_empty()).then(|| {
        let id = id
            .iter()
            .fold(String::with_capacity(id.len() * 2), |mut output, byte| {
                let _ = write!(output, "{byte:02x}");
                output
            });
        format!("https://i.scdn.co/image/{id}")
    })
}

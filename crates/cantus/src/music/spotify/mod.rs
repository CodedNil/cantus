use super::{
    LyricSegment, MusicResult, MusicService, PlaybackCommand, PlaylistId, PlaylistTracks, Track,
    TrackId, TrackRuntime,
};
use crate::app::{
    AppUpdater,
    config::{self, Config},
    send_update,
};
use crate::render::track::MAX_PILL_PLAYLIST_ICONS;
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
};
use parking_lot::Mutex;
use protobuf::{EnumOrUnknown, Message as _, MessageField};
use ring::rand::{SecureRandom as _, SystemRandom};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{self, Receiver, Sender},
    },
    thread::{sleep, spawn},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tracing::{error, warn};
use ureq::http::Method;

mod client;
mod dealer;
mod playlists;
mod session;

use client::SpotifyClient;

const CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
const SPOTIFY_SESSION_CACHE: &str = "spotify_session.json";
const PLAYLIST_TRACKS_CACHE: &str = "cantus_playlist_tracks.json";
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

fn random<const N: usize>() -> ClientResult<[u8; N]> {
    let mut bytes = [0; N];
    SystemRandom::new()
        .fill(&mut bytes)
        .map_err(|_| client_error("operating-system randomness unavailable"))?;
    Ok(bytes)
}

fn random_token<const N: usize>() -> ClientResult<String> {
    Ok(URL_SAFE_NO_PAD.encode(random::<N>()?))
}

type ClientResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn client_error(message: &'static str) -> Box<dyn Error + Send + Sync> {
    io::Error::other(message).into()
}

fn config_path(file: &str) -> PathBuf {
    config::directory().join(file)
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
    client: Arc<Mutex<Option<SpotifyClient>>>,
}

enum WorkerEvent {
    Command(PlaybackCommand),
    Connected(String),
    Cluster(Cluster),
    Metadata(HashMap<String, TrackDetails>),
    PlaylistsChanged,
}

impl SpotifyBackend {
    pub fn new(config: &Config, updater: &AppUpdater, http: ureq::Agent) -> Self {
        if let Err(error) = fs::create_dir_all(config::directory()) {
            warn!(%error, "Failed to create Cantus config directory");
        }
        let (events, receiver) = mpsc::channel();
        let client = Arc::new(Mutex::new(None));
        let connected_client = Arc::clone(&client);
        let worker_events = events.clone();
        let updater = updater.clone();
        let playlist_targets = config.playlists.clone();
        let ratings_enabled = config.ratings_enabled;
        spawn(move || {
            loop {
                match SpotifyClient::new(http.clone()) {
                    Ok(spotify) => {
                        *connected_client.lock() = Some(spotify.clone());
                        dealer::connect(spotify.clone(), worker_events.clone());
                        SpotifyWorker {
                            client: spotify,
                            events: worker_events,
                            updater,
                            connection_id: None,
                            active_device: None,
                            playlist_targets,
                            playlist_cache: read_cache(&config_path(PLAYLIST_TRACKS_CACHE))
                                .unwrap_or_default(),
                            track_metadata: HashMap::new(),
                            queue: None,
                            ratings_enabled,
                        }
                        .run(&receiver);
                        break;
                    }
                    Err(error) => {
                        warn!(%error, "Spotify unavailable; retrying");
                        sleep(Duration::from_secs(5));
                    }
                }
            }
        });
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
        let client = self
            .client
            .lock()
            .clone()
            .ok_or_else(|| client_error("Spotify is not connected"))?;
        let response = match client.request(
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
                    line_end: true,
                })
            })
            .collect())
    }
}

struct SpotifyWorker {
    client: SpotifyClient,
    events: Sender<WorkerEvent>,
    updater: AppUpdater,
    connection_id: Option<String>,
    active_device: Option<String>,
    playlist_targets: ArrayVec<String, MAX_PILL_PLAYLIST_ICONS>,
    playlist_cache: PlaylistCache,
    /// `None` marks metadata currently being fetched.
    track_metadata: HashMap<String, Option<TrackDetails>>,
    queue: Option<QueueSnapshot>,
    ratings_enabled: bool,
}

#[derive(Clone, Copy)]
struct PlaybackUpdate {
    playing: bool,
    position_ms: f32,
    rate: f32,
    volume: Option<u8>,
    observed_at: Instant,
}

struct QueueSnapshot {
    tracks: Vec<ProvidedTrack>,
    current: usize,
    current_duration_ms: Option<u32>,
    playback: PlaybackUpdate,
}

#[derive(Clone, Copy)]
enum PlayerCommand {
    Playing(bool),
    Seek(u32),
    Skip(bool),
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
            PlaybackCommand::Skip(count) => {
                for _ in 0..count.unsigned_abs() {
                    self.player_command(PlayerCommand::Skip(count > 0));
                }
            }
            PlaybackCommand::UpdateLibrary {
                track_id,
                playlists,
                liked,
            } => self.update_library(track_id, &playlists, liked),
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
                self.track_metadata.retain(|_, metadata| metadata.is_some());
                if !metadata.is_empty() {
                    self.track_metadata
                        .extend(metadata.into_iter().map(|(uri, metadata)| (uri, Some(metadata))));
                    self.publish_snapshot(true);
                }
            }
            WorkerEvent::PlaylistsChanged => self.refresh_playlists(),
        }
    }

    fn register(&self) -> ClientResult<Cluster> {
        let device_id = self.client.session.lock().device_id.clone();
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
                    client_id: CLIENT_ID.into(),
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
        let bytes = self.request_connected_proto(
            Method::PUT,
            &format!("connect-state/v1/devices/{device_id}"),
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
        self.track_metadata
            .retain(|uri, _| provided.iter().any(|track| track.uri == *uri));
        self.schedule_metadata(&provided);
        let volume = self
            .active_device
            .as_ref()
            .and_then(|id| cluster.device.get(id))
            .map(|device| (device.volume.saturating_mul(100) / 65_535) as u8);
        let current_duration_ms = u32::try_from(player.duration).ok();
        let rebuild_queue = self.queue.as_ref().is_none_or(|previous| {
            previous.current != current_position
                || previous.current_duration_ms != current_duration_ms
                || previous.tracks != provided
        });
        self.queue = Some(QueueSnapshot {
            tracks: provided,
            current: current_position,
            current_duration_ms,
            playback: PlaybackUpdate {
                playing,
                position_ms: position,
                rate,
                volume,
                observed_at,
            },
        });
        self.publish_snapshot(rebuild_queue);
    }

    fn publish_snapshot(&self, rebuild_queue: bool) {
        let Some(snapshot) = &self.queue else { return };
        let index = snapshot.tracks[..snapshot.current.min(snapshot.tracks.len())]
            .iter()
            .filter(|track| !track.uri.ends_with(":delimiter"))
            .count();
        let queue = rebuild_queue.then(|| {
            snapshot
                .tracks
                .iter()
                .enumerate()
                .filter(|(_, track)| !track.uri.ends_with(":delimiter"))
                .map(|(provided_index, track)| {
                    track_from_provided(
                        track,
                        self.track_metadata.get(&track.uri).and_then(Option::as_ref),
                        snapshot
                            .current_duration_ms
                            .filter(|_| provided_index == snapshot.current),
                    )
                })
                .collect()
        });
        self.publish_queue(queue, index, snapshot.playback);
    }

    fn publish_queue(&self, queue: Option<Vec<Track>>, index: usize, playback: PlaybackUpdate) {
        send_update(&self.updater, move |app| {
            let state = &mut app.playback;
            state.volume = playback.volume;
            let queue_changed = if let Some(queue) = queue {
                state.replace_queue(
                    queue,
                    index,
                    playback.position_ms,
                    playback.rate,
                    playback.observed_at,
                );
                true
            } else {
                state.observe(index, playback.position_ms, playback.rate, playback.observed_at);
                false
            };
            if playback.playing && !state.playing {
                app.render.last_toggle_time = app.render.start_time.elapsed().as_secs_f32();
            }
            state.playing = playback.playing;
            if queue_changed {
                app.refresh_track_enrichment();
            }
        });
    }

    fn schedule_metadata(&mut self, tracks: &[ProvidedTrack]) {
        let requested = tracks
            .iter()
            .filter(|track| {
                track.uri.starts_with("spotify:track:")
                    && !track.metadata.contains_key("duration")
                    && !self.track_metadata.contains_key(&track.uri)
                    && {
                        self.track_metadata.insert(track.uri.clone(), None);
                        true
                    }
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
            PlayerCommand::Playing(true) => ("resume", None),
            PlayerCommand::Playing(false) => ("pause", None),
            PlayerCommand::Seek(position) => ("seek_to", Some(position)),
            PlayerCommand::Skip(true) => ("skip_next", None),
            PlayerCommand::Skip(false) => ("skip_prev", None),
        };
        let mut command = json!({
            "endpoint": endpoint,
            "options": {
                "override_restrictions": false,
                "only_for_local_device": false,
                "system_initiated": false,
            },
        });
        if let Some(value) = value {
            command["value"] = value.into();
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
        let Some(target) = &self.active_device else {
            return;
        };
        let command = SetVolumeCommand {
            volume: i32::from(percent) * 65_535 / 100,
            ..Default::default()
        };
        if let Err(error) = self.request_connected_proto(
            Method::PUT,
            &format!(
                "connect-state/v1/connect/volume/from/{}/to/{target}",
                self.device_id()
            ),
            &command,
        ) {
            error!(%error, "Spotify volume command failed");
        }
    }

    fn device_id(&self) -> String {
        self.client.session.lock().device_id.clone()
    }

    fn request_connected(&self, method: Method, path: &str, body: Vec<u8>) -> ClientResult<Vec<u8>> {
        self.client.request(
            method,
            path,
            &[
                ("content-type", "application/x-protobuf"),
                (
                    "x-spotify-connection-id",
                    self.connection_id.as_deref().unwrap_or_default(),
                ),
            ],
            body,
        )
    }

    fn request_connected_proto<T: protobuf::Message>(
        &self,
        method: Method,
        path: &str,
        message: &T,
    ) -> ClientResult<Vec<u8>> {
        self.request_connected(method, path, message.write_to_bytes()?)
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

fn fetch_track_metadata(
    client: &SpotifyClient,
    tracks: &[ProvidedTrack],
) -> HashMap<String, TrackDetails> {
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
    let result: ClientResult<BatchedExtensionResponse> =
        client.request_proto(Method::POST, "extended-metadata/v0/extended-metadata", &request);
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
            Some((
                data.entity_uri,
                TrackDetails::from_spotify(&metadata::Track::parse_from_bytes(&bytes).ok()?),
            ))
        })
        .collect()
}

#[derive(Clone)]
struct TrackDetails {
    name: String,
    artist: String,
    album: String,
    image: Option<String>,
    duration_ms: u32,
}

impl TrackDetails {
    fn from_spotify(track: &metadata::Track) -> Self {
        Self {
            name: track.name().to_owned(),
            artist: track
                .artist
                .first()
                .map_or_else(String::new, |artist| artist.name().to_owned()),
            album: track.album.get_or_default().name().to_owned(),
            image: track_image_url(track),
            duration_ms: u32::try_from(track.duration()).unwrap_or_default(),
        }
    }
}

fn track_from_provided(
    track: &ProvidedTrack,
    track_metadata: Option<&TrackDetails>,
    fallback_duration_ms: Option<u32>,
) -> Track {
    let metadata = &track.metadata;
    let text = |key, fallback: fn(&TrackDetails) -> &String| {
        metadata
            .get(key)
            .filter(|value| !value.is_empty())
            .cloned()
            .or_else(|| track_metadata.map(fallback).cloned())
            .unwrap_or_default()
    };
    Track {
        id: track
            .uri
            .strip_prefix("spotify:track:")
            .and_then(|id| id.parse().ok()),
        uri: track.uri.clone(),
        name: text("title", |details| &details.name),
        artist: text("artist_name", |details| &details.artist),
        album: text("album_title", |details| &details.album),
        image: ["image_xlarge_url", "image_large_url", "image_url"]
            .into_iter()
            .find_map(|key| metadata.get(key))
            .map(|url| {
                url.strip_prefix("spotify:image:")
                    .map_or_else(|| url.clone(), |id| format!("https://i.scdn.co/image/{id}"))
            })
            .or_else(|| track_metadata.and_then(|details| details.image.clone())),
        duration_ms: metadata
            .get("duration")
            .and_then(|duration| duration.parse().ok())
            .or(fallback_duration_ms)
            .or_else(|| track_metadata.map(|details| details.duration_ms))
            .unwrap_or_default(),
        runtime: TrackRuntime::default(),
    }
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
    (!id.is_empty()).then(|| format!("https://i.scdn.co/image/{}", hex::encode(id)))
}

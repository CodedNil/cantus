use crate::PANEL_HEIGHT_BASE;
use anyhow::Result;
use auto_palette::Palette;
use dashmap::DashMap;
use image::{GenericImageView, RgbaImage};
use parking_lot::Mutex;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use reqwest::Client;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{AdditionalType, ArtistId, Context, FullTrack, PlayableItem, TrackId},
    prelude::{BaseClient, OAuthClient},
    scopes,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, hash_map::DefaultHasher},
    convert::TryInto,
    hash::{Hash, Hasher},
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering as AtomicOrdering},
    },
    time::Instant,
};
use tokio::{
    sync::OnceCell,
    task::JoinSet,
    time::{Duration, sleep},
};
use tracing::{error, info, warn};
use vello::peniko::{Blob, ImageAlphaType, ImageData, ImageFormat};
use zbus::{
    Connection,
    fdo::{DBusProxy, PropertiesProxy},
    names::InterfaceName,
    zvariant::OwnedValue,
};

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

/// Maximum number of historical tracks to keep before trimming.
const MAX_HISTORY: usize = 20;

/// Dimensions of the generated palette-based textures.
const PALETTE_IMAGE_HEIGHT: u32 = PANEL_HEIGHT_BASE as u32;
const PALETTE_IMAGE_WIDTH: u32 = PALETTE_IMAGE_HEIGHT * 4;

/// Number of refinement passes when synthesising the background texture.
const PALETTE_PASS_COUNT: usize = 10;
/// Maximum number of brush placements per pass.
const PALETTE_STROKES_PER_PASS: usize = 20;

/// MPRIS interface identifier used for playback control.
const PLAYER_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2.Player");
/// Root MPRIS interface that exposes metadata and identity.
const ROOT_INTERFACE: InterfaceName<'static> =
    InterfaceName::from_static_str_unchecked("org.mpris.MediaPlayer2");
/// Object path for the Spotify MPRIS instance on D-Bus.
const MPRIS_OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PlaybackState::default())));
pub static IMAGES_CACHE: LazyLock<DashMap<String, ImageData>> = LazyLock::new(DashMap::new);
pub static TRACK_DATA_CACHE: LazyLock<DashMap<TrackId<'static>, TrackData>> =
    LazyLock::new(DashMap::new);
pub static ARTIST_DATA_CACHE: LazyLock<DashMap<ArtistId<'static>, ArtistData>> =
    LazyLock::new(DashMap::new);
static BRUSHES: LazyLock<[RgbaImage; 5]> = LazyLock::new(|| {
    let bytes = (
        include_bytes!("../brushes/brush1.png"),
        include_bytes!("../brushes/brush2.png"),
        include_bytes!("../brushes/brush3.png"),
        include_bytes!("../brushes/brush4.png"),
        include_bytes!("../brushes/brush5.png"),
    );
    [
        image::load_from_memory(bytes.0).unwrap().to_rgba8(),
        image::load_from_memory(bytes.1).unwrap().to_rgba8(),
        image::load_from_memory(bytes.2).unwrap().to_rgba8(),
        image::load_from_memory(bytes.3).unwrap().to_rgba8(),
        image::load_from_memory(bytes.4).unwrap().to_rgba8(),
    ]
});

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
static SPOTIFY_CLIENT: OnceCell<AuthCodeSpotify> = OnceCell::const_new();
static SPOTIFY_INTERACTION_ACTIVE: AtomicBool = AtomicBool::new(false);

struct SpotifyInteractionGuard;

impl SpotifyInteractionGuard {
    fn try_acquire() -> Option<Self> {
        SPOTIFY_INTERACTION_ACTIVE
            .compare_exchange(
                false,
                true,
                AtomicOrdering::Acquire,
                AtomicOrdering::Relaxed,
            )
            .is_ok()
            .then(|| Self)
    }
}

impl Drop for SpotifyInteractionGuard {
    fn drop(&mut self) {
        SPOTIFY_INTERACTION_ACTIVE.store(false, AtomicOrdering::Release);
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackState {
    pub last_updated: Instant,
    pub playing: bool,
    pub shuffle: bool,
    pub progress: u32,
    pub queue: Vec<Track>,
    pub queue_index: usize,
    pub current_context: Option<Context>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            last_updated: Instant::now(),
            playing: false,
            shuffle: false,
            progress: 0,
            queue: Vec::new(),
            queue_index: 0,
            current_context: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: TrackId<'static>,
    pub title: String,
    pub artist_id: ArtistId<'static>,
    pub artist_name: String,
    pub album_name: String,
    pub image: Image,
    pub release_date: String,
    pub milliseconds: u32,
}

#[derive(Debug, Clone)]
pub struct TrackData {
    /// Simplified color palette (RGBA, alpha = percentage 0-100).
    pub primary_colors: Vec<[u8; 4]>,
    /// Generated texture derived from the palette for shader backgrounds.
    pub palette_image: ImageData,
}

pub struct ArtistData {
    pub name: String,
    pub genres: Vec<String>,
    pub popularity: u8,
    pub image: Image,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

impl Track {
    fn from_rspotify(track: FullTrack) -> Self {
        let FullTrack {
            id,
            name,
            artists,
            album,
            duration,
            ..
        } = track;
        let album_image = album
            .images
            .into_iter()
            .min_by_key(|img| img.width.unwrap())
            .unwrap();
        let image = Image {
            url: album_image.url,
            width: album_image.width.unwrap() as u16,
            height: album_image.height.unwrap() as u16,
        };
        let artist = artists.first().unwrap();
        Self {
            id: id.unwrap(),
            title: name,
            artist_id: artist.id.clone().unwrap(),
            artist_name: artist.name.clone(),
            album_name: album.name,
            image,
            release_date: album.release_date.unwrap(),
            milliseconds: duration.num_milliseconds() as u32,
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

/// Init the spotify client
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
                "playlist-modify-public"
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
    SPOTIFY_CLIENT.set(spotify).unwrap();

    // Begin polling
    tokio::spawn(polling_task());
}

/// Asynchronous task to poll MPRIS every 500ms and Spotify API every X seconds or on song change.
async fn polling_task() {
    let mut last_mpris_track_id: Option<String> = None; // Local state for track ID
    let connection = match Connection::session().await {
        Ok(conn) => conn,
        Err(err) => panic!("Failed to connect to D-Bus session: {err}"),
    };
    let dbus_proxy = match DBusProxy::new(&connection).await {
        Ok(proxy) => proxy,
        Err(err) => panic!("Failed creating D-Bus proxy: {err}"),
    };
    let mut spotify_poll_counter = 100; // Counter for Spotify API polling

    loop {
        // --- MPRIS Polling Logic ---
        let (should_refresh_spotify, used_mpris_progress) =
            update_state_from_mpris(&connection, &dbus_proxy, &mut last_mpris_track_id).await;

        // --- Spotify API Polling Logic ---
        spotify_poll_counter += 1;
        if spotify_poll_counter >= 3 || should_refresh_spotify {
            spotify_poll_counter = 0; // Reset counter
            update_state_from_spotify(used_mpris_progress).await;
        }

        sleep(Duration::from_millis(500)).await;
    }
}

/// Synchronizes playback information with the MPRIS interface and returns whether Spotify data should refresh, and whether MPRIS data was fetched.
async fn update_state_from_mpris(
    connection: &Connection,
    dbus_proxy: &DBusProxy<'_>,
    last_track_id: &mut Option<String>,
) -> (bool, bool) {
    let Ok(names) = dbus_proxy.list_names().await else {
        return (false, false);
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
        let Ok(identity) = proxy.get(ROOT_INTERFACE, "Identity").await else {
            continue;
        };
        if identity.to_string() == "\"Spotify\"" {
            properties_proxy = Some(proxy);
            break;
        }
    }
    let Some(properties_proxy) = properties_proxy else {
        return (false, false);
    };

    let new_track_id = properties_proxy
        .get(PLAYER_INTERFACE, "Metadata")
        .await
        .ok()
        .and_then(|metadata| {
            Some(
                HashMap::<String, OwnedValue>::try_from(metadata)
                    .ok()?
                    .get("mpris:trackid")?
                    .to_string(),
            )
        });
    let mut should_refresh = new_track_id
        .filter(|track_id| last_track_id.as_ref() != Some(track_id))
        .is_some_and(|track_id| {
            *last_track_id = Some(track_id);
            true
        });

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

    let mut updated_progress = false;
    if playing.is_some() || progress.is_some() {
        update_playback_state(|state| {
            if let Some(playing) = playing
                && playing != state.playing
            {
                should_refresh = true;
                state.playing = playing;
                if let Some(progress) = progress {
                    state.progress = progress as u32;
                    state.last_updated = Instant::now();
                    updated_progress = true;
                }
            }
        });
    }

    (should_refresh, updated_progress)
}

/// Pulls the current playback queue and status from the Spotify Web API and updates shared state.
async fn update_state_from_spotify(used_mpris_progress: bool) {
    // Fetch current playback and queue concurrently
    let request_start = Instant::now();
    let spotify_client = SPOTIFY_CLIENT.get().unwrap();
    let (current_playback, queue) = match tokio::try_join!(
        spotify_client.current_playback(None, None::<Vec<&AdditionalType>>),
        spotify_client.current_user_queue(),
    ) {
        Ok((Some(playback), queue)) => (playback, queue),
        Ok((None, _)) => {
            error!("Failed to fetch current playback from Spotify API: Returned None");
            return;
        }
        Err(err) => {
            error!("Failed to fetch current playback and queue: {}", err);
            return;
        }
    };

    let current_track = if let Some(PlayableItem::Track(track)) = queue.currently_playing {
        Track::from_rspotify(track)
    } else {
        return;
    };

    let new_queue: Vec<Track> = std::iter::once(current_track.clone())
        .chain(queue.queue.into_iter().filter_map(|item| match item {
            PlayableItem::Track(track) => Some(Track::from_rspotify(track)),
            PlayableItem::Episode(_) | PlayableItem::Unknown(_) => None,
        }))
        .collect();

    // Start a task to fetch missing artists & images
    let missing_urls = new_queue
        .iter()
        .filter(|track| !IMAGES_CACHE.contains_key(&track.image.url))
        .map(|track| track.image.url.clone())
        .collect::<HashSet<_>>();
    let missing_artists = new_queue
        .iter()
        .filter_map(|track| {
            if ARTIST_DATA_CACHE.contains_key(&track.artist_id) {
                None
            } else {
                Some(track.artist_id.clone())
            }
        })
        .collect::<HashSet<_>>();
    if !missing_urls.is_empty() || !missing_artists.is_empty() {
        tokio::spawn(async move {
            // Grab artists in one go from spotify
            let Ok(artists) = spotify_client.artists(missing_artists).await else {
                return;
            };

            // Start downloading missing album images
            let mut set = JoinSet::new();
            for url in missing_urls {
                set.spawn(async move {
                    if let Err(err) = ensure_image_cached(url.as_str()).await {
                        warn!("failed to cache image {url}: {err}");
                    }
                });
            }

            // Cache artists, and download images
            for artist in artists {
                let artist_image = artist
                    .images
                    .into_iter()
                    .min_by_key(|img| img.width.unwrap())
                    .unwrap();
                ARTIST_DATA_CACHE.insert(
                    artist.id,
                    ArtistData {
                        name: artist.name.clone(),
                        genres: artist.genres.clone(),
                        popularity: artist.popularity as u8,
                        image: Image {
                            url: artist_image.url.clone(),
                            width: artist_image.width.unwrap() as u16,
                            height: artist_image.height.unwrap() as u16,
                        },
                    },
                );
                set.spawn(async move {
                    let url = artist_image.url.clone();
                    if let Err(err) = ensure_image_cached(url.as_str()).await {
                        warn!("failed to cache image {url}: {err}");
                    }
                });
            }

            // Concurrently run all tasks
            while set.join_next().await.is_some() {}

            // Now that we have both the album and artist images downloaded, update track color palette
            if let Err(err) = update_color_palettes() {
                warn!("failed to update color palettes: {err}");
            }
        });
    }

    // Update the playback state
    update_playback_state(move |state| {
        if state.current_context == current_playback.context
            && let Some(new_index) = state
                .queue
                .iter()
                .position(|track| track.title == current_track.title)
        {
            // Delete everything past the new_index, and append the new tracks at the end
            state.queue_index = new_index;
            state.queue.truncate(new_index);
            state.queue.extend(new_queue);
        } else {
            // Context switched - reset queue entirely
            info!("Context changed, resetting queue");
            state.queue = new_queue;
            state.queue_index = 0;
            state.current_context = current_playback.context;
        }

        // Trim old history to prevent unbounded growth
        if state.queue_index > MAX_HISTORY {
            state.queue.drain(0..(state.queue_index - MAX_HISTORY));
            state.queue_index = MAX_HISTORY;
        }

        state.playing = current_playback.is_playing;
        state.shuffle = current_playback.shuffle_state;
        if !used_mpris_progress {
            let progress = current_playback
                .progress
                .map_or(0, |p| p.num_milliseconds()) as u32;
            let http_delay = (request_start.elapsed().as_millis() / 2) as u32;
            state.progress = progress + http_delay;
            state.last_updated = Instant::now();
        }
    });
}

/// Downloads and caches an image from the given URL.
async fn ensure_image_cached(url: &str) -> Result<()> {
    if IMAGES_CACHE.contains_key(url) {
        return Ok(());
    }
    let response = HTTP_CLIENT.get(url).send().await?.error_for_status()?;
    let dynamic_image = image::load_from_memory(&response.bytes().await?)?;
    let (width, height) = dynamic_image.dimensions();
    let rgba = dynamic_image.to_rgba8();
    IMAGES_CACHE.insert(
        url.to_owned(),
        ImageData {
            data: Blob::from(rgba.into_raw()),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width,
            height,
        },
    );
    Ok(())
}

/// Downloads and caches an image from the given URL.
fn update_color_palettes() -> Result<()> {
    let state = PLAYBACK_STATE.lock().clone();
    for track in &state.queue {
        if !TRACK_DATA_CACHE.contains_key(&track.id)
            && let Some(image) = IMAGES_CACHE.get(&track.image.url)
            && let Some(artist_image_ref) = ARTIST_DATA_CACHE.get(&track.artist_id)
            && let Some(artist_image) = IMAGES_CACHE.get(&artist_image_ref.image.url)
        {
            // Merge the images side by side
            let width = image.width;
            let height = image.height;
            let artist_new_width = (width as f32 * 0.1).round() as u32;
            let mut new_img = RgbaImage::new(width + artist_new_width, height);
            image::imageops::overlay(
                &mut new_img,
                &RgbaImage::from_raw(width, height, image.data.data().to_vec()).unwrap(),
                0,
                0,
            );
            let artist_img_resized = image::imageops::resize(
                &image::RgbaImage::from_raw(
                    artist_image.width,
                    artist_image.height,
                    artist_image.data.data().to_vec(),
                )
                .unwrap(),
                artist_new_width,
                height,
                image::imageops::FilterType::Triangle,
            );
            image::imageops::overlay(&mut new_img, &artist_img_resized, i64::from(width), 0);

            // Get palette
            let palette: Palette<f64> = Palette::builder()
                .algorithm(auto_palette::Algorithm::KMeans)
                .filter(ChromaFilter { threshold: 20 })
                .build(&auto_palette::ImageData::new(
                    width + artist_new_width,
                    height,
                    &new_img.into_vec(),
                )?)?;
            let swatches = palette
                .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                .or_else(|_| palette.find_swatches(NUM_SWATCHES))?;
            let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
            let mut primary_colors = swatches
                .iter()
                .map(|s| {
                    let rgb = s.color().to_rgb();
                    // Sometimes ratios can be tiny like 0.05%, this brings them a little closer to even
                    let lerped_ratio = lerp(
                        0.5,
                        (s.ratio() / total_ratio_sum) as f32,
                        1.0 / swatches.len() as f32,
                    );
                    [rgb.r, rgb.g, rgb.b, (lerped_ratio * 255.0).round() as u8]
                })
                .collect::<Vec<_>>();
            primary_colors.sort_by(|a, b| b[3].cmp(&a[3]));

            let palette_seed = {
                let mut hasher = DefaultHasher::new();
                track.id.hash(&mut hasher);
                hasher.finish()
            };

            let palette_image = ImageData {
                data: Blob::from(generate_palette_image(&primary_colors, palette_seed)),
                format: ImageFormat::Rgba8,
                alpha_type: ImageAlphaType::Alpha,
                width: PALETTE_IMAGE_WIDTH,
                height: PALETTE_IMAGE_HEIGHT,
            };

            TRACK_DATA_CACHE.insert(
                track.id.clone(),
                TrackData {
                    primary_colors,
                    palette_image,
                },
            );
        }
    }
    drop(state);

    Ok(())
}

/// Skip to the specified track in the queue.
pub async fn skip_to_track(track_id: TrackId<'static>) {
    let Some(_interaction_guard) = SpotifyInteractionGuard::try_acquire() else {
        warn!("Spotify interaction already in progress; skip_to_track returning early");
        return;
    };

    let playback_state = PLAYBACK_STATE.lock().clone();
    let queue_index = playback_state.queue_index;
    let Some(position_in_queue) = playback_state.queue.iter().position(|t| t.id == track_id) else {
        error!("Track not found in queue");
        return;
    };
    match queue_index.cmp(&position_in_queue) {
        Ordering::Equal => {
            info!("Already playing track {track_id}");
        }
        Ordering::Greater => {
            let position_difference = queue_index - position_in_queue;
            info!("Rewinding to track {track_id}, {position_difference} skips");
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().previous_track(None).await {
                    error!("Failed to skip to track: {err}");
                }
            }
        }
        Ordering::Less => {
            let position_difference = position_in_queue - queue_index;
            info!("Skipping to track {track_id}, {position_difference} skips");
            for _ in 0..(position_difference.min(10)) {
                if let Err(err) = SPOTIFY_CLIENT.get().unwrap().next_track(None).await {
                    error!("Failed to skip to track: {err}");
                }
            }
        }
    }

    update_state_from_spotify(false).await;
}

/// A filter that filters chroma values.
#[derive(Debug)]
pub struct ChromaFilter {
    threshold: u8,
}
impl auto_palette::Filter for ChromaFilter {
    fn test(&self, pixel: &auto_palette::Rgba) -> bool {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max - min) > self.threshold
    }
}

fn generate_palette_image(colors: &[[u8; 4]], seed: u64) -> Vec<u8> {
    let mut canvas = RgbaImage::from_pixel(
        PALETTE_IMAGE_WIDTH,
        PALETTE_IMAGE_HEIGHT,
        image::Rgba([12, 14, 18, 255]),
    );

    if colors.is_empty() {
        return canvas.into_raw();
    }

    let mut rng = SmallRng::seed_from_u64(seed);

    let mut targets = colors
        .iter()
        .map(|c| f32::from(c[3]).max(1.0))
        .collect::<Vec<_>>();
    let total_target = targets.iter().copied().sum::<f32>().max(1.0);
    for weight in &mut targets {
        *weight /= total_target;
    }

    let base = colors[0];
    for pixel in canvas.pixels_mut() {
        pixel.0 = [base[0], base[1], base[2], 255];
    }

    // Fill with the first colour; refinement passes will rebalance ratios.
    let total_pixels = (PALETTE_IMAGE_WIDTH * PALETTE_IMAGE_HEIGHT) as f32;

    let compute_ratios = |image: &RgbaImage| {
        let mut counts = vec![0u32; colors.len()];
        for pixel in image.pixels() {
            let mut best_index = 0usize;
            let mut best_distance = f32::MAX;
            for (index, color) in colors.iter().enumerate() {
                let dr = f32::from(pixel[0]) - f32::from(color[0]);
                let dg = f32::from(pixel[1]) - f32::from(color[1]);
                let db = f32::from(pixel[2]) - f32::from(color[2]);
                let distance = dr * dr + dg * dg + db * db;
                if distance < best_distance {
                    best_distance = distance;
                    best_index = index;
                }
            }
            counts[best_index] += 1;
        }
        counts
            .into_iter()
            .map(|count| count as f32 / total_pixels)
            .collect::<Vec<_>>()
    };

    for pass in 0..PALETTE_PASS_COUNT {
        let base_height = lerp(
            pass as f32 / PALETTE_PASS_COUNT as f32,
            PALETTE_IMAGE_HEIGHT as f32 * 0.7,
            PALETTE_IMAGE_HEIGHT as f32 * 0.3,
        );

        // Get how far we are off in total
        let coverage = compute_ratios(&canvas);
        let total_coverage_diff = coverage
            .iter()
            .zip(targets.iter())
            .map(|(&c, &t)| (c - t).abs())
            .sum::<f32>()
            .abs();
        // Divvy out a portion of the PALETTE_STROKES_PER_PASS per color
        let mut per_color_strokes: Vec<u8> = coverage
            .iter()
            .zip(targets.iter())
            .map(|(&c, &t)| {
                if total_coverage_diff == 0.0 {
                    // Handle division by zero case (e.g., if we are perfectly covered)
                    0
                } else {
                    // Calculate proportional strokes and use floor to ensure we don't exceed the pass limit
                    (((c - t).abs() / total_coverage_diff) * PALETTE_STROKES_PER_PASS as f32)
                        .floor() as u8
                }
            })
            .collect();

        for _ in 0..PALETTE_STROKES_PER_PASS {
            // Collect all indices of colors that still need strokes (c > 0)
            let available_indices: Vec<usize> = per_color_strokes
                .iter()
                .enumerate()
                .filter_map(|(i, &c)| (c > 0).then_some(i))
                .collect();
            if available_indices.is_empty() {
                break;
            }

            // Randomly select an index from the available candidates
            let index_to_pick = rng.random_range(0..available_indices.len());
            let color_index = available_indices[index_to_pick];
            per_color_strokes[color_index] -= 1;
            let color = colors[color_index];

            // Pick a random brush
            let template = BRUSHES[rng.random_range(0..BRUSHES.len())].clone();
            let brush_size = (base_height * rng.random_range(0.75..1.2))
                .round()
                .clamp(6.0, PALETTE_IMAGE_HEIGHT as f32) as u32;

            let mut stamp = image::imageops::resize(
                &template,
                brush_size,
                brush_size,
                image::imageops::FilterType::Triangle,
            );

            for pixel in stamp.pixels_mut() {
                let alpha = pixel[3];
                if alpha == 0 {
                    continue;
                }
                let faded_alpha = ((f32::from(alpha) * rng.random_range(0.55..0.9))
                    .round()
                    .clamp(1.0, 255.0)) as u8;
                pixel[0] = color[0];
                pixel[1] = color[1];
                pixel[2] = color[2];
                pixel[3] = faded_alpha;
            }

            let offset_x =
                i64::from(rng.random_range(0..=PALETTE_IMAGE_WIDTH)) - i64::from(brush_size / 2);
            let offset_y =
                i64::from(rng.random_range(0..=PALETTE_IMAGE_HEIGHT)) - i64::from(brush_size / 2);

            image::imageops::overlay(&mut canvas, &stamp, offset_x, offset_y);
        }
    }

    // Blur the image
    image::imageops::blur(&canvas, 16.0).into_raw()
}

fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

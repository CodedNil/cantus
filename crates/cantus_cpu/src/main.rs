use crate::interaction::InteractionState;
use crate::pipelines::{IMAGE_SIZE, MAX_TEXTURE_LAYERS};
use crate::render::{
    BackgroundPill, GlobalUniforms, IconInstance, Particle, PlayheadUniforms, RenderState,
};
use crate::text_render::TextRenderer;
use arrayvec::ArrayString;
use dashmap::DashMap;
use image::RgbaImage;
use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::{
    collections::HashMap,
    sync::{
        Arc, OnceLock,
        mpsc::{self, Receiver, Sender},
    },
    time::Instant,
};
use wgpu::{
    BindGroup, Buffer, Color, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, Instance,
    LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    StoreOp, Surface, SurfaceConfiguration, Texture, TextureViewDescriptor,
};

mod config;
mod interaction;
mod layer_shell;
mod pipelines;
mod render;
mod text_render;

#[cfg(feature = "spotify")]
mod spotify;

#[cfg(not(feature = "spotify"))]
mod spotify_debug;

const PANEL_START: f32 = 6.0;
const PANEL_EXTENSION: f32 = 44.0;

struct PlaybackState {
    playing: bool,
    progress: u32,
    volume: Option<u8>,
    queue: Vec<Track>,
    queue_index: usize,
    playlists: HashMap<PlaylistId, CondensedPlaylist>,

    last_interaction: Instant,
    last_progress_update: Instant,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            playing: false,
            progress: 0,
            volume: None,
            queue: Vec::new(),
            queue_index: 0,
            playlists: HashMap::new(),
            last_interaction: Instant::now(),
            last_progress_update: Instant::now(),
        }
    }
}

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

type TrackId = ArrayString<22>;
type AlbumId = ArrayString<22>;
type ArtistId = ArrayString<22>;
type PlaylistId = ArrayString<22>;

#[derive(Deserialize)]
struct Track {
    id: Option<TrackId>,
    name: String,
    album: Album,
    #[serde(deserialize_with = "deserialize_first_artist", rename = "artists")]
    artist: Artist,
    duration_ms: u32,
    #[serde(skip)]
    display_name: String,
}

impl Track {
    fn prepare(&mut self) {
        let without_suffix = self
            .name
            .split_once(" -")
            .map_or(self.name.as_str(), |(s, _)| s);
        self.display_name = without_suffix
            .split_once('(')
            .map_or(without_suffix, |(s, _)| s)
            .trim()
            .to_owned();
        if self.display_name.is_empty() {
            self.display_name = self.name.trim().to_owned();
        }
    }
}

#[derive(Deserialize)]
struct Album {
    id: Option<AlbumId>,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    image: Option<String>,
}

#[derive(Deserialize)]
struct Artist {
    id: Option<ArtistId>,
    name: String,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    image: Option<String>,
}

struct CondensedPlaylist {
    id: PlaylistId,
    name: String,
    image_url: Option<String>,
    tracks: HashSet<TrackId>,
    rating_index: Option<u8>,
}

#[derive(Deserialize)]
struct Image {
    url: String,
    width: Option<u32>,
}

type PlaybackUpdate = Box<dyn FnOnce(&mut PlaybackState) + Send>;
static PLAYBACK_UPDATES: OnceLock<Sender<PlaybackUpdate>> = OnceLock::new();

fn queue_playback_update<F>(update: F)
where
    F: FnOnce(&mut PlaybackState) + Send + 'static,
{
    if let Some(sender) = PLAYBACK_UPDATES.get() {
        let _ = sender.send(Box::new(update));
    }
}

#[derive(Default)]
struct AppCaches {
    images: DashMap<String, Option<Arc<RgbaImage>>>,
    album_palettes: DashMap<AlbumId, Option<[u32; NUM_SWATCHES]>>,
    artist_images: DashMap<ArtistId, Option<String>>,
}

struct CantusApp {
    // Core Graphics
    instance: Instance,
    gpu_resources: Option<GpuResources>,

    // Application State
    start_time: Instant,
    render_state: RenderState,
    interaction: InteractionState,
    playback_state: PlaybackState,
    playback_updates: Receiver<PlaybackUpdate>,
    config: config::Config,
    #[cfg(feature = "spotify")]
    spotify: spotify::SpotifyBackend,
    caches: Arc<AppCaches>,
    last_toggle_playing: Instant,
    particles: [Particle; 64],
    particles_accumulator: f32,
    scale_factor: f32,

    // Scene & Resources
    text_renderer: Option<TextRenderer>,
    global_uniforms: GlobalUniforms,
    background_pills: Vec<BackgroundPill>,
    icon_pills: Vec<IconInstance>,
    playhead_info: PlayheadUniforms,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (playback_tx, playback_updates) = mpsc::channel();
        let _ = PLAYBACK_UPDATES.set(playback_tx);
        let caches = Arc::new(AppCaches::default());
        let config = config::load();
        Self {
            instance: Instance::default(),
            gpu_resources: None,

            start_time: Instant::now(),
            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            #[cfg(feature = "spotify")]
            playback_state: PlaybackState::default(),
            #[cfg(not(feature = "spotify"))]
            playback_state: spotify_debug::debug_playbackstate(Arc::clone(&caches)),
            playback_updates,
            #[cfg(feature = "spotify")]
            spotify: spotify::SpotifyBackend::new(&config, &caches),
            config,
            caches,
            last_toggle_playing: Instant::now(),
            particles: [Particle::default(); 64],
            particles_accumulator: 0.0,
            scale_factor: 1.0,

            text_renderer: None,
            global_uniforms: GlobalUniforms::default(),
            background_pills: Vec::new(),
            icon_pills: Vec::new(),
            playhead_info: PlayheadUniforms::default(),
        }
    }
}

struct GpuResources {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,

    // Pipelines
    playhead_pipeline: RenderPipeline,
    background_pipeline: RenderPipeline,
    icon_pipeline: RenderPipeline,
    particle_pipeline: RenderPipeline,

    // Uniform/Storage Buffers
    uniform_buffer: Buffer,
    particles_buffer: Buffer,
    playhead_buffer: Buffer,
    background_storage_buffer: Buffer,
    icon_storage_buffer: Buffer,

    // Bind Groups
    playhead_bind_group: BindGroup,
    background_bind_group: BindGroup,
    icon_bind_group: BindGroup,
    particle_bind_group: BindGroup,

    // Image Management
    texture_array: Texture,
    url_to_image_index: HashMap<String, (i32, bool)>, // (index, used_this_frame)
}

fn cache_decoded_image(caches: Arc<AppCaches>, url: String, image: image::DynamicImage) {
    let image = if image.width() != IMAGE_SIZE || image.height() != IMAGE_SIZE {
        image.resize_to_fill(
            IMAGE_SIZE,
            IMAGE_SIZE,
            image::imageops::FilterType::Lanczos3,
        )
    } else {
        image
    };
    caches.images.insert(url, Some(Arc::new(image.to_rgba8())));
    queue_playback_update(move |state| render::update_color_palettes(&caches, &state.queue));
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            ["warn", "cantus=info", "wgpu_hal=error"].join(","),
        ))
        .with_writer(std::io::stderr)
        .init();

    layer_shell::run();
}

impl CantusApp {
    fn render(&mut self) {
        #[cfg(feature = "spotify")]
        self.spotify.tick();

        let Some(gpu) = self.gpu_resources.as_mut() else {
            return;
        };

        while let Ok(update) = self.playback_updates.try_recv() {
            update(&mut self.playback_state);
        }
        self.icon_pills.clear();

        for (_, used) in gpu.url_to_image_index.values_mut() {
            *used = false;
        }

        self.create_scene();

        let gpu = self.gpu_resources.as_mut().unwrap();
        gpu.url_to_image_index.retain(|_, (_, used)| *used);
        gpu.queue.write_buffer(
            &gpu.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.global_uniforms),
        );
        gpu.queue.write_buffer(
            &gpu.particles_buffer,
            0,
            bytemuck::cast_slice(&self.particles),
        );
        gpu.queue.write_buffer(
            &gpu.playhead_buffer,
            0,
            bytemuck::bytes_of(&self.playhead_info),
        );

        if !self.background_pills.is_empty() {
            gpu.queue.write_buffer(
                &gpu.background_storage_buffer,
                0,
                bytemuck::cast_slice(&self.background_pills),
            );
        }
        if !self.icon_pills.is_empty() {
            gpu.queue.write_buffer(
                &gpu.icon_storage_buffer,
                0,
                bytemuck::cast_slice(&self.icon_pills),
            );
        }

        let CurrentSurfaceTexture::Success(surface_texture) = gpu.surface.get_current_texture()
        else {
            gpu.surface.configure(&gpu.device, &gpu.surface_config);
            return;
        };
        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            if !self.background_pills.is_empty() {
                rpass.set_pipeline(&gpu.background_pipeline);
                rpass.set_bind_group(0, &gpu.background_bind_group, &[]);
                rpass.draw(0..4, 0..self.background_pills.len() as u32);
            }

            if let Some(text_renderer) = &mut self.text_renderer {
                text_renderer.draw(
                    &gpu.device,
                    &gpu.queue,
                    &mut rpass,
                    gpu.surface_config.width,
                    gpu.surface_config.height,
                    self.scale_factor,
                );
            }

            if !self.icon_pills.is_empty() {
                rpass.set_pipeline(&gpu.icon_pipeline);
                rpass.set_bind_group(0, &gpu.icon_bind_group, &[]);
                rpass.draw(0..4, 0..self.icon_pills.len() as u32);
            }

            rpass.set_pipeline(&gpu.particle_pipeline);
            rpass.set_bind_group(0, &gpu.particle_bind_group, &[]);
            rpass.draw(0..4, 0..64);

            rpass.set_pipeline(&gpu.playhead_pipeline);
            rpass.set_bind_group(0, &gpu.playhead_bind_group, &[]);
            rpass.draw(0..4, 0..1);
        }

        gpu.queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    fn get_image_index(&mut self, url: &str) -> i32 {
        let Some(gpu) = self.gpu_resources.as_mut() else {
            return -1;
        };

        if let Some(entry) = gpu.url_to_image_index.get_mut(url) {
            entry.1 = true;
            return entry.0;
        }

        if let Some(img_ref) = self.caches.images.get(url)
            && let Some(image) = img_ref.as_ref()
        {
            let mut used_slots = vec![false; MAX_TEXTURE_LAYERS as usize];
            for (idx, _) in gpu.url_to_image_index.values() {
                used_slots[*idx as usize] = true;
            }

            if let Some(slot) = used_slots.iter().position(|&used| !used) {
                gpu.queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &gpu.texture_array,
                        mip_level: 0,
                        aspect: wgpu::TextureAspect::All,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: slot as u32,
                        },
                    },
                    image.as_raw(),
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * IMAGE_SIZE),
                        rows_per_image: Some(IMAGE_SIZE),
                    },
                    wgpu::Extent3d {
                        width: IMAGE_SIZE,
                        height: IMAGE_SIZE,
                        depth_or_array_layers: 1,
                    },
                );

                gpu.url_to_image_index
                    .insert(url.to_owned(), (slot as i32, true));
                return slot as i32;
            }
        }
        -1
    }
}

fn deserialize_images<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Vec::<Image>::deserialize(deserializer)?
        .into_iter()
        .min_by_key(|img| img.width)
        .map(|img| img.url))
}

fn deserialize_first_artist<'de, D>(deserializer: D) -> Result<Artist, D::Error>
where
    D: Deserializer<'de>,
{
    let artists: Vec<Artist> = Vec::deserialize(deserializer)?;
    artists
        .into_iter()
        .next()
        .ok_or_else(|| serde::de::Error::custom("artists array is empty"))
}

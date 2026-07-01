use crate::{
    cache::{AppCaches, ImageCacheState, cache_decoded_image},
    interaction::InteractionState,
    pipelines::{IMAGE_SIZE, MAX_TEXTURE_IMAGES},
    render::{
        BackgroundPill, GlobalUniforms, IconInstance, Particle, PlayheadUniforms, RenderState,
    },
    text_render::TextRenderer,
};
use arrayvec::ArrayString;
use serde::{Deserialize, Deserializer};
use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc,
        mpsc::{self, Receiver, Sender},
    },
    time::Instant,
};
use wgpu::{
    BindGroup, Buffer, Color, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, Instance,
    LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    StoreOp, Surface, SurfaceConfiguration, Texture, TextureViewDescriptor,
};

mod cache;
mod config;
mod interaction;
mod layer_shell;
mod pipelines;
mod render;
mod spotify;
mod text_render;

const PANEL_START: f32 = 6.0;
const PANEL_EXTENSION: f32 = 44.0;
const PARTICLE_COUNT: usize = 64;
const MAX_RENDER_INSTANCES: usize = 256;

struct CantusApp {
    // Core Graphics
    instance: Instance,
    gpu_resources: Option<GpuResources>,

    // Application State
    start_time: Instant,
    render_state: RenderState,
    interaction: InteractionState,
    playback_state: PlaybackState,
    updater: AppUpdater,
    app_updates: Receiver<AppUpdate>,
    config: config::Config,
    spotify: spotify::SpotifyBackend,
    caches: AppCaches,
    image_cache: ImageCacheState,
    last_toggle_playing: Instant,
    particles: [Particle; PARTICLE_COUNT],
    particle_dirty_mask: u64,
    particles_accumulator: f32,
    /// Physical buffer pixels per logical Wayland surface pixel.
    render_scale: f32,

    // Scene & Resources
    text_renderer: Option<TextRenderer>,
    global_uniforms: GlobalUniforms,
    background_pills: Vec<BackgroundPill>,
    icon_pills: Vec<IconInstance>,
    playhead_info: PlayheadUniforms,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (update_tx, app_updates) = mpsc::channel();
        let updater = AppUpdater(update_tx);
        let config = config::load();
        let spotify = spotify::SpotifyBackend::new(&config, updater.clone());
        Self {
            instance: Instance::default(),
            gpu_resources: None,

            start_time: Instant::now(),
            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            playback_state: PlaybackState::default(),
            updater,
            app_updates,
            spotify,
            config,
            caches: AppCaches::default(),
            image_cache: ImageCacheState {
                dirty: true,
                ..Default::default()
            },
            last_toggle_playing: Instant::now(),
            particles: [Particle::default(); PARTICLE_COUNT],
            particle_dirty_mask: u64::MAX,
            particles_accumulator: 0.0,
            render_scale: 1.0,

            text_renderer: None,
            global_uniforms: GlobalUniforms::default(),
            background_pills: Vec::new(),
            icon_pills: Vec::new(),
            playhead_info: PlayheadUniforms::default(),
        }
    }
}

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
type PlaylistId = ArrayString<22>;

#[derive(Deserialize)]
struct Track {
    id: Option<TrackId>,
    name: String,
    album: Album,
    #[serde(deserialize_with = "deserialize_first_artist", rename = "artists")]
    artist: Artist,
    duration_ms: u32,
}

#[derive(Deserialize)]
struct Album {
    id: Option<AlbumId>,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    image: Option<String>,
}

#[derive(Deserialize)]
struct Artist {
    name: String,
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

type AppUpdate = Box<dyn FnOnce(&mut CantusApp) + Send>;

#[derive(Clone)]
struct AppUpdater(Sender<AppUpdate>);

impl AppUpdater {
    fn send(&self, update: impl FnOnce(&mut CantusApp) + Send + 'static) {
        let _ = self.0.send(Box::new(update));
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
    image_slots: HashMap<String, ImageSlot>,
}

struct ImageSlot {
    layer: u32,
    used_this_frame: bool,
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
    fn logical_surface_size(&self) -> (f32, f32) {
        (
            self.config.width,
            self.config.height + PANEL_START + PANEL_EXTENSION,
        )
    }

    fn buffer_size(&self) -> (u32, u32) {
        let (width, height) = self.logical_surface_size();
        (
            (width * self.render_scale).round() as u32,
            (height * self.render_scale).round() as u32,
        )
    }

    fn render(&mut self) {
        self.spotify.tick();

        if self.gpu_resources.is_none() {
            return;
        }

        while let Ok(update) = self.app_updates.try_recv() {
            update(self);
            self.image_cache.mark_dirty();
        }
        if self.image_cache.dirty {
            let client = Arc::clone(&self.spotify.client);
            let updater = self.updater.clone();
            self.image_cache
                .sync(&mut self.caches, &self.playback_state, |url| {
                    spotify::download_image(Arc::clone(&client), updater.clone(), url);
                });
            self.image_cache.dirty = false;
        }
        self.icon_pills.clear();

        for slot in self
            .gpu_resources
            .as_mut()
            .unwrap()
            .image_slots
            .values_mut()
        {
            slot.used_this_frame = false;
        }

        self.create_scene();

        let gpu = self.gpu_resources.as_mut().unwrap();
        gpu.image_slots.retain(|_, slot| slot.used_this_frame);
        gpu.queue.write_buffer(
            &gpu.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.global_uniforms),
        );
        while self.particle_dirty_mask != 0 {
            let start = self.particle_dirty_mask.trailing_zeros() as usize;
            let count = (self.particle_dirty_mask >> start).trailing_ones() as usize;
            let end = start + count;
            gpu.queue.write_buffer(
                &gpu.particles_buffer,
                (start * std::mem::size_of::<Particle>()) as u64,
                bytemuck::cast_slice(&self.particles[start..end]),
            );
            if count == PARTICLE_COUNT {
                self.particle_dirty_mask = 0;
            } else {
                self.particle_dirty_mask &= !(((1u64 << count) - 1) << start);
            }
        }
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
                    self.render_scale,
                );
            }

            if !self.icon_pills.is_empty() {
                rpass.set_pipeline(&gpu.icon_pipeline);
                rpass.set_bind_group(0, &gpu.icon_bind_group, &[]);
                rpass.draw(0..4, 0..self.icon_pills.len() as u32);
            }

            rpass.set_pipeline(&gpu.particle_pipeline);
            rpass.set_bind_group(0, &gpu.particle_bind_group, &[]);
            rpass.draw(0..4, 0..PARTICLE_COUNT as u32);

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

        if let Some(slot) = gpu.image_slots.get_mut(url) {
            slot.used_this_frame = true;
            return slot.layer as i32;
        }

        if let Some(image) = self.caches.images.get(url) {
            let slot = (0..MAX_TEXTURE_IMAGES)
                .map(|image| image * 2)
                .find(|&layer| !gpu.image_slots.values().any(|slot| slot.layer == layer));
            if let Some(slot) = slot {
                let blurred = image::imageops::blur(image.as_ref(), 3.0);
                let write_layer = |layer: u32, bytes: &[u8]| {
                    gpu.queue.write_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &gpu.texture_array,
                            mip_level: 0,
                            aspect: wgpu::TextureAspect::All,
                            origin: wgpu::Origin3d {
                                x: 0,
                                y: 0,
                                z: layer,
                            },
                        },
                        bytes,
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
                };
                write_layer(slot, image.as_raw());
                write_layer(slot + 1, blurred.as_raw());

                gpu.image_slots.insert(
                    url.to_owned(),
                    ImageSlot {
                        layer: slot,
                        used_this_frame: true,
                    },
                );
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

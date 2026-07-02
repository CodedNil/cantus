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
use image::RgbaImage;
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
    LoadOp, Operations, Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, StoreOp, Surface, SurfaceConfiguration, Texture, TextureViewDescriptor,
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
    particles_accumulator: f32,
    /// Physical buffer pixels per logical Wayland surface pixel.
    render_scale: f32,
    /// Logical width assigned by the layer-shell compositor.
    surface_width: Option<f32>,

    // Scene & Resources
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
            particles_accumulator: 0.0,
            render_scale: 1.0,
            surface_width: None,

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

    uniform_buffer: Buffer,
    playhead: GpuPass,
    background: GpuPass,
    icons: GpuPass,
    text: GpuPass,
    particles: GpuPass,
    images: ImageAtlas,
    text_renderer: TextRenderer,
}

struct GpuPass {
    pipeline: RenderPipeline,
    buffer: Buffer,
    bind_group: BindGroup,
}

impl GpuPass {
    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>, instances: u32) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..4, 0..instances);
    }

    fn draw_data<'pass, T: bytemuck::NoUninit>(
        &'pass self,
        queue: &Queue,
        pass: &mut RenderPass<'pass>,
        data: &[T],
    ) {
        if !data.is_empty() {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
            self.draw(pass, data.len() as u32);
        }
    }
}

impl GpuResources {
    fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn resize_surface(&mut self, width: u32, height: u32) {
        if (self.surface_config.width, self.surface_config.height) != (width, height) {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.configure_surface();
        }
    }
}

struct ImageAtlas {
    texture: Texture,
    slots: [Option<String>; MAX_TEXTURE_IMAGES as usize],
    used: u32,
}

impl ImageAtlas {
    const fn begin_frame(&mut self) {
        self.used = 0;
    }

    fn image_index(&mut self, queue: &Queue, url: &str, image: Option<&RgbaImage>) -> i32 {
        if let Some(index) = self
            .slots
            .iter()
            .position(|slot| slot.as_deref() == Some(url))
        {
            self.used |= 1 << index;
            return (index * 2) as i32;
        }

        let Some(image) = image else {
            return -1;
        };
        let index = (!self.used).trailing_zeros();
        if index >= MAX_TEXTURE_IMAGES {
            return -1;
        }
        self.used |= 1 << index;
        let layer = index * 2;

        let blurred = image::imageops::blur(image, 3.0);
        let mut bytes = Vec::with_capacity(image.len() + blurred.len());
        bytes.extend_from_slice(image);
        bytes.extend_from_slice(&blurred);
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: layer,
                },
            },
            &bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * IMAGE_SIZE),
                rows_per_image: Some(IMAGE_SIZE),
            },
            wgpu::Extent3d {
                width: IMAGE_SIZE,
                height: IMAGE_SIZE,
                depth_or_array_layers: 2,
            },
        );
        self.slots[index as usize] = Some(url.to_owned());
        layer as i32
    }
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
            self.surface_width.unwrap_or(self.config.width),
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

    /// Renders a frame and reports whether the surface must be recreated.
    fn render(&mut self) -> bool {
        self.spotify.tick();

        if self.gpu_resources.is_none() {
            return false;
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
        let gpu = self.gpu_resources.as_mut().unwrap();
        let (surface_texture, reconfigure_after_present) = match gpu.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => (texture, false),
            CurrentSurfaceTexture::Suboptimal(texture) => (texture, true),
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return false,
            CurrentSurfaceTexture::Outdated => {
                gpu.configure_surface();
                return false;
            }
            CurrentSurfaceTexture::Lost => return true,
            CurrentSurfaceTexture::Validation => {
                tracing::error!("surface texture acquisition failed validation");
                return false;
            }
        };

        self.icon_pills.clear();

        let gpu = self.gpu_resources.as_mut().unwrap();
        gpu.images.begin_frame();
        gpu.text_renderer.begin_frame();

        self.create_scene();

        let gpu = self.gpu_resources.as_mut().unwrap();
        gpu.queue.write_buffer(
            &gpu.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.global_uniforms),
        );
        gpu.queue.write_buffer(
            &gpu.particles.buffer,
            0,
            bytemuck::cast_slice(&self.particles),
        );
        gpu.queue.write_buffer(
            &gpu.playhead.buffer,
            0,
            bytemuck::bytes_of(&self.playhead_info),
        );
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

            gpu.background
                .draw_data(&gpu.queue, &mut rpass, &self.background_pills);

            gpu.text
                .draw_data(&gpu.queue, &mut rpass, gpu.text_renderer.glyphs());

            gpu.icons
                .draw_data(&gpu.queue, &mut rpass, &self.icon_pills);

            gpu.particles.draw(&mut rpass, PARTICLE_COUNT as u32);

            gpu.playhead.draw(&mut rpass, 1);
        }

        gpu.queue.submit([encoder.finish()]);
        gpu.queue.present(surface_texture);
        if reconfigure_after_present {
            gpu.configure_surface();
        }
        false
    }

    fn get_image_index(&mut self, url: &str) -> i32 {
        let Some(gpu) = self.gpu_resources.as_mut() else {
            return -1;
        };
        gpu.images.image_index(
            &gpu.queue,
            url,
            self.caches.images.get(url).map(AsRef::as_ref),
        )
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

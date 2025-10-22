use anyhow::{Result, anyhow};
use bytemuck::{Pod, Zeroable};
use eframe::{
    HardwareAcceleration,
    egui_wgpu::{
        Callback as WgpuCallback, CallbackResources, CallbackTrait, ScreenDescriptor,
        WgpuConfiguration,
    },
};
use egui::{
    Align2, CentralPanel, Color32, ColorImage, Context, FontId, Frame, Margin, Rect, TextureHandle,
    TextureOptions, Vec2, ViewportBuilder, pos2, vec2,
};
use log::warn;
use mpris::PlayerFinder;
use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use url::Url;
use uuid::Uuid;

const PANEL_MARGIN: f32 = 12.0;
const BLUR_SIGMA: f32 = 60.0;
const WARP_STRENGTH: f32 = 2.0;
const SWIRL_STRENGTH: f32 = 0.4;
const WARP_TIME_SCALE: f32 = 0.8;

const WARP_SHADER: &str = include_str!("warp_background.wgsl");

const TEXTURE_BIND_GROUP_ENTRIES: &[wgpu::BindGroupLayoutEntry] = &[
    // Binding 0: Texture
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    },
    // Binding 1: Sampler
    wgpu::BindGroupLayoutEntry {
        binding: 1,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    },
];

const UNIFORM_BIND_GROUP_ENTRIES: &[wgpu::BindGroupLayoutEntry] = &[wgpu::BindGroupLayoutEntry {
    binding: 0,
    visibility: wgpu::ShaderStages::FRAGMENT,
    ty: wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
    },
    count: None,
}];

/// Runs the eframe application.
fn main() -> eframe::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    eframe::run_native(
        "Cantus",
        eframe::NativeOptions {
            viewport: ViewportBuilder {
                title: Some("Cantus".to_string()),
                app_id: Some("cantus".to_string()),
                position: Some(pos2(100.0, 100.0)),
                inner_size: Some(vec2(320.0, 120.0)),
                resizable: Some(true),
                transparent: Some(true),
                decorations: Some(false),
                active: Some(true),
                window_level: Some(egui::WindowLevel::AlwaysOnTop),
                ..Default::default()
            },
            hardware_acceleration: HardwareAcceleration::Required,
            wgpu_options: WgpuConfiguration {
                present_mode: wgpu::PresentMode::Fifo,
                desired_maximum_frame_latency: Some(2),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(CantusApp::new()))),
    )
}

type ArtResponseData = Option<(String, Vec<u8>)>;
type ArtResponse = Arc<Mutex<ArtResponseData>>;

/// Represents the state of the album art currently displayed or being fetched.
enum AlbumArtState {
    /// No album art is associated with the current track.
    None,
    /// Album art is currently being fetched from the given URL.
    Loading(String),
    /// Album art has been successfully loaded from the given URL.
    Loaded(String, AlbumArtTextures),
}

/// The main application state for Cantus.
struct CantusApp {
    last_poll: Instant,
    track: Option<TrackInfo>,
    art_state: AlbumArtState,
    art_response: ArtResponse,
    start_time: Instant,
}

impl CantusApp {
    /// Creates a new instance of the Cantus application.
    fn new() -> Self {
        Self {
            last_poll: Instant::now(),
            track: None,
            art_state: AlbumArtState::None,
            art_response: ArtResponse::new(Mutex::new(None)),
            start_time: Instant::now(),
        }
    }

    /// Polls the MPRIS player to get the current track metadata and ensures the corresponding album art is loaded if available.
    fn refresh_track(&mut self, ctx: &Context) {
        let track = PlayerFinder::new()
            .ok()
            .and_then(|finder| finder.find_active().ok())
            .and_then(|player| player.get_metadata().ok())
            .map(|metadata| TrackInfo::from_metadata(&metadata));

        let previous_art_url = self.track.as_ref().and_then(|t| t.album_art_url.clone());
        let new_art_url = track.as_ref().and_then(|t| t.album_art_url.clone());
        let art_changed = previous_art_url != new_art_url;

        self.track = track;

        if art_changed {
            self.art_state = AlbumArtState::None;
        }

        // If no new art URL, we are done.
        let Some(url) = new_art_url else {
            return;
        };

        // If we already have the art or are currently fetching it, skip loading.
        match &self.art_state {
            AlbumArtState::Loading(loading_url) if loading_url == &url => return,
            AlbumArtState::Loaded(loaded_url, _) if loaded_url == &url => return,
            _ => {}
        }

        println!("Loading album art from {url}");

        let parsed = Url::parse(&url);
        let Ok(parsed) = parsed else {
            warn!(
                "Failed to parse album art URL {url}: {}",
                parsed.unwrap_err()
            );
            return;
        };

        if parsed.scheme() == "file" {
            // Handle local file synchronously
            match Self::load_album_art_from_file(&parsed) {
                Ok(bytes) => match Self::process_album_art_bytes(ctx, &bytes) {
                    Ok(textures) => {
                        self.art_state = AlbumArtState::Loaded(url, textures);
                    }
                    Err(err) => {
                        warn!("Failed to process album art bytes for {url}: {err}");
                        self.art_state = AlbumArtState::None;
                    }
                },
                Err(err) => {
                    warn!("Failed to load album art from file {url}: {err}");
                    self.art_state = AlbumArtState::None;
                }
            }
        } else {
            // Handle remote URL asynchronously using ehttp
            self.art_state = AlbumArtState::Loading(url.clone());
            let art_response = self.art_response.clone();
            let ctx_clone = ctx.clone();

            ehttp::fetch(ehttp::Request::get(url), move |response| {
                if let Ok(response) = response
                    && response.ok
                    && !response.bytes.is_empty()
                    && let Ok(mut response_lock) = art_response.lock()
                {
                    *response_lock = Some((response.url, response.bytes));
                }
                ctx_clone.request_repaint();
            });
        }
    }

    /// Loads album art from a file URL and returns the raw bytes.
    fn load_album_art_from_file(parsed_url: &Url) -> Result<Vec<u8>> {
        let path = parsed_url
            .to_file_path()
            .map_err(|()| anyhow!("Unsupported file path in URL: {parsed_url}"))?;

        fs::read(&path).map_err(Into::into)
    }

    /// Decodes image bytes and creates two egui textures: the original image and a heavily blurred version.
    fn process_album_art_bytes(ctx: &Context, bytes: &[u8]) -> Result<AlbumArtTextures> {
        // Decode and process image
        let rgba = image::load_from_memory(bytes)?.to_rgba8();

        // Create a heavily blurred version for the background effect
        let blurred_img = image::imageops::blur(&rgba, BLUR_SIGMA);

        // Load textures into egui
        let texture_name_base = Uuid::new_v4().to_string();
        Ok(AlbumArtTextures {
            original: ctx.load_texture(
                format!("album_{texture_name_base}_original"),
                ColorImage::from_rgba_unmultiplied(
                    [rgba.width() as usize, rgba.height() as usize],
                    rgba.as_raw(),
                ),
                TextureOptions::LINEAR,
            ),
            blurred: ctx.load_texture(
                format!("album_{texture_name_base}_blurred"),
                ColorImage::from_rgba_unmultiplied(
                    [blurred_img.width() as usize, blurred_img.height() as usize],
                    blurred_img.as_raw(),
                ),
                TextureOptions::LINEAR,
            ),
        })
    }

    /// Adds a custom wgpu callback to render the warped, blurred album art as a background.
    fn add_dynamic_background(
        &self,
        painter: &egui::Painter,
        ctx: &Context,
        frame: &eframe::Frame,
        rect: Rect,
        album_art: &AlbumArtTextures,
    ) {
        let render_state = frame
            .wgpu_render_state()
            .expect("Cantus requires a wgpu render state");

        // Retrieve the wgpu TextureView for the blurred egui texture.
        let renderer = render_state.renderer.read();
        let Some(wgpu_texture) = renderer
            .texture(&album_art.blurred.id())
            .and_then(|t| t.texture.as_ref())
        else {
            return;
        };
        let texture_view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
        drop(renderer);

        let sampler = render_state
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label: Some("cantus_warp_sampler"),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                address_mode_u: wgpu::AddressMode::MirrorRepeat,
                address_mode_v: wgpu::AddressMode::MirrorRepeat,
                address_mode_w: wgpu::AddressMode::MirrorRepeat,
                ..Default::default()
            });

        let pixels_per_point = ctx.pixels_per_point();
        let width_px = (rect.width() * pixels_per_point).max(1.0);
        let height_px = (rect.height() * pixels_per_point).max(1.0);

        let texture_size = album_art.blurred.size_vec2();
        let uniforms = WarpUniforms {
            resolution: [width_px, height_px, 1.0 / width_px, 1.0 / height_px],
            params: [
                self.start_time.elapsed().as_secs_f32() * WARP_TIME_SCALE,
                WARP_STRENGTH,
                SWIRL_STRENGTH,
                texture_size.x / texture_size.y,
            ],
        };

        painter.add(WgpuCallback::new_paint_callback(
            rect,
            WarpedBackgroundCallback {
                texture_view,
                sampler,
                uniforms,
                target_format: render_state.target_format,
            },
        ));
    }
}

impl eframe::App for CantusApp {
    /// The main update loop for the application.
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Poll for new track info every second
        if self.last_poll.elapsed() > Duration::from_secs(1) {
            self.refresh_track(ctx);
            self.last_poll = Instant::now();
        }

        // Process any album art response received asynchronously
        if let Ok(mut response_lock) = self.art_response.lock()
            && let Some((url, bytes)) = response_lock.take()
            && let AlbumArtState::Loading(loading_url) = &self.art_state
            && loading_url == &url
        {
            match Self::process_album_art_bytes(ctx, &bytes) {
                Ok(textures) => {
                    self.art_state = AlbumArtState::Loaded(url, textures);
                }
                Err(err) => {
                    warn!("Failed to process album art bytes for {url}: {err}");
                    self.art_state = AlbumArtState::None;
                }
            }
        }

        CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let full_rect = ui.max_rect();
                let painter = ui.painter_at(full_rect);

                // Render the blurred album art over the entire background
                if let AlbumArtState::Loaded(_, album_art) = &self.art_state {
                    self.add_dynamic_background(&painter, ctx, frame, full_rect, album_art);
                    painter.rect_filled(
                        full_rect,
                        0.0,
                        Color32::from_rgba_unmultiplied(10, 10, 10, 100),
                    );
                }

                let content_rect = full_rect.shrink2(Vec2::splat(PANEL_MARGIN));

                let album_art_drawn = if let AlbumArtState::Loaded(_, album_art) = &self.art_state {
                    let texture_size = album_art.original.size_vec2();
                    let art_edge = content_rect.height().max(0.0);
                    let art_rect = Rect::from_center_size(
                        pos2(
                            art_edge.mul_add(0.5, content_rect.min.x),
                            content_rect.center().y,
                        ),
                        Vec2::splat(art_edge),
                    );

                    // Calculate UV coordinates to crop the image to a square aspect ratio (center crop).
                    let uv_rect = if texture_size.x >= texture_size.y {
                        let crop = (texture_size.x - texture_size.y) / (2.0 * texture_size.x);
                        Rect::from_min_max(pos2(crop, 0.0), pos2(1.0 - crop, 1.0))
                    } else {
                        let crop = (texture_size.y - texture_size.x) / (2.0 * texture_size.y);
                        Rect::from_min_max(pos2(0.0, crop), pos2(1.0, 1.0 - crop))
                    };

                    painter.image(album_art.original.id(), art_rect, uv_rect, Color32::WHITE);

                    Some((art_rect, texture_size))
                } else {
                    None
                };

                let text_color = if matches!(self.art_state, AlbumArtState::Loaded(..)) {
                    Color32::from_rgb(240, 240, 240)
                } else {
                    ui.visuals().strong_text_color()
                };

                let text_start_x = album_art_drawn
                    .map_or(content_rect.min.x, |(art_rect, _)| art_rect.max.x + 10.0);

                let mut lines: Vec<(String, FontId, Color32)> = Vec::new();
                if let Some(track) = &self.track {
                    lines.push((track.title.clone(), FontId::proportional(20.0), text_color));
                    lines.push((track.artist.clone(), FontId::proportional(16.0), text_color));
                    if let Some(album) = &track.album
                        && !album.is_empty()
                    {
                        lines.push((
                            album.clone(),
                            FontId::proportional(14.0),
                            text_color.gamma_multiply(0.85),
                        ));
                    }
                } else {
                    lines.push((
                        "Nothing playing right now.".to_owned(),
                        FontId::proportional(16.0),
                        text_color,
                    ));
                }

                let text_rows: Vec<_> = lines
                    .into_iter()
                    .map(|(text, font_id, color)| {
                        let row_height = font_id.size;
                        (text, font_id, color, row_height)
                    })
                    .collect();

                let text_height: f32 = text_rows.iter().map(|(_, _, _, height)| *height).sum();
                let gap_count = text_rows.len().saturating_sub(1);
                // Calculate total height needed for text rows and the 4.0pt gaps between them.
                let gap_height = f32::from(gap_count as u8) * 4.0;
                let total_height = text_height + gap_height;
                let half_total_height = total_height * 0.5;
                let mut current_y = content_rect.center().y - half_total_height;

                for (text, font_id, color, row_height) in text_rows {
                    painter.text(
                        pos2(text_start_x, current_y),
                        Align2::LEFT_TOP,
                        text,
                        font_id,
                        color,
                    );
                    current_y += row_height + 4.0;
                }
            });

        // Ensure it refreshes every frame
        ctx.request_repaint();
    }
}

/// Uniform data passed to the warp shader.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
struct WarpUniforms {
    resolution: [f32; 4],
    params: [f32; 4],
}

/// Custom egui callback to render the warped background using wgpu.
struct WarpedBackgroundCallback {
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    uniforms: WarpUniforms,
    target_format: wgpu::TextureFormat,
}

/// Resources needed to render the warped background effect.
struct PipelineResources {
    pipeline: wgpu::RenderPipeline,
    texture_layout: wgpu::BindGroupLayout,
    uniform_layout: wgpu::BindGroupLayout,
}

impl PipelineResources {
    /// Creates the wgpu render pipeline and bind group layouts for the warp effect.
    fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cantus_warp_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(WARP_SHADER)),
        });

        // Bind Group Layout 0: Texture and Sampler
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cantus_warp_texture_layout"),
            entries: TEXTURE_BIND_GROUP_ENTRIES,
        });

        // Bind Group Layout 1: Uniforms
        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cantus_warp_uniform_layout"),
            entries: UNIFORM_BIND_GROUP_ENTRIES,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("cantus_warp_pipeline"),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("cantus_warp_pipeline_layout"),
                    bind_group_layouts: &[&texture_layout, &uniform_layout],
                    push_constant_ranges: &[],
                }),
            ),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            texture_layout,
            uniform_layout,
        }
    }
}

/// Caches wgpu resources (pipelines, buffers, bind groups) across frames.
#[derive(Default)]
struct WarpedBackgroundCache {
    pipelines: HashMap<wgpu::TextureFormat, PipelineResources>,
    uniform_buffer: Option<wgpu::Buffer>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    texture_bind_group: Option<wgpu::BindGroup>,
}

impl CallbackTrait for WarpedBackgroundCallback {
    /// Prepares necessary wgpu resources (pipeline, uniform buffer, bind groups)
    /// and uploads uniform data to the GPU.
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let cache = callback_resources
            .entry::<WarpedBackgroundCache>()
            .or_insert_with(Default::default);

        let pipeline_resources = cache
            .pipelines
            .entry(self.target_format)
            .or_insert_with(|| PipelineResources::new(device, self.target_format));

        // Ensure Uniform Buffer exists
        if cache.uniform_buffer.is_none() {
            cache.uniform_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("cantus_warp_uniform_buffer"),
                size: std::mem::size_of::<WarpUniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Ensure Uniform Bind Group exists
        if cache.uniform_bind_group.is_none() {
            cache.uniform_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("cantus_warp_uniform_bind_group"),
                layout: &pipeline_resources.uniform_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: cache.uniform_buffer.as_ref().unwrap().as_entire_binding(),
                }],
            }));
        }

        // Create Texture Bind Group
        cache.texture_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cantus_warp_texture_bind_group"),
            layout: &pipeline_resources.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        }));

        // Upload dynamic uniform data
        queue.write_buffer(
            cache.uniform_buffer.as_ref().unwrap(),
            0,
            bytemuck::bytes_of(&self.uniforms),
        );

        Vec::new()
    }

    /// Executes the rendering commands for the warp effect.
    fn paint(
        &self,
        _info: egui::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &CallbackResources,
    ) {
        let Some(cache) = callback_resources.get::<WarpedBackgroundCache>() else {
            return;
        };
        let (Some(pipeline_resources), Some(texture_bind_group), Some(uniform_bind_group)) = (
            cache.pipelines.get(&self.target_format),
            &cache.texture_bind_group,
            &cache.uniform_bind_group,
        ) else {
            return;
        };

        render_pass.set_pipeline(&pipeline_resources.pipeline);
        render_pass.set_bind_group(0, texture_bind_group, &[]);
        render_pass.set_bind_group(1, uniform_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}

/// Holds relevant metadata for the currently playing track.
struct TrackInfo {
    title: String,
    artist: String,
    album: Option<String>,
    album_art_url: Option<String>,
}

impl TrackInfo {
    /// Creates a `TrackInfo` from MPRIS metadata.
    fn from_metadata(metadata: &mpris::Metadata) -> Self {
        let title = metadata
            .title()
            .map_or_else(|| "Unknown Title".to_owned(), ToOwned::to_owned);

        let artist = metadata
            .artists()
            .filter(|artists| !artists.is_empty())
            .map_or_else(|| "Unknown Artist".to_owned(), |artists| artists.join(", "));

        let album = metadata.album_name().map(std::borrow::ToOwned::to_owned);
        let album_art_url = metadata
            .art_url()
            .map(std::borrow::ToOwned::to_owned)
            .filter(|url| !url.is_empty());

        Self {
            title,
            artist,
            album,
            album_art_url,
        }
    }
}

/// Holds the egui texture handles for the album art.
struct AlbumArtTextures {
    original: TextureHandle,
    blurred: TextureHandle,
}

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
    TextureOptions, Vec2, pos2,
};
use log::warn;
use mpris::PlayerFinder;
use std::{
    borrow::Cow,
    collections::HashMap,
    convert::TryFrom,
    fs,
    io::Read,
    time::{Duration, Instant},
};
use url::Url;

const PANEL_MARGIN: f32 = 12.0;
const BLUR_SIGMA: f32 = 60.0;
const WARP_STRENGTH: f32 = 2.0;
const SWIRL_STRENGTH: f32 = 0.4;

const WARP_SHADER: &str = include_str!("warp_background.wgsl");

fn main() -> eframe::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Cantus")
            .with_app_id("cantus")
            .with_inner_size([320.0, 120.0])
            .with_active(false)
            .with_window_level(egui::WindowLevel::AlwaysOnTop)
            .with_decorations(false),
        hardware_acceleration: HardwareAcceleration::Required,
        wgpu_options: WgpuConfiguration {
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: Some(2),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Cantus",
        native_options,
        Box::new(|_cc| Ok(Box::new(CantusApp::new()))),
    )
}

struct CantusApp {
    last_poll: Instant,
    track: Option<TrackInfo>,
    album_art: Option<AlbumArtTextures>,
    current_art_url: Option<String>,
    next_art_retry: Option<Instant>,
    texture_seq: u64,
    start_time: Instant,
}

impl CantusApp {
    fn new() -> Self {
        let mut app = Self {
            last_poll: Instant::now(),
            track: None,
            album_art: None,
            current_art_url: None,
            next_art_retry: None,
            texture_seq: 0,
            start_time: Instant::now(),
        };

        app.refresh_track();
        app
    }

    fn refresh_track(&mut self) {
        let art_was = self
            .track
            .as_ref()
            .and_then(|track| track.album_art_url.as_deref());

        let track = PlayerFinder::new()
            .ok()
            .and_then(|finder| finder.find_active().ok())
            .and_then(|player| player.get_metadata().ok())
            .map(|metadata| TrackInfo::from_metadata(&metadata));

        let track_missing = track.is_none();
        let art_changed = art_was != track.as_ref().and_then(|t| t.album_art_url.as_deref());

        self.track = track;

        if track_missing || art_changed {
            self.album_art = None;
            self.current_art_url = None;
            self.next_art_retry = None;
        }
    }

    fn ensure_album_art(&mut self, ctx: &Context) {
        let Some(url) = self
            .track
            .as_ref()
            .and_then(|track| track.album_art_url.as_ref())
            .filter(|value| !value.is_empty())
            .cloned()
        else {
            self.album_art = None;
            self.current_art_url = None;
            self.next_art_retry = None;
            return;
        };

        let url_str = url.as_str();
        let same_url = self.current_art_url.as_deref() == Some(url_str);

        if same_url && self.album_art.is_some() {
            return;
        }

        if same_url
            && self
                .next_art_retry
                .is_some_and(|instant| Instant::now() < instant)
        {
            return;
        }

        if !same_url {
            self.album_art = None;
            self.next_art_retry = None;
        }

        match self.load_album_art(ctx, url_str) {
            Ok(textures) => {
                self.album_art = Some(textures);
                self.next_art_retry = None;
            }
            Err(err) => {
                warn!("Failed to load album art from {url}: {err}");
                self.album_art = None;
                self.next_art_retry = Some(Instant::now() + Duration::from_secs(10));
            }
        }

        self.current_art_url = Some(url);
    }

    fn load_album_art(&mut self, ctx: &Context, url: &str) -> Result<AlbumArtTextures, String> {
        let fetch_remote = |url: &str| -> Result<Vec<u8>, String> {
            let mut bytes = Vec::new();
            let response = ureq::get(url)
                .call()
                .map_err(|err| format!("HTTP request failed: {err}"))?;

            let status = response.status();
            if !status.is_success() {
                return Err(format!("HTTP request returned status {}", status.as_u16()));
            }

            response
                .into_body()
                .into_reader()
                .read_to_end(&mut bytes)
                .map_err(|err| format!("failed to read HTTP body: {err}"))?;

            Ok(bytes)
        };

        let bytes = match Url::parse(url) {
            Ok(parsed) if parsed.scheme() == "file" => {
                let path = parsed
                    .to_file_path()
                    .map_err(|()| format!("Unsupported file path in URL: {url}"))?;
                fs::read(&path).map_err(|err| {
                    format!("failed to read album art from {}: {err}", path.display())
                })?
            }
            Ok(parsed) if parsed.scheme() == "http" || parsed.scheme() == "https" => {
                fetch_remote(url)?
            }
            Ok(_) => fetch_remote(url)?,
            Err(_) if url.starts_with('/') => fs::read(url)
                .map_err(|err| format!("failed to read album art from {url}: {err}"))?,
            Err(_) => fetch_remote(url)?,
        };

        let rgba = image::load_from_memory(&bytes)
            .map_err(|err| format!("failed to decode album art image: {err}"))?
            .to_rgba8();
        let blurred = image::imageops::blur(&rgba, BLUR_SIGMA);

        let next_name = |seq: &mut u64| {
            *seq = seq.wrapping_add(1);
            format!("album_art_{seq:010}")
        };

        let original = ctx.load_texture(
            next_name(&mut self.texture_seq),
            ColorImage::from_rgba_unmultiplied(
                [rgba.width() as usize, rgba.height() as usize],
                rgba.as_raw(),
            ),
            TextureOptions::LINEAR,
        );

        let blurred = ctx.load_texture(
            next_name(&mut self.texture_seq),
            ColorImage::from_rgba_unmultiplied(
                [blurred.width() as usize, blurred.height() as usize],
                blurred.as_raw(),
            ),
            TextureOptions::LINEAR,
        );

        Ok(AlbumArtTextures { original, blurred })
    }

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

        let texture_id = album_art.blurred.id();
        let texture_view = {
            let renderer = render_state.renderer.read();
            let Some(texture) = renderer.texture(&texture_id) else {
                return;
            };
            let Some(wgpu_texture) = texture.texture.as_ref() else {
                return;
            };

            let view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
            drop(renderer);
            view
        };

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

        let size = rect.size();
        let pixels_per_point = ctx.pixels_per_point();
        let width_px = (size.x * pixels_per_point).max(1.0);
        let height_px = (size.y * pixels_per_point).max(1.0);

        let elapsed_time = self.start_time.elapsed().as_secs_f32() * 0.6;
        let texture_size = album_art.blurred.size_vec2();
        let texture_aspect = if texture_size.y > 0.0 {
            texture_size.x / texture_size.y
        } else {
            1.0
        };
        let uniforms = WarpUniforms {
            resolution: [width_px, height_px, 1.0 / width_px, 1.0 / height_px],
            params: [elapsed_time, WARP_STRENGTH, SWIRL_STRENGTH, texture_aspect],
        };

        let callback = WarpedBackgroundCallback::new(
            texture_view,
            sampler,
            uniforms,
            render_state.target_format,
        );

        painter.add(WgpuCallback::new_paint_callback(rect, callback));
    }
}

impl eframe::App for CantusApp {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if self.last_poll.elapsed() >= Duration::from_millis(500) {
            self.refresh_track();
            self.last_poll = Instant::now();
        }

        self.ensure_album_art(ctx);

        CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let full_rect = ui.max_rect();
                let painter = ui.painter_at(full_rect);

                // Render the blurred album art over the entire background
                if let Some(album_art) = &self.album_art {
                    self.add_dynamic_background(&painter, ctx, frame, full_rect, album_art);
                    painter.rect_filled(
                        full_rect,
                        0.0,
                        Color32::from_rgba_unmultiplied(10, 10, 10, 170),
                    );
                }

                let content_rect = full_rect.shrink2(Vec2::splat(PANEL_MARGIN));

                let album_art_drawn = self.album_art.as_ref().map(|album_art| {
                    let texture_size = album_art.original.size_vec2();
                    let art_edge = content_rect.height().max(0.0);
                    let art_rect = Rect::from_center_size(
                        pos2(
                            art_edge.mul_add(0.5, content_rect.min.x),
                            content_rect.center().y,
                        ),
                        Vec2::splat(art_edge),
                    );

                    let uv_rect = if texture_size.x > texture_size.y {
                        let crop = (texture_size.x - texture_size.y) / (2.0 * texture_size.x);
                        Rect::from_min_max(pos2(crop, 0.0), pos2(1.0 - crop, 1.0))
                    } else if texture_size.y > texture_size.x {
                        let crop = (texture_size.y - texture_size.x) / (2.0 * texture_size.y);
                        Rect::from_min_max(pos2(0.0, crop), pos2(1.0, 1.0 - crop))
                    } else {
                        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0))
                    };

                    painter.image(album_art.original.id(), art_rect, uv_rect, Color32::WHITE);

                    (art_rect, texture_size)
                });

                let text_color = if self.album_art.is_some() {
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
                let gap_height = u32::try_from(gap_count).map_or_else(
                    |_| (f64::from(u32::MAX) * 4.0) as f32,
                    |count| (f64::from(count) * 4.0) as f32,
                );
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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
struct WarpUniforms {
    resolution: [f32; 4],
    params: [f32; 4],
}

struct WarpedBackgroundCallback {
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    uniforms: WarpUniforms,
    target_format: wgpu::TextureFormat,
}

impl WarpedBackgroundCallback {
    const fn new(
        texture_view: wgpu::TextureView,
        sampler: wgpu::Sampler,
        uniforms: WarpUniforms,
        target_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            texture_view,
            sampler,
            uniforms,
            target_format,
        }
    }
}

struct PipelineResources {
    pipeline: wgpu::RenderPipeline,
    texture_layout: wgpu::BindGroupLayout,
    uniform_layout: wgpu::BindGroupLayout,
}

impl PipelineResources {
    fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cantus_warp_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(WARP_SHADER)),
        });

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cantus_warp_texture_layout"),
            entries: &[
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
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cantus_warp_uniform_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cantus_warp_pipeline_layout"),
            bind_group_layouts: &[&texture_layout, &uniform_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("cantus_warp_pipeline"),
            layout: Some(&pipeline_layout),
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
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
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

#[derive(Default)]
struct WarpedBackgroundCache {
    pipelines: HashMap<wgpu::TextureFormat, PipelineResources>,
    uniform_buffer: Option<wgpu::Buffer>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    texture_bind_group: Option<wgpu::BindGroup>,
}

impl CallbackTrait for WarpedBackgroundCallback {
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

        if cache.uniform_buffer.is_none() {
            cache.uniform_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("cantus_warp_uniform_buffer"),
                size: std::mem::size_of::<WarpUniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

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

        queue.write_buffer(
            cache.uniform_buffer.as_ref().unwrap(),
            0,
            bytemuck::bytes_of(&self.uniforms),
        );

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &CallbackResources,
    ) {
        let Some(cache) = callback_resources.get::<WarpedBackgroundCache>() else {
            return;
        };
        let Some(pipeline_resources) = cache.pipelines.get(&self.target_format) else {
            return;
        };
        let Some(texture_bind_group) = &cache.texture_bind_group else {
            return;
        };
        let Some(uniform_bind_group) = &cache.uniform_bind_group else {
            return;
        };

        render_pass.set_pipeline(&pipeline_resources.pipeline);
        render_pass.set_bind_group(0, texture_bind_group, &[]);
        render_pass.set_bind_group(1, uniform_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}

struct TrackInfo {
    title: String,
    artist: String,
    album: Option<String>,
    album_art_url: Option<String>,
}

impl TrackInfo {
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

struct AlbumArtTextures {
    original: TextureHandle,
    blurred: TextureHandle,
}

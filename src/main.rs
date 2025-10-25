use anyhow::{Result, bail};
use parley::{
    FontContext, FontWeight, Layout, LayoutContext, layout::PositionedLayoutItem,
    style::StyleProperty,
};
use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use rspotify::model::PlayableItem;
use std::{ffi::c_void, ptr::NonNull, sync::Arc};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, Glyph, Renderer, RendererOptions, Scene,
    kurbo::{Affine, RoundedRect},
    peniko::{Blob, Color, Fill, color::palette},
    util::{RenderContext, RenderSurface},
    wgpu::{
        CommandEncoderDescriptor, CompositeAlphaMode, PollType, PresentMode, SurfaceError,
        SurfaceTargetUnsafe, TextureViewDescriptor,
    },
};
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle,
    protocol::{
        wl_callback::{self, WlCallback},
        wl_compositor::{self, WlCompositor},
        wl_output::{self, WlOutput},
        wl_registry::{self, WlRegistry},
        wl_surface::{self, WlSurface},
    },
};
use wayland_protocols::wp::{
    fractional_scale::v1::client::{
        wp_fractional_scale_manager_v1::{self, WpFractionalScaleManagerV1},
        wp_fractional_scale_v1::{self, WpFractionalScaleV1},
    },
    viewporter::client::{
        wp_viewport::{self, WpViewport},
        wp_viewporter::{self, WpViewporter},
    },
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, ZwlrLayerSurfaceV1},
};

use crate::spotify::CURRENT_SONGS;

mod spotify;

const PANEL_WIDTH: f64 = 600.0;
const PANEL_HEIGHT: f64 = 80.0;

const PANEL_MARGIN: f64 = 3.0;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(LevelFilter::WARN.to_string())),
        )
        .init();

    spotify::init().await.unwrap();
    run_layer_shell();
}

/// Initialize the Wayland layer shell and create a layer surface.
fn run_layer_shell() {
    let connection = Connection::connect_to_env().unwrap();
    let mut event_queue = connection.new_event_queue();
    let qh = event_queue.handle();

    let _registry = connection.display().get_registry(&qh, ());

    let display_ptr = NonNull::new(connection.backend().display_ptr().cast::<c_void>()).unwrap();

    let mut app = CantusLayer::new(display_ptr);

    // Initial roundtrip to get globals
    if let Err(err) = event_queue.roundtrip(&mut app) {
        error!("Initial roundtrip failed: {:?}", err);
        return;
    }

    // Now extract bound globals
    let Some(compositor) = app.compositor.take() else {
        error!("No wl_compositor global available");
        return;
    };
    let Some(layer_shell) = app.layer_shell.take() else {
        error!("No zwlr_layer_shell_v1 global available");
        return;
    };

    let viewporter = app.viewporter.take();
    let fractional_manager = app.fractional_manager.take();
    if app.outputs.is_empty() {
        error!("No Wayland outputs found");
        return;
    }

    let output = app.outputs.remove(0); // Use first output (primary)
    let wl_surface = compositor.create_surface(&qh, ());
    let surface_ptr = NonNull::new(wl_surface.id().as_ptr().cast::<c_void>()).unwrap();

    app.wl_surface = Some(wl_surface);
    app.surface_ptr = Some(surface_ptr);
    app.output = Some(output);

    // Attach viewport and fractional scale BEFORE creating layer surface
    if let (Some(viewporter), Some(fractional_manager)) = (viewporter, fractional_manager) {
        app.viewport = Some(viewporter.get_viewport(app.wl_surface.as_ref().unwrap(), &qh, ()));
        app.fractional = Some(fractional_manager.get_fractional_scale(
            app.wl_surface.as_ref().unwrap(),
            &qh,
            (),
        ));
    }

    // Create the layer surface
    let layer_surface = layer_shell.get_layer_surface(
        app.wl_surface.as_ref().unwrap(),
        Some(app.output.as_ref().unwrap()),
        zwlr_layer_shell_v1::Layer::Top,
        "cantus".to_string(),
        &qh,
        (),
    );
    app.layer_surface = Some(layer_surface);

    // Configure layer surface
    app.layer_surface
        .as_ref()
        .unwrap()
        .set_size(PANEL_WIDTH as u32, PANEL_HEIGHT as u32);
    app.layer_surface
        .as_ref()
        .unwrap()
        .set_anchor(zwlr_layer_surface_v1::Anchor::Top | zwlr_layer_surface_v1::Anchor::Left);
    app.layer_surface
        .as_ref()
        .unwrap()
        .set_margin(PANEL_MARGIN as i32, 0, 0, PANEL_MARGIN as i32);
    app.layer_surface.as_ref().unwrap().set_exclusive_zone(0);

    // Commit to trigger configure
    app.wl_surface.as_ref().unwrap().commit();

    // Flush to send the commit
    if let Err(err) = connection.flush() {
        error!("Failed to flush initial commit: {:?}", err);
        return;
    }

    // Wait for initial configure
    app.is_configured = false;
    while !app.is_configured && !app.should_exit {
        if let Err(err) = event_queue.blocking_dispatch(&mut app) {
            error!("Waiting for configure failed: {:?}", err);
            return;
        }
    }

    // Event loop
    while !app.should_exit {
        if let Err(err) = event_queue.blocking_dispatch(&mut app) {
            error!("Wayland dispatch error: {:?}", err);
            app.should_exit = true;
        }
    }
}

struct CantusLayer {
    wl_surface: Option<WlSurface>,
    layer_surface: Option<ZwlrLayerSurfaceV1>,
    output: Option<WlOutput>,
    outputs: Vec<WlOutput>,
    compositor: Option<WlCompositor>,
    layer_shell: Option<ZwlrLayerShellV1>,
    viewporter: Option<WpViewporter>,
    fractional_manager: Option<WpFractionalScaleManagerV1>,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    render_context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    render_surface: Option<RenderSurface<'static>>,
    scene: Scene,
    font_context: FontContext,
    layout_context: LayoutContext<()>,
    logical_width: f64,
    logical_height: f64,
    scale_factor: f64,
    frame_callback: Option<WlCallback>,
    should_exit: bool,
    is_configured: bool,
    display_ptr: NonNull<c_void>,
    surface_ptr: Option<NonNull<c_void>>,
}

impl CantusLayer {
    fn new(display_ptr: NonNull<c_void>) -> Self {
        let mut font_context = FontContext::new();
        font_context.collection.register_fonts(
            Blob::new(Arc::new(include_bytes!("../assets/epilogue.ttf"))),
            None,
        );
        // Verify the font was added correctly
        font_context.collection.family_id("epilogue").unwrap();

        Self {
            wl_surface: None,
            layer_surface: None,
            output: None,
            outputs: Vec::new(),
            compositor: None,
            layer_shell: None,
            viewporter: None,
            fractional_manager: None,
            viewport: None,
            fractional: None,
            render_context: RenderContext::new(),
            renderers: Vec::new(),
            render_surface: None,
            scene: Scene::new(),
            font_context,
            layout_context: LayoutContext::new(),
            logical_width: PANEL_WIDTH,
            logical_height: PANEL_HEIGHT,
            scale_factor: 1.0,
            frame_callback: None,
            should_exit: false,
            is_configured: false,
            display_ptr,
            surface_ptr: None,
        }
    }

    fn request_frame(&mut self, qh: &QueueHandle<Self>) {
        if self.frame_callback.is_some() {
            return;
        }
        let callback = self.wl_surface.as_ref().unwrap().frame(qh, ());
        self.frame_callback = Some(callback);
    }

    fn ensure_surface(&mut self, buffer_width: f64, buffer_height: f64) -> Result<()> {
        if buffer_width == 0.0 || buffer_height == 0.0 {
            return Ok(());
        }

        // Check if we need to create or resize
        let needs_creation = self.render_surface.is_none();
        let needs_resize = self.render_surface.as_ref().is_some_and(|surface| {
            surface.config.width != buffer_width as u32
                || surface.config.height != buffer_height as u32
        });

        if !needs_creation && !needs_resize {
            return Ok(());
        }

        if needs_resize
            && !needs_creation
            && let Some(surface) = &mut self.render_surface
        {
            self.render_context
                .resize_surface(surface, buffer_width as u32, buffer_height as u32);
            return Ok(());
        }

        // Only create if configured
        if !self.is_configured {
            return Ok(());
        }

        let Some(surface_ptr) = self.surface_ptr else {
            return Ok(());
        };

        let raw_display_handle =
            RawDisplayHandle::Wayland(WaylandDisplayHandle::new(self.display_ptr));
        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(surface_ptr));
        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle,
            raw_window_handle,
        };

        let surface = unsafe { self.render_context.instance.create_surface_unsafe(target) }?;

        let mut render_surface = pollster::block_on(self.render_context.create_render_surface(
            surface,
            buffer_width as u32,
            buffer_height as u32,
            PresentMode::Fifo,
        ))?;

        if render_surface.config.alpha_mode != CompositeAlphaMode::PreMultiplied {
            render_surface.config.alpha_mode = CompositeAlphaMode::PreMultiplied;
            render_surface.surface.configure(
                &self.render_context.devices[render_surface.dev_id].device,
                &render_surface.config,
            );
        }

        self.renderers
            .resize_with(self.render_context.devices.len(), || None);
        self.render_surface = Some(render_surface);
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        let Some(surface) = &mut self.render_surface else {
            return Ok(());
        };

        self.scene.reset();
        create_scene(
            &mut self.scene,
            &mut self.font_context,
            &mut self.layout_context,
            f64::from(surface.config.width),
            f64::from(surface.config.height),
            self.scale_factor,
        );

        let dev_id = surface.dev_id;
        let device_handle = &self.render_context.devices[dev_id];

        if self.renderers[dev_id].is_none() {
            let renderer = Renderer::new(&device_handle.device, RendererOptions::default())?;
            self.renderers[dev_id] = Some(renderer);
        }

        let renderer = self.renderers[dev_id]
            .as_mut()
            .expect("renderer must be initialized");

        renderer.render_to_texture(
            &device_handle.device,
            &device_handle.queue,
            &self.scene,
            &surface.target_view,
            &vello::RenderParams {
                base_color: palette::css::TRANSPARENT,
                width: surface.config.width,
                height: surface.config.height,
                antialiasing_method: AaConfig::Area,
            },
        )?;

        let surface_texture = match surface.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(SurfaceError::Outdated | SurfaceError::Lost) => {
                self.render_surface = None;
                return Ok(());
            }
            Err(SurfaceError::Timeout) => return Ok(()),
            Err(err) => bail!("Failed to acquire surface texture: {err:?}"),
        };

        let mut encoder = device_handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Cantus Surface Blit"),
            });
        surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &surface.target_view,
            &surface_texture
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );
        device_handle.queue.submit([encoder.finish()]);
        surface_texture.present();

        device_handle.device.poll(PollType::Poll)?;

        Ok(())
    }

    fn update_scale_and_viewport(&self) {
        let buffer_width = (self.logical_width * self.scale_factor).round() as u32;
        let buffer_height = (self.logical_height * self.scale_factor).round() as u32;

        let buffer_scale = if self.viewport.is_some() && self.fractional.is_some() {
            1
        } else {
            self.scale_factor.ceil() as i32
        };

        if let Some(surface) = &self.wl_surface {
            surface.set_buffer_scale(buffer_scale);
        }

        if let Some(viewport) = &self.viewport {
            viewport.set_source(0.0, 0.0, f64::from(buffer_width), f64::from(buffer_height));
            viewport.set_destination(self.logical_width as i32, self.logical_height as i32);
        }
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for CantusLayer {
    fn event(
        state: &mut Self,
        proxy: &ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        match event {
            zwlr_layer_surface_v1::Event::Configure {
                serial,
                width,
                height,
            } => {
                // Step 1: Update dimensions
                state.logical_width = if width == 0 {
                    PANEL_WIDTH
                } else {
                    f64::from(width)
                };
                state.logical_height = if height == 0 {
                    PANEL_HEIGHT
                } else {
                    f64::from(height)
                };

                // Step 2: Acknowledge immediately
                proxy.ack_configure(serial);

                // Step 3: Update scale and viewport
                state.update_scale_and_viewport();

                // Step 4: Commit all changes together
                if let Some(surface) = &state.wl_surface {
                    surface.commit();
                }

                // Step 5: Mark as configured
                state.is_configured = true;

                // Step 6: Create surface if needed (only after configured)
                let buffer_width = (state.logical_width * state.scale_factor).round();
                let buffer_height = (state.logical_height * state.scale_factor).round();

                if let Err(err) = state.ensure_surface(buffer_width, buffer_height) {
                    error!("Failed to prepare render surface: {err:?}");
                    state.should_exit = true;
                    return;
                }

                // Step 7: Render and request frame
                if let Err(err) = state.render() {
                    error!("Rendering failed: {err:?}");
                }
                state.request_frame(qhandle);
            }
            zwlr_layer_surface_v1::Event::Closed => {
                state.should_exit = true;
            }
            _ => {}
        }
    }
}

impl Dispatch<WpFractionalScaleV1, ()> for CantusLayer {
    fn event(
        state: &mut Self,
        _proxy: &WpFractionalScaleV1,
        event: wp_fractional_scale_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let wp_fractional_scale_v1::Event::PreferredScale { scale } = event {
            state.scale_factor = f64::from(scale) / 120.0;

            if state.is_configured {
                state.update_scale_and_viewport();

                if let Some(surface) = &state.wl_surface {
                    surface.commit();
                }

                let buffer_width = (state.logical_width * state.scale_factor).round();
                let buffer_height = (state.logical_height * state.scale_factor).round();

                if let Err(err) = state.ensure_surface(buffer_width, buffer_height) {
                    error!("Failed to prepare render surface: {err:?}");
                    state.should_exit = true;
                    return;
                }

                if let Err(err) = state.render() {
                    error!("Rendering failed after scale change: {err:?}");
                }
            }
        }
    }
}

impl Dispatch<WlCallback, ()> for CantusLayer {
    fn event(
        state: &mut Self,
        _proxy: &WlCallback,
        event: wl_callback::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_callback::Event::Done { .. } = event {
            state.frame_callback.take();
            if let Err(err) = state.render() {
                error!("Rendering failed: {err:?}");
                return;
            }
            state.request_frame(qhandle);
        }
    }
}

// Simplified Dispatch implementations (no changes needed)
impl Dispatch<WlSurface, ()> for CantusLayer {
    fn event(
        _state: &mut Self,
        _proxy: &WlSurface,
        _event: wl_surface::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlOutput, ()> for CantusLayer {
    fn event(
        _state: &mut Self,
        _proxy: &WlOutput,
        _event: wl_output::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlRegistry, ()> for CantusLayer {
    fn event(
        state: &mut Self,
        registry: &WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_ref() {
                "wl_compositor" => {
                    state.compositor =
                        Some(registry.bind::<WlCompositor, (), Self>(name, version, qh, ()));
                }
                "zwlr_layer_shell_v1" => {
                    state.layer_shell =
                        Some(registry.bind::<ZwlrLayerShellV1, (), Self>(name, 4, qh, ()));
                }
                "wp_viewporter" => {
                    state.viewporter =
                        Some(registry.bind::<WpViewporter, (), Self>(name, 1, qh, ()));
                }
                "wp_fractional_scale_manager_v1" => {
                    state.fractional_manager = Some(
                        registry.bind::<WpFractionalScaleManagerV1, (), Self>(name, 1, qh, ()),
                    );
                }
                "wl_output" => {
                    state
                        .outputs
                        .push(registry.bind::<WlOutput, (), Self>(name, version, qh, ()));
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<ZwlrLayerShellV1, ()> for CantusLayer {
    fn event(
        _state: &mut Self,
        _registry: &ZwlrLayerShellV1,
        _event: zwlr_layer_shell_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WpFractionalScaleManagerV1, ()> for CantusLayer {
    fn event(
        _state: &mut Self,
        _registry: &WpFractionalScaleManagerV1,
        _event: wp_fractional_scale_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WpViewporter, ()> for CantusLayer {
    fn event(
        _state: &mut Self,
        _registry: &WpViewporter,
        _event: wp_viewporter::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WpViewport, ()> for CantusLayer {
    fn event(
        _state: &mut Self,
        _registry: &WpViewport,
        _event: wp_viewport::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlCompositor, ()> for CantusLayer {
    fn event(
        _state: &mut Self,
        _registry: &WlCompositor,
        _event: wl_compositor::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

fn create_scene(
    scene: &mut Scene,
    font_context: &mut FontContext,
    layout_context: &mut LayoutContext<()>,
    width: f64,
    height: f64,
    scale_factor: f64,
) {
    let scaled_panel_margin = PANEL_MARGIN * scale_factor;

    // Draw a rectangle filling the screen
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::new([0.9, 0.5, 0.6, 1.0]),
        None,
        &RoundedRect::new(0.0, 0.0, width, height, 14.0 * scale_factor),
    );

    // Draw the album art
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::new([0.5, 0.0, 0.0, 1.0]),
        None,
        &RoundedRect::new(
            scaled_panel_margin,
            scaled_panel_margin,
            height - scaled_panel_margin,
            height - scaled_panel_margin,
            10.0 * scale_factor,
        ),
    );

    // Draw the text for song, album, and artist
    let album_art_right_edge = scaled_panel_margin + (height - 2.0 * scaled_panel_margin);
    let text_x = album_art_right_edge + (10.0 * scale_factor);
    let text_y = height * 0.5;

    // Get current queue and song
    let Some(current_queue) = CURRENT_SONGS.lock().clone() else {
        return;
    };
    let Some(PlayableItem::Track(song)) = current_queue.currently_playing else {
        return;
    };

    let song_text_width = draw_text(
        scene,
        font_context,
        layout_context,
        &song.name,
        15.0 * scale_factor,
        Color::from_rgb8(240, 240, 240),
        FontWeight::SEMI_BOLD,
        text_x,
        text_y,
    ) + 8.0 * scale_factor;
    draw_text(
        scene,
        font_context,
        layout_context,
        song.artists.first().map_or("Unknown", |a| &a.name),
        12.0 * scale_factor,
        Color::from_rgb8(240, 240, 240),
        FontWeight::EXTRA_BLACK,
        text_x + song_text_width,
        text_y,
    );
}

fn draw_text(
    scene: &mut Scene,
    font_context: &mut FontContext,
    layout_context: &mut LayoutContext<()>,
    text: &str,
    font_size: f64,
    font_color: Color,
    font_weight: FontWeight,
    text_x: f64,
    text_y: f64,
) -> f64 {
    let mut builder = layout_context.ranged_builder(font_context, text, 1.0, false);
    builder.push_default(StyleProperty::FontSize(font_size as f32));
    builder.push_default(StyleProperty::FontWeight(font_weight));

    let mut layout: Layout<()> = builder.build(text);
    layout.break_all_lines(None);
    let text_transform = Affine::translate((text_x, text_y - (f64::from(layout.height()) / 2.0)));

    for item in layout.lines().flat_map(|line| line.items()) {
        let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
            continue;
        };
        let glyphs = glyph_run.positioned_glyphs().map(|g| Glyph {
            id: g.id,
            x: g.x,
            y: g.y,
        });
        let run = glyph_run.run();
        scene
            .draw_glyphs(run.font())
            .font_size(run.font_size())
            .normalized_coords(run.normalized_coords())
            .transform(text_transform)
            .hint(true)
            .brush(font_color)
            .draw(Fill::NonZero, glyphs);
    }

    f64::from(layout.width())
}

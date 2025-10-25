use crate::spotify::CURRENT_SONGS;
use anyhow::Result;
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
        CommandEncoderDescriptor, CompositeAlphaMode, PollType, PresentMode, SurfaceTargetUnsafe,
        TextureViewDescriptor,
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
    connection.display().get_registry(&qh, ());

    let display_ptr = NonNull::new(connection.backend().display_ptr().cast::<c_void>()).unwrap();
    let mut app = CantusLayer::new(display_ptr);

    // Initial roundtrip to get globals
    if let Err(err) = event_queue.roundtrip(&mut app) {
        error!("Initial roundtrip failed: {:?}", err);
        return;
    }
    let (Some(compositor), Some(layer_shell)) = (app.compositor.take(), app.layer_shell.take())
    else {
        error!("Missing compositor or layer shell");
        return;
    };
    if app.outputs.is_empty() {
        error!("No Wayland outputs found");
        return;
    }

    let wl_surface = compositor.create_surface(&qh, ());
    let surface_ptr = NonNull::new(wl_surface.id().as_ptr().cast::<c_void>()).unwrap();

    app.surface_ptr = Some(surface_ptr);
    app.output = Some(app.outputs.remove(0));
    app.wl_surface = Some(wl_surface);

    if let (Some(vp), Some(fm)) = (app.viewporter.take(), app.fractional_manager.take()) {
        app.viewport = Some(vp.get_viewport(app.wl_surface.as_ref().unwrap(), &qh, ()));
        app.fractional = Some(fm.get_fractional_scale(app.wl_surface.as_ref().unwrap(), &qh, ()));
    }

    let layer_surface = layer_shell.get_layer_surface(
        app.wl_surface.as_ref().unwrap(),
        app.output.as_ref(),
        zwlr_layer_shell_v1::Layer::Top,
        "cantus".into(),
        &qh,
        (),
    );
    app.layer_surface = Some(layer_surface);
    let ls = app.layer_surface.as_ref().unwrap();
    ls.set_size(PANEL_WIDTH as u32, PANEL_HEIGHT as u32);
    ls.set_anchor(zwlr_layer_surface_v1::Anchor::Top | zwlr_layer_surface_v1::Anchor::Left);
    ls.set_margin(PANEL_MARGIN as i32, 0, 0, PANEL_MARGIN as i32);
    ls.set_exclusive_zone(0);

    app.wl_surface.as_ref().unwrap().commit();
    if connection.flush().is_err() {
        error!("Failed to flush initial commit");
        return;
    }

    while !app.is_configured && !app.should_exit {
        if event_queue.blocking_dispatch(&mut app).is_err() {
            error!("Error awaiting configure");
            return;
        }
    }

    while !app.should_exit {
        if event_queue.blocking_dispatch(&mut app).is_err() {
            error!("Wayland dispatch error");
            break;
        }
    }
}

struct CantusLayer {
    // --- Wayland globals ---
    compositor: Option<WlCompositor>,
    layer_shell: Option<ZwlrLayerShellV1>,
    viewporter: Option<WpViewporter>,
    fractional_manager: Option<WpFractionalScaleManagerV1>,
    output: Option<WlOutput>,
    outputs: Vec<WlOutput>,

    // --- Surface and layer resources ---
    wl_surface: Option<WlSurface>,
    layer_surface: Option<ZwlrLayerSurfaceV1>,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    frame_callback: Option<WlCallback>,

    // --- Rendering ---
    render_context: RenderContext,
    render_surface: Option<RenderSurface<'static>>,
    renderers: Vec<Option<Renderer>>,
    scene: Scene,

    // --- Text ---
    font_context: FontContext,
    layout_context: LayoutContext<()>,

    // --- State ---
    logical_width: f64,
    logical_height: f64,
    scale_factor: f64,
    is_configured: bool,
    should_exit: bool,
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
            // --- Wayland globals ---
            compositor: None,
            layer_shell: None,
            viewporter: None,
            fractional_manager: None,
            output: None,
            outputs: Vec::new(),

            // --- Surface and layer resources ---
            wl_surface: None,
            layer_surface: None,
            viewport: None,
            fractional: None,
            frame_callback: None,

            // --- Rendering ---
            render_context: RenderContext::new(),
            render_surface: None,
            renderers: Vec::new(),
            scene: Scene::new(),

            // --- Text ---
            font_context,
            layout_context: LayoutContext::new(),

            // --- State ---
            logical_width: PANEL_WIDTH,
            logical_height: PANEL_HEIGHT,
            scale_factor: 1.0,
            is_configured: false,
            should_exit: false,
            display_ptr,
            surface_ptr: None,
        }
    }

    fn request_frame(&mut self, qh: &QueueHandle<Self>) {
        if self.frame_callback.is_none() {
            self.frame_callback = Some(self.wl_surface.as_ref().unwrap().frame(qh, ()));
        }
    }

    fn ensure_surface(&mut self, w: f64, h: f64) -> Result<()> {
        if w == 0.0 || h == 0.0 || !self.is_configured {
            return Ok(());
        }

        let recreate = self
            .render_surface
            .as_ref()
            .is_none_or(|s| s.config.width != w as u32 || s.config.height != h as u32);
        if !recreate {
            return Ok(());
        }

        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                self.display_ptr,
            )),
            raw_window_handle: RawWindowHandle::Wayland(WaylandWindowHandle::new(
                self.surface_ptr.unwrap(),
            )),
        };
        let surface = unsafe { self.render_context.instance.create_surface_unsafe(target) }?;
        let mut rs = pollster::block_on(self.render_context.create_render_surface(
            surface,
            w as u32,
            h as u32,
            PresentMode::Fifo,
        ))?;
        rs.config.alpha_mode = CompositeAlphaMode::PreMultiplied;
        self.renderers
            .resize_with(self.render_context.devices.len(), || None);
        self.render_surface = Some(rs);
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
            surface.config.width.into(),
            surface.config.height.into(),
            self.scale_factor,
        );

        let dev_id = surface.dev_id;
        let device_handle = &self.render_context.devices[dev_id];
        let renderer = self.renderers[dev_id].get_or_insert(Renderer::new(
            &device_handle.device,
            RendererOptions::default(),
        )?);

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

        let Ok(tex) = surface.surface.get_current_texture() else {
            self.render_surface = None;
            return Ok(());
        };
        let mut encoder = device_handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Cantus blit"),
            });
        surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &surface.target_view,
            &tex.texture.create_view(&TextureViewDescriptor::default()),
        );
        device_handle.queue.submit([encoder.finish()]);
        tex.present();
        device_handle.device.poll(PollType::Poll)?;
        Ok(())
    }

    fn update_scale_and_viewport(&self) {
        let bw = (self.logical_width * self.scale_factor).round();
        let bh = (self.logical_height * self.scale_factor).round();

        if let Some(s) = &self.wl_surface {
            s.set_buffer_scale(if self.viewport.is_some() {
                1
            } else {
                self.scale_factor.ceil() as i32
            });
        }
        if let Some(v) = &self.viewport {
            v.set_source(0.0, 0.0, bw, bh);
            v.set_destination(self.logical_width as i32, self.logical_height as i32);
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

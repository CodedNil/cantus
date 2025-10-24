use anyhow::{Result, bail};
use parley::{
    FontContext, Layout, LayoutContext, layout::PositionedLayoutItem, style::StyleProperty,
};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle, WindowHandle,
};
use rspotify::model::PlayableItem;
use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_layer, delegate_output, delegate_registry,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    shell::{
        WaylandSurface,
        wlr_layer::{
            Anchor, KeyboardInteractivity, Layer, LayerShell, LayerShellHandler, LayerSurface,
            LayerSurfaceConfigure,
        },
    },
};
use std::{ffi::c_void, ptr::NonNull};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, Glyph, Renderer, RendererOptions, Scene,
    kurbo::{Affine, RoundedRect},
    peniko::{Color, Fill, color::palette},
    util::{RenderContext, RenderSurface},
    wgpu::{
        CommandEncoderDescriptor, CompositeAlphaMode, PollType, PresentMode, SurfaceError,
        SurfaceTarget, TextureViewDescriptor,
    },
};
use wayland_client::{
    Connection, Proxy, QueueHandle,
    globals::registry_queue_init,
    protocol::{wl_callback, wl_output, wl_surface},
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
    let (globals, mut event_queue) = registry_queue_init(&connection).unwrap();
    let qh = event_queue.handle();

    let compositor = CompositorState::bind(&globals, &qh).unwrap();
    let layer_shell = LayerShell::bind(&globals, &qh).unwrap();

    let layer_surface = layer_shell.create_layer_surface(
        &qh,
        compositor.create_surface(&qh),
        Layer::Overlay,
        Some("cantus"),
        None,
    );
    layer_surface.set_anchor(Anchor::TOP | Anchor::LEFT);
    layer_surface.set_keyboard_interactivity(KeyboardInteractivity::None);
    layer_surface.set_exclusive_zone(-1);
    layer_surface.set_margin(4, 0, 0, 4);
    layer_surface.set_size(PANEL_WIDTH as u32, PANEL_HEIGHT as u32);
    layer_surface.commit();

    let registry_state = RegistryState::new(&globals);
    let output_state = OutputState::new(&globals, &qh);

    let display_ptr = NonNull::new(connection.backend().display_ptr().cast::<c_void>()).unwrap();
    let surface_ptr =
        NonNull::new(layer_surface.wl_surface().id().as_ptr().cast::<c_void>()).unwrap();

    let mut app = CantusLayer::new(
        registry_state,
        output_state,
        layer_surface,
        display_ptr,
        surface_ptr,
    );

    while !app.should_exit {
        event_queue.blocking_dispatch(&mut app).unwrap();
    }
}

struct CantusLayer {
    registry_state: RegistryState,
    output_state: OutputState,
    layer_surface: LayerSurface,
    render_context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    render_surface: Option<RenderSurface<'static>>,
    scene: Scene,
    width: f64,
    height: f64,
    frame_callback: Option<wl_callback::WlCallback>,
    should_exit: bool,
    display_ptr: NonNull<c_void>,
    surface_ptr: NonNull<c_void>,
    scale_factor: i32,
}

impl CantusLayer {
    fn new(
        registry_state: RegistryState,
        output_state: OutputState,
        layer_surface: LayerSurface,
        display_ptr: NonNull<c_void>,
        surface_ptr: NonNull<c_void>,
    ) -> Self {
        Self {
            registry_state,
            output_state,
            layer_surface,
            render_context: RenderContext::new(),
            renderers: Vec::new(),
            render_surface: None,
            scene: Scene::new(),
            width: PANEL_WIDTH,
            height: PANEL_HEIGHT,
            frame_callback: None,
            should_exit: false,
            display_ptr,
            surface_ptr,
            scale_factor: 1,
        }
    }

    fn request_frame(&mut self, qh: &QueueHandle<Self>) {
        if self.frame_callback.is_some() {
            return;
        }
        let callback = self
            .layer_surface
            .wl_surface()
            .frame(qh, self.layer_surface.wl_surface().clone());
        self.frame_callback = Some(callback);
    }

    fn ensure_surface(&mut self, width: f64, height: f64) -> Result<()> {
        if width == 0.0 || height == 0.0 {
            self.render_surface = None;
            return Ok(());
        }

        if let Some(surface) = &mut self.render_surface {
            if surface.config.width != width as u32 || surface.config.height != height as u32 {
                self.render_context
                    .resize_surface(surface, width as u32, height as u32);
            }
            return Ok(());
        }

        let handles = WaylandHandles::new(self.display_ptr, self.surface_ptr);
        let surface = self
            .render_context
            .instance
            .create_surface(SurfaceTarget::Window(Box::new(handles)))?;

        let mut render_surface = pollster::block_on(self.render_context.create_render_surface(
            surface,
            width as u32,
            height as u32,
            PresentMode::AutoNoVsync,
        ))?;

        let dev_id = render_surface.dev_id;
        let device_handle = &self.render_context.devices[dev_id];
        let capabilities = render_surface
            .surface
            .get_capabilities(device_handle.adapter());
        let mut desired_alpha = render_surface.config.alpha_mode;
        for mode in [
            CompositeAlphaMode::PreMultiplied,
            CompositeAlphaMode::PostMultiplied,
            CompositeAlphaMode::Inherit,
        ] {
            if capabilities.alpha_modes.contains(&mode) {
                desired_alpha = mode;
                break;
            }
        }
        if desired_alpha != render_surface.config.alpha_mode {
            render_surface.config.alpha_mode = desired_alpha;
            render_surface
                .surface
                .configure(&device_handle.device, &render_surface.config);
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
            f64::from(surface.config.width),
            f64::from(surface.config.height),
            f64::from(self.scale_factor),
        );

        let dev_id = surface.dev_id;
        self.renderers
            .resize_with(self.render_context.devices.len(), || None);
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
                self.ensure_surface(self.width, self.height)?;
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
}

impl CompositorHandler for CantusLayer {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        new_factor: i32,
    ) {
        self.scale_factor = new_factor;
        self.layer_surface.wl_surface().set_buffer_scale(new_factor);
        self.layer_surface.wl_surface().commit();
        // Re-render to apply the new scale factor
        if let Err(err) = self.render() {
            error!("Rendering failed after scale factor change: {err:?}");
        }
        self.request_frame(qh);
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_transform: wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
        let _ = self.frame_callback.take();
        if self.layer_surface.wl_surface().id() != surface.id() {
            return;
        }
        if let Err(err) = self.render() {
            error!("Rendering failed: {err:?}");
            return;
        }
        self.request_frame(qh);
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }
}

impl LayerShellHandler for CantusLayer {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {
        self.should_exit = true;
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        let (width, height) = configure.new_size;
        self.width = if width == 0 {
            PANEL_WIDTH * f64::from(self.scale_factor)
        } else {
            f64::from(width)
        };
        self.height = if height == 0 {
            PANEL_HEIGHT * f64::from(self.scale_factor)
        } else {
            f64::from(height)
        };

        if let Err(err) = self.ensure_surface(self.width, self.height) {
            error!("Failed to prepare render surface: {err:?}");
            self.should_exit = true;
            return;
        }
        if let Err(err) = self.render() {
            error!("Rendering failed: {err:?}");
        }
        self.request_frame(qh);
    }
}

impl OutputHandler for CantusLayer {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

impl ProvidesRegistryState for CantusLayer {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    fn runtime_add_global(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _name: u32,
        _interface: &str,
        _version: u32,
    ) {
    }

    fn runtime_remove_global(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _name: u32,
        _interface: &str,
    ) {
    }
}

delegate_compositor!(CantusLayer);
delegate_output!(CantusLayer);
delegate_layer!(CantusLayer);
delegate_registry!(CantusLayer);

struct WaylandHandles {
    display: NonNull<c_void>,
    surface: NonNull<c_void>,
}

impl WaylandHandles {
    const fn new(display: NonNull<c_void>, surface: NonNull<c_void>) -> Self {
        Self { display, surface }
    }
}

impl HasDisplayHandle for WaylandHandles {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = WaylandDisplayHandle::new(self.display);
        // Safety: the Wayland compositor guarantees that display pointers remain valid
        // while the connection is alive, which matches the lifetime of `CantusLayer`.
        unsafe { Ok(DisplayHandle::borrow_raw(RawDisplayHandle::Wayland(handle))) }
    }
}

impl HasWindowHandle for WaylandHandles {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = WaylandWindowHandle::new(self.surface);
        // Safety: the wl_surface stays alive for the lifetime of the layer surface.
        unsafe { Ok(WindowHandle::borrow_raw(RawWindowHandle::Wayland(handle))) }
    }
}

// SAFETY: The wl_display and wl_surface referenced by the pointers remain valid for the lifetime
// of `CantusLayer`, and wgpu only reads the pointers when creating the surface.
unsafe impl Send for WaylandHandles {}
unsafe impl Sync for WaylandHandles {}

fn create_scene(scene: &mut Scene, width: f64, height: f64, scale_factor: f64) {
    let scaled_panel_margin = PANEL_MARGIN * scale_factor;

    // Draw a rectangle filling the screen
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::new([0.9, 0.5, 0.6, 1.0]),
        None,
        &RoundedRect::new(0.0, 0.0, width, height, 10.0 * scale_factor),
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
        &song.name,
        15.0 * scale_factor,
        Color::from_rgb8(240, 240, 240),
        text_x,
        text_y,
    ) + 8.0 * scale_factor;
    draw_text(
        scene,
        song.artists
            .first()
            .map_or("Unknown Artist", |artist| artist.name.as_str()),
        12.0 * scale_factor,
        Color::from_rgb8(240, 240, 240),
        text_x + song_text_width,
        text_y,
    );
}

fn draw_text(
    scene: &mut Scene,
    text: &str,
    font_size: f64,
    font_color: Color,
    text_x: f64,
    text_y: f64,
) -> f64 {
    let mut font_cx = FontContext::new();
    let mut layout_cx = LayoutContext::new();

    let mut builder = layout_cx.ranged_builder(&mut font_cx, text, 1.0, false);
    builder.push_default(StyleProperty::FontSize(font_size as f32));

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

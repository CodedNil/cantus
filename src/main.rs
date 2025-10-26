use crate::background::WarpBackground;
use anyhow::Result;
use parley::{FontContext, LayoutContext};
use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use std::{env, ffi::c_void, ptr::NonNull, sync::Arc, time::Instant};
use tracing::error;
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, Renderer, RendererOptions, Scene,
    peniko::{Blob, color::palette},
    util::{RenderContext, RenderSurface},
    wgpu::{
        BlendComponent, BlendFactor, BlendOperation, BlendState, CommandEncoderDescriptor,
        CompositeAlphaMode, InstanceDescriptor, PollType, PresentMode, SurfaceTargetUnsafe,
        TextureViewDescriptor, util::TextureBlitterBuilder,
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

mod background;
mod render;
mod spotify;

const PANEL_WIDTH: f64 = 500.0;
const PANEL_HEIGHT: f64 = 40.0;

/// Launch the application entry point.
#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(
            ["warn", "cantus=info", "wgpu_hal=error"].join(","),
        ))
        .init();

    spotify::init().await;
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
    event_queue
        .roundtrip(&mut app)
        .unwrap_or_else(|err| panic!("Initial roundtrip failed: {err}"));
    let compositor = app.compositor.take().expect("Missing compositor");
    let layer_shell = app.layer_shell.take().expect("Missing layer shell");
    assert!(!app.outputs.is_empty(), "No Wayland outputs found");

    event_queue
        .roundtrip(&mut app)
        .unwrap_or_else(|err| panic!("Failed to fetch output details: {err}"));

    let wl_surface = compositor.create_surface(&qh, ());
    let surface_ptr = NonNull::new(wl_surface.id().as_ptr().cast::<c_void>()).unwrap();

    app.surface_ptr = Some(surface_ptr);
    assert!(app.try_select_output(), "Failed to select a Wayland output");
    app.wl_surface = Some(wl_surface);

    let surface = app.wl_surface.as_ref().unwrap();
    if let (Some(vp), Some(fm)) = (app.viewporter.take(), app.fractional_manager.take()) {
        app.viewport = Some(vp.get_viewport(surface, &qh, ()));
        app.fractional = Some(fm.get_fractional_scale(surface, &qh, ()));
    }

    let layer_surface = layer_shell.get_layer_surface(
        surface,
        app.output.as_ref(),
        zwlr_layer_shell_v1::Layer::Overlay,
        "cantus".into(),
        &qh,
        (),
    );
    layer_surface.set_size(PANEL_WIDTH as u32, PANEL_HEIGHT as u32);
    layer_surface
        .set_anchor(zwlr_layer_surface_v1::Anchor::Top | zwlr_layer_surface_v1::Anchor::Left);
    layer_surface.set_margin(4, 0, 0, 4);
    layer_surface.set_exclusive_zone(-1);

    app.layer_surface = Some(layer_surface);

    surface.commit();
    connection
        .flush()
        .unwrap_or_else(|err| panic!("Failed to flush initial commit: {err}"));

    while !app.should_exit {
        event_queue
            .blocking_dispatch(&mut app)
            .unwrap_or_else(|err| panic!("Wayland dispatch error: {err}"));
    }
}

#[derive(Clone)]
struct OutputInfo {
    handle: WlOutput,
    name: Option<String>,
    description: Option<String>,
    make: Option<String>,
    model: Option<String>,
}

impl OutputInfo {
    /// Check if this output metadata matches the target string.
    fn matches(&self, target: &str) -> bool {
        self.name.as_ref().is_some_and(|name| name.contains(target))
            || self
                .make
                .as_ref()
                .zip(self.model.as_ref())
                .is_some_and(|(make, model)| format!("{make} {model}").contains(target))
            || self
                .description
                .as_ref()
                .is_some_and(|description| description.contains(target))
    }
}

struct CantusLayer {
    // --- Wayland globals ---
    compositor: Option<WlCompositor>,
    layer_shell: Option<ZwlrLayerShellV1>,
    viewporter: Option<WpViewporter>,
    fractional_manager: Option<WpFractionalScaleManagerV1>,
    output: Option<WlOutput>,
    outputs: Vec<OutputInfo>,
    output_matched: bool,

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
    shader_backgrounds: Vec<Option<WarpBackground>>,
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
    time_origin: Instant,
    frame_index: u64,
}

impl CantusLayer {
    /// Create a new layer shell state container.
    fn new(display_ptr: NonNull<c_void>) -> Self {
        let mut font_context = FontContext::new();
        font_context.collection.register_fonts(
            Blob::new(Arc::new(include_bytes!("../assets/epilogue.ttf"))),
            None,
        );
        // Verify the font was added correctly
        font_context.collection.family_id("epilogue").unwrap();

        // Create a RenderContext with Vulkan backend
        let mut render_context = RenderContext::new();
        render_context.instance = vello::wgpu::Instance::new(&InstanceDescriptor {
            backends: vello::wgpu::Backends::VULKAN,
            ..Default::default()
        });

        Self {
            // --- Wayland globals ---
            compositor: None,
            layer_shell: None,
            viewporter: None,
            fractional_manager: None,
            output: None,
            outputs: Vec::new(),
            output_matched: false,

            // --- Surface and layer resources ---
            wl_surface: None,
            layer_surface: None,
            viewport: None,
            fractional: None,
            frame_callback: None,

            // --- Rendering ---
            render_context,
            render_surface: None,
            renderers: Vec::new(),
            shader_backgrounds: Vec::new(),
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
            time_origin: Instant::now(),
            frame_index: 0,
        }
    }

    /// Ask Wayland for the next frame callback.
    fn request_frame(&mut self, qh: &QueueHandle<Self>) {
        if self.frame_callback.is_none()
            && let Some(surface) = self.wl_surface.as_ref()
        {
            self.frame_callback = Some(surface.frame(qh, ()));
        }
    }

    /// Make sure the GPU surface matches the requested size.
    fn ensure_surface(&mut self, w: f64, h: f64) -> Result<()> {
        // Ignore requests while the surface is not ready or has zero size.
        if w == 0.0 || h == 0.0 || !self.is_configured {
            return Ok(());
        }

        // Reuse the existing surface if dimensions already match.
        let recreate = self
            .render_surface
            .as_ref()
            .is_none_or(|s| s.config.width != w as u32 || s.config.height != h as u32);
        if !recreate {
            return Ok(());
        }

        // Build a raw Wayland surface handle for wgpu.
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
        let device_handle = &self.render_context.devices[rs.dev_id];
        let alpha_modes = rs
            .surface
            .get_capabilities(device_handle.adapter())
            .alpha_modes;
        let alpha_mode = [
            CompositeAlphaMode::PostMultiplied,
            CompositeAlphaMode::PreMultiplied,
        ]
        .into_iter()
        .find(|mode| alpha_modes.contains(mode))
        .or_else(|| alpha_modes.first().copied())
        .unwrap_or(CompositeAlphaMode::Auto);
        rs.config.alpha_mode = alpha_mode;
        if alpha_mode != CompositeAlphaMode::PostMultiplied {
            rs.blitter = TextureBlitterBuilder::new(&device_handle.device, rs.config.format)
                .blend_state(BlendState {
                    color: BlendComponent {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::Zero,
                        operation: BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::Zero,
                        operation: BlendOperation::Add,
                    },
                })
                .build();
        }
        rs.surface.configure(&device_handle.device, &rs.config);
        self.renderers
            .resize_with(self.render_context.devices.len(), || None);
        self.shader_backgrounds
            .resize_with(self.render_context.devices.len(), || None);
        self.render_surface = Some(rs);
        Ok(())
    }

    /// Choose an output for the layer surface if possible.
    fn try_select_output(&mut self) -> bool {
        if self.outputs.is_empty() {
            return false;
        }

        let target_monitor = env::var("TARGET_MONITOR").ok();
        let target = target_monitor.as_deref();
        let selected_index = target
            .and_then(|needle| self.outputs.iter().position(|info| info.matches(needle)))
            .unwrap_or(0);
        let info = &self.outputs[selected_index];
        let matches_target = target.is_some_and(|needle| info.matches(needle));

        if self
            .output
            .as_ref()
            .is_none_or(|id| id.id() != info.handle.id())
            || (matches_target && !self.output_matched)
        {
            self.output = Some(info.handle.clone());
            self.output_matched = matches_target;
        }
        true
    }
    /// Render a frame and present it if the surface is available.
    fn try_render_frame(&mut self, qh: &QueueHandle<Self>) -> Result<()> {
        self.frame_index = self.frame_index.wrapping_add(1);
        // Auto-recover if surface lost
        if self.render_surface.is_none() {
            let buffer_width = (self.logical_width * self.scale_factor).round();
            let buffer_height = (self.logical_height * self.scale_factor).round();
            self.ensure_surface(buffer_width, buffer_height)?;
        }

        let rendering = {
            if self.render_surface.is_none() {
                return Ok(());
            }

            let id = self.render_surface.as_ref().unwrap().dev_id;
            // Ensure the renderer exists
            if self.renderers[id].is_none() {
                self.renderers[id] = Some(Renderer::new(
                    &self.render_context.devices[id].device,
                    RendererOptions::default(),
                )?);
            }

            // Prepare scene
            self.scene.reset();
            self.create_scene(id);

            let Some(surface) = self.render_surface.as_mut() else {
                return Ok(());
            };
            let device_handle = &self.render_context.devices[id];
            let renderer = self.renderers[id].get_or_insert(Renderer::new(
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

            match surface.surface.get_current_texture() {
                Ok(acquired) => {
                    let mut encoder =
                        device_handle
                            .device
                            .create_command_encoder(&CommandEncoderDescriptor {
                                label: Some("Cantus blit"),
                            });
                    surface.blitter.copy(
                        &device_handle.device,
                        &mut encoder,
                        &surface.target_view,
                        &acquired
                            .texture
                            .create_view(&TextureViewDescriptor::default()),
                    );
                    Ok((id, acquired, encoder.finish()))
                }
                Err(_) => Err(()),
            }
        };

        let Ok((dev_id, tex, command_buffer)) = rendering else {
            // Keep the surface dropped to force recreation; schedule a new frame before
            // committing so the compositor will notify us once it is ready again.
            self.render_surface = None;
            self.request_frame(qh);
            if let Some(surface) = &self.wl_surface {
                surface.commit();
            }
            return Ok(());
        };

        // Queue frame request after rendering but before presenting so the callback associates
        self.request_frame(qh);

        let device_handle = &self.render_context.devices[dev_id];
        device_handle.queue.submit([command_buffer]);
        tex.present();
        device_handle.device.poll(PollType::Poll)?;

        Ok(())
    }

    /// Push the computed scale and viewport to Wayland objects.
    fn update_scale_and_viewport(&self) {
        let bw = (self.logical_width * self.scale_factor).round();
        let bh = (self.logical_height * self.scale_factor).round();

        if let Some(surface) = &self.wl_surface {
            let buffer_scale = if self.viewport.is_some() {
                1
            } else {
                self.scale_factor.ceil() as i32
            };
            surface.set_buffer_scale(buffer_scale);
        }
        if let Some(v) = &self.viewport {
            v.set_source(0.0, 0.0, bw, bh);
            v.set_destination(self.logical_width as i32, self.logical_height as i32);
        }
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for CantusLayer {
    /// Handle layer surface protocol events.
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
                // Update dimensions
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

                proxy.ack_configure(serial);
                state.update_scale_and_viewport();
                if let Some(surface) = &state.wl_surface {
                    surface.commit();
                }
                state.is_configured = true;

                let buffer_width = (state.logical_width * state.scale_factor).round();
                let buffer_height = (state.logical_height * state.scale_factor).round();

                if let Err(err) = state.ensure_surface(buffer_width, buffer_height) {
                    error!("Failed to prepare render surface: {err}");
                    state.should_exit = true;
                    return;
                }

                // Render first frame and request next.
                if let Err(err) = state.try_render_frame(qhandle) {
                    error!("Initial rendering failed: {err}");
                }
            }
            zwlr_layer_surface_v1::Event::Closed => {
                state.should_exit = true;
            }
            _ => {}
        }
    }
}

impl Dispatch<WpFractionalScaleV1, ()> for CantusLayer {
    /// Handle fractional scale updates from the compositor.
    fn event(
        state: &mut Self,
        _proxy: &WpFractionalScaleV1,
        event: wp_fractional_scale_v1::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
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
                    error!("Failed to prepare render surface: {err}");
                    state.should_exit = true;
                    return;
                }

                if let Err(err) = state.try_render_frame(qhandle) {
                    error!("Rendering failed after scale change: {err}");
                }
            }
        }
    }
}

impl Dispatch<WlCallback, ()> for CantusLayer {
    /// Handle frame callbacks to drive rendering.
    fn event(
        state: &mut Self,
        _proxy: &WlCallback,
        event: wl_callback::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_callback::Event::Done { .. } = event
            && let Some(_) = state.frame_callback.take()
            && let Err(err) = state.try_render_frame(qhandle)
        {
            error!("Rendering failed: {err}");
        }
    }
}

impl Dispatch<WlSurface, ()> for CantusLayer {
    /// Ignore surface events that need no action.
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
    /// Track output metadata announcements.
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: wl_output::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        let id = proxy.id();
        if let Some(info) = state.outputs.iter_mut().find(|info| info.handle.id() == id) {
            match event {
                wl_output::Event::Geometry { make, model, .. } => {
                    info.make = Some(make);
                    info.model = Some(model);
                }
                wl_output::Event::Name { name } => {
                    info.name = Some(name);
                }
                wl_output::Event::Description { description } => {
                    info.description = Some(description);
                }
                _ => {}
            }
        }
        state.try_select_output();
    }
}

impl Dispatch<WlRegistry, ()> for CantusLayer {
    /// Bind required globals when the compositor advertises them.
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
                    state.outputs.push(OutputInfo {
                        handle: registry.bind::<WlOutput, (), Self>(name, version.min(4), qh, ()),
                        name: None,
                        description: None,
                        make: None,
                        model: None,
                    });
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<ZwlrLayerShellV1, ()> for CantusLayer {
    /// Ignore global layer shell events not used by the client.
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
    /// Ignore fractional scale manager events we do not use.
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
    /// Ignore viewporter global events that need no handling.
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
    /// Ignore viewport events because configuration is static.
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
    /// Ignore compositor events that are not actionable for the client.
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

use crate::{background::WarpBackground, render::NowPlayingParticle};
use anyhow::Result;
use parley::{FontContext, LayoutContext};
use rand::{SeedableRng, rngs::SmallRng};
use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use rspotify::model::TrackId;
use std::{
    collections::{HashMap, hash_map},
    env,
    ffi::c_void,
    ptr::NonNull,
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{debug, error};
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, Renderer, RendererOptions, Scene,
    kurbo::{Point, Rect},
    peniko::{Blob, color::palette},
    util::{DeviceHandle, RenderContext, RenderSurface},
    wgpu::{
        BlendComponent, BlendFactor, BlendOperation, BlendState, CommandEncoderDescriptor,
        CompositeAlphaMode, InstanceDescriptor, PollType, PresentMode, SurfaceTargetUnsafe,
        TextureViewDescriptor, util::TextureBlitterBuilder,
    },
};
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle, WEnum,
    protocol::{
        wl_callback::{self, WlCallback},
        wl_compositor::{self, WlCompositor},
        wl_output::{self, WlOutput},
        wl_pointer::{self, WlPointer},
        wl_region::{self, WlRegion},
        wl_registry::{self, WlRegistry},
        wl_seat::{self, WlSeat},
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

const PANEL_WIDTH: f64 = 1050.0;
const PANEL_HEIGHT_BASE: f64 = 40.0;
const PANEL_HEIGHT_EXTENSION: f64 = 70.0;
const PANEL_HEIGHT: f64 = PANEL_HEIGHT_BASE + PANEL_HEIGHT_EXTENSION;

/// Launch the application entry point.
#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().unwrap();

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
    let connection = Connection::connect_to_env().expect("Failed to connect to Wayland display");
    let mut event_queue = connection.new_event_queue();
    let qhandle = event_queue.handle();
    connection.display().get_registry(&qhandle, ());

    let display_ptr = NonNull::new(connection.backend().display_ptr().cast::<c_void>())
        .expect("Failed to get display pointer");
    let mut app = CantusLayer::new(display_ptr);

    // Initial roundtrip to get globals
    event_queue
        .roundtrip(&mut app)
        .expect("Initial roundtrip failed");
    let compositor = app.compositor.take().expect("Missing compositor");
    let layer_shell = app.layer_shell.take().expect("Missing layer shell");
    assert!(!app.outputs.is_empty(), "No Wayland outputs found");

    event_queue
        .roundtrip(&mut app)
        .expect("Failed to fetch output details");

    let wl_surface = compositor.create_surface(&qhandle, ());
    let surface_ptr = NonNull::new(wl_surface.id().as_ptr().cast::<c_void>())
        .expect("Failed to get surface pointer");
    let surface = app.wl_surface.insert(wl_surface).clone();
    app.surface_ptr = Some(surface_ptr);
    assert!(app.try_select_output(), "Failed to select a Wayland output");

    if let (Some(vp), Some(fm)) = (app.viewporter.take(), app.fractional_manager.take()) {
        app.viewport = Some(vp.get_viewport(&surface, &qhandle, ()));
        app.fractional = Some(fm.get_fractional_scale(&surface, &qhandle, ()));
    }

    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        app.outputs.get(app.active_output).map(|info| &info.handle),
        zwlr_layer_shell_v1::Layer::Top,
        "cantus".into(),
        &qhandle,
        (),
    );
    layer_surface.set_size(PANEL_WIDTH as u32, PANEL_HEIGHT as u32);
    layer_surface
        .set_anchor(zwlr_layer_surface_v1::Anchor::Top | zwlr_layer_surface_v1::Anchor::Left);
    layer_surface.set_margin(4, 0, 0, 4);
    layer_surface.set_exclusive_zone(-1);

    surface.commit();
    connection.flush().expect("Failed to flush initial commit");

    // Add compositor back into app
    app.compositor = Some(compositor);

    while !app.should_exit {
        event_queue
            .blocking_dispatch(&mut app)
            .expect("Wayland dispatch error");
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
    seat: Option<WlSeat>,
    pointer: Option<WlPointer>,
    outputs: Vec<OutputInfo>,
    active_output: usize,
    output_matched: bool,

    // --- Surface and layer resources ---
    wl_surface: Option<WlSurface>,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    frame_callback: Option<WlCallback>,

    // --- Rendering ---
    render_context: RenderContext,
    render_surface: Option<RenderSurface<'static>>,
    render_devices: HashMap<usize, RenderDevice>,
    scene: Scene,

    // --- Text ---
    font_context: FontContext,
    layout_context: LayoutContext<()>,

    // --- State ---
    scale_factor: f64,
    is_configured: bool,
    should_exit: bool,
    display_ptr: NonNull<c_void>,
    surface_ptr: Option<NonNull<c_void>>,
    time_origin: Instant,
    frame_index: u64,

    // --- Interaction ---
    pointer_position: (f64, f64),
    last_hitbox_update: Instant,
    track_hitboxes: HashMap<TrackId<'static>, Rect>,

    // --- Animation ---
    track_start_ms: f64,
    track_spacing: f64,

    // --- Particles ---
    now_playing_particles: Vec<NowPlayingParticle>,
    rng: SmallRng,
    last_particle_update: Instant,
    particle_spawn_accumulator: f32,
}

impl CantusLayer {
    /// Create a new layer shell state container.
    fn new(display_ptr: NonNull<c_void>) -> Self {
        let mut font_context = FontContext::new();
        font_context.collection.register_fonts(
            Blob::new(Arc::new(include_bytes!("../assets/NotoSans.ttf"))),
            None,
        );

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
            seat: None,
            pointer: None,
            outputs: Vec::new(),
            active_output: 0,
            output_matched: false,

            // --- Surface and layer resources ---
            wl_surface: None,
            viewport: None,
            fractional: None,
            frame_callback: None,

            // --- Rendering ---
            render_context,
            render_surface: None,
            render_devices: HashMap::new(),
            scene: Scene::new(),

            // --- Text ---
            font_context,
            layout_context: LayoutContext::new(),

            // --- State ---
            scale_factor: 1.0,
            is_configured: false,
            should_exit: false,
            display_ptr,
            surface_ptr: None,
            time_origin: Instant::now(),
            frame_index: 0,

            // --- Animation ---
            track_start_ms: 0.0,
            track_spacing: 0.0,

            // --- Interaction ---
            pointer_position: (0.0, 0.0),
            track_hitboxes: HashMap::new(),
            last_hitbox_update: Instant::now(),

            // --- Particles ---
            now_playing_particles: Vec::new(),
            rng: SmallRng::from_os_rng(),
            last_particle_update: Instant::now(),
            particle_spawn_accumulator: 0.0,
        }
    }

    /// Ask Wayland for the next frame callback.
    fn request_frame(&mut self, qhandle: &QueueHandle<Self>) {
        if self.frame_callback.is_none()
            && let Some(surface) = self.wl_surface.as_ref()
        {
            self.frame_callback = Some(surface.frame(qhandle, ()));
        }
    }

    /// Make sure the GPU surface matches the requested size.
    fn ensure_surface(&mut self, width: f64, height: f64) -> Result<()> {
        // Ignore requests while the surface is not ready or has zero size.
        if width == 0.0 || height == 0.0 || !self.is_configured {
            return Ok(());
        }

        // Reuse the existing surface if dimensions already match.
        let recreate = self.render_surface.as_ref().is_none_or(|surface| {
            surface.config.width != width as u32 || surface.config.height != height as u32
        });
        if !recreate {
            return Ok(());
        }

        // Build a raw Wayland surface handle for wgpu.
        let Some(surface_ptr) = self.surface_ptr else {
            return Ok(());
        };
        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                self.display_ptr,
            )),
            raw_window_handle: RawWindowHandle::Wayland(WaylandWindowHandle::new(surface_ptr)),
        };
        let surface = unsafe { self.render_context.instance.create_surface_unsafe(target) }?;
        let mut rs = pollster::block_on(self.render_context.create_render_surface(
            surface,
            width as u32,
            height as u32,
            PresentMode::AutoVsync,
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
        self.render_surface = Some(rs);
        Ok(())
    }

    fn refresh_surface(&mut self, qhandle: &QueueHandle<Self>) {
        let buffer_width = (PANEL_WIDTH * self.scale_factor).round();
        let buffer_height = (PANEL_HEIGHT * self.scale_factor).round();

        if let Err(err) = self.ensure_surface(buffer_width, buffer_height) {
            error!("Failed to prepare render surface: {err}");
            self.should_exit = true;
            return;
        }

        if let Err(err) = self.try_render_frame(qhandle) {
            error!("Rendering step failed: {err}");
        }
    }

    /// Choose an output for the layer surface if possible.
    fn try_select_output(&mut self) -> bool {
        if self.outputs.is_empty() {
            return false;
        }

        let target = env::var("TARGET_MONITOR").ok();
        let (index, matched_target) = target
            .as_deref()
            .and_then(|needle| {
                self.outputs
                    .iter()
                    .position(|info| info.matches(needle))
                    .map(|idx| (idx, true))
            })
            .unwrap_or((0, false));

        if self.active_output != index || (matched_target && !self.output_matched) {
            self.active_output = index;
            self.output_matched = matched_target;
        }
        true
    }

    /// Handle pointer click events.
    fn handle_pointer_click(&self) -> bool {
        let point = Point::new(self.pointer_position.0, self.pointer_position.1);
        if let Some((id, rect)) = self
            .track_hitboxes
            .iter()
            .find(|(_, rect)| rect.contains(point))
        {
            let id = id.clone();
            let rect = *rect;
            tokio::spawn(async move {
                spotify::skip_to_track(id, point, rect).await;
            });
            return true;
        }
        false
    }

    /// Update the input region for the surface.
    fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        if self.last_hitbox_update.elapsed() <= Duration::from_millis(500) {
            return;
        }

        let (Some(wl_surface), Some(compositor)) = (&self.wl_surface, &self.compositor) else {
            return;
        };

        let region = compositor.create_region(qhandle, ());
        for rect in self.track_hitboxes.values() {
            region.add(
                rect.x0.round() as i32,
                rect.y0.round() as i32,
                (rect.x1 - rect.x0).round() as i32,
                (rect.y1 - rect.y0).round() as i32,
            );
        }

        wl_surface.set_input_region(Some(&region));
        wl_surface.commit();
        self.last_hitbox_update = Instant::now();
    }

    /// Render a frame and present it if the surface is available.
    fn try_render_frame(&mut self, qhandle: &QueueHandle<Self>) -> Result<()> {
        self.frame_index = self.frame_index.wrapping_add(1);
        if self.render_surface.is_none() {
            let buffer_width = (PANEL_WIDTH * self.scale_factor).round();
            let buffer_height = (PANEL_HEIGHT * self.scale_factor).round();
            self.ensure_surface(buffer_width, buffer_height)?;
        }

        let Some(render_surface) = self.render_surface.take() else {
            return Ok(());
        };
        let dev_id = render_surface.dev_id;
        let handle = &self.render_context.devices[dev_id];
        let device = handle.device.clone();
        let queue = handle.queue.clone();
        if let hash_map::Entry::Vacant(entry) = self.render_devices.entry(dev_id) {
            entry.insert(RenderDevice::new(handle)?);
        }

        self.scene.reset();
        self.create_scene(dev_id);
        self.update_input_region(qhandle);

        let bundle = self
            .render_devices
            .get_mut(&dev_id)
            .expect("render device must exist");
        bundle.renderer.render_to_texture(
            &device,
            &queue,
            &self.scene,
            &render_surface.target_view,
            &vello::RenderParams {
                base_color: palette::css::TRANSPARENT,
                width: render_surface.config.width,
                height: render_surface.config.height,
                antialiasing_method: AaConfig::Area,
            },
        )?;

        let acquired = match render_surface.surface.get_current_texture() {
            Ok(acquired) => acquired,
            Err(err) => {
                debug!("Surface acquisition failed: {err}");
                self.render_surface = None;
                self.request_frame(qhandle);
                if let Some(surface) = &self.wl_surface {
                    surface.commit();
                }
                return Ok(());
            }
        };

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Cantus blit"),
        });
        render_surface.blitter.copy(
            &device,
            &mut encoder,
            &render_surface.target_view,
            &acquired
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );

        self.request_frame(qhandle);
        queue.submit([encoder.finish()]);
        acquired.present();
        device.poll(PollType::Poll)?;

        self.render_surface = Some(render_surface);
        Ok(())
    }

    /// Push the computed scale and viewport to Wayland objects.
    fn update_scale_and_viewport(&self) {
        if let Some(surface) = &self.wl_surface {
            surface.set_buffer_scale(if self.viewport.is_some() {
                1
            } else {
                self.scale_factor.ceil() as i32
            });
        }
        if let Some(viewport) = &self.viewport {
            viewport.set_source(
                0.0,
                0.0,
                (PANEL_WIDTH * self.scale_factor).round(),
                (PANEL_HEIGHT * self.scale_factor).round(),
            );
            viewport.set_destination(PANEL_WIDTH as i32, PANEL_HEIGHT as i32);
        }
    }
}

struct RenderDevice {
    renderer: Renderer,
    background: WarpBackground,
}

impl RenderDevice {
    fn new(handle: &DeviceHandle) -> Result<Self> {
        Ok(Self {
            renderer: Renderer::new(&handle.device, RendererOptions::default())?,
            background: WarpBackground::new(&handle.device),
        })
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
                width: _,
                height: _,
            } => {
                proxy.ack_configure(serial);
                state.update_scale_and_viewport();
                if let Some(surface) = &state.wl_surface {
                    surface.commit();
                }
                state.is_configured = true;

                state.refresh_surface(qhandle);
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

                state.refresh_surface(qhandle);
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
                wl_output::Event::Mode { .. }
                | wl_output::Event::Done
                | wl_output::Event::Scale { .. }
                | _ => {}
            }
        }
        state.try_select_output();
    }
}

impl Dispatch<WlSeat, ()> for CantusLayer {
    /// Track seat capabilities and manage pointer objects.
    fn event(
        state: &mut Self,
        proxy: &WlSeat,
        event: wl_seat::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities { capabilities } = event
            && let WEnum::Value(caps) = capabilities
        {
            if caps.contains(wl_seat::Capability::Pointer) {
                if state.pointer.is_none() {
                    state.pointer = Some(proxy.get_pointer(qhandle, ()));
                }
            } else if let Some(pointer) = state.pointer.take() {
                pointer.release();
            }
        }
    }
}

impl Dispatch<WlPointer, ()> for CantusLayer {
    /// Track pointer movement and react to button presses.
    fn event(
        state: &mut Self,
        _proxy: &WlPointer,
        event: wl_pointer::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Enter {
                surface,
                surface_x,
                surface_y,
                ..
            } => {
                if state
                    .wl_surface
                    .as_ref()
                    .is_some_and(|wl_surface| wl_surface.id() == surface.id())
                {
                    state.pointer_position = (surface_x, surface_y);
                }
            }
            wl_pointer::Event::Motion {
                surface_x,
                surface_y,
                ..
            } => {
                state.pointer_position = (surface_x, surface_y);
            }
            wl_pointer::Event::Leave { .. } => {
                state.pointer_position = (-1.0, -1.0);
            }
            wl_pointer::Event::Button {
                button,
                state: button_state,
                ..
            } => {
                if button == 0x110
                    && matches!(button_state, WEnum::Value(wl_pointer::ButtonState::Pressed))
                {
                    let _ = state.handle_pointer_click();
                }
            }
            wl_pointer::Event::Axis { .. }
            | wl_pointer::Event::Frame
            | wl_pointer::Event::AxisSource { .. }
            | wl_pointer::Event::AxisStop { .. }
            | wl_pointer::Event::AxisDiscrete { .. }
            | wl_pointer::Event::AxisValue120 { .. }
            | wl_pointer::Event::AxisRelativeDirection { .. }
            | _ => {}
        }
    }
}

impl Dispatch<WlRegistry, ()> for CantusLayer {
    /// Bind required globals when the compositor advertises them.
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
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
                        Some(proxy.bind::<WlCompositor, (), Self>(name, version, qhandle, ()));
                }
                "zwlr_layer_shell_v1" => {
                    state.layer_shell =
                        Some(proxy.bind::<ZwlrLayerShellV1, (), Self>(name, 4, qhandle, ()));
                }
                "wp_viewporter" => {
                    state.viewporter =
                        Some(proxy.bind::<WpViewporter, (), Self>(name, 1, qhandle, ()));
                }
                "wp_fractional_scale_manager_v1" => {
                    state.fractional_manager = Some(
                        proxy.bind::<WpFractionalScaleManagerV1, (), Self>(name, 1, qhandle, ()),
                    );
                }
                "wl_seat" => {
                    state.seat =
                        Some(proxy.bind::<WlSeat, (), Self>(name, version.min(7), qhandle, ()));
                }
                "wl_output" => {
                    state.outputs.push(OutputInfo {
                        handle: proxy.bind::<WlOutput, (), Self>(name, version.min(4), qhandle, ()),
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

// No-op dispatch implementations for events the client does not handle.
macro_rules! impl_noop_dispatch {
    ($ty:ty, $event:ty) => {
        impl Dispatch<$ty, ()> for CantusLayer {
            fn event(
                _state: &mut Self,
                _proxy: &$ty,
                _event: $event,
                _data: &(),
                _conn: &Connection,
                _qhandle: &QueueHandle<Self>,
            ) {
            }
        }
    };
}

impl_noop_dispatch!(WlSurface, wl_surface::Event);
impl_noop_dispatch!(ZwlrLayerShellV1, zwlr_layer_shell_v1::Event);
impl_noop_dispatch!(
    WpFractionalScaleManagerV1,
    wp_fractional_scale_manager_v1::Event
);
impl_noop_dispatch!(WpViewporter, wp_viewporter::Event);
impl_noop_dispatch!(WpViewport, wp_viewport::Event);
impl_noop_dispatch!(WlCompositor, wl_compositor::Event);
impl_noop_dispatch!(WlRegion, wl_region::Event);

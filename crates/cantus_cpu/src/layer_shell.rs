use crate::{
    CantusApp, Rect,
    config::{Layer as ConfigLayer, LayerAnchor as ConfigLayerAnchor},
};
use glam::vec2;
use std::{
    collections::hash_map::DefaultHasher,
    ffi::c_void,
    hash::{Hash, Hasher},
    ptr::NonNull,
};
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle, WEnum, delegate_noop,
    protocol::{
        wl_callback::{self, WlCallback},
        wl_compositor::WlCompositor,
        wl_output::{self, WlOutput},
        wl_pointer::{self, WlPointer},
        wl_region::WlRegion,
        wl_registry::{self, WlRegistry},
        wl_seat::{self, WlSeat},
        wl_surface::WlSurface,
    },
};
use wayland_protocols::wp::{
    fractional_scale::v1::client::{
        wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1,
        wp_fractional_scale_v1::{self, WpFractionalScaleV1},
    },
    viewporter::client::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{Layer as LayerStyle, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor as LayerAnchor, ZwlrLayerSurfaceV1},
};
use wgpu::SurfaceTargetUnsafe;
use wgpu::rwh::{RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle};

pub fn run() {
    let connection = Connection::connect_to_env().expect("Failed to connect to Wayland display");
    let mut event_queue = connection.new_event_queue();
    let qhandle = event_queue.handle();
    connection.display().get_registry(&qhandle, ());

    let display_ptr = NonNull::new(connection.backend().display_ptr().cast::<c_void>())
        .expect("Failed to get display pointer");
    let mut app = LayerShellApp::new(display_ptr);

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
    app.surface_ptr = Some(surface_ptr);
    app.select_output();

    let surface = app.wl_surface.insert(wl_surface);
    if let (Some(vp), Some(fm)) = (app.viewporter.take(), app.fractional_manager.take()) {
        app.viewport = Some(vp.get_viewport(surface, &qhandle, ()));
        app.fractional = Some(fm.get_fractional_scale(surface, &qhandle, ()));
    }

    let layer_surface = layer_shell.get_layer_surface(
        surface,
        app.outputs.get(app.output_index).map(|info| &info.handle),
        match app.cantus.config.layer {
            ConfigLayer::Background => LayerStyle::Background,
            ConfigLayer::Bottom => LayerStyle::Bottom,
            ConfigLayer::Top => LayerStyle::Top,
            ConfigLayer::Overlay => LayerStyle::Overlay,
        },
        "cantus".into(),
        &qhandle,
        (),
    );
    let (_, total_height) = app.cantus.logical_surface_size();
    layer_surface.set_size(0, total_height as u32);
    layer_surface.set_anchor(match app.cantus.config.layer_anchor {
        ConfigLayerAnchor::Top => LayerAnchor::Top | LayerAnchor::Left | LayerAnchor::Right,
        ConfigLayerAnchor::Bottom => LayerAnchor::Bottom | LayerAnchor::Left | LayerAnchor::Right,
    });
    layer_surface.set_exclusive_zone(-1);

    surface.commit();
    connection.flush().expect("Failed to flush initial commit");

    app.compositor = Some(compositor);

    while !app.should_exit {
        event_queue
            .blocking_dispatch(&mut app)
            .expect("Wayland dispatch error");
    }
}

struct OutputInfo {
    handle: WlOutput,
    name: Option<String>,
    description: Option<String>,
    make_model: Option<String>,
}

impl OutputInfo {
    fn matches(&self, target: &str) -> bool {
        self.name.as_ref().is_some_and(|name| name.contains(target))
            || self
                .make_model
                .as_ref()
                .is_some_and(|make_model| make_model.contains(target))
            || self
                .description
                .as_ref()
                .is_some_and(|description| description.contains(target))
    }
}

pub struct LayerShellApp {
    pub cantus: CantusApp,

    is_configured: bool,
    should_exit: bool,

    compositor: Option<WlCompositor>,
    layer_shell: Option<ZwlrLayerShellV1>,
    seat: Option<WlSeat>,
    pointer: Option<WlPointer>,
    outputs: Vec<OutputInfo>,
    output_index: usize,
    last_hitbox_hash: u64,

    surface_ptr: Option<NonNull<c_void>>,
    wl_surface: Option<WlSurface>,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    frame_callback: Option<WlCallback>,
    viewporter: Option<WpViewporter>,
    fractional_manager: Option<WpFractionalScaleManagerV1>,
    display_ptr: NonNull<c_void>,
}

impl LayerShellApp {
    fn new(display_ptr: NonNull<c_void>) -> Self {
        Self {
            cantus: CantusApp::default(),
            is_configured: false,
            should_exit: false,
            compositor: None,
            layer_shell: None,
            seat: None,
            pointer: None,
            outputs: Vec::new(),
            output_index: 0,
            last_hitbox_hash: 0,
            surface_ptr: None,
            wl_surface: None,
            viewport: None,
            fractional: None,
            frame_callback: None,
            viewporter: None,
            fractional_manager: None,
            display_ptr,
        }
    }

    fn request_frame(&mut self, qhandle: &QueueHandle<Self>) {
        if self.frame_callback.is_none()
            && let Some(surface) = &self.wl_surface
        {
            self.frame_callback = Some(surface.frame(qhandle, ()));
        }
    }

    fn ensure_surface(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 || !self.is_configured {
            return;
        }

        if let Some(gpu) = &mut self.cantus.gpu_resources {
            gpu.resize_surface(width, height);
            return;
        }

        let Some(surface_ptr) = self.surface_ptr else {
            return;
        };
        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                self.display_ptr,
            ))),
            raw_window_handle: RawWindowHandle::Wayland(WaylandWindowHandle::new(surface_ptr)),
        };
        let surface = unsafe { self.cantus.instance.create_surface_unsafe(target) }
            .expect("Failed to create surface");

        self.cantus.configure_render_surface(surface, width, height);
    }

    fn select_output(&mut self) {
        self.output_index = self
            .cantus
            .config
            .monitor
            .as_ref()
            .and_then(|target| self.outputs.iter().position(|info| info.matches(target)))
            .unwrap_or(0);
    }

    fn try_render_frame(&mut self, qhandle: &QueueHandle<Self>) {
        let (buffer_width, buffer_height) = self.cantus.buffer_size();
        self.ensure_surface(buffer_width, buffer_height);

        if self.cantus.render() {
            tracing::warn!("wgpu surface was lost; recreating it");
            self.cantus.gpu_resources = None;
            self.ensure_surface(buffer_width, buffer_height);
        }
        self.update_input_region(qhandle);
        self.request_frame(qhandle);
        if let Some(surface) = &self.wl_surface {
            surface.commit();
        }
    }

    fn update_scale_and_viewport(&self) {
        let (logical_width, logical_height) = self.cantus.logical_surface_size();
        let (buffer_width, buffer_height) = self.cantus.buffer_size();
        if let Some(surface) = &self.wl_surface {
            surface.set_buffer_scale(
                self.viewport
                    .as_ref()
                    .map_or_else(|| self.cantus.render_scale.ceil() as i32, |_| 1),
            );
        }
        if let Some(viewport) = &self.viewport {
            viewport.set_source(0.0, 0.0, f64::from(buffer_width), f64::from(buffer_height));
            viewport.set_destination(logical_width as i32, logical_height as i32);
        }
    }

    fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        let (Some(wl_surface), Some(compositor)) = (&self.wl_surface, &self.compositor) else {
            return;
        };
        let mut hasher = DefaultHasher::new();
        for r in self.cantus.input_rects() {
            [r.x0, r.y0, r.x1, r.y1]
                .map(|value| value.round() as i32)
                .hash(&mut hasher);
        }
        let hash = hasher.finish();

        if hash != self.last_hitbox_hash {
            let region = compositor.create_region(qhandle, ());
            for r in self.cantus.input_rects() {
                region.add(
                    r.x0.round() as i32,
                    r.y0.round() as i32,
                    (r.x1 - r.x0).round() as i32,
                    (r.y1 - r.y0).round() as i32,
                );
            }
            wl_surface.set_input_region(Some(&region));
            self.last_hitbox_hash = hash;
        }
    }
}

impl CantusApp {
    fn input_rects(&self) -> impl Iterator<Item = Rect> + '_ {
        self.playback_state.queue.iter().flat_map(|track| {
            track
                .runtime
                .rect(self.config.height)
                .into_iter()
                .chain(self.icon_row_rects(track).into_iter().flatten())
        })
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for LayerShellApp {
    fn event(
        state: &mut Self,
        proxy: &ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        match event {
            zwlr_layer_surface_v1::Event::Configure { serial, width, .. } => {
                proxy.ack_configure(serial);
                if width > 0 {
                    state.cantus.surface_width = Some(width as f32);
                }
                state.is_configured = true;
                state.update_scale_and_viewport();
                state.try_render_frame(qhandle);
            }
            zwlr_layer_surface_v1::Event::Closed => state.should_exit = true,
            _ => {}
        }
    }
}

impl Dispatch<WpFractionalScaleV1, ()> for LayerShellApp {
    fn event(
        state: &mut Self,
        _proxy: &WpFractionalScaleV1,
        event: wp_fractional_scale_v1::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wp_fractional_scale_v1::Event::PreferredScale { scale } = event {
            state.cantus.render_scale = scale as f32 / 120.0;

            if state.is_configured {
                state.update_scale_and_viewport();
                state.try_render_frame(qhandle);
            }
        }
    }
}

impl Dispatch<WlCallback, ()> for LayerShellApp {
    fn event(
        state: &mut Self,
        _proxy: &WlCallback,
        event: wl_callback::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if matches!(event, wl_callback::Event::Done { .. }) && state.frame_callback.take().is_some()
        {
            state.try_render_frame(qhandle);
        }
    }
}

impl Dispatch<WlOutput, ()> for LayerShellApp {
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
                    info.make_model = Some(format!("{make} {model}"));
                }
                wl_output::Event::Name { name } => info.name = Some(name),
                wl_output::Event::Description { description } => {
                    info.description = Some(description);
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<WlSeat, ()> for LayerShellApp {
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

impl Dispatch<WlPointer, ()> for LayerShellApp {
    fn event(
        state: &mut Self,
        _proxy: &WlPointer,
        event: wl_pointer::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        let cantus = &mut state.cantus;
        let interaction = &mut cantus.interaction;

        let surface_id = state.wl_surface.as_ref().map(Proxy::id);
        match event {
            wl_pointer::Event::Enter {
                surface,
                surface_x,
                surface_y,
                ..
            } if surface_id == Some(surface.id()) => {
                cantus.global_uniforms.mouse_pos = vec2(surface_x as f32, surface_y as f32);
                interaction.mouse_pressure = 1.0;
            }
            wl_pointer::Event::Motion {
                surface_x,
                surface_y,
                ..
            } => {
                cantus.global_uniforms.mouse_pos = vec2(surface_x as f32, surface_y as f32);
                cantus.handle_mouse_drag();
            }
            wl_pointer::Event::Leave { .. } => {
                interaction.mouse_pressure = 0.0;
                cantus.cancel_drag();
            }
            wl_pointer::Event::Button {
                button,
                state: button_state,
                ..
            } => match (button, button_state) {
                (0x110, WEnum::Value(wl_pointer::ButtonState::Pressed)) => cantus.left_click(),
                (0x110, WEnum::Value(wl_pointer::ButtonState::Released)) => {
                    cantus.left_click_released();
                }
                (0x111, WEnum::Value(wl_pointer::ButtonState::Pressed)) if interaction.dragging => {
                    cantus.right_click();
                }
                _ => {}
            },
            wl_pointer::Event::AxisDiscrete {
                axis: WEnum::Value(wl_pointer::Axis::VerticalScroll),
                discrete,
                ..
            }
            | wl_pointer::Event::AxisValue120 {
                axis: WEnum::Value(wl_pointer::Axis::VerticalScroll),
                value120: discrete,
                ..
            } => state.cantus.handle_scroll(discrete.signum()),
            _ => {}
        }
    }
}

impl Dispatch<WlRegistry, ()> for LayerShellApp {
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
                        make_model: None,
                    });
                }
                _ => {}
            }
        }
    }
}

delegate_noop!(LayerShellApp: ignore WlSurface);
delegate_noop!(LayerShellApp: ignore ZwlrLayerShellV1);
delegate_noop!(LayerShellApp: ignore WpFractionalScaleManagerV1);
delegate_noop!(LayerShellApp: ignore WpViewporter);
delegate_noop!(LayerShellApp: ignore WpViewport);
delegate_noop!(LayerShellApp: ignore WlCompositor);
delegate_noop!(LayerShellApp: ignore WlRegion);

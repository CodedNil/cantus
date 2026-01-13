use crate::{
    CantusApp, PANEL_EXTENSION, PANEL_START, config::CONFIG, interaction::InteractionState,
    render::Point,
};
use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use std::{
    ffi::c_void,
    ptr::NonNull,
    time::{Duration, Instant},
};
use tracing::error;
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
    zwlr_layer_shell_v1::{self, Layer as LayerStyle, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor as LayerAnchor, ZwlrLayerSurfaceV1},
};
use wgpu::SurfaceTargetUnsafe;

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
    assert!(app.try_select_output(), "Failed to select a Wayland output");

    let surface = app.wl_surface.insert(wl_surface);
    if let (Some(vp), Some(fm)) = (app.viewporter.take(), app.fractional_manager.take()) {
        app.viewport = Some(vp.get_viewport(surface, &qhandle, ()));
        app.fractional = Some(fm.get_fractional_scale(surface, &qhandle, ()));
    }

    let layer_surface = layer_shell.get_layer_surface(
        surface,
        app.outputs.get(app.output_index).map(|info| &info.handle),
        match CONFIG.layer.as_str() {
            "background" => LayerStyle::Background,
            "bottom" => LayerStyle::Bottom,
            "top" => LayerStyle::Top,
            "overlay" => LayerStyle::Overlay,
            other => {
                error!("Invalid layer '{other}', defaulting to 'top'");
                LayerStyle::Top
            }
        },
        "cantus".into(),
        &qhandle,
        (),
    );
    let width = CONFIG.width;
    let total_height = CONFIG.height + PANEL_EXTENSION + PANEL_START;
    layer_surface.set_size(width as u32, total_height as u32);
    layer_surface.set_anchor(match CONFIG.layer_anchor.as_str() {
        "top" => LayerAnchor::Top,
        "topright" => LayerAnchor::Top | LayerAnchor::Right,
        "right" => LayerAnchor::Right,
        "bottomright" => LayerAnchor::Bottom | LayerAnchor::Right,
        "bottom" => LayerAnchor::Bottom,
        "bottomleft" => LayerAnchor::Bottom | LayerAnchor::Left,
        "left" => LayerAnchor::Left,
        "topleft" => LayerAnchor::Top | LayerAnchor::Left,
        other => {
            error!("Invalid layer anchor '{other}', defaulting to 'topleft'");
            LayerAnchor::Top | LayerAnchor::Left
        }
    });
    layer_surface.set_margin(0, 0, 0, 0);
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
    make: Option<String>,
    model: Option<String>,
}

impl OutputInfo {
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
        if self.frame_callback.is_some() {
            return;
        }
        if let Some(surface) = &self.wl_surface {
            self.frame_callback = Some(surface.frame(qhandle, ()));
        }
    }

    fn ensure_surface(&mut self, width: f32, height: f32) {
        if width == 0.0 || height == 0.0 || !self.is_configured {
            return;
        }

        let recreate = self.cantus.render_surface.as_ref().is_none_or(|surface| {
            surface.config.width != width as u32 || surface.config.height != height as u32
        });
        if !recreate {
            return;
        }

        let Some(surface_ptr) = self.surface_ptr else {
            return;
        };
        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                self.display_ptr,
            )),
            raw_window_handle: RawWindowHandle::Wayland(WaylandWindowHandle::new(surface_ptr)),
        };
        let surface = unsafe {
            self.cantus
                .render_context
                .instance
                .create_surface_unsafe(target)
        }
        .expect("Failed to create surface");

        self.cantus
            .configure_render_surface(surface, width as u32, height as u32);
    }

    fn try_select_output(&mut self) -> bool {
        if self.outputs.is_empty() {
            return false;
        }

        self.output_index = CONFIG
            .monitor
            .as_ref()
            .and_then(|target| self.outputs.iter().position(|info| info.matches(target)))
            .unwrap_or(0);
        true
    }

    fn try_render_frame(&mut self, qhandle: &QueueHandle<Self>) {
        let scale = self.cantus.scale_factor;
        let width = CONFIG.width;
        let total_height = CONFIG.height + PANEL_EXTENSION + PANEL_START;
        let buffer_width = (width * scale).round();
        let buffer_height = (total_height * scale).round();
        self.ensure_surface(buffer_width, buffer_height);

        self.update_input_region(qhandle);

        self.cantus.render();
        self.request_frame(qhandle);
        if let Some(surface) = &self.wl_surface {
            surface.commit();
        }
    }

    fn update_scale_and_viewport(&self) {
        let scale = self.cantus.scale_factor;
        let width = CONFIG.width;
        let total_height = CONFIG.height + PANEL_EXTENSION + PANEL_START;
        let viewport = self.viewport.as_ref();
        if let Some(surface) = &self.wl_surface {
            surface.set_buffer_scale(if viewport.is_some() {
                1
            } else {
                scale.ceil() as i32
            });
        }
        if let Some(viewport) = viewport {
            viewport.set_source(
                0.0,
                0.0,
                f64::from(width * scale).round(),
                f64::from(total_height * scale).round(),
            );
            viewport.set_destination(width as i32, total_height as i32);
        }
    }

    fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        if self.cantus.interaction.last_hitbox_update.elapsed() <= Duration::from_millis(500) {
            return;
        }

        let (Some(wl_surface), Some(compositor)) = (&self.wl_surface, &self.compositor) else {
            return;
        };

        let scale = self.cantus.scale_factor;

        let region = compositor.create_region(qhandle, ());
        {
            let interaction = &self.cantus.interaction;
            for rect in interaction
                .track_hitboxes
                .iter()
                .map(|(_, rect, _)| rect)
                .chain(interaction.icon_hitboxes.iter().map(|hitbox| &hitbox.rect))
            {
                region.add(
                    (rect.x0 * scale).round() as i32,
                    (rect.y0 * scale).round() as i32,
                    ((rect.x1 - rect.x0) * scale).round() as i32,
                    ((rect.y1 - rect.y0) * scale).round() as i32,
                );
            }
        }

        wl_surface.set_input_region(Some(&region));
        wl_surface.commit();
        self.cantus.interaction.last_hitbox_update = Instant::now();
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
            zwlr_layer_surface_v1::Event::Configure { serial, .. } => {
                proxy.ack_configure(serial);
                state.update_scale_and_viewport();
                if let Some(surface) = &state.wl_surface {
                    surface.commit();
                }
                state.is_configured = true;

                state.try_render_frame(qhandle);
            }
            zwlr_layer_surface_v1::Event::Closed => {
                state.should_exit = true;
            }
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
            state.cantus.scale_factor = scale as f32 / 120.0;

            if state.is_configured {
                state.update_scale_and_viewport();

                if let Some(surface) = &state.wl_surface {
                    surface.commit();
                }

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
            if caps.contains(wl_seat::Capability::Pointer) && state.pointer.is_none() {
                state.pointer = Some(proxy.get_pointer(qhandle, ()));
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
        let interaction = &mut state.cantus.interaction;

        let surface_id = state.wl_surface.as_ref().map(wayland_client::Proxy::id);
        match event {
            wl_pointer::Event::Enter {
                surface,
                surface_x,
                surface_y,
                ..
            } if surface_id == Some(surface.id()) => {
                interaction.mouse_position = Point::new(surface_x as f32, surface_y as f32);
            }
            wl_pointer::Event::Motion {
                surface_x,
                surface_y,
                ..
            } => {
                interaction.mouse_position = Point::new(surface_x as f32, surface_y as f32);
                interaction.handle_mouse_drag();
            }
            wl_pointer::Event::Leave { .. } => {
                interaction.mouse_position = Point::new(-100.0, -100.0);
                interaction.cancel_drag();
                interaction.mouse_down = false;
            }
            wl_pointer::Event::Button {
                button,
                state: button_state,
                ..
            } => match (button, button_state) {
                (0x110, WEnum::Value(wl_pointer::ButtonState::Pressed)) => interaction.left_click(),
                (0x110, WEnum::Value(wl_pointer::ButtonState::Released)) => {
                    interaction.left_click_released();
                }
                (0x111, WEnum::Value(wl_pointer::ButtonState::Pressed)) if interaction.dragging => {
                    interaction.right_click();
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
            } => {
                InteractionState::handle_scroll(discrete.signum());
            }
            wl_pointer::Event::Axis { .. }
            | wl_pointer::Event::Frame
            | wl_pointer::Event::AxisSource { .. }
            | wl_pointer::Event::AxisStop { .. }
            | wl_pointer::Event::AxisRelativeDirection { .. }
            | _ => {}
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
                        make: None,
                        model: None,
                    });
                }
                _ => {}
            }
        }
    }
}

macro_rules! impl_noop_dispatch {
    ($ty:ty, $event:ty) => {
        impl Dispatch<$ty, ()> for LayerShellApp {
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

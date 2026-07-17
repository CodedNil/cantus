use crate::{
    CantusApp, PANEL_START,
    config::{Layer as ConfigLayer, LayerAnchor as ConfigLayerAnchor},
    model::Rect,
    status::Status,
    weather,
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
use wayland_protocols::ext::background_effect::v1::client::{
    ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1,
    ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1,
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
use wgpu::rwh::{RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle};
use wgpu::{Surface, SurfaceTargetUnsafe};

pub fn run() {
    let connection = Connection::connect_to_env().expect("Failed to connect to Wayland display");
    let mut event_queue = connection.new_event_queue();
    let qhandle = event_queue.handle();
    connection.display().get_registry(&qhandle, ());

    let display_ptr = NonNull::new(connection.backend().display_ptr().cast::<c_void>())
        .expect("Failed to get display pointer");
    let mut app = LayerShellApp::default();

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
    let target = SurfaceTargetUnsafe::RawHandle {
        raw_display_handle: Some(RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            display_ptr,
        ))),
        raw_window_handle: RawWindowHandle::Wayland(WaylandWindowHandle::new(surface_ptr)),
    };
    app.pending_surface = Some(
        unsafe { app.cantus.render.instance.create_surface_unsafe(target) }
            .expect("Failed to create surface"),
    );
    let output_index = app
        .cantus
        .config
        .monitor
        .as_ref()
        .and_then(|target| {
            app.outputs.iter().position(|info| {
                info.identifiers
                    .iter()
                    .flatten()
                    .any(|value| value.contains(target))
            })
        })
        .unwrap_or(0);

    let surface = app.wl_surface.insert(wl_surface);
    if let (Some(vp), Some(fm)) = (app.viewporter.take(), app.fractional_manager.take()) {
        app.viewport = Some(vp.get_viewport(surface, &qhandle, ()));
        app.fractional = Some(fm.get_fractional_scale(surface, &qhandle, ()));
    }
    if let Some(manager) = &app.background_effect_manager {
        app.background_effect = Some(manager.get_background_effect(surface, &qhandle, ()));
    }

    let layer_surface = layer_shell.get_layer_surface(
        surface,
        Some(&app.outputs[output_index].handle),
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
    layer_surface.set_exclusive_zone((PANEL_START + app.cantus.config.height) as i32);

    surface.commit();
    connection.flush().expect("Failed to flush initial commit");

    app.compositor = Some(compositor);

    while !app.should_exit {
        event_queue
            .blocking_dispatch(&mut app)
            .expect("Wayland dispatch error");
    }
}

fn region_rect(rect: Rect) -> [i32; 4] {
    [rect.x0, rect.y0, rect.x1 - rect.x0, rect.y1 - rect.y0].map(|value| value.round() as i32)
}

fn add_rect(region: &WlRegion, [x, y, width, height]: [i32; 4]) {
    region.add(x, y, width, height);
}

/// Approximate a capsule with one region rectangle per logical-pixel row.
fn add_capsule(region: &WlRegion, rect: Rect) {
    // Integer Wayland regions cannot exactly follow an anti-aliased edge. Keeping
    // the effect one pixel inside prevents blur leaking past the rendered pill.
    let [x, y, width, height] = region_rect(rect);
    let (x, y, width, height) = (x + 1, y + 1, width - 2, height - 2);
    let radius = height as f32 * 0.5;
    for row in 0..height {
        let y_from_center = (row as f32 + 0.5 - radius).abs();
        let inset =
            (radius - (radius * radius - y_from_center * y_from_center).sqrt()).ceil() as i32;
        add_rect(region, [x + inset, y + row, width - inset * 2, 1]);
    }
}

struct OutputInfo {
    handle: WlOutput,
    identifiers: [Option<String>; 3],
}

#[derive(Default)]
pub struct LayerShellApp {
    pub cantus: CantusApp,

    is_configured: bool,
    should_exit: bool,

    compositor: Option<WlCompositor>,
    layer_shell: Option<ZwlrLayerShellV1>,
    seat: Option<WlSeat>,
    pointer: Option<WlPointer>,
    outputs: Vec<OutputInfo>,
    last_hitbox_hash: u64,

    pending_surface: Option<Surface<'static>>,
    wl_surface: Option<WlSurface>,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    frame_callback: Option<WlCallback>,
    viewporter: Option<WpViewporter>,
    fractional_manager: Option<WpFractionalScaleManagerV1>,
    background_effect_manager: Option<ExtBackgroundEffectManagerV1>,
    background_effect: Option<ExtBackgroundEffectSurfaceV1>,
}

macro_rules! dispatch {
    ($proxy:ty, |$state:ident, $object:ident, $value:ident, $queue:ident| $body:block) => {
        impl Dispatch<$proxy, ()> for LayerShellApp {
            fn event(
                $state: &mut Self,
                $object: &$proxy,
                $value: <$proxy as Proxy>::Event,
                _data: &(),
                _conn: &Connection,
                $queue: &QueueHandle<Self>,
            ) $body
        }
    };
}

impl LayerShellApp {
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

        if let Some(gpu) = &mut self.cantus.render.gpu {
            gpu.resize_surface(width, height);
            return;
        }

        let Some(surface) = self.pending_surface.take() else {
            return;
        };
        self.cantus.configure_render_surface(surface, width, height);
    }

    fn try_render_frame(&mut self, qhandle: &QueueHandle<Self>) {
        let (buffer_width, buffer_height) = self.cantus.buffer_size();
        self.ensure_surface(buffer_width, buffer_height);

        self.cantus.render();
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
                    .map_or_else(|| self.cantus.render.scale.ceil() as i32, |_| 1),
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
        for rect in self.cantus.input_rects().map(region_rect) {
            rect.hash(&mut hasher);
        }
        let hash = hasher.finish();

        if hash != self.last_hitbox_hash {
            let region = compositor.create_region(qhandle, ());
            for rect in self.cantus.input_rects() {
                add_rect(&region, region_rect(rect));
            }
            wl_surface.set_input_region(Some(&region));
            region.destroy();
            self.last_hitbox_hash = hash;
        }
    }

    fn update_blur_region(&self, qhandle: &QueueHandle<Self>) {
        let (Some(effect), Some(compositor)) = (&self.background_effect, &self.compositor) else {
            return;
        };
        let region = compositor.create_region(qhandle, ());
        let width = self.cantus.logical_surface_size().0;
        let pill = self.cantus.status.pill(width);
        add_capsule(
            &region,
            Rect::pill(pill.x, pill.width, self.cantus.config.height),
        );
        add_capsule(&region, weather::rect(pill, self.cantus.config.height));
        effect.set_blur_region(Some(&region));
        region.destroy();
    }
}

impl CantusApp {
    fn input_rects(&self) -> impl Iterator<Item = Rect> + '_ {
        self.playback
            .queue
            .iter()
            .flat_map(|track| {
                track
                    .runtime
                    .rect(self.config.height)
                    .into_iter()
                    .chain(self.icon_row_rects(track).into_iter().flatten())
            })
            .chain([
                weather::rect(self.render.status, self.config.height),
                Status::controls_rect(self.render.status, self.config.height),
            ])
    }
}

dispatch!(ZwlrLayerSurfaceV1, |state, proxy, event, qhandle| {
    match event {
        zwlr_layer_surface_v1::Event::Configure { serial, width, .. } => {
            proxy.ack_configure(serial);
            if width > 0 {
                state.cantus.render.surface_width = Some(width as f32);
            }
            state.is_configured = true;
            state.update_scale_and_viewport();
            state.update_blur_region(qhandle);
            state.try_render_frame(qhandle);
        }
        zwlr_layer_surface_v1::Event::Closed => state.should_exit = true,
        _ => {}
    }
});

dispatch!(WpFractionalScaleV1, |state, _proxy, event, qhandle| {
    if let wp_fractional_scale_v1::Event::PreferredScale { scale } = event {
        state.cantus.render.scale = scale as f32 / 120.0;

        if state.is_configured {
            state.update_scale_and_viewport();
            state.try_render_frame(qhandle);
        }
    }
});

dispatch!(WlCallback, |state, _proxy, event, qhandle| {
    if matches!(event, wl_callback::Event::Done { .. }) && state.frame_callback.take().is_some() {
        state.try_render_frame(qhandle);
    }
});

dispatch!(WlOutput, |state, proxy, event, _qhandle| {
    let id = proxy.id();
    if let Some(info) = state.outputs.iter_mut().find(|info| info.handle.id() == id) {
        match event {
            wl_output::Event::Geometry { make, model, .. } => {
                info.identifiers[2] = Some(format!("{make} {model}"));
            }
            wl_output::Event::Name { name } => info.identifiers[0] = Some(name),
            wl_output::Event::Description { description } => {
                info.identifiers[1] = Some(description);
            }
            _ => {}
        }
    }
});

dispatch!(WlSeat, |state, proxy, event, qhandle| {
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
});

dispatch!(WlPointer, |state, _proxy, event, _qhandle| {
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
            cantus.render.uniforms.mouse_pos = vec2(surface_x as f32, surface_y as f32);
            interaction.mouse_pressure = 1.0;
        }
        wl_pointer::Event::Motion {
            surface_x,
            surface_y,
            ..
        } => {
            cantus.render.uniforms.mouse_pos = vec2(surface_x as f32, surface_y as f32);
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
});

dispatch!(WlRegistry, |state, proxy, event, qhandle| {
    if let wl_registry::Event::Global {
        name,
        interface,
        version,
    } = event
    {
        macro_rules! bind {
            ($type:ty, $version:expr) => {
                proxy.bind::<$type, (), Self>(name, $version, qhandle, ())
            };
        }
        match interface.as_ref() {
            "wl_compositor" => state.compositor = Some(bind!(WlCompositor, version)),
            "zwlr_layer_shell_v1" => state.layer_shell = Some(bind!(ZwlrLayerShellV1, 4)),
            "wp_viewporter" => state.viewporter = Some(bind!(WpViewporter, 1)),
            "wp_fractional_scale_manager_v1" => {
                state.fractional_manager = Some(bind!(WpFractionalScaleManagerV1, 1));
            }
            "ext_background_effect_manager_v1" => {
                state.background_effect_manager =
                    Some(bind!(ExtBackgroundEffectManagerV1, version.min(1)));
            }
            "wl_seat" => state.seat = Some(bind!(WlSeat, version.min(7))),
            "wl_output" => {
                state.outputs.push(OutputInfo {
                    handle: bind!(WlOutput, version.min(4)),
                    identifiers: [None, None, None],
                });
            }
            _ => {}
        }
    }
});

delegate_noop!(LayerShellApp: ignore WlSurface);
delegate_noop!(LayerShellApp: ignore ZwlrLayerShellV1);
delegate_noop!(LayerShellApp: ignore WpFractionalScaleManagerV1);
delegate_noop!(LayerShellApp: ignore WpViewporter);
delegate_noop!(LayerShellApp: ignore WpViewport);
delegate_noop!(LayerShellApp: ignore WlCompositor);
delegate_noop!(LayerShellApp: ignore WlRegion);
delegate_noop!(LayerShellApp: ignore ExtBackgroundEffectSurfaceV1);
delegate_noop!(LayerShellApp: ignore ExtBackgroundEffectManagerV1);

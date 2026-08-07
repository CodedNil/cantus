use crate::{
    app::{
        AppUpdater, CantusApp,
        config::{Layer as ConfigLayer, LayerAnchor as ConfigLayerAnchor},
        send_update,
    },
    render::{
        shared::PANEL_START,
        status::{AUDIO_SPECTRUM_BANDS, BATTERY_HIDDEN, ProcessorStatus},
    },
};
use isthmus::glam::vec2;
use isthmus::wgpu::{
    Surface, SurfaceTargetUnsafe,
    rwh::{RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle},
};
use microfft::real::rfft_1024;
use serde_json::Value;
use std::{
    collections::HashMap,
    ffi::c_void,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    ptr::NonNull,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread,
    time::Duration,
};
use sysinfo::{Gpus, System};
use tracing::warn;
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle, WEnum, delegate_noop,
    globals::{GlobalListContents, registry_queue_init},
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

pub const STATUS_SAMPLE_INTERVAL: Duration = Duration::from_millis(500);
const AUDIO_SAMPLE_RATE: u32 = 48_000;
const AUDIO_WINDOW_SIZE: usize = 1024;
const AUDIO_BAND_EDGES: [f32; AUDIO_SPECTRUM_BANDS + 1] =
    [60.0, 120.0, 250.0, 500.0, 1_000.0, 2_000.0, 4_000.0, 12_000.0];

pub fn start_status_monitor(updater: AppUpdater, spectrum: Arc<[AtomicU32; AUDIO_SPECTRUM_BANDS]>) {
    let volume_updater = updater.clone();
    thread::spawn(move || monitor_playback(&spectrum));
    thread::spawn(move || monitor_volume(&volume_updater));
    thread::spawn(move || monitor_status(&updater));
}

pub fn set_volume(volume: f32) {
    let volume = format!("{volume:.3}");
    if let Err(error) = Command::new("wpctl")
        .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &volume])
        .spawn()
    {
        warn!(%error, "Failed to set PipeWire volume");
    }
}

pub fn run_power_action(action: usize) {
    let command = ["poweroff", "reboot"][action];
    if let Err(error) = Command::new("systemctl").arg(command).spawn() {
        warn!(%error, %command, "Failed to run held power action");
    }
}

fn monitor_status(updater: &AppUpdater) {
    let Ok(mut system) = System::new() else {
        warn!("sysinfo unavailable; system status monitor disabled");
        return;
    };
    let mut gpus = Gpus::new_with_refreshed_list().ok();
    let battery = find_battery();
    loop {
        system.refresh_cpu_usage();
        system.refresh_cpu_temperature();
        system.refresh_memory();
        if let Some(gpus) = &mut gpus {
            gpus.refresh(false);
        }
        let cpu = [
            system.cpus().first().map_or(0.0, sysinfo::Cpu::temperature),
            system.global_cpu_usage() / 100.0,
            system.used_memory() as f32 / system.total_memory().max(1) as f32,
        ];
        let gpu = gpus.as_ref().and_then(gpu_sample);
        let battery_level = battery
            .as_deref()
            .and_then(battery_level)
            .map_or(BATTERY_HIDDEN, |level| {
                if battery.as_deref().is_some_and(battery_charging) {
                    -level.max(f32::EPSILON)
                } else if level >= 0.995 {
                    // Hidden while idle at full charge; layout reads this once per poll, not per frame.
                    BATTERY_HIDDEN
                } else {
                    level
                }
            });
        if !send_update(updater, move |app| {
            let status = app.render.program().passes_mut().status.as_mut().unwrap();
            let data = &mut *status.pill;
            apply_processor(&mut data.cpu, cpu);
            if let Some(gpu) = gpu {
                apply_processor(&mut data.gpu, gpu);
            }
            data.battery_level = battery_level;
            data.history_scroll = 0.0;
        }) {
            break;
        }
        thread::sleep(STATUS_SAMPLE_INTERVAL);
    }
}

fn apply_processor(status: &mut ProcessorStatus, [temperature, usage, memory]: [f32; 3]) {
    status.temperature = temperature;
    status.usage.push(usage);
    status.memory.push(memory);
}

fn gpu_sample(gpus: &Gpus) -> Option<[f32; 3]> {
    let device = gpus
        .iter()
        .max_by_key(|gpu| gpu.total_memory().unwrap_or_default())?;
    Some([
        device.temperature().unwrap_or_default(),
        device.usage().unwrap_or_default() / 100.0,
        device.used_memory().unwrap_or_default() as f32
            / device.total_memory().unwrap_or_default().max(1) as f32,
    ])
}

fn find_battery() -> Option<PathBuf> {
    fs::read_dir("/sys/class/power_supply")
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            fs::read_to_string(path.join("type"))
                .is_ok_and(|kind| kind.trim().eq_ignore_ascii_case("battery"))
        })
}

fn battery_level(path: &Path) -> Option<f32> {
    fs::read_to_string(path.join("capacity"))
        .ok()?
        .trim()
        .parse::<f32>()
        .ok()
        .map(|level| level / 100.0)
}

fn battery_charging(path: &Path) -> bool {
    fs::read_to_string(path.join("status"))
        .is_ok_and(|status| status.trim().eq_ignore_ascii_case("charging"))
}

fn monitor_playback(levels: &[AtomicU32; AUDIO_SPECTRUM_BANDS]) {
    loop {
        if let Err(error) = capture_playback(levels) {
            warn!(%error, "PipeWire playback meter stopped");
        }
        for level in levels {
            level.store(0.0f32.to_bits(), Ordering::Relaxed);
        }
        thread::sleep(Duration::from_secs(1));
    }
}

#[derive(Default)]
struct PipeWireState {
    default_sink: Option<String>,
    sinks: HashMap<String, f32>,
}

impl PipeWireState {
    fn update(&mut self, object: &Value) -> Option<f32> {
        if let Some(metadata) = object["metadata"]
            .as_array()
            .and_then(|items| items.iter().find(|item| item["key"] == "default.audio.sink"))
        {
            self.default_sink = metadata["value"]["name"].as_str().map(str::to_owned);
            return self.sinks.get(self.default_sink.as_ref()?).copied();
        }
        let info = &object["info"];
        if info["props"]["media.class"] == "Audio/Sink"
            && let Some(name) = info["props"]["node.name"].as_str()
            && let Some(props) = info["params"]["Props"]
                .as_array()
                .and_then(|items| items.iter().find(|props| props["channelVolumes"].is_array()))
        {
            let volumes = props["channelVolumes"].as_array().unwrap();
            let mut volume = (volumes.iter().filter_map(Value::as_f64).sum::<f64>()
                / volumes.len().max(1) as f64)
                .cbrt() as f32;
            if props["mute"].as_bool().unwrap_or_default() {
                volume = -volume;
            }
            self.sinks.insert(name.to_owned(), volume);
            return (Some(name) == self.default_sink.as_deref()).then_some(volume);
        }
        None
    }
}

fn monitor_volume(updater: &AppUpdater) {
    loop {
        match capture_volume(updater) {
            Ok(false) => break,
            Err(error) => warn!(%error, "PipeWire volume monitor stopped"),
            Ok(true) => {}
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn capture_volume(updater: &AppUpdater) -> io::Result<bool> {
    let mut child = Command::new("pw-dump")
        .args(["--monitor", "--no-colors", "--indent", "0"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let output = child.stdout.take().expect("piped PipeWire output");
    let mut state = PipeWireState::default();
    for batch in serde_json::Deserializer::from_reader(output).into_iter::<Vec<Value>>() {
        for object in batch.map_err(io::Error::other)? {
            if let Some(volume) = state.update(&object)
                && !send_update(updater, move |app| {
                    app.render
                        .program()
                        .passes_mut()
                        .status
                        .as_mut()
                        .unwrap()
                        .pill
                        .volume = volume;
                })
            {
                child.kill()?;
                child.wait()?;
                return Ok(false);
            }
        }
    }
    child.wait()?;
    Ok(true)
}

fn capture_playback(levels: &[AtomicU32; AUDIO_SPECTRUM_BANDS]) -> io::Result<()> {
    let mut child = Command::new("pw-record")
        .args([
            "--properties",
            "stream.capture.sink=true",
            "--rate",
            "48000",
            "--channels",
            "1",
            "--format",
            "f32",
            "--raw",
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let mut output = child.stdout.take().expect("piped PipeWire output");
    let mut window = [0.0; AUDIO_WINDOW_SIZE];
    loop {
        match output.read_exact(bytemuck::cast_slice_mut(&mut window)) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(error) => return Err(error),
        }
        let spectrum = rfft_1024(&mut window);
        for (band, level) in levels.iter().enumerate() {
            let bin = |frequency: f32| {
                (frequency * AUDIO_WINDOW_SIZE as f32 / AUDIO_SAMPLE_RATE as f32).ceil() as usize
            };
            let bins = &spectrum[bin(AUDIO_BAND_EDGES[band])..bin(AUDIO_BAND_EDGES[band + 1])];
            let rms = (bins.iter().map(microfft::Complex32::norm_sqr).sum::<f32>()
                / bins.len() as f32
                / AUDIO_WINDOW_SIZE as f32)
                .sqrt();
            let value = ((20.0 * rms.log10() + 30.0) / 30.0).clamp(0.0, 1.0);
            level.store(value.to_bits(), Ordering::Relaxed);
        }
    }
    child.wait()?;
    Ok(())
}

pub fn run() {
    let connection = Connection::connect_to_env().expect("Failed to connect to Wayland display");
    let (globals, mut event_queue) =
        registry_queue_init::<LayerShellApp>(&connection).expect("Failed to read Wayland registry");
    let qhandle = event_queue.handle();
    let compositor: WlCompositor = globals.bind(&qhandle, 1..=7, ()).expect("Missing wl_compositor");
    let layer_shell: ZwlrLayerShellV1 = globals
        .bind(&qhandle, 4..=4, ())
        .expect("Missing zwlr_layer_shell_v1");
    let viewporter: Option<WpViewporter> = globals.bind(&qhandle, 1..=1, ()).ok();
    let fractional_manager: Option<WpFractionalScaleManagerV1> = globals.bind(&qhandle, 1..=1, ()).ok();

    let display_ptr = NonNull::new(connection.backend().display_ptr().cast::<c_void>())
        .expect("Failed to get display pointer");
    let mut app = LayerShellApp {
        compositor: Some(compositor.clone()),
        ..LayerShellApp::default()
    };

    let registry = globals.registry();
    globals.contents().with_list(|list| {
        for global in list {
            match global.interface.as_str() {
                "wl_seat" => drop(registry.bind::<WlSeat, (), LayerShellApp>(
                    global.name,
                    global.version.min(7),
                    &qhandle,
                    (),
                )),
                "wl_output" => {
                    let output = registry.bind::<WlOutput, (), LayerShellApp>(
                        global.name,
                        global.version.min(4),
                        &qhandle,
                        (),
                    );
                    app.output.get_or_insert(output);
                }
                _ => {}
            }
        }
    });
    event_queue
        .roundtrip(&mut app)
        .expect("Failed to fetch output details");

    let wl_surface = compositor.create_surface(&qhandle, ());
    let surface_ptr =
        NonNull::new(wl_surface.id().as_ptr().cast::<c_void>()).expect("Failed to get surface pointer");
    app.surface_handles = Some((
        RawDisplayHandle::Wayland(WaylandDisplayHandle::new(display_ptr)),
        RawWindowHandle::Wayland(WaylandWindowHandle::new(surface_ptr)),
    ));
    app.pending_surface = Some(app.create_render_surface());
    let output = app.output.take().expect("No Wayland outputs found");

    let surface = app.wl_surface.insert(wl_surface);
    if let (Some(vp), Some(fm)) = (viewporter, fractional_manager) {
        app.viewport = Some(vp.get_viewport(surface, &qhandle, ()));
        app.fractional = Some(fm.get_fractional_scale(surface, &qhandle, ()));
    }
    let layer_surface = layer_shell.get_layer_surface(
        surface,
        Some(&output),
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
    layer_surface.set_size(0, app.cantus.logical_surface_size().1 as u32);
    layer_surface.set_anchor(match app.cantus.config.layer_anchor {
        ConfigLayerAnchor::Top => LayerAnchor::Top | LayerAnchor::Left | LayerAnchor::Right,
        ConfigLayerAnchor::Bottom => LayerAnchor::Bottom | LayerAnchor::Left | LayerAnchor::Right,
    });
    layer_surface.set_exclusive_zone((PANEL_START + app.cantus.config.height) as i32);

    surface.commit();
    connection.flush().expect("Failed to flush initial commit");

    while !app.should_exit {
        event_queue
            .blocking_dispatch(&mut app)
            .expect("Wayland dispatch error");
    }
}

#[derive(Default)]
struct LayerShellApp {
    cantus: CantusApp,

    should_exit: bool,

    compositor: Option<WlCompositor>,
    pointer: Option<WlPointer>,
    output: Option<WlOutput>,
    pending_surface: Option<Surface<'static>>,
    surface_handles: Option<(RawDisplayHandle, RawWindowHandle)>,
    wl_surface: Option<WlSurface>,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    frame_callback: Option<WlCallback>,
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
    fn create_render_surface(&self) -> Surface<'static> {
        let (display, window) = self.surface_handles.expect("missing Wayland surface handles");
        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(display),
            raw_window_handle: window,
        };
        unsafe { self.cantus.render.instance.create_surface_unsafe(target) }
            .expect("Failed to create surface")
    }

    fn try_render_frame(&mut self, qhandle: &QueueHandle<Self>) {
        let (buffer_width, buffer_height) = self.cantus.buffer_size();
        if buffer_width > 0 && buffer_height > 0 {
            if let Some(program) = &mut self.cantus.render.program {
                program.resize(buffer_width, buffer_height);
            } else if let Some(surface) = self.pending_surface.take() {
                self.cantus.initialize_gpu(surface, buffer_width, buffer_height);
            }
        }

        if self.cantus.render() {
            let surface = self.create_render_surface();
            self.cantus.replace_render_surface(surface);
        }
        self.update_input_region(qhandle);
        let surface = self.wl_surface.as_ref().unwrap();
        if self.frame_callback.is_none() {
            self.frame_callback = Some(surface.frame(qhandle, ()));
        }
        surface.commit();
    }

    fn update_scale_and_viewport(&self) {
        let (logical_width, logical_height) = self.cantus.logical_surface_size();
        let (buffer_width, buffer_height) = self.cantus.buffer_size();
        self.wl_surface.as_ref().unwrap().set_buffer_scale(
            self.viewport
                .as_ref()
                .map_or_else(|| self.cantus.render.scale.ceil() as i32, |_| 1),
        );
        if let Some(viewport) = &self.viewport {
            viewport.set_source(0.0, 0.0, f64::from(buffer_width), f64::from(buffer_height));
            viewport.set_destination(logical_width as i32, logical_height as i32);
        }
    }

    fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        let wl_surface = self.wl_surface.as_ref().unwrap();
        let compositor = self.compositor.as_ref().unwrap();
        let region = compositor.create_region(qhandle, ());
        for rect in self.cantus.interaction.take_regions() {
            let [x, y, width, height] = [rect.x0, rect.y0, rect.x1 - rect.x0, rect.y1 - rect.y0]
                .map(|value| value.round() as i32);
            region.add(x, y, width, height);
        }
        wl_surface.set_input_region(Some(&region));
        region.destroy();
    }
}

dispatch!(ZwlrLayerSurfaceV1, |state, proxy, event, qhandle| {
    match event {
        zwlr_layer_surface_v1::Event::Configure { serial, width, .. } => {
            proxy.ack_configure(serial);
            if width > 0 {
                state.cantus.render.surface_width = Some(width as f32);
            }
            state.update_scale_and_viewport();
            state.try_render_frame(qhandle);
        }
        zwlr_layer_surface_v1::Event::Closed => state.should_exit = true,
        _ => {}
    }
});

dispatch!(WpFractionalScaleV1, |state, _proxy, event, qhandle| {
    if let wp_fractional_scale_v1::Event::PreferredScale { scale } = event {
        state.cantus.render.scale = scale as f32 / 120.0;

        if state.cantus.render.program.is_some() {
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
    let identifier = match event {
        wl_output::Event::Geometry { make, model, .. } => format!("{make} {model}"),
        wl_output::Event::Name { name } | wl_output::Event::Description { description: name } => name,
        _ => return,
    };
    if state
        .cantus
        .config
        .monitor
        .as_ref()
        .is_some_and(|target| identifier.contains(target))
    {
        state.output = Some(proxy.clone());
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
            interaction.motion(vec2(surface_x as f32, surface_y as f32));
            interaction.hover();
        }
        wl_pointer::Event::Motion {
            surface_x, surface_y, ..
        } => {
            let position = vec2(surface_x as f32, surface_y as f32);
            interaction.motion(position);
        }
        wl_pointer::Event::Leave { .. } => {
            interaction.leave();
        }
        wl_pointer::Event::Button {
            button,
            state: button_state,
            ..
        } => match (button, button_state) {
            (0x110, WEnum::Value(wl_pointer::ButtonState::Pressed)) => {
                interaction.press(interaction.pointer);
            }
            (0x110, WEnum::Value(wl_pointer::ButtonState::Released)) => {
                interaction.release();
            }
            (0x111, WEnum::Value(wl_pointer::ButtonState::Pressed)) if interaction.dragging => {
                interaction.cancel_drag();
                interaction.hover();
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
        } if discrete != 0 => {
            state.cantus.interaction.scroll_input(discrete.signum());
        }
        _ => {}
    }
});

impl Dispatch<WlRegistry, GlobalListContents> for LayerShellApp {
    fn event(
        _: &mut Self,
        _: &WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

delegate_noop!(LayerShellApp: ignore WlSurface);
delegate_noop!(LayerShellApp: ignore ZwlrLayerShellV1);
delegate_noop!(LayerShellApp: ignore WpFractionalScaleManagerV1);
delegate_noop!(LayerShellApp: ignore WpViewporter);
delegate_noop!(LayerShellApp: ignore WpViewport);
delegate_noop!(LayerShellApp: ignore WlCompositor);
delegate_noop!(LayerShellApp: ignore WlRegion);

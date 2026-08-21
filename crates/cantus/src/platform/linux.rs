use crate::{
    app::{
        AppUpdater, Background, CantusApp,
        config::{Layer as ConfigLayer, LayerAnchor as ConfigLayerAnchor},
        send_update,
    },
    render::{
        PANEL_START,
        launcher::{BACKGROUND_RADIUS, LauncherKey, background_bounds},
        lyrics::EXTENSION as LYRICS_EXTENSION,
        status::{AUDIO_SPECTRUM_BANDS, BATTERY_HIDDEN},
    },
};
use freedesktop_desktop_entry::{desktop_entries, get_languages_from_env};
use isthmus::glam::vec2;
use isthmus::wgpu::{
    Surface, SurfaceTargetUnsafe,
    rwh::{RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle},
};
use microfft::real::rfft_1024;
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    env,
    ffi::c_void,
    fs::{self, File},
    io::{self, Read, Write},
    os::{fd::AsFd, unix::net::UnixDatagram as BlockingUnixDatagram},
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
    ptr::NonNull,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
use sysinfo::{Gpus, System};
use tokio::net::UnixDatagram;
use tracing::warn;
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle, WEnum, delegate_noop, event_created_child,
    globals::{GlobalListContents, registry_queue_init},
    protocol::{
        wl_callback::{self, WlCallback},
        wl_compositor::WlCompositor,
        wl_data_device::{self, WlDataDevice},
        wl_data_device_manager::WlDataDeviceManager,
        wl_data_offer::{self, WlDataOffer},
        wl_data_source::{self, WlDataSource},
        wl_keyboard::{self, KeyState, KeymapFormat, WlKeyboard},
        wl_output::{self, WlOutput},
        wl_pointer::{self, WlPointer},
        wl_region::WlRegion,
        wl_registry::{self, WlRegistry},
        wl_seat::{self, WlSeat},
        wl_surface::WlSurface,
    },
};
use wayland_protocols::ext::background_effect::v1::client::{
    ext_background_effect_manager_v1::ExtBackgroundEffectManagerV1, ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1,
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
    zwlr_layer_surface_v1::{self, Anchor as LayerAnchor, KeyboardInteractivity, ZwlrLayerSurfaceV1},
};
use xkbcommon::xkb;
use zbus::Connection as DbusConnection;

const AUDIO_SAMPLE_RATE: u32 = 48_000;
const AUDIO_WINDOW_SIZE: usize = 1024;
const AUDIO_BAND_EDGES: [f32; AUDIO_SPECTRUM_BANDS + 1] = [60.0, 120.0, 250.0, 500.0, 1_000.0, 2_000.0, 4_000.0, 12_000.0];
const LAUNCHER_SOCKET_NAME: &str = "cantus-launcher.sock";
const TEXT_MIME: &str = "text/plain;charset=utf-8";

/// One launchable desktop entry.
pub struct DesktopApp {
    pub name: String,
    pub exec: String,
    pub comment: String,
    pub icon_path: Option<PathBuf>,
    pub action: Option<(String, String)>,
    pub icon_layer: i32,
}

/// Host integration used by app and render code.
pub trait Platform {
    const STATUS_SAMPLE_INTERVAL: Duration;

    fn start_status_monitor(background: &Background, updater: AppUpdater, spectrum: Arc<[AtomicU32; AUDIO_SPECTRUM_BANDS]>);
    fn set_volume(volume: f32);
    fn run_power_action(background: &Background, action: usize);
    fn desktop_apps() -> Vec<DesktopApp>;
    fn spawn(exec: &str);
    fn open_url(url: &str);
    fn start_launcher_listener(background: &Background, updater: &AppUpdater);
    fn trigger_launcher() -> !;
}

/// The Linux desktop [`Platform`].
pub struct Linux;
pub type Current = Linux;

impl Platform for Linux {
    const STATUS_SAMPLE_INTERVAL: Duration = Duration::from_millis(500);

    fn start_status_monitor(background: &Background, updater: AppUpdater, spectrum: Arc<[AtomicU32; AUDIO_SPECTRUM_BANDS]>) {
        let volume_updater = updater.clone();
        background.run(move || monitor_playback(&spectrum));
        background.run(move || monitor_volume(&volume_updater));
        background.run(move || monitor_status(&updater));
    }

    fn set_volume(volume: f32) {
        let volume = format!("{volume:.3}");
        if let Err(error) = Command::new("wpctl").args(["set-volume", "@DEFAULT_AUDIO_SINK@", &volume]).spawn() {
            warn!(%error, "Failed to set PipeWire volume");
        }
    }

    /// Calls logind directly, which is what `systemctl poweroff` does under the hood.
    fn run_power_action(background: &Background, action: usize) {
        let method = ["PowerOff", "Reboot"][action];
        background.spawn(async move {
            let result = async {
                DbusConnection::system()
                    .await?
                    .call_method(
                        Some("org.freedesktop.login1"),
                        "/org/freedesktop/login1",
                        Some("org.freedesktop.login1.Manager"),
                        method,
                        &(false,),
                    )
                    .await?;
                Ok::<_, zbus::Error>(())
            }
            .await;
            if let Err(error) = result {
                warn!(%error, method, "Failed to run held power action");
            }
            None
        });
    }

    fn desktop_apps() -> Vec<DesktopApp> {
        let mut seen = HashSet::new();
        let locales = get_languages_from_env();
        desktop_entries(&locales)
            .into_iter()
            .filter(|entry| seen.insert(entry.id().to_owned()))
            .filter(|entry| !entry.no_display() && !entry.hidden() && !entry.terminal())
            .filter_map(|entry| {
                let action = entry
                    .actions()
                    .and_then(|actions| actions.into_iter().find(|action| !action.is_empty()))
                    .and_then(|action| entry.action_entry_localized(action, "Name", &locales).zip(entry.action_entry(action, "Exec")))
                    .map(|(name, exec)| (name.into_owned(), exec.to_owned()));
                Some(DesktopApp {
                    name: entry.name(&locales)?.into_owned(),
                    exec: entry.exec()?.to_owned(),
                    comment: entry.comment(&locales).unwrap_or_default().into_owned(),
                    icon_path: entry.icon().and_then(resolve_icon),
                    action,
                    icon_layer: -1,
                })
            })
            .collect()
    }

    /// Strips desktop-entry field codes (`%f %F %u %U %i %c %k`) and launches the command, detached.
    fn spawn(exec: &str) {
        let mut args = exec.split_whitespace().filter(|token| !token.starts_with('%'));
        let Some(program) = args.next() else { return };
        if let Err(error) = Command::new(program).args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            warn!(%error, program, "Failed to launch application");
        }
    }

    fn open_url(url: &str) {
        if let Err(error) = Command::new("xdg-open").arg(url).spawn() {
            warn!(%error, %url, "Failed to open URL");
        }
    }

    fn start_launcher_listener(background: &Background, updater: &AppUpdater) {
        let path = launcher_socket_path();
        let _ = fs::remove_file(&path);
        let updater = updater.clone();
        background.spawn(async move {
            let socket = match UnixDatagram::bind(&path) {
                Ok(socket) => socket,
                Err(error) => {
                    warn!(%error, ?path, "Failed to bind launcher toggle socket");
                    return None;
                }
            };
            let mut buffer = [0u8; 1];
            while socket.recv(&mut buffer).await.is_ok() {
                if !send_update(&updater, |app| app.launcher.toggle()) {
                    warn!("Launcher toggle update was discarded");
                    break;
                }
            }
            None
        });
    }

    fn trigger_launcher() -> ! {
        let path = launcher_socket_path();
        if let Err(error) = BlockingUnixDatagram::unbound().and_then(|socket| socket.send_to(&[0], &path)) {
            eprintln!("Failed to reach a running Cantus instance at {}: {error}", path.display());
            process::exit(1);
        }
        process::exit(0);
    }
}

fn launcher_socket_path() -> PathBuf {
    let runtime_dir = env::var_os("XDG_RUNTIME_DIR").unwrap_or_else(|| "/tmp".into());
    PathBuf::from(runtime_dir).join(LAUNCHER_SOCKET_NAME)
}

fn resolve_icon(icon: &str) -> Option<PathBuf> {
    let path = Path::new(icon);
    if path.is_absolute() {
        return path.exists().then(|| path.to_owned());
    }
    freedesktop_icons::lookup(icon).with_size(64).find()
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
        let [cpu_temperature, cpu_usage, cpu_memory] = [
            system.cpus().first().map_or(0.0, sysinfo::Cpu::temperature),
            system.global_cpu_usage() / 100.0,
            system.used_memory() as f32 / system.total_memory().max(1) as f32,
        ];
        let gpu = gpus.as_ref().and_then(gpu_sample);
        let battery_level = battery_sample(battery.as_deref());
        if !send_update(updater, move |app| {
            let status = app.status_pass();
            status.temperature_targets[0] = cpu_temperature;
            status.pill.cpu.usage.push(cpu_usage);
            status.pill.cpu.memory.push(cpu_memory);
            if let Some([temperature, usage, memory]) = gpu {
                status.temperature_targets[1] = temperature;
                status.pill.gpu.usage.push(usage);
                status.pill.gpu.memory.push(memory);
            }
            status.pill.battery_level = battery_level;
            status.pill.history_scroll = 0.0;
        }) {
            break;
        }
        thread::sleep(Linux::STATUS_SAMPLE_INTERVAL);
    }
}

fn gpu_sample(gpus: &Gpus) -> Option<[f32; 3]> {
    let device = gpus.iter().max_by_key(|gpu| gpu.total_memory().unwrap_or_default())?;
    Some([
        device.temperature().unwrap_or_default(),
        device.usage().unwrap_or_default() / 100.0,
        device.used_memory().unwrap_or_default() as f32 / device.total_memory().unwrap_or_default().max(1) as f32,
    ])
}

fn find_battery() -> Option<PathBuf> {
    fs::read_dir("/sys/class/power_supply")
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| fs::read_to_string(path.join("type")).is_ok_and(|kind| kind.trim().eq_ignore_ascii_case("battery")))
}

/// Charge level, negated while charging, or `BATTERY_HIDDEN` with no battery or idle at full.
fn battery_sample(path: Option<&Path>) -> f32 {
    let Some(path) = path else {
        return BATTERY_HIDDEN;
    };
    let read = |name: &str| fs::read_to_string(path.join(name));
    let Ok(level) = read("capacity").map(|level| level.trim().parse::<f32>()) else {
        return BATTERY_HIDDEN;
    };
    let Ok(level) = level.map(|level| level / 100.0) else {
        return BATTERY_HIDDEN;
    };
    if read("status").is_ok_and(|status| status.trim().eq_ignore_ascii_case("charging")) {
        -level.max(f32::EPSILON)
    } else if level >= 0.995 {
        BATTERY_HIDDEN
    } else {
        level
    }
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
            let mut volume = (volumes.iter().filter_map(Value::as_f64).sum::<f64>() / volumes.len().max(1) as f64).cbrt() as f32;
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

fn piped(command: &mut Command) -> io::Result<(process::Child, process::ChildStdout)> {
    let mut child = command.stdout(Stdio::piped()).stderr(Stdio::null()).spawn()?;
    let output = child.stdout.take().ok_or_else(|| io::Error::other("command stdout was not piped"))?;
    Ok((child, output))
}

fn capture_volume(updater: &AppUpdater) -> io::Result<bool> {
    let (mut child, output) = piped(Command::new("pw-dump").args(["--monitor", "--no-colors", "--indent", "0"]))?;
    let mut state = PipeWireState::default();
    for batch in serde_json::Deserializer::from_reader(output).into_iter::<Vec<Value>>() {
        for object in batch.map_err(io::Error::other)? {
            if let Some(volume) = state.update(&object)
                && !send_update(updater, move |app| app.status_pass().pill.volume = volume)
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
    let (mut child, mut output) = piped(Command::new("pw-record").args([
        "--properties",
        "stream.capture.sink=true",
        "--rate",
        &AUDIO_SAMPLE_RATE.to_string(),
        "--channels",
        "1",
        "--format",
        "f32",
        "--raw",
        "-",
    ]))?;
    let mut window = [0.0; AUDIO_WINDOW_SIZE];
    loop {
        match output.read_exact(bytemuck::cast_slice_mut(&mut window)) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(error) => return Err(error),
        }
        let spectrum = rfft_1024(&mut window);
        for (band, level) in levels.iter().enumerate() {
            let bin = |frequency: f32| (frequency * AUDIO_WINDOW_SIZE as f32 / AUDIO_SAMPLE_RATE as f32).ceil() as usize;
            let bins = &spectrum[bin(AUDIO_BAND_EDGES[band])..bin(AUDIO_BAND_EDGES[band + 1])];
            let rms = (bins.iter().map(microfft::Complex32::norm_sqr).sum::<f32>() / bins.len() as f32 / AUDIO_WINDOW_SIZE as f32).sqrt();
            let value = ((20.0 * rms.log10() + 30.0) / 30.0).clamp(0.0, 1.0);
            level.store(value.to_bits(), Ordering::Relaxed);
        }
    }
    child.wait()?;
    Ok(())
}

/// Runs the Wayland application event loop.
///
/// # Panics
///
/// Panics when required Wayland globals or rendering resources cannot be initialized.
pub fn run() {
    let connection = Connection::connect_to_env().expect("Failed to connect to Wayland display");
    let (globals, mut event_queue) = registry_queue_init::<LayerShellApp>(&connection).expect("Failed to read Wayland registry");
    let qhandle = event_queue.handle();
    let compositor: WlCompositor = globals.bind(&qhandle, 1..=7, ()).expect("Missing wl_compositor");
    let layer_shell: ZwlrLayerShellV1 = globals.bind(&qhandle, 4..=4, ()).expect("Missing zwlr_layer_shell_v1");
    let seat: WlSeat = globals.bind(&qhandle, 1..=7, ()).expect("Missing wl_seat");

    let mut app = LayerShellApp {
        compositor: Some(compositor.clone()),
        layer_shell: Some(layer_shell.clone()),
        repeat_delay: Duration::from_millis(600),
        repeat_interval: Duration::from_millis(40),
        clipboard: globals.bind::<WlDataDeviceManager, _, _>(&qhandle, 1..=3, ()).ok().map(|manager| {
            let device = manager.get_data_device(&seat, &qhandle, ());
            (manager, device)
        }),
        ..LayerShellApp::default()
    };

    // Every output is bound so its name arrives; the configured monitor replaces the first one.
    let registry = globals.registry();
    for global in globals.contents().clone_list() {
        if global.interface == "wl_output" {
            let version = global.version.min(4);
            let output = registry.bind::<WlOutput, (), LayerShellApp>(global.name, version, &qhandle, ());
            app.output.get_or_insert(output);
        }
    }
    event_queue.roundtrip(&mut app).expect("Failed to fetch output details");

    let wl_surface = compositor.create_surface(&qhandle, ());
    let handle = |pointer: Option<NonNull<c_void>>| pointer.expect("Failed to get Wayland pointer");
    app.display_handle = Some(RawDisplayHandle::Wayland(WaylandDisplayHandle::new(handle(NonNull::new(
        connection.backend().display_ptr().cast(),
    )))));
    let output = app.output.take().expect("No Wayland outputs found");

    app.wl_surface = Some(wl_surface);
    let surface = app.wl_surface.as_ref().unwrap();
    // Fractional scaling needs both halves: the viewport scales the buffer the scale factor sizes.
    if let (Ok(viewporter), Ok(fractional)) = (
        globals.bind::<WpViewporter, _, _>(&qhandle, 1..=1, ()),
        globals.bind::<WpFractionalScaleManagerV1, _, _>(&qhandle, 1..=1, ()),
    ) {
        app.viewport = Some(viewporter.get_viewport(surface, &qhandle, ()));
        app.fractional = Some(fractional.get_fractional_scale(surface, &qhandle, ()));
        app.viewporter = Some(viewporter);
        app.fractional_manager = Some(fractional);
    }
    if let Ok(manager) = globals.bind::<ExtBackgroundEffectManagerV1, _, _>(&qhandle, 1..=1, ()) {
        app.background_manager = Some(manager.clone());
        app.background_effect = Some(manager.get_background_effect(surface, &qhandle, ()));
    }

    let config = &app.cantus.config;
    let layer_surface = layer_shell.get_layer_surface(
        surface,
        Some(&output),
        match config.layer {
            ConfigLayer::Background => LayerStyle::Background,
            ConfigLayer::Bottom => LayerStyle::Bottom,
            ConfigLayer::Top => LayerStyle::Top,
            ConfigLayer::Overlay => LayerStyle::Overlay,
        },
        "cantus".into(),
        &qhandle,
        (),
    );
    layer_surface.set_anchor(match config.layer_anchor {
        ConfigLayerAnchor::Top => LayerAnchor::Top | LayerAnchor::Left | LayerAnchor::Right,
        ConfigLayerAnchor::Bottom => LayerAnchor::Bottom | LayerAnchor::Left | LayerAnchor::Right,
    });
    layer_surface.set_exclusive_zone((PANEL_START + config.height + f32::from(config.lyrics_enabled) * LYRICS_EXTENSION) as i32);
    resize_layer_surface(&layer_surface, &app.cantus);
    app.layer_surface = Some(layer_surface);

    app.pending_surface = Some(app.create_render_surface(surface));
    surface.commit();
    connection.flush().expect("Failed to flush initial commit");

    while !app.should_exit {
        event_queue.blocking_dispatch(&mut app).expect("Wayland dispatch error");
    }
}

#[derive(Default)]
struct LayerShellApp {
    cantus: CantusApp,

    should_exit: bool,

    compositor: Option<WlCompositor>,
    layer_shell: Option<ZwlrLayerShellV1>,
    pointer: Option<WlPointer>,
    keyboard: Option<WlKeyboard>,
    xkb_state: Option<xkb::State>,
    /// Repeat timing advertised by the compositor; `run` seeds the usual X11 rate.
    repeat_delay: Duration,
    repeat_interval: Duration,
    /// The held key waiting to repeat and when it next fires, pumped each frame.
    repeat: Option<(xkb::Keycode, Instant)>,
    /// Latest keyboard serial, which the compositor requires to claim the selection.
    key_serial: u32,
    clipboard: Option<(WlDataDeviceManager, WlDataDevice)>,
    /// Text this instance put on the clipboard, served on demand until another client claims it.
    copied: Arc<str>,
    /// The selection offer to read on paste, kept only while it advertises text.
    selection: Option<WlDataOffer>,
    offer_is_text: bool,
    output: Option<WlOutput>,
    pending_surface: Option<Surface<'static>>,
    display_handle: Option<RawDisplayHandle>,
    wl_surface: Option<WlSurface>,
    launcher_wl_surface: Option<WlSurface>,
    layer_surface: Option<ZwlrLayerSurfaceV1>,
    launcher_layer_surface: Option<ZwlrLayerSurfaceV1>,
    viewporter: Option<WpViewporter>,
    fractional_manager: Option<WpFractionalScaleManagerV1>,
    background_manager: Option<ExtBackgroundEffectManagerV1>,
    viewport: Option<WpViewport>,
    fractional: Option<WpFractionalScaleV1>,
    background_effect: Option<ExtBackgroundEffectSurfaceV1>,
    launcher_viewport: Option<WpViewport>,
    launcher_fractional: Option<WpFractionalScaleV1>,
    launcher_background_effect: Option<ExtBackgroundEffectSurfaceV1>,
    launcher_configured: bool,
    frame_callback: Option<WlCallback>,
}

macro_rules! destroy_proxies {
    ($state:expr, $($field:ident),+ $(,)?) => {
        $(if let Some(proxy) = $state.$field.take() {
            proxy.destroy();
        })+
    };
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

/// Resizes the bar's layer surface to fit the launcher panel and sets its keyboard focus.
fn resize_layer_surface(layer_surface: &ZwlrLayerSurfaceV1, cantus: &CantusApp) {
    layer_surface.set_size(0, cantus.logical_surface_size().1 as u32);
    layer_surface.set_keyboard_interactivity(if cantus.launcher.open {
        KeyboardInteractivity::Exclusive
    } else {
        KeyboardInteractivity::None
    });
}

impl LayerShellApp {
    /// Re-fires the held key for as long as it stays down, once the initial delay has passed.
    fn pump_key_repeat(&mut self) {
        let now = Instant::now();
        while let Some((keycode, next)) = self.repeat
            && next <= now
            && self.cantus.launcher.open
        {
            self.repeat = Some((keycode, next + self.repeat_interval));
            handle_launcher_key(self, keycode);
        }
    }

    /// Claims the clipboard selection, serving `text` to whoever pastes next.
    fn set_clipboard(&mut self, text: &str, qhandle: &QueueHandle<Self>) {
        let Some((manager, device)) = &self.clipboard else {
            return;
        };
        let source = manager.create_data_source(qhandle, ());
        source.offer(TEXT_MIME.to_owned());
        device.set_selection(Some(&source), self.key_serial);
        self.copied = text.into();
    }

    /// Reads the selection as text, blocking on the pipe the owning client writes into.
    fn paste(&self) -> Option<String> {
        let offer = self.selection.as_ref()?;
        let (mut reader, writer) = io::pipe().ok()?;
        offer.receive(TEXT_MIME.to_owned(), writer.as_fd());
        drop(writer);
        Connection::from_backend(offer.backend().upgrade()?).flush().ok()?;
        let mut text = String::new();
        reader.read_to_string(&mut text).ok()?;
        Some(text)
    }

    fn create_render_surface(&self, wl_surface: &WlSurface) -> Surface<'static> {
        let display = self.display_handle.expect("missing Wayland display handle");
        let window = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(wl_surface.id().as_ptr().cast()).expect("missing Wayland surface pointer"),
        ));
        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(display),
            raw_window_handle: window,
        };
        unsafe { self.cantus.render.instance.create_surface_unsafe(target) }.expect("Failed to create surface")
    }

    const fn active_surface(&self) -> &WlSurface {
        if let Some(surface) = self.launcher_wl_surface.as_ref() {
            surface
        } else {
            self.wl_surface.as_ref().unwrap()
        }
    }

    const fn active_viewport(&self) -> Option<&WpViewport> {
        if self.launcher_wl_surface.is_some() {
            self.launcher_viewport.as_ref()
        } else {
            self.viewport.as_ref()
        }
    }

    const fn active_background_effect(&self) -> Option<&ExtBackgroundEffectSurfaceV1> {
        if self.launcher_wl_surface.is_some() {
            self.launcher_background_effect.as_ref()
        } else {
            self.background_effect.as_ref()
        }
    }

    fn sync_launcher_surface(&mut self, qhandle: &QueueHandle<Self>) {
        let open = self.cantus.launcher.open;
        if open == self.launcher_layer_surface.is_some() {
            return;
        }
        self.repeat = None;
        if open {
            self.launcher_configured = false;
            self.cantus.render.launcher_width = None;
            self.cantus.render.launcher_height = None;
            let surface = self.compositor.as_ref().unwrap().create_surface(qhandle, ());
            let layer = self
                .layer_shell
                .as_ref()
                .unwrap()
                .get_layer_surface(&surface, None, LayerStyle::Overlay, "cantus-launcher".into(), qhandle, ());
            layer.set_anchor(LayerAnchor::Top | LayerAnchor::Bottom | LayerAnchor::Left | LayerAnchor::Right);
            layer.set_size(0, 0);
            layer.set_exclusive_zone(0);
            layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
            if let Some(manager) = &self.viewporter {
                self.launcher_viewport = Some(manager.get_viewport(&surface, qhandle, ()));
            }
            if let Some(manager) = &self.fractional_manager {
                self.launcher_fractional = Some(manager.get_fractional_scale(&surface, qhandle, ()));
            }
            if let Some(manager) = &self.background_manager {
                self.launcher_background_effect = Some(manager.get_background_effect(&surface, qhandle, ()));
            }
            self.launcher_layer_surface = Some(layer);
            self.launcher_wl_surface = Some(surface);
            let surface = self.launcher_wl_surface.as_ref().unwrap();
            self.pending_surface = Some(self.create_render_surface(surface));
            surface.commit();
        } else {
            drop(self.frame_callback.take());
            destroy_proxies!(
                self,
                launcher_layer_surface,
                launcher_viewport,
                launcher_fractional,
                launcher_background_effect,
                launcher_wl_surface
            );
            let surface = self.wl_surface.as_ref().unwrap();
            self.pending_surface = Some(self.create_render_surface(surface));
            surface.commit();
        }
    }

    fn try_render_frame(&mut self, qhandle: &QueueHandle<Self>) {
        self.pump_key_repeat();
        self.cantus.apply_pending_updates();
        self.sync_launcher_surface(qhandle);
        if self.launcher_wl_surface.is_some() && !self.launcher_configured {
            return;
        }

        // Initialize the program before draining updates so startup jobs cannot race surface configuration.
        if self.cantus.render.program.is_none()
            && let Some(surface) = self.pending_surface.take()
        {
            let (width, height) = self.cantus.buffer_size();
            if width > 0 && height > 0 {
                self.cantus.initialize_gpu(surface, width, height);
            } else {
                self.pending_surface = Some(surface);
            }
        } else if let Some(surface) = self.pending_surface.take() {
            self.cantus.replace_render_surface(surface);
        }
        self.update_scale_and_viewport();
        self.update_blur_region(qhandle);

        let (buffer_width, buffer_height) = self.cantus.buffer_size();
        if buffer_width > 0
            && buffer_height > 0
            && let Some(program) = &mut self.cantus.render.program
        {
            program.resize(buffer_width, buffer_height);
        }

        if self.cantus.render() {
            let surface = self.create_render_surface(self.active_surface());
            self.cantus.replace_render_surface(surface);
        }
        if let Some(text) = self.cantus.launcher.pending_copy.take() {
            self.set_clipboard(&text, qhandle);
        }
        self.update_input_region(qhandle);
        let surface = self.active_surface().clone();
        if self.frame_callback.is_none() {
            self.frame_callback = Some(surface.frame(qhandle, ()));
        }
        surface.commit();
    }

    fn update_scale_and_viewport(&self) {
        let (logical_width, logical_height) = self.cantus.logical_surface_size();
        let viewport = self.active_viewport();
        self.active_surface()
            .set_buffer_scale(viewport.map_or_else(|| self.cantus.render.scale.ceil() as i32, |_| 1));
        if let Some(viewport) = viewport {
            // Leave the source unset so it always refers to the full attached buffer.
            viewport.set_destination(logical_width as i32, logical_height as i32);
        }
    }

    fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        let wl_surface = self.active_surface().clone();
        let compositor = self.compositor.as_ref().unwrap();
        let region = compositor.create_region(qhandle, ());
        for rect in self.cantus.interaction.take_regions() {
            let [x, y, width, height] = [rect.x0, rect.y0, rect.x1 - rect.x0, rect.y1 - rect.y0].map(|value| value.round() as i32);
            region.add(x, y, width, height);
        }
        wl_surface.set_input_region(Some(&region));
        region.destroy();
    }

    fn update_blur_region(&self, qhandle: &QueueHandle<Self>) {
        let Some(effect) = self.active_background_effect() else {
            return;
        };
        if !self.cantus.launcher.open {
            effect.set_blur_region(None);
            return;
        }

        let compositor = self.compositor.as_ref().unwrap();
        let region = compositor.create_region(qhandle, ());
        let (width, height) = self.cantus.logical_surface_size();
        let (origin, size) = background_bounds(vec2(width, height), self.cantus.config.height);
        // Keep the integer input region one pixel inside the shader's antialiased edge.
        let x = origin.x.ceil() as i32 + 1;
        let y = origin.y.ceil() as i32 + 1;
        let width = (origin.x + size.x).floor() as i32 - 1 - x;
        let height = (origin.y + size.y).floor() as i32 - 1 - y;
        let radius = (BACKGROUND_RADIUS - 1).min(width / 2).min(height / 2);
        region.add(x, y + radius, width, height - radius * 2);
        for row in 0..radius {
            let dy = radius as f32 - row as f32 - 0.5;
            let dx = ((radius * radius) as f32 - dy * dy).sqrt();
            let inset = radius - (dx + 0.5).round() as i32;
            region.add(x + inset, y + row, width - inset * 2, 1);
            region.add(x + inset, y + height - row - 1, width - inset * 2, 1);
        }
        effect.set_blur_region(Some(&region));
        region.destroy();
    }
}

dispatch!(ZwlrLayerSurfaceV1, |state, proxy, event, qhandle| {
    match event {
        zwlr_layer_surface_v1::Event::Configure { serial, width, height } => {
            proxy.ack_configure(serial);
            let is_launcher = state.launcher_layer_surface.as_ref().is_some_and(|launcher| launcher.id() == proxy.id());
            if width > 0 {
                if is_launcher {
                    state.cantus.render.launcher_width = Some(width as f32);
                } else {
                    state.cantus.render.surface_width = Some(width as f32);
                }
            }
            if height > 0 && is_launcher {
                state.cantus.render.launcher_height = Some(height as f32);
            }
            if is_launcher {
                state.launcher_configured = true;
            }
            state.update_scale_and_viewport();
            state.update_blur_region(qhandle);
            state.try_render_frame(qhandle);
        }
        zwlr_layer_surface_v1::Event::Closed => {
            if state.launcher_layer_surface.as_ref().is_some_and(|launcher| launcher.id() == proxy.id()) {
                state.cantus.launcher.open = false;
                state.sync_launcher_surface(qhandle);
            } else {
                state.should_exit = true;
            }
        }
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
    match event {
        wl_output::Event::Mode {
            flags: WEnum::Value(flags),
            height,
            ..
        } if flags.contains(wl_output::Mode::Current) => {
            state.cantus.render.output_height = Some(height as f32);
        }
        wl_output::Event::Name { name } | wl_output::Event::Description { description: name }
            if state.cantus.config.monitor.as_ref().is_none_or(|target| name.contains(target)) =>
        {
            state.output = Some(proxy.clone());
        }
        _ => {}
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
        if caps.contains(wl_seat::Capability::Keyboard) {
            if state.keyboard.is_none() {
                state.keyboard = Some(proxy.get_keyboard(qhandle, ()));
            }
        } else if let Some(keyboard) = state.keyboard.take() {
            keyboard.release();
        }
    }
});

impl Dispatch<WlDataDevice, ()> for LayerShellApp {
    fn event(state: &mut Self, _proxy: &WlDataDevice, event: wl_data_device::Event, _data: &(), _conn: &Connection, _qhandle: &QueueHandle<Self>) {
        match event {
            wl_data_device::Event::DataOffer { .. } => state.offer_is_text = false,
            wl_data_device::Event::Selection { id } => {
                // Offers are per-selection objects; whatever we held before is ours to release.
                if let Some(stale) = state.selection.take() {
                    stale.destroy();
                }
                state.selection = id.filter(|_| state.offer_is_text);
            }
            _ => {}
        }
    }

    event_created_child!(Self, WlDataDevice, [
        wl_data_device::EVT_DATA_OFFER_OPCODE => (WlDataOffer, ()),
    ]);
}

dispatch!(WlDataOffer, |state, _proxy, event, _qhandle| {
    if let wl_data_offer::Event::Offer { mime_type } = event {
        state.offer_is_text |= mime_type == TEXT_MIME;
    }
});

dispatch!(WlDataSource, |state, proxy, event, _qhandle| {
    match event {
        // Serving `copied` rather than a per-source copy lets a stale source hand out the latest.
        wl_data_source::Event::Send { fd, .. } => {
            if let Err(error) = File::from(fd).write_all(state.copied.as_bytes()) {
                warn!(%error, "Failed to serve the clipboard selection");
            }
        }
        wl_data_source::Event::Cancelled => proxy.destroy(),
        _ => {}
    }
});

dispatch!(WlKeyboard, |state, _proxy, event, _qhandle| {
    match event {
        wl_keyboard::Event::Keymap {
            format: WEnum::Value(KeymapFormat::XkbV1),
            fd,
            size,
        } => {
            let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
            let keymap = unsafe { xkb::Keymap::new_from_fd(&context, fd, size as usize, xkb::KEYMAP_FORMAT_TEXT_V1, xkb::KEYMAP_COMPILE_NO_FLAGS) };
            state.xkb_state = keymap.ok().flatten().map(|keymap| xkb::State::new(&keymap));
        }
        wl_keyboard::Event::Modifiers {
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
            ..
        } => {
            if let Some(xkb_state) = &mut state.xkb_state {
                xkb_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
            }
        }
        wl_keyboard::Event::RepeatInfo { rate, delay } if rate > 0 => {
            state.repeat_delay = Duration::from_millis(delay.max(0) as u64);
            state.repeat_interval = Duration::from_micros(1_000_000 / rate as u64);
        }
        wl_keyboard::Event::Leave { .. } => state.repeat = None,
        wl_keyboard::Event::Key {
            serial,
            key,
            state: WEnum::Value(key_state),
            ..
        } => {
            let keycode = xkb::Keycode::new(key + 8);
            state.key_serial = serial;
            if key_state == KeyState::Pressed {
                // Modifiers are marked as non-repeating by the keymap, so they never latch here.
                let repeats = state.xkb_state.as_ref().is_some_and(|xkb_state| xkb_state.get_keymap().key_repeats(keycode));
                state.repeat = repeats.then(|| (keycode, Instant::now() + state.repeat_delay));
                handle_launcher_key(state, keycode);
            } else if state.repeat.is_some_and(|(held, _)| held == keycode) {
                state.repeat = None;
            }
            if let Some(xkb_state) = &mut state.xkb_state {
                let direction = if key_state == KeyState::Released {
                    xkb::KeyDirection::Up
                } else {
                    xkb::KeyDirection::Down
                };
                xkb_state.update_key(keycode, direction);
            }
        }
        _ => {}
    }
});

/// Applies one key press to the launcher's search field and selection while it is open.
fn handle_launcher_key(state: &mut LayerShellApp, keycode: xkb::Keycode) {
    if !state.cantus.launcher.open {
        return;
    }
    let Some(xkb_state) = &state.xkb_state else {
        return;
    };
    let sym = xkb_state.key_get_one_sym(keycode);
    if !state.launcher_configured && matches!(sym.raw(), xkb::keysyms::KEY_Return | xkb::keysyms::KEY_KP_Enter) {
        return;
    }
    let shift = xkb_state.mod_name_is_active(xkb::MOD_NAME_SHIFT, xkb::STATE_MODS_EFFECTIVE);
    let control = xkb_state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE);
    let character = sym.key_char();
    // Held control turns `key_char` into a control code, so the shortcuts read the keysym instead.
    let letter = char::from_u32(sym.raw()).filter(char::is_ascii_alphabetic).map(|letter| letter.to_ascii_lowercase());

    if control && letter == Some('v') {
        if let Some(pasted) = state.paste() {
            let pasted = pasted.replace(['\n', '\r'], " ");
            state.cantus.launcher.edit(|field| field.insert(&pasted));
        }
        return;
    }
    let key = match sym.raw() {
        xkb::keysyms::KEY_Escape => Some(LauncherKey::Escape),
        xkb::keysyms::KEY_Return | xkb::keysyms::KEY_KP_Enter => Some(LauncherKey::Activate),
        xkb::keysyms::KEY_Up => Some(LauncherKey::Up),
        xkb::keysyms::KEY_Down => Some(LauncherKey::Down),
        xkb::keysyms::KEY_BackSpace => Some(LauncherKey::Backspace),
        xkb::keysyms::KEY_Delete => Some(LauncherKey::Delete),
        xkb::keysyms::KEY_Left => Some(LauncherKey::Left),
        xkb::keysyms::KEY_Right => Some(LauncherKey::Right),
        xkb::keysyms::KEY_Home => Some(LauncherKey::Home),
        xkb::keysyms::KEY_End => Some(LauncherKey::End),
        _ if control && letter == Some('a') => Some(LauncherKey::SelectAll),
        _ if control && letter == Some('c') => Some(LauncherKey::Copy),
        _ if control && letter == Some('x') => Some(LauncherKey::Cut),
        _ => None,
    };
    if let Some(key) = key {
        state.cantus.launcher.key(key, shift);
    } else if let Some(typed) = character.filter(|typed| !typed.is_control() && !control) {
        state.cantus.launcher.edit(|field| field.insert(typed.encode_utf8(&mut [0u8; 4])));
    }
}

dispatch!(WlPointer, |state, _proxy, event, _qhandle| {
    let cantus = &mut state.cantus;
    let interaction = &mut cantus.interaction;

    let surface_id = state.wl_surface.as_ref().map(Proxy::id);
    match event {
        wl_pointer::Event::Enter {
            surface, surface_x, surface_y, ..
        } if surface_id == Some(surface.id()) => {
            interaction.motion(vec2(surface_x as f32, surface_y as f32));
            interaction.hover();
        }
        wl_pointer::Event::Motion { surface_x, surface_y, .. } => {
            let position = vec2(surface_x as f32, surface_y as f32);
            interaction.motion(position);
        }
        wl_pointer::Event::Leave { .. } => {
            interaction.leave();
        }
        wl_pointer::Event::Button { button, state: button_state, .. } => match (button, button_state) {
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
    fn event(_: &mut Self, _: &WlRegistry, _: wl_registry::Event, _: &GlobalListContents, _: &Connection, _: &QueueHandle<Self>) {}
}

delegate_noop!(LayerShellApp: ignore WlSurface);
delegate_noop!(LayerShellApp: ignore ZwlrLayerShellV1);
delegate_noop!(LayerShellApp: ignore WpFractionalScaleManagerV1);
delegate_noop!(LayerShellApp: ignore WpViewporter);
delegate_noop!(LayerShellApp: ignore WpViewport);
delegate_noop!(LayerShellApp: ignore WlCompositor);
delegate_noop!(LayerShellApp: ignore WlRegion);
delegate_noop!(LayerShellApp: ignore WlDataDeviceManager);
delegate_noop!(LayerShellApp: ignore ExtBackgroundEffectManagerV1);
delegate_noop!(LayerShellApp: ignore ExtBackgroundEffectSurfaceV1);

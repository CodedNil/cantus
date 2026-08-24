use crate::interaction::InputEvent;
use crate::{
    app::{
        AppUpdater, Background, CantusApp,
        config::{Layer as ConfigLayer, LayerAnchor as ConfigLayerAnchor},
        send_update,
    },
    render::{
        PANEL_START,
        launcher::{BACKGROUND_RADIUS, LauncherKey},
        lyrics::EXTENSION as LYRICS_EXTENSION,
        status::{AUDIO_SPECTRUM_BANDS, AudioMonitor, ProcessorSample, SystemSample},
        tempestas::EXTENSION as WEATHER_EXTENSION,
    },
};
use freedesktop_desktop_entry::{desktop_entries, get_languages_from_env};
use isthmus::glam::vec2;
use isthmus::{FloatExt, Image, SurfaceHandle};
use microfft::real::rfft_1024;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle, WindowHandle,
};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    env,
    ffi::c_void,
    fs::{self, File},
    io::{self, Read, Write},
    os::{
        fd::AsFd,
        unix::{net::UnixDatagram as BlockingUnixDatagram, process::CommandExt},
    },
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
    ptr::NonNull,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
        mpsc::Sender,
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

const PANEL_OVERFLOW: f32 = 16.0;

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
    pub icon: Option<Image>,
}

/// Host integration used by app and render code.
pub trait Platform {
    const STATUS_SAMPLE_INTERVAL: Duration;

    fn start_status_monitor(background: &Background, updates: Sender<SystemSample>, audio: Arc<AudioMonitor>);
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

impl Platform for Linux {
    const STATUS_SAMPLE_INTERVAL: Duration = Duration::from_millis(500);

    fn start_status_monitor(background: &Background, updates: Sender<SystemSample>, audio: Arc<AudioMonitor>) {
        let volume = Arc::clone(&audio);
        background.spawn_blocking(move || monitor_playback(&audio.spectrum));
        background.spawn_blocking(move || monitor_volume(&volume.volume));
        background.spawn_blocking(move || monitor_status(&updates));
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
        background.spawn_update(async move {
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
                    icon: None,
                })
            })
            .collect()
    }

    /// Strips desktop-entry field codes (`%f %F %u %U %i %c %k`) and launches the command, detached.
    fn spawn(exec: &str) {
        let mut fields = exec.split_whitespace().filter(|token| !token.starts_with('%'));
        let Some(program) = fields.next() else { return };
        let args = fields.collect::<Vec<_>>();
        if Command::new("systemd-run")
            .args(["--user", "--collect", "--quiet", "--"])
            .arg(program)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .is_ok()
        {
            return;
        }
        if let Err(error) = Command::new(program)
            .args(args)
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
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
        background.spawn_update(async move {
            let socket = match UnixDatagram::bind(&path) {
                Ok(socket) => socket,
                Err(error) => {
                    warn!(%error, ?path, "Failed to bind launcher toggle socket");
                    return None;
                }
            };
            let mut buffer = [0u8; 1];
            while socket.recv(&mut buffer).await.is_ok() {
                if !send_update(&updater, |app| app.ui.launcher.toggle()) {
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

fn monitor_status(updates: &Sender<SystemSample>) {
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
        let cpu = ProcessorSample {
            temperature: system.cpus().first().map_or(0.0, sysinfo::Cpu::temperature),
            usage: system.global_cpu_usage() / 100.0,
            memory: system.used_memory() as f32 / system.total_memory().max(1) as f32,
        };
        let gpu = gpus.as_ref().and_then(gpu_sample);
        let battery_level = battery_sample(battery.as_deref());
        if updates.send(SystemSample { cpu, gpu, battery_level }).is_err() {
            break;
        }
        thread::sleep(Linux::STATUS_SAMPLE_INTERVAL);
    }
}

fn gpu_sample(gpus: &Gpus) -> Option<ProcessorSample> {
    let device = gpus.iter().max_by_key(|gpu| gpu.total_memory().unwrap_or_default())?;
    Some(ProcessorSample {
        temperature: device.temperature().unwrap_or_default(),
        usage: device.usage().unwrap_or_default() / 100.0,
        memory: device.used_memory().unwrap_or_default() as f32 / device.total_memory().unwrap_or_default().max(1) as f32,
    })
}

fn find_battery() -> Option<PathBuf> {
    fs::read_dir("/sys/class/power_supply")
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| fs::read_to_string(path.join("type")).is_ok_and(|kind| kind.trim().eq_ignore_ascii_case("battery")))
}

/// Charge level, negated while charging, or absent with no battery or idle at full.
fn battery_sample(path: Option<&Path>) -> Option<f32> {
    let path = path?;
    let read = |name: &str| fs::read_to_string(path.join(name));
    let level = read("capacity").ok()?.trim().parse::<f32>().ok()? / 100.0;
    if read("status").is_ok_and(|status| status.trim().eq_ignore_ascii_case("charging")) {
        Some(-level.max(f32::EPSILON))
    } else if level >= 0.995 {
        None
    } else {
        Some(level)
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

fn monitor_volume(volume: &AtomicU32) {
    loop {
        if let Err(error) = capture_volume(volume) {
            warn!(%error, "PipeWire volume monitor stopped");
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn piped(command: &mut Command) -> io::Result<(process::Child, process::ChildStdout)> {
    let mut child = command.stdout(Stdio::piped()).stderr(Stdio::null()).spawn()?;
    let output = child.stdout.take().ok_or_else(|| io::Error::other("command stdout was not piped"))?;
    Ok((child, output))
}

fn capture_volume(volume: &AtomicU32) -> io::Result<()> {
    let (mut child, output) = piped(Command::new("pw-dump").args(["--monitor", "--no-colors", "--indent", "0"]))?;
    let mut state = PipeWireState::default();
    for batch in serde_json::Deserializer::from_reader(output).into_iter::<Vec<Value>>() {
        for object in batch.map_err(io::Error::other)? {
            if let Some(level) = state.update(&object) {
                volume.store(level.to_bits(), Ordering::Relaxed);
            }
        }
    }
    child.wait()?;
    Ok(())
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
            let value = ((20.0 * rms.log10() + 30.0) / 30.0).saturate();
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
    app.display_handle = Some(handle(NonNull::new(connection.backend().display_ptr().cast())));
    let output = app.output.take().expect("No Wayland outputs found");
    // wl_pointer exposes a surface, not its output. Keep the configured output as the
    // best protocol-level target for the launcher rather than letting the compositor choose.
    app.output = Some(output.clone());

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
    resize_layer_surface(&layer_surface, &app);
    app.layer_surface = Some(layer_surface);

    app.pending_bar_surface = Some(app.create_render_surface(surface));
    surface.commit();
    connection.flush().expect("Failed to flush initial commit");

    while !app.should_exit {
        event_queue.blocking_dispatch(&mut app).expect("Wayland dispatch error");
    }
}

#[derive(Clone, Copy)]
struct NativeSurface {
    display: NonNull<c_void>,
    window: NonNull<c_void>,
}

impl HasDisplayHandle for NativeSurface {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(self.display));
        Ok(unsafe { DisplayHandle::borrow_raw(handle) })
    }
}

impl HasWindowHandle for NativeSurface {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(self.window));
        Ok(unsafe { WindowHandle::borrow_raw(handle) })
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
    pending_bar_surface: Option<NativeSurface>,
    pending_launcher_surface: Option<NativeSurface>,
    launcher_surface: Option<SurfaceHandle>,
    scale: Option<f32>,
    surface_width: Option<f32>,
    launcher_width: Option<f32>,
    launcher_height: Option<f32>,
    output_height: Option<f32>,
    display_handle: Option<NonNull<c_void>>,
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
    bar_frame_callback: Option<WlCallback>,
    launcher_frame_callback: Option<WlCallback>,
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

/// Resizes the bar surface and sets keyboard focus for the launcher.
fn resize_layer_surface(layer_surface: &ZwlrLayerSurfaceV1, app: &LayerShellApp) {
    layer_surface.set_size(0, app.bar_surface_size().1 as u32);
    layer_surface.set_keyboard_interactivity(if app.cantus.ui.launcher.open {
        KeyboardInteractivity::Exclusive
    } else {
        KeyboardInteractivity::None
    });
}

impl LayerShellApp {
    fn scale(&self) -> f32 {
        self.scale.unwrap_or(1.0)
    }

    fn bar_surface_size(&self) -> (f32, f32) {
        let extension = if self.cantus.config.tempestas_enabled {
            WEATHER_EXTENSION
        } else if self.cantus.config.lyrics_enabled {
            LYRICS_EXTENSION
        } else {
            0.0
        } + PANEL_OVERFLOW;
        (self.surface_width.unwrap_or(1920.0), self.cantus.config.height + PANEL_START + extension)
    }

    fn launcher_surface_size(&self) -> (f32, f32) {
        (
            self.launcher_width.or(self.surface_width).unwrap_or(1920.0),
            self.launcher_height.or_else(|| self.output_height.map(|height| height / self.scale())).unwrap_or(1080.0),
        )
    }

    fn buffer_size(&self, logical: (f32, f32)) -> (u32, u32) {
        let scale = self.scale();
        ((logical.0 * scale).round() as u32, (logical.1 * scale).round() as u32)
    }

    /// Re-fires the held key for as long as it stays down, once the initial delay has passed.
    fn pump_key_repeat(&mut self) {
        let now = Instant::now();
        while let Some((keycode, next)) = self.repeat
            && next <= now
            && self.cantus.ui.launcher.open
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

    fn create_render_surface(&self, wl_surface: &WlSurface) -> NativeSurface {
        let display = self.display_handle.expect("missing Wayland display handle");
        let window = NonNull::new(wl_surface.id().as_ptr().cast()).expect("missing Wayland surface pointer");
        NativeSurface { display, window }
    }

    const fn active_surface(&self) -> &WlSurface {
        if let Some(surface) = self.launcher_wl_surface.as_ref() {
            surface
        } else {
            self.wl_surface.as_ref().unwrap()
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
        let open = self.cantus.ui.launcher.open;
        self.cantus.interaction.set_launcher_active(open);
        if open == self.launcher_layer_surface.is_some() {
            return;
        }
        self.repeat = None;
        if open {
            drop(self.bar_frame_callback.take());
            self.launcher_configured = false;
            self.launcher_width = None;
            self.launcher_height = None;
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
            self.pending_launcher_surface = Some(self.create_render_surface(surface));
            surface.commit();
        } else {
            drop(self.launcher_frame_callback.take());
            if let Some(gpu) = self.cantus.gpu.as_mut()
                && let Some(surface) = self.launcher_surface.take()
            {
                gpu.remove_surface(surface);
            }
            destroy_proxies!(
                self,
                launcher_layer_surface,
                launcher_viewport,
                launcher_fractional,
                launcher_background_effect,
                launcher_wl_surface
            );
            let surface = self.wl_surface.as_ref().unwrap();
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
        if self.cantus.gpu.is_none()
            && let Some(surface) = self.pending_bar_surface.take()
        {
            let (width, height) = self.buffer_size(self.bar_surface_size());
            if width > 0 && height > 0 {
                self.cantus.initialize_renderer(&surface, width, height);
            } else {
                self.pending_bar_surface = Some(surface);
            }
        }
        if let Some(surface) = self.pending_launcher_surface.take() {
            let (width, height) = self.buffer_size(self.launcher_surface_size());
            if let Some(gpu) = &mut self.cantus.gpu {
                self.launcher_surface = Some(gpu.add_surface(&surface, (width, height).into()).expect("launcher surface is incompatible"));
            }
        }
        self.update_scale_and_viewport();
        self.update_blur_region(qhandle);

        let bar_size = self.bar_surface_size();
        let launcher_logical_size = self.launcher_surface_size();
        let (buffer_width, buffer_height) = self.buffer_size(bar_size);
        let launcher_size = self.buffer_size(launcher_logical_size);
        if buffer_width > 0
            && buffer_height > 0
            && let Some(gpu) = &mut self.cantus.gpu
        {
            if let Some(surface) = self.cantus.bar_surface {
                gpu.resize(surface, [buffer_width, buffer_height]);
            }
            if let Some(surface) = self.launcher_surface {
                let (width, height) = launcher_size;
                gpu.resize(surface, (width, height).into());
            }
        }

        self.cantus
            .render(bar_size.into(), self.launcher_surface.map(|surface| (surface, launcher_logical_size.into())));
        if let Some(text) = self.cantus.ui.launcher.pending_copy.take() {
            self.set_clipboard(&text, qhandle);
        }
        self.update_input_region(qhandle);
        if let Some(launcher) = self.launcher_wl_surface.clone() {
            if self.launcher_frame_callback.is_none() {
                self.launcher_frame_callback = Some(launcher.frame(qhandle, ()));
            }
            launcher.commit();
        } else {
            let bar = self.wl_surface.as_ref().unwrap().clone();
            if self.bar_frame_callback.is_none() {
                self.bar_frame_callback = Some(bar.frame(qhandle, ()));
            }
            bar.commit();
        }
    }

    fn update_scale_and_viewport(&self) {
        let scale = self.scale();
        let (bar_width, bar_height) = self.bar_surface_size();
        self.wl_surface
            .as_ref()
            .unwrap()
            .set_buffer_scale(self.viewport.as_ref().map_or_else(|| scale.ceil() as i32, |_| 1));
        if let Some(viewport) = &self.viewport {
            viewport.set_destination(bar_width as i32, bar_height as i32);
        }
        if let Some(surface) = &self.launcher_wl_surface {
            let (width, height) = self.launcher_surface_size();
            surface.set_buffer_scale(self.launcher_viewport.as_ref().map_or_else(|| scale.ceil() as i32, |_| 1));
            if let Some(viewport) = &self.launcher_viewport {
                viewport.set_destination(width as i32, height as i32);
            }
        }
    }

    fn update_input_region(&mut self, qhandle: &QueueHandle<Self>) {
        let wl_surface = self.active_surface().clone();
        let compositor = self.compositor.as_ref().unwrap();
        let region = compositor.create_region(qhandle, ());
        if self.cantus.gpu.is_some() {
            let interaction = &mut self.cantus.interaction;
            for rect in interaction.take_regions() {
                let [x, y, width, height] = [rect.min.x, rect.min.y, rect.max.x - rect.min.x, rect.max.y - rect.min.y].map(|value| value.round() as i32);
                region.add(x, y, width, height);
            }
        }
        wl_surface.set_input_region(Some(&region));
        region.destroy();
    }

    fn update_blur_region(&self, qhandle: &QueueHandle<Self>) {
        let Some(effect) = self.active_background_effect() else {
            return;
        };
        if !self.cantus.ui.launcher.open {
            effect.set_blur_region(None);
            return;
        }

        let compositor = self.compositor.as_ref().unwrap();
        let region = compositor.create_region(qhandle, ());
        let (width, height) = self.launcher_surface_size();
        let (origin, size) = self.cantus.ui.launcher.bounds(vec2(width, height));
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
                    state.launcher_width = Some(width as f32);
                } else {
                    state.surface_width = Some(width as f32);
                }
            }
            if height > 0 && is_launcher {
                state.launcher_height = Some(height as f32);
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
                state.cantus.ui.launcher.open = false;
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
        state.scale = Some(scale as f32 / 120.0);

        if state.cantus.gpu.is_some() {
            state.update_scale_and_viewport();
            state.try_render_frame(qhandle);
        }
    }
});

dispatch!(WlCallback, |state, proxy, event, qhandle| {
    if matches!(event, wl_callback::Event::Done { .. }) {
        let consumed = if state.bar_frame_callback.as_ref().is_some_and(|callback| callback.id() == proxy.id()) {
            state.bar_frame_callback.take();
            true
        } else if state.launcher_frame_callback.as_ref().is_some_and(|callback| callback.id() == proxy.id()) {
            state.launcher_frame_callback.take();
            true
        } else {
            false
        };
        if consumed {
            state.try_render_frame(qhandle);
        }
    }
});

dispatch!(WlOutput, |state, proxy, event, _qhandle| {
    match event {
        wl_output::Event::Mode {
            flags: WEnum::Value(flags),
            height,
            ..
        } if flags.contains(wl_output::Mode::Current) => {
            state.output_height = Some(height as f32);
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
    if !state.cantus.ui.launcher.open {
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
            state.cantus.ui.launcher.edit(|field| field.insert(&pasted));
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
        state.cantus.ui.launcher.key(key, shift);
    } else if let Some(typed) = character.filter(|typed| !typed.is_control() && !control) {
        state.cantus.ui.launcher.edit(|field| field.insert(typed.encode_utf8(&mut [0u8; 4])));
    }
}

dispatch!(WlPointer, |state, _proxy, event, _qhandle| {
    let surface_id = state.active_surface().id();
    let surface = state.launcher_surface;
    if state.cantus.gpu.is_none() {
        return;
    }
    if surface.or(state.cantus.bar_surface).is_none() {
        return;
    }
    let interaction = &mut state.cantus.interaction;
    match event {
        wl_pointer::Event::Enter {
            surface: wl_surface,
            surface_x,
            surface_y,
            ..
        } if surface_id == wl_surface.id() => {
            interaction.apply(InputEvent::Enter(vec2(surface_x as f32, surface_y as f32)));
        }
        wl_pointer::Event::Motion { surface_x, surface_y, .. } => {
            let position = vec2(surface_x as f32, surface_y as f32);
            interaction.apply(InputEvent::Motion(position));
        }
        wl_pointer::Event::Leave { .. } => {
            interaction.apply(InputEvent::Leave);
        }
        wl_pointer::Event::Button { button, state: button_state, .. } => match (button, button_state) {
            (0x110, WEnum::Value(wl_pointer::ButtonState::Pressed)) => {
                interaction.apply(InputEvent::Press);
            }
            (0x110, WEnum::Value(wl_pointer::ButtonState::Released)) => {
                interaction.apply(InputEvent::Release);
            }
            (0x111, WEnum::Value(wl_pointer::ButtonState::Pressed)) if interaction.dragging() => {
                interaction.apply(InputEvent::CancelDrag);
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
            interaction.apply(InputEvent::Scroll(discrete.signum()));
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

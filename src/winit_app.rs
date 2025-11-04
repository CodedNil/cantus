use crate::{CantusApp, PANEL_HEIGHT, PANEL_WIDTH};
use anyhow::Result;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use tracing::error;
use vello::wgpu::{PresentMode, SurfaceTargetUnsafe};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

pub fn run() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let mut app = WinitApp::new();
    event_loop
        .run_app(&mut app)
        .expect("winit event loop exited unexpectedly");
}

struct WinitApp {
    cantus: CantusApp,
    window: Option<Window>,
    needs_surface_recreate: bool,
}

impl WinitApp {
    fn new() -> Self {
        Self {
            cantus: CantusApp::default(),
            window: None,
            needs_surface_recreate: true,
        }
    }

    const fn window(&self) -> &Window {
        self.window.as_ref().expect("window not created")
    }

    fn recreate_surface(&mut self) -> Result<()> {
        let (raw_display_handle, raw_window_handle, size) = {
            let window = self.window();
            let display_handle = window
                .display_handle()
                .expect("failed to get display handle")
                .as_raw();
            let window_handle = window
                .window_handle()
                .expect("failed to get window handle")
                .as_raw();
            let size = window.inner_size();
            (display_handle, window_handle, size)
        };

        if size.width == 0 || size.height == 0 {
            return Ok(());
        }

        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle,
            raw_window_handle,
        };
        let surface = unsafe {
            self.cantus
                .render_context
                .instance
                .create_surface_unsafe(target)?
        };
        self.cantus.configure_render_surface(
            surface,
            size.width,
            size.height,
            PresentMode::AutoVsync,
        )?;
        self.needs_surface_recreate = false;
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        if self.needs_surface_recreate || self.cantus.render_surface.is_none() {
            self.recreate_surface()?;
        }

        if self.cantus.render_surface.is_none() {
            return Ok(());
        }

        let rendered = self.cantus.render(|| self.needs_surface_recreate = true)?;
        if rendered {
            self.window().request_redraw();
        }
        Ok(())
    }

    fn handle_cursor_move(&mut self, position: PhysicalPosition<f64>) {
        self.cantus.interaction.pointer_position = (position.x, position.y);
        self.cantus.handle_pointer_drag_motion();
    }

    fn handle_pointer_leave(&mut self) {
        self.cantus.interaction.pointer_position = (-1.0, -1.0);
        self.cantus.interaction.end_drag();
    }

    fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button != MouseButton::Left {
            return;
        }
        match state {
            ElementState::Pressed => self.cantus.interaction.start_drag(),
            ElementState::Released => {
                if !self.cantus.interaction.pointer_dragging {
                    let _ = self.cantus.handle_pointer_click();
                }
                self.cantus.interaction.end_drag();
            }
        }
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let size = PhysicalSize::new(PANEL_WIDTH as u32, PANEL_HEIGHT as u32);
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Cantus")
                    .with_inner_size(size)
                    .with_transparent(true)
                    .with_resizable(false),
            )
            .expect("failed to create window");

        self.cantus.scale_factor = window.scale_factor();
        self.window = Some(window);
        self.needs_surface_recreate = true;
        let _ = self.recreate_surface();
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.window.as_ref().map(winit::window::Window::id) != Some(window_id) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_size) => {
                self.needs_surface_recreate = true;
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.cantus.scale_factor = scale_factor;
                self.needs_surface_recreate = true;
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.render() {
                    error!("Rendering failed: {err}");
                }
            }
            WindowEvent::CursorMoved { position, .. } => self.handle_cursor_move(position),
            WindowEvent::CursorLeft { .. } => self.handle_pointer_leave(),
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_input(state, button);
            }
            WindowEvent::ActivationTokenDone { .. }
            | WindowEvent::Moved(_)
            | WindowEvent::Destroyed
            | WindowEvent::DroppedFile(_)
            | WindowEvent::HoveredFile(_)
            | WindowEvent::HoveredFileCancelled
            | WindowEvent::Focused(_)
            | WindowEvent::KeyboardInput { .. }
            | WindowEvent::ModifiersChanged(_)
            | WindowEvent::Ime(_)
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::MouseWheel { .. }
            | WindowEvent::PinchGesture { .. }
            | WindowEvent::PanGesture { .. }
            | WindowEvent::DoubleTapGesture { .. }
            | WindowEvent::RotationGesture { .. }
            | WindowEvent::TouchpadPressure { .. }
            | WindowEvent::AxisMotion { .. }
            | WindowEvent::Touch(_)
            | WindowEvent::ThemeChanged(_)
            | WindowEvent::Occluded(_) => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

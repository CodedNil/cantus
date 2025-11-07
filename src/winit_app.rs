use crate::{CantusApp, PANEL_HEIGHT, PANEL_WIDTH, interaction::InteractionState};
use anyhow::Result;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use tracing::error;
use vello::{
    kurbo::Point,
    wgpu::{PresentMode, SurfaceTargetUnsafe},
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
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
                    .with_decorations(false)
                    .with_active(false)
                    .with_position(LogicalPosition::new(5.0, 5.0))
                    .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
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
            WindowEvent::CursorMoved { position, .. } => {
                self.cantus.interaction.mouse_position = Point::new(position.x, position.y);
                self.cantus.interaction.handle_mouse_drag();
            }
            WindowEvent::CursorLeft { .. } => {
                self.cantus.interaction.mouse_position = Point::new(-100.0, -100.0);
                self.cantus.interaction.cancel_drag();
                self.cantus.interaction.mouse_down = false;
            }
            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left => match state {
                    ElementState::Pressed => {
                        self.cantus.interaction.left_click();
                    }
                    ElementState::Released => {
                        self.cantus
                            .interaction
                            .left_click_released(self.cantus.scale_factor);
                    }
                },
                MouseButton::Right => match state {
                    ElementState::Pressed => {
                        self.cantus.interaction.right_click();
                    }
                    ElementState::Released => {}
                },
                MouseButton::Middle
                | MouseButton::Back
                | MouseButton::Forward
                | MouseButton::Other(_) => {}
            },
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, y) => {
                    if y.abs() > 0.0 {
                        InteractionState::handle_scroll(-(y as i32).signum());
                    }
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    if pos.y.abs() > 0.0 {
                        InteractionState::handle_scroll(-(pos.y as i32).signum());
                    }
                }
            },
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

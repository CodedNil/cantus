use crate::{CantusApp, PANEL_EXTENSION, config::CONFIG, interaction::InteractionState};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use tracing::error;
use vello::{kurbo::Point, wgpu::SurfaceTargetUnsafe};
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
}

impl WinitApp {
    fn new() -> Self {
        Self {
            cantus: CantusApp::default(),
            window: None,
        }
    }

    const fn window(&self) -> &Window {
        self.window.as_ref().expect("window not created")
    }

    fn recreate_surface(&mut self) {
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
            return;
        }

        self.cantus.render_surface = None;

        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle,
            raw_window_handle,
        };
        let surface = unsafe {
            self.cantus
                .render_context
                .instance
                .create_surface_unsafe(target)
                .expect("Failed to create surface")
        };
        self.cantus
            .configure_render_surface(surface, size.width, size.height);
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let size = PhysicalSize::new(
            CONFIG.width as u32,
            (CONFIG.height + PANEL_EXTENSION) as u32,
        );
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
        if window
            .request_inner_size(PhysicalSize::new(
                (CONFIG.width * self.cantus.scale_factor) as u32,
                ((CONFIG.height + PANEL_EXTENSION) * self.cantus.scale_factor) as u32,
            ))
            .is_none()
        {
            error!("Failed to set inner size");
        }
        self.window = Some(window);
        self.recreate_surface();
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
                self.cantus.render_surface = None;
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.cantus.scale_factor = scale_factor;
                self.cantus.render_surface = None;
            }
            WindowEvent::RedrawRequested => {
                if self.cantus.render_surface.is_none() {
                    self.recreate_surface();
                }
                self.cantus.render();
                self.window().request_redraw();
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
            WindowEvent::MouseWheel { delta, .. } => {
                let signum_y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y.signum() as i32,
                    MouseScrollDelta::PixelDelta(pos) => pos.y.signum() as i32,
                };
                InteractionState::handle_scroll(-signum_y);
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

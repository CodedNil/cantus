use parley::{
    FontContext, Layout, LayoutContext, layout::PositionedLayoutItem, style::StyleProperty,
};
use std::sync::Arc;
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, Glyph, Renderer, RendererOptions, Scene,
    kurbo::{Affine, RoundedRect},
    peniko::{Color, Fill, color::palette},
    util::{RenderContext, RenderSurface},
    wgpu::{CommandEncoderDescriptor, PollType, PresentMode, TextureViewDescriptor},
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    platform::wayland::WindowAttributesExtWayland,
    window::{Window, WindowLevel},
};

mod spotify;

const PANEL_MARGIN: f64 = 12.0;
const BLUR_SIGMA: f32 = 60.0;
const WARP_STRENGTH: f32 = 2.0;
const SWIRL_STRENGTH: f32 = 0.4;
const WARP_TIME_SCALE: f32 = 0.8;

const WARP_SHADER: &str = include_str!("warp_background.wgsl");

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(LevelFilter::WARN.to_string())),
        )
        .init();

    // Run the spotify async task
    if let Err(e) = spotify::init().await {
        error!("Spotify init failed: {}", e);
    }

    // Create and run a winit event loop
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop
        .run_app(&mut CantusApp {
            context: RenderContext::new(),
            renderers: vec![],
            state: RenderState::Suspended(None),
            scene: Scene::new(),
        })
        .expect("Couldn't run event loop");
}

#[derive(Debug)]
enum RenderState {
    /// `RenderSurface` and `Window` for active rendering.
    Active {
        surface: Box<RenderSurface<'static>>,
        valid_surface: bool,
        window: Arc<Window>,
    },
    /// Cache a window so that it can be reused when the app is resumed after being suspended.
    Suspended(Option<Arc<Window>>),
}

struct CantusApp {
    /// The Vello `RenderContext` which is a global context that lasts for the lifetime of the application
    context: RenderContext,
    /// An array of renderers, one per wgpu device
    renderers: Vec<Option<Renderer>>,
    /// State with the winit Window and the wgpu Surface
    state: RenderState,
    /// Vello scene structure which is passed for rendering
    scene: Scene,
}

impl ApplicationHandler for CantusApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.state else {
            return;
        };

        // Get the winit window cached in a previous Suspended event or else create a new window
        let window = cached_window.take().unwrap_or_else(|| {
            let attr = Window::default_attributes()
                .with_title("Cantus")
                .with_name("cantus", "")
                .with_inner_size(LogicalSize::new(250, 80))
                .with_min_inner_size(LogicalSize::new(200, 60))
                .with_max_inner_size(LogicalSize::new(300, 150))
                .with_position(LogicalPosition::new(100, 100))
                .with_resizable(true)
                .with_active(false)
                .with_blur(true)
                .with_decorations(false)
                .with_transparent(true)
                .with_window_level(WindowLevel::AlwaysOnTop);
            Arc::new(event_loop.create_window(attr).unwrap())
        });

        // Create a vello Surface
        let size = window.inner_size();
        let surface_future = self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Error creating surface");

        // Create a vello Renderer for the surface (using its device id)
        self.renderers
            .resize_with(self.context.devices.len(), || None);
        self.renderers[surface.dev_id].get_or_insert_with(|| {
            Renderer::new(
                &self.context.devices[surface.dev_id].device,
                RendererOptions::default(),
            )
            .expect("Couldn't create renderer")
        });

        // Save the Window and Surface to a state variable
        self.state = RenderState::Active {
            surface: Box::new(surface),
            valid_surface: true,
            window,
        };
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.state {
            self.state = RenderState::Suspended(Some(window.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Only process events for our window, and only when we have a surface.
        let (surface, valid_surface) = match &mut self.state {
            RenderState::Active {
                surface,
                valid_surface,
                window,
            } if window.id() == window_id => (surface, valid_surface),
            _ => return,
        };

        match event {
            // Exit the event loop when a close is requested (e.g. window's close button is pressed)
            WindowEvent::CloseRequested => event_loop.exit(),

            // Resize the surface when the window is resized
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    self.context
                        .resize_surface(surface, size.width, size.height);
                    *valid_surface = true;
                } else {
                    *valid_surface = false;
                }
            }

            // This is where all the rendering happens
            WindowEvent::RedrawRequested => {
                if !*valid_surface {
                    return;
                }

                // Empty the scene of objects to draw.
                self.scene.reset();

                // Re-add the objects to draw to the scene.
                create_scene(
                    &mut self.scene,
                    f64::from(surface.config.width),
                    f64::from(surface.config.height),
                );

                // Get a handle to the device
                let device_handle = &self.context.devices[surface.dev_id];

                // Render to a texture, which we will later copy into the surface
                self.renderers[surface.dev_id]
                    .as_mut()
                    .unwrap()
                    .render_to_texture(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.scene,
                        &surface.target_view,
                        &vello::RenderParams {
                            base_color: palette::css::TRANSPARENT,
                            width: surface.config.width,
                            height: surface.config.height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .expect("failed to render to surface");

                let RenderState::Active {
                    surface, window, ..
                } = &mut self.state
                else {
                    return;
                };

                // Get the surface's texture
                let surface_texture = match surface.surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(vello::wgpu::SurfaceError::Outdated | vello::wgpu::SurfaceError::Lost) => {
                        let size = window.inner_size();
                        surface.config.width = size.width;
                        surface.config.height = size.height;
                        surface.surface.configure(
                            &self.context.devices[surface.dev_id].device,
                            &surface.config,
                        );
                        return;
                    }
                    Err(e) => {
                        panic!("Failed to get surface texture: {e:?}");
                    }
                };

                // Perform the copy
                let mut encoder =
                    device_handle
                        .device
                        .create_command_encoder(&CommandEncoderDescriptor {
                            label: Some("Surface Blit"),
                        });
                surface.blitter.copy(
                    &device_handle.device,
                    &mut encoder,
                    &surface.target_view,
                    &surface_texture
                        .texture
                        .create_view(&TextureViewDescriptor::default()),
                );
                device_handle.queue.submit([encoder.finish()]);
                // Queue the texture to be presented on the surface
                surface_texture.present();

                device_handle.device.poll(PollType::Poll).unwrap();
            }
            _ => {}
        }
    }
}

/// Create the vello scene structure
fn create_scene(scene: &mut Scene, width: f64, height: f64) {
    // Draw a rectangle filling the screen
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::new([0.9, 0.5, 0.6, 1.0]),
        None,
        &RoundedRect::new(0.0, 0.0, width, height, 10.0),
    );

    // Draw the album art
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::new([0.5, 0.0, 0.0, 1.0]),
        None,
        &RoundedRect::new(
            PANEL_MARGIN,
            PANEL_MARGIN,
            height - PANEL_MARGIN,
            height - PANEL_MARGIN,
            10.0,
        ),
    );

    // Draw the text for song, album, and artist
    let song_text_x = height - PANEL_MARGIN + 20.0;
    let text_color = Color::from_rgb8(240, 240, 240);
    let text_items = vec![
        ("Song: TODO", 22.0, -22.0),
        ("Album: TODO", 14.0, 0.0),
        ("Artist: TODO", 18.0, 22.0),
    ];
    for (text, size, y_offset) in text_items {
        draw_text(
            scene,
            text,
            size,
            text_color,
            song_text_x,
            height.mul_add(0.5, y_offset),
        );
    }
}

fn draw_text(
    scene: &mut Scene,
    text: &str,
    font_size: f32,
    font_color: Color,
    text_x: f64,
    text_y: f64,
) {
    let mut font_cx = FontContext::new();
    let mut layout_cx = LayoutContext::new();

    let mut builder = layout_cx.ranged_builder(&mut font_cx, text, 1.0, false);
    builder.push_default(StyleProperty::FontSize(font_size));

    let mut layout: Layout<()> = builder.build(text);
    layout.break_all_lines(None);
    let text_transform = Affine::translate((text_x, text_y - (f64::from(layout.height()) / 2.0)));

    for item in layout.lines().flat_map(|line| line.items()) {
        let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
            continue;
        };
        let glyphs = glyph_run.positioned_glyphs().map(|g| Glyph {
            id: g.id,
            x: g.x,
            y: g.y,
        });
        let run = glyph_run.run();
        scene
            .draw_glyphs(run.font())
            .font_size(run.font_size())
            .normalized_coords(run.normalized_coords())
            .transform(text_transform)
            .brush(font_color)
            .draw(Fill::NonZero, glyphs);
    }
}

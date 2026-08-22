use super::{
    canvas::Canvas,
    context::{Context, RenderFrame, SetupError, transition_image},
    surface::{Present, Surface, SurfaceFrame, SurfaceTarget},
};
use crate::{Frame, FrameGlobals, InputEvent, Rect, SurfaceHandle, glam::Vec3, input::Input, text::Text};
use ash::{util::read_spv, vk};
use core::slice::from_ref;
use std::{io::Cursor, time::Instant, vec, vec::Vec};

pub struct Renderer<Shared: FrameGlobals> {
    context: Context,
    surfaces: Vec<Option<SurfaceSlot<Shared>>>,
    shader: vk::ShaderModule,
    canvas: Canvas<Shared>,
    text: Text,
    started: Instant,
    last_frame: f32,
}

pub struct SurfaceSlot<Shared> {
    pub target: SurfaceTarget,
    pub globals: Shared,
    pub input: Input,
}

#[derive(Clone, Copy)]
pub struct ShaderModule<Shared> {
    bytes: &'static [u8],
    register: fn(&mut Canvas<Shared>),
    root: &'static str,
}

impl<Shared> ShaderModule<Shared> {
    #[doc(hidden)]
    pub const fn new(bytes: &'static [u8], register: fn(&mut Canvas<Shared>), root: &'static str) -> Self {
        Self { bytes, register, root }
    }
}

impl<Shared: FrameGlobals + Default> Renderer<Shared> {
    /// Creates the Vulkan renderer and its primary presentation surface.
    ///
    /// # Errors
    /// Returns an error when Vulkan initialization or shader loading fails.
    pub fn new(surface: Surface, [width, height]: [u32; 2], module: ShaderModule<Shared>, font: &[u8], text_color: Vec3) -> Result<Self, SetupError> {
        let context = Context::new(&surface)?;
        let target = SurfaceTarget::new(&context, surface, width, height)?;
        let words = read_spv(&mut Cursor::new(module.bytes)).map_err(|_| SetupError::InvalidShader)?;
        let shader = unsafe { context.0.device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None) }?;
        let mut canvas = Canvas::new(&context, shader, target.format(), module.root);
        (module.register)(&mut canvas);
        let text = Text::new(&mut canvas, font, text_color);
        Ok(Self {
            context,
            surfaces: vec![Some(SurfaceSlot {
                target,
                globals: Shared::default(),
                input: Input::default(),
            })],
            shader,
            canvas,
            text,
            started: Instant::now(),
            last_frame: 0.0,
        })
    }

    /// Starts the next upload and instance-collection frame.
    ///
    /// # Errors
    /// Returns an error when the next mapped arena region is still in use.
    fn begin_frame(&mut self) -> Result<(f32, f32), SetupError> {
        self.context.begin_uploads()?;
        self.canvas.begin_frame();
        self.text.begin_frame();
        let elapsed = self.started.elapsed().as_secs_f32();
        let delta = (elapsed - self.last_frame).min(0.1);
        self.last_frame = elapsed;
        Ok((elapsed, delta))
    }

    /// Collects and presents one immediate-mode frame.
    ///
    /// # Errors
    /// Returns an error when the next mapped upload arena is still in use.
    pub fn render<Services>(&mut self, services: Services, draw: impl FnOnce(&mut Frame<'_, Shared, Services>)) {
        let Ok((elapsed, delta)) = self.begin_frame() else {
            tracing::error!("GPU frame could not begin");
            return;
        };
        for surface in self.surfaces.iter_mut().flatten() {
            surface.input.begin_frame(delta);
            let info = surface.globals.frame_info_mut();
            info.time = elapsed;
            info.mouse_pos = surface.input.pointer();
            info.mouse_pressure = surface.input.pressure();
        }
        {
            let mut frame = Frame::new(&mut self.surfaces, delta, &mut self.text, &mut self.canvas, services);
            draw(&mut frame);
        }
        for surface in self.surfaces.iter_mut().flatten() {
            surface.input.end_frame();
        }
        self.text.finish_frame();
        self.canvas.prepare();
        let present = self.present(SurfaceHandle::PRIMARY);
        for index in 1..self.surfaces.len() {
            if self.surfaces[index].is_some() {
                let _ = self.present(SurfaceHandle(index));
            }
        }
        match present {
            Present::Lost => tracing::error!("Vulkan presentation surface was lost"),
            Present::Validation => tracing::error!("Vulkan frame failed validation"),
            Present::Rendered | Present::Unavailable => {}
        }
    }

    fn present(&mut self, surface: SurfaceHandle) -> Present {
        let Some(slot) = self.surfaces.get_mut(surface.index()).and_then(Option::as_mut) else {
            return Present::Unavailable;
        };
        let frame = FrameEncoder::new(&self.context, &mut slot.target, bytemuck::bytes_of(&slot.globals));
        match frame {
            Ok(frame) => {
                self.canvas.draw_surface(frame.render_frame(), surface);
                frame.present()
            }
            Err(present) => present,
        }
    }

    pub fn device_name(&self) -> &str {
        self.context.device_name()
    }

    pub fn input(&mut self, surface: SurfaceHandle, event: InputEvent) {
        if let Some(slot) = self.surfaces.get_mut(surface.index()).and_then(Option::as_mut) {
            slot.input.apply(event);
        }
    }

    pub fn dragging(&self, surface: SurfaceHandle) -> bool {
        self.surfaces.get(surface.index()).and_then(Option::as_ref).is_some_and(|slot| slot.input.dragging())
    }

    pub fn take_input_regions(&mut self, surface: SurfaceHandle) -> Vec<Rect> {
        self.surfaces
            .get_mut(surface.index())
            .and_then(Option::as_mut)
            .map(|slot| slot.input.take_regions().collect())
            .unwrap_or_default()
    }

    pub fn resize(&mut self, surface: SurfaceHandle, [width, height]: [u32; 2]) {
        if let Some(slot) = self.surfaces.get_mut(surface.index()).and_then(Option::as_mut) {
            slot.target.resize(width, height);
        }
    }

    /// Adds the optional second presentation surface.
    ///
    /// # Errors
    /// Returns an error when the surface is incompatible with this Vulkan device.
    pub fn add_surface(&mut self, target: Surface, [width, height]: [u32; 2]) -> Result<SurfaceHandle, SetupError> {
        let target = SurfaceTarget::new(&self.context, target, width, height)?;
        let Some(main) = self.surfaces.first().and_then(Option::as_ref) else {
            return Err(SetupError::IncompatibleSurface);
        };
        if target.format() != main.target.format() {
            return Err(SetupError::IncompatibleSurface);
        }
        let surface = SurfaceHandle(self.surfaces.len());
        self.surfaces.push(Some(SurfaceSlot {
            target,
            globals: Shared::default(),
            input: Input::default(),
        }));
        Ok(surface)
    }

    pub fn remove_surface(&mut self, surface: SurfaceHandle) {
        if surface.index() > 0
            && let Some(target) = self.surfaces.get_mut(surface.index())
        {
            *target = None;
        }
    }
}

impl<Shared: FrameGlobals> Drop for Renderer<Shared> {
    fn drop(&mut self) {
        unsafe {
            let _ = self.context.0.device.device_wait_idle();
            self.context.0.device.destroy_shader_module(self.shader, None);
        }
    }
}

#[must_use = "a frame encoder must be presented"]
struct FrameEncoder<'a> {
    context: &'a Context,
    target: &'a mut SurfaceTarget,
    surface: SurfaceFrame,
    frame: RenderFrame<'a>,
}

impl<'a> FrameEncoder<'a> {
    fn new(context: &'a Context, target: &'a mut SurfaceTarget, shared: &'a [u8]) -> Result<Self, Present> {
        let surface = target.acquire()?;
        let command = surface.command;
        let inner = &context.0;
        unsafe {
            inner
                .device
                .begin_command_buffer(command, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT))
                .map_err(|_| Present::Validation)?;
        }
        context.record_uploads(command);
        unsafe {
            transition_image(&inner.device, command, surface.image, surface.old_layout, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, 1);
            let attachment = vk::RenderingAttachmentInfo::default()
                .image_view(surface.view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(vk::ClearValue {
                    color: vk::ClearColorValue { float32: [0.0; 4] },
                });
            inner.device.cmd_begin_rendering(
                command,
                &vk::RenderingInfo::default()
                    .render_area(vk::Rect2D {
                        offset: vk::Offset2D::default(),
                        extent: target.extent(),
                    })
                    .layer_count(1)
                    .color_attachments(from_ref(&attachment)),
            );
            inner.device.cmd_set_viewport(
                command,
                0,
                &[vk::Viewport {
                    width: target.extent().width as f32,
                    height: target.extent().height as f32,
                    max_depth: 1.0,
                    ..Default::default()
                }],
            );
            inner.device.cmd_set_scissor(
                command,
                0,
                &[vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: target.extent(),
                }],
            );
        }
        Ok(Self {
            context,
            target,
            surface,
            frame: RenderFrame::new(command, shared),
        })
    }

    pub const fn render_frame(&self) -> &RenderFrame<'_> {
        &self.frame
    }

    pub fn present(mut self) -> Present {
        if self.finish().is_err() {
            return Present::Validation;
        }
        self.target.present(&self.surface)
    }

    fn finish(&mut self) -> Result<(), SetupError> {
        let inner = &self.context.0;
        let command = self.surface.command;
        unsafe {
            inner.device.cmd_end_rendering(command);
            transition_image(
                &inner.device,
                command,
                self.surface.image,
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vk::ImageLayout::PRESENT_SRC_KHR,
                1,
            );
            inner.device.end_command_buffer(command)?;
            let timeline = inner.reserve_timeline();
            let waits = [vk::SemaphoreSubmitInfo::default()
                .semaphore(self.surface.acquire)
                .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)];
            let command_info = [vk::CommandBufferSubmitInfo::default().command_buffer(command)];
            let signals = [
                vk::SemaphoreSubmitInfo::default()
                    .semaphore(self.surface.complete)
                    .stage_mask(vk::PipelineStageFlags2::ALL_GRAPHICS),
                vk::SemaphoreSubmitInfo::default()
                    .semaphore(inner.timeline)
                    .value(timeline)
                    .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS),
            ];
            inner.device.queue_submit2(
                inner.queue,
                &[vk::SubmitInfo2::default()
                    .wait_semaphore_infos(&waits)
                    .command_buffer_infos(&command_info)
                    .signal_semaphore_infos(&signals)],
                vk::Fence::null(),
            )?;
            self.target.submitted(&self.surface, timeline);
            inner.uploads_submitted(timeline);
        }
        Ok(())
    }
}

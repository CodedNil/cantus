use super::{
    canvas::Canvas,
    context::{Context, Inner, RenderFrame, SetupError, transition_image},
    surface::{Present, SurfaceFrame, SurfaceTarget},
};
use crate::{
    Frame, SurfaceHandle,
    contract::{Program, PushBlock},
    glam::{Vec2, Vec3},
    text::Text,
};
use ash::{util::read_spv, vk};
use core::{error::Error, fmt, slice::from_ref};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{io::Cursor, rc::Rc, time::Instant, vec, vec::Vec};

pub struct Renderer {
    context: Context,
    surfaces: Vec<SurfaceEntry>,
    canvas: Canvas,
    text: Text,
    started: Instant,
    last_frame: f32,
}

struct SurfaceEntry {
    generation: u32,
    slot: Option<SurfaceSlot>,
}

struct SurfaceSlot {
    target: SurfaceTarget,
    push: PushBlock,
}

fn surface_slot(entries: &mut [SurfaceEntry], handle: SurfaceHandle) -> Option<&mut SurfaceSlot> {
    entries.get_mut(handle.index()).filter(|entry| entry.generation == handle.generation())?.slot.as_mut()
}

pub struct Render<'a> {
    surfaces: &'a mut [SurfaceEntry],
    time: f32,
    delta_time: f32,
    text: &'a mut Text,
    canvas: &'a mut Canvas,
}

#[derive(Debug)]
pub enum RenderError {
    Upload(SetupError),
    SurfaceLost,
    Validation,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Upload(error) => error.fmt(f),
            Self::SurfaceLost => f.write_str("presentation surface was lost"),
            Self::Validation => f.write_str("frame failed Vulkan validation"),
        }
    }
}

impl Error for RenderError {}

impl Render<'_> {
    pub fn surface(&mut self, surface: SurfaceHandle, screen_size: Vec2, draw: impl FnOnce(Frame<'_>)) {
        let Some(slot) = surface_slot(self.surfaces, surface) else {
            return;
        };
        slot.push.screen_size = screen_size;
        slot.push.time = self.time;
        draw(Frame::new(&slot.push, self.time, self.delta_time, self.text, self.canvas, surface));
        self.canvas.ensure_globals(surface);
    }
}

pub struct ShaderModule {
    bytes: &'static [u8],
    root: &'static str,
}

impl ShaderModule {
    pub const fn new(bytes: &'static [u8], root: &'static str) -> Self {
        Self { bytes, root }
    }
}

impl Renderer {
    /// Creates the Vulkan renderer and its primary presentation surface.
    ///
    /// # Errors
    /// Returns an error when Vulkan initialization or shader loading fails.
    pub fn new(
        program: Program,
        surface: &(impl HasDisplayHandle + HasWindowHandle),
        [width, height]: [u32; 2],
        font: &[u8],
        text_color: Vec3,
    ) -> Result<(Self, SurfaceHandle), SetupError> {
        let (context, surface) = Context::new(surface)?;
        let target = SurfaceTarget::from_raw(&context, surface, width, height)?;
        let module = program.shader;
        let words = read_spv(&mut Cursor::new(module.bytes)).map_err(|_| SetupError::InvalidShader)?;
        let mut canvas = Canvas::new(&context, words, target.format, module.root);
        let text = Text::new(&mut canvas, font, text_color);
        Ok((
            Self {
                context,
                surfaces: vec![SurfaceEntry {
                    generation: 0,
                    slot: Some(SurfaceSlot {
                        target,
                        push: PushBlock::default(),
                    }),
                }],
                canvas,
                text,
                started: Instant::now(),
                last_frame: 0.0,
            },
            SurfaceHandle::new(0, 0),
        ))
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
    pub fn render(&mut self, draw: impl FnOnce(&mut Render<'_>)) -> Result<(), RenderError> {
        let (elapsed, delta) = self.begin_frame().map_err(RenderError::Upload)?;
        {
            let mut render = Render {
                surfaces: &mut self.surfaces,
                time: elapsed,
                delta_time: delta,
                text: &mut self.text,
                canvas: &mut self.canvas,
            };
            draw(&mut render);
        }
        let placed_glyphs = self.text.finish_frame();
        self.canvas.prepare(placed_glyphs);
        for index in 0..self.surfaces.len() {
            let Some(generation) = self.surfaces[index].slot.as_ref().map(|_| self.surfaces[index].generation) else {
                continue;
            };
            let surface = SurfaceHandle::new(index, generation);
            if !self.canvas.has_draws(surface) {
                continue;
            }
            match self.present(surface) {
                Present::Lost => return Err(RenderError::SurfaceLost),
                Present::Validation => return Err(RenderError::Validation),
                Present::Rendered | Present::Unavailable => {}
            }
        }
        Ok(())
    }

    fn present(&mut self, surface: SurfaceHandle) -> Present {
        let Some(slot) = surface_slot(&mut self.surfaces, surface) else {
            return Present::Unavailable;
        };
        let frame = FrameEncoder::new(&self.context, &mut slot.target, bytemuck::bytes_of(&slot.push));
        match frame {
            Ok(frame) => {
                self.canvas.draw_surface(&frame.frame, surface);
                frame.present()
            }
            Err(present) => present,
        }
    }

    pub fn device_name(&self) -> &str {
        self.context.device_name()
    }

    pub fn resize(&mut self, surface: SurfaceHandle, [width, height]: [u32; 2]) {
        if let Some(slot) = surface_slot(&mut self.surfaces, surface) {
            slot.target.resize(width, height);
        }
    }

    /// Adds another presentation surface.
    ///
    /// # Errors
    /// Returns an error when the surface is incompatible with this Vulkan device.
    pub fn add_surface(&mut self, target: &(impl HasDisplayHandle + HasWindowHandle), [width, height]: [u32; 2]) -> Result<SurfaceHandle, SetupError> {
        let target = SurfaceTarget::new(&self.context, target, width, height)?;
        if target.format != self.canvas.format() {
            return Err(SetupError::IncompatibleSurface);
        }
        let slot = SurfaceSlot {
            target,
            push: PushBlock::default(),
        };
        if let Some((index, entry)) = self.surfaces.iter_mut().enumerate().find(|(_, entry)| entry.slot.is_none()) {
            entry.slot = Some(slot);
            Ok(SurfaceHandle::new(index, entry.generation))
        } else {
            let index = self.surfaces.len();
            self.surfaces.push(SurfaceEntry { generation: 0, slot: Some(slot) });
            Ok(SurfaceHandle::new(index, 0))
        }
    }

    pub fn remove_surface(&mut self, surface: SurfaceHandle) {
        if let Some(entry) = self.surfaces.get_mut(surface.index())
            && entry.generation == surface.generation()
        {
            entry.slot = None;
            entry.generation = entry.generation.wrapping_add(1);
        }
    }
}

#[must_use = "a frame encoder must be presented"]
struct FrameEncoder<'a> {
    inner: Rc<Inner>,
    target: &'a mut SurfaceTarget,
    surface: SurfaceFrame,
    pub(super) frame: RenderFrame<'a>,
}

impl<'a> FrameEncoder<'a> {
    fn new(context: &Context, target: &'a mut SurfaceTarget, shared: &'a [u8]) -> Result<Self, Present> {
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
                        extent: target.extent,
                    })
                    .layer_count(1)
                    .color_attachments(from_ref(&attachment)),
            );
            inner.device.cmd_set_viewport(
                command,
                0,
                &[vk::Viewport {
                    width: target.extent.width as f32,
                    height: target.extent.height as f32,
                    max_depth: 1.0,
                    ..Default::default()
                }],
            );
            inner.device.cmd_set_scissor(
                command,
                0,
                &[vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: target.extent,
                }],
            );
        }
        Ok(Self {
            inner: context.0.clone(),
            target,
            surface,
            frame: RenderFrame::new(command, shared),
        })
    }
    fn present(mut self) -> Present {
        if self.finish().is_err() {
            return Present::Validation;
        }
        self.target.present(&self.surface)
    }

    fn finish(&mut self) -> Result<(), SetupError> {
        let command = self.surface.command;
        unsafe {
            self.inner.device.cmd_end_rendering(command);
            transition_image(
                &self.inner.device,
                command,
                self.surface.image,
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vk::ImageLayout::PRESENT_SRC_KHR,
                1,
            );
            self.inner.device.end_command_buffer(command)?;
            let timeline = self.inner.reserve_timeline();
            let waits = [vk::SemaphoreSubmitInfo::default()
                .semaphore(self.surface.acquire)
                .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)];
            let command_info = [vk::CommandBufferSubmitInfo::default().command_buffer(command)];
            let signals = [
                vk::SemaphoreSubmitInfo::default()
                    .semaphore(self.surface.complete)
                    .stage_mask(vk::PipelineStageFlags2::ALL_GRAPHICS),
                vk::SemaphoreSubmitInfo::default()
                    .semaphore(self.inner.timeline)
                    .value(timeline)
                    .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS),
            ];
            self.inner.device.queue_submit2(
                self.inner.queue,
                &[vk::SubmitInfo2::default()
                    .wait_semaphore_infos(&waits)
                    .command_buffer_infos(&command_info)
                    .signal_semaphore_infos(&signals)],
                vk::Fence::null(),
            )?;
            self.target.submitted(&self.surface, timeline);
            self.inner.uploads_submitted(timeline);
        }
        Ok(())
    }
}

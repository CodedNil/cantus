use super::{
    context::{Context, Render, RenderFrame, SetupError, transition_image},
    pass::PassBuilder,
    surface::{Present, Surface, SurfaceFrame, SurfaceTarget},
};
use crate::data::ShaderData;
use ash::{util::read_spv, vk};
use core::slice::from_ref;
use std::io::Cursor;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
impl Color {
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

pub struct Program<Shared, Passes> {
    context: Context,
    target: SurfaceTarget,
    shared: Shared,
    passes: Passes,
    shader: vk::ShaderModule,
    secondary: Option<SurfaceTarget>,
}
impl<Shared: ShaderData + Default, Passes: Render> Program<Shared, Passes> {
    /// Creates the Vulkan device, swapchain, shader module, and typed passes.
    ///
    /// # Errors
    /// Returns an error if Vulkan setup or SPIR-V loading fails.
    pub fn surface(surface: Surface, width: u32, height: u32, shader: &[u8], build: impl FnOnce(&PassBuilder<'_, Shared>) -> Passes) -> Result<Self, SetupError> {
        let context = Context::new(&surface)?;
        let target = SurfaceTarget::new(&context, surface, width, height)?;
        let words = read_spv(&mut Cursor::new(shader)).map_err(|_| SetupError::InvalidShader)?;
        let module = unsafe { context.0.device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None) }?;
        let shared = Shared::default();
        let passes = build(&PassBuilder::new(&context, module, target.format()));
        Ok(Self {
            context,
            target,
            shared,
            passes,
            shader: module,
            secondary: None,
        })
    }
    pub fn update(&mut self, update: impl FnOnce(&mut Shared, &mut Passes)) {
        update(&mut self.shared, &mut self.passes);
        self.passes.prepare();
    }
    pub fn update_shared(&mut self, update: impl FnOnce(&mut Shared)) {
        update(&mut self.shared);
    }
    pub const fn passes_mut(&mut self) -> &mut Passes {
        &mut self.passes
    }
    pub fn device_name(&self) -> &str {
        self.context.device_name()
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(width, height);
    }
    pub fn resize_secondary(&mut self, width: u32, height: u32) {
        if let Some(t) = &mut self.secondary {
            t.resize(width, height);
        }
    }
    /// Creates the optional secondary presentation swapchain.
    ///
    /// # Errors
    /// Returns an error when the surface cannot be configured for this device.
    pub fn add_secondary_surface(&mut self, surface: Surface, width: u32, height: u32) -> Result<(), SetupError> {
        self.secondary = Some(SurfaceTarget::new(&self.context, surface, width, height)?);
        Ok(())
    }
    pub fn remove_secondary_surface(&mut self) {
        self.secondary = None;
    }
    pub fn render_custom(&mut self, clear: Color, update: impl FnOnce(&mut Shared, &mut Passes), draw: impl FnOnce(&Passes, &RenderFrame<'_>)) -> Present {
        self.update(update);
        render_target(&self.context, &mut self.target, clear, bytemuck::bytes_of(&self.shared), |command| {
            draw(&self.passes, command);
        })
    }
    pub fn render_secondary(&mut self, clear: Color, draw: impl FnOnce(&Passes, &RenderFrame<'_>)) -> Present {
        let Some(target) = &mut self.secondary else { return Present::Unavailable };
        render_target(&self.context, target, clear, bytemuck::bytes_of(&self.shared), |command| draw(&self.passes, command))
    }
}
impl<Shared, Passes> Drop for Program<Shared, Passes> {
    fn drop(&mut self) {
        unsafe {
            let _ = self.context.0.device.device_wait_idle();
            self.context.0.device.destroy_shader_module(self.shader, None);
        }
    }
}

fn render_target(context: &Context, target: &mut SurfaceTarget, clear: Color, shared: &[u8], draw: impl FnOnce(&RenderFrame<'_>)) -> Present {
    let frame = match target.acquire() {
        Ok(frame) => frame,
        Err(status) => return status,
    };
    let result = record(context, target, &frame, clear, shared, draw);
    if result.is_err() {
        return Present::Validation;
    }
    target.present(&frame)
}
fn record(context: &Context, target: &mut SurfaceTarget, frame: &SurfaceFrame, clear: Color, shared: &[u8], draw: impl FnOnce(&RenderFrame<'_>)) -> Result<(), SetupError> {
    let inner = &context.0;
    let command = frame.command;
    unsafe {
        inner
            .device
            .begin_command_buffer(command, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT))?;
        transition_image(&inner.device, command, frame.image, frame.old_layout, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, 1);
        let attachment = vk::RenderingAttachmentInfo::default()
            .image_view(frame.view)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [clear.r as f32, clear.g as f32, clear.b as f32, clear.a as f32],
                },
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
                x: 0.0,
                y: 0.0,
                width: target.extent().width as f32,
                height: target.extent().height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
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
        draw(&RenderFrame::new(command, shared));
        inner.device.cmd_end_rendering(command);
        transition_image(
            &inner.device,
            command,
            frame.image,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            vk::ImageLayout::PRESENT_SRC_KHR,
            1,
        );
        inner.device.end_command_buffer(command)?;
        let timeline = inner.reserve_timeline();
        let waits = [vk::SemaphoreSubmitInfo::default()
            .semaphore(frame.acquire)
            .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)];
        let command_info = [vk::CommandBufferSubmitInfo::default().command_buffer(command)];
        let signals = [
            vk::SemaphoreSubmitInfo::default()
                .semaphore(frame.complete)
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
        target.submitted(frame, timeline);
    }
    Ok(())
}

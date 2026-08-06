use super::buffer::DataBuffer;
use super::surface::Acquire;
use crate::{BufferData, Context, PassBuilder, Present, Render, SetupError, SurfaceTarget};
use core::slice::from_ref;
use wgpu::{
    AdapterInfo, Color, Instance, PowerPreference, ShaderModuleDescriptor, Surface, TextureFormat,
    TextureView,
};

/// A GPU context with typed shared data, passes, and a render target.
pub struct Program<SharedData, Passes, Target = SurfaceTarget<'static>> {
    context: Context,
    target: Target,
    shared: SharedData,
    shared_buffer: DataBuffer,
    passes: Passes,
}

impl<SharedData, Passes, Target> Program<SharedData, Passes, Target> {
    /// Creates and validates a typed GPU program.
    ///
    /// # Errors
    /// Returns an error when WGPU rejects its shader, pipelines, or bindings.
    pub async fn new(
        context: Context,
        target: Target,
        format: TextureFormat,
        shader: ShaderModuleDescriptor<'_>,
        build: impl FnOnce(&PassBuilder<'_, SharedData>) -> Passes,
    ) -> Result<Self, SetupError>
    where
        SharedData: BufferData + Default,
    {
        let validation = context.device().push_error_scope(wgpu::ErrorFilter::Validation);
        let shared = SharedData::default();
        let shared_buffer =
            DataBuffer::new::<SharedData>(context.device(), context.queue(), "Shared", 1);
        let shader = context.device().create_shader_module(shader);
        let passes = build(&PassBuilder::new(
            context.device(),
            context.queue(),
            &shader,
            format,
            shared_buffer.raw(),
        ));
        let program = Self {
            context,
            target,
            shared,
            shared_buffer,
            passes,
        };
        validation
            .pop()
            .await
            .map_or_else(|| Ok(program), |error| Err(SetupError::Validation(error)))
    }

    pub fn update(&mut self, update: impl FnOnce(&mut SharedData, &mut Passes))
    where
        SharedData: BufferData,
        Passes: Render,
    {
        update(&mut self.shared, &mut self.passes);
        self.shared_buffer.upload(from_ref(&self.shared));
        self.passes.prepare();
    }

    pub const fn device(&self) -> &wgpu::Device {
        self.context.device()
    }

    pub const fn queue(&self) -> &wgpu::Queue {
        self.context.queue()
    }

    pub const fn passes_mut(&mut self) -> &mut Passes {
        &mut self.passes
    }

    /// Draws the state prepared by [`Self::update`] to a texture view.
    pub fn draw_to(&self, target: &TextureView, clear: Color)
    where
        Passes: Render,
    {
        self.context.draw(target, clear, &self.passes);
    }
}

impl<'window, SharedData: BufferData + Default, Passes: Render>
    Program<SharedData, Passes, SurfaceTarget<'window>>
{
    /// Creates a program presented to a surface.
    ///
    /// # Errors
    /// Returns an error when WGPU or surface setup fails.
    pub async fn surface(
        instance: &Instance,
        surface: Surface<'window>,
        width: u32,
        height: u32,
        power_preference: PowerPreference,
        shader: ShaderModuleDescriptor<'_>,
        build: impl FnOnce(&PassBuilder<'_, SharedData>) -> Passes,
    ) -> Result<(Self, AdapterInfo), SetupError> {
        let (context, target, info) =
            SurfaceTarget::create(instance, surface, width, height, power_preference).await?;
        let format = target.format();
        Ok((Self::new(context, target, format, shader, build).await?, info))
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(&self.context, width, height);
    }

    /// Replaces the presentation surface.
    ///
    /// # Errors
    /// Returns an error when the surface is incompatible.
    pub fn replace_surface(&mut self, surface: Surface<'window>) -> Result<(), SetupError> {
        self.target.replace(&self.context, surface)
    }

    pub fn render(
        &mut self,
        clear: Color,
        update: impl FnOnce(&mut SharedData, &mut Passes),
    ) -> Present {
        let frame = match self.target.acquire(&self.context) {
            Acquire::Frame(frame) => frame,
            Acquire::Unavailable => return Present::Unavailable,
            Acquire::Lost => return Present::Lost,
            Acquire::Validation => return Present::Validation,
        };
        self.update(update);
        self.context.draw(&frame.view, clear, &self.passes);
        self.target.present(&self.context, frame);
        Present::Rendered
    }
}

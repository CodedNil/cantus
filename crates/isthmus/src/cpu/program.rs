use super::buffer::DataBuffer;
use crate::{
    cpu::{
        context::{Context, Render, SetupError},
        pass::PassBuilder,
        surface::{Present, SurfaceTarget},
    },
    data::BufferData,
};
use core::slice::from_ref;
use wgpu::{Color, Instance, PowerPreference, ShaderModuleDescriptor, Surface, TextureFormat};

/// A GPU context with typed shared data, passes, and a render target.
pub struct Program<SharedData, Passes, Target = SurfaceTarget<'static>> {
    context: Context,
    target: Target,
    shared: SharedData,
    shared_buffer: DataBuffer<SharedData>,
    passes: Passes,
    secondary: Option<SurfaceTarget<'static>>,
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
        let shared_buffer = DataBuffer::new(context.device(), context.queue(), "Shared", 1);
        let shader = context.device().create_shader_module(shader);
        let passes = build(&PassBuilder::new(context.device(), context.queue(), &shader, format, &shared_buffer));
        let program = Self {
            context,
            target,
            shared,
            shared_buffer,
            passes,
            secondary: None,
        };
        validation.pop().await.map_or_else(|| Ok(program), |error| Err(SetupError::Validation(error)))
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

    pub fn update_shared(&mut self, update: impl FnOnce(&mut SharedData))
    where
        SharedData: BufferData,
    {
        update(&mut self.shared);
        self.shared_buffer.upload(from_ref(&self.shared));
    }

    pub const fn device(&self) -> &wgpu::Device {
        self.context.device()
    }

    pub fn adapter_info(&self) -> wgpu::AdapterInfo {
        self.context.adapter().get_info()
    }

    pub const fn passes_mut(&mut self) -> &mut Passes {
        &mut self.passes
    }
}

impl<'window, SharedData: BufferData + Default, Passes: Render> Program<SharedData, Passes, SurfaceTarget<'window>> {
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
    ) -> Result<Self, SetupError> {
        let context = Context::new(instance, Some(&surface), power_preference).await?;
        let target = SurfaceTarget::new(&context, surface, width, height)?;
        let format = target.format();
        Self::new(context, target, format, shader, build).await
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(&self.context, width, height);
    }

    pub fn render_custom(&mut self, clear: Color, update: impl FnOnce(&mut SharedData, &mut Passes), draw: impl for<'a> FnOnce(&'a Passes, wgpu::RenderPass<'a>)) -> Present {
        let frame = match self.target.acquire(&self.context) {
            Ok(frame) => frame,
            Err(status) => return status,
        };
        self.update(update);
        self.context.draw_custom(&frame.view, clear, |pass| draw(&self.passes, pass));
        self.target.present(&self.context, frame);
        Present::Rendered
    }

    /// Replaces the presentation surface.
    ///
    /// # Errors
    /// Returns an error when the surface is incompatible.
    pub fn replace_surface(&mut self, surface: Surface<'window>) -> Result<(), SetupError> {
        self.target.replace(&self.context, surface)
    }

    pub fn render(&mut self, clear: Color, update: impl FnOnce(&mut SharedData, &mut Passes)) -> Present {
        let frame = match self.target.acquire(&self.context) {
            Ok(frame) => frame,
            Err(status) => return status,
        };
        self.update(update);
        self.context.draw(&frame.view, clear, &self.passes);
        self.target.present(&self.context, frame);
        Present::Rendered
    }
}

impl<SharedData: BufferData + Default, Passes: Render> Program<SharedData, Passes, SurfaceTarget<'static>> {
    /// Adds the launcher presentation target while retaining the bar target.
    pub fn add_secondary_surface(&mut self, surface: Surface<'static>, width: u32, height: u32) -> Result<(), SetupError> {
        let target = SurfaceTarget::new(&self.context, surface, width, height)?;
        self.secondary = Some(target);
        Ok(())
    }

    pub fn remove_secondary_surface(&mut self) {
        self.secondary = None;
    }

    pub fn resize_secondary(&mut self, width: u32, height: u32) {
        if let Some(target) = self.secondary.as_mut() {
            target.resize(&self.context, width, height);
        }
    }

    pub fn render_secondary(&mut self, clear: Color, draw: impl for<'a> FnOnce(&'a Passes, wgpu::RenderPass<'a>)) -> Present {
        let Some(target) = self.secondary.as_ref() else {
            return Present::Unavailable;
        };
        let frame = match target.acquire(&self.context) {
            Ok(frame) => frame,
            Err(status) => return status,
        };
        self.context.draw_custom(&frame.view, clear, |pass| draw(&self.passes, pass));
        target.present(&self.context, frame);
        Present::Rendered
    }
}

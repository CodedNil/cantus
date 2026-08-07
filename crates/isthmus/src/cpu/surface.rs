use crate::cpu::context::{Context, SetupError};
use wgpu::{
    AdapterInfo, CompositeAlphaMode, CurrentSurfaceTexture, Instance, PowerPreference, Surface,
    SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureView, TextureViewDescriptor,
};

pub(super) enum Acquire {
    Frame(SurfaceFrame),
    Unavailable,
    Lost,
    Validation,
}

pub(super) struct SurfaceFrame {
    pub(super) texture: SurfaceTexture,
    pub(super) view: TextureView,
    pub(super) reconfigure_after_present: bool,
}

pub enum Present {
    Rendered,
    Unavailable,
    Lost,
    Validation,
}

pub struct SurfaceTarget<'window> {
    surface: Surface<'window>,
    config: SurfaceConfiguration,
}

impl<'window> SurfaceTarget<'window> {
    pub(super) async fn create(
        instance: &Instance,
        surface: Surface<'window>,
        width: u32,
        height: u32,
        power_preference: PowerPreference,
    ) -> Result<(Context, Self, AdapterInfo), SetupError> {
        let (context, info) = Context::new(instance, Some(&surface), power_preference).await?;
        let target = Self::new(&context, surface, width, height)?;
        Ok((context, target, info))
    }

    /// Configures a surface for an existing context.
    ///
    /// # Errors
    /// Returns an error when the adapter cannot present to the surface.
    pub fn new(
        context: &Context,
        surface: Surface<'window>,
        width: u32,
        height: u32,
    ) -> Result<Self, SetupError> {
        let capabilities = surface.get_capabilities(context.adapter());
        let format = [TextureFormat::Rgba8Unorm, TextureFormat::Bgra8Unorm]
            .into_iter()
            .find(|format| capabilities.formats.contains(format))
            .or_else(|| capabilities.formats.first().copied())
            .ok_or(SetupError::UnsupportedSurface)?;
        let alpha_mode = [
            CompositeAlphaMode::PreMultiplied,
            CompositeAlphaMode::PostMultiplied,
        ]
        .into_iter()
        .find(|mode| capabilities.alpha_modes.contains(mode))
        .or_else(|| capabilities.alpha_modes.first().copied())
        .ok_or(SetupError::UnsupportedSurface)?;
        let mut config = surface
            .get_default_config(context.adapter(), width, height)
            .ok_or(SetupError::UnsupportedSurface)?;
        config.desired_maximum_frame_latency = 1;
        config.format = format;
        config.alpha_mode = alpha_mode;
        surface.configure(context.device(), &config);
        Ok(Self { surface, config })
    }

    pub const fn format(&self) -> TextureFormat {
        self.config.format
    }

    pub(super) fn resize(&mut self, context: &Context, width: u32, height: u32) {
        if width > 0 && height > 0 && (self.config.width, self.config.height) != (width, height) {
            self.config.width = width;
            self.config.height = height;
            self.configure(context);
        }
    }

    pub(super) fn replace(
        &mut self,
        context: &Context,
        surface: Surface<'window>,
    ) -> Result<(), SetupError> {
        let capabilities = surface.get_capabilities(context.adapter());
        if !supports(&capabilities, &self.config) {
            return Err(SetupError::IncompatibleSurface);
        }
        surface.configure(context.device(), &self.config);
        self.surface = surface;
        Ok(())
    }

    pub(super) fn acquire(&self, context: &Context) -> Acquire {
        match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => Acquire::Frame(Self::frame(texture, false)),
            CurrentSurfaceTexture::Suboptimal(texture) => Acquire::Frame(Self::frame(texture, true)),
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => Acquire::Unavailable,
            CurrentSurfaceTexture::Outdated => {
                self.configure(context);
                Acquire::Unavailable
            }
            CurrentSurfaceTexture::Lost => Acquire::Lost,
            CurrentSurfaceTexture::Validation => Acquire::Validation,
        }
    }

    pub(super) fn present(&self, context: &Context, frame: SurfaceFrame) {
        context.queue().present(frame.texture);
        if frame.reconfigure_after_present {
            self.configure(context);
        }
    }

    fn frame(texture: SurfaceTexture, reconfigure_after_present: bool) -> SurfaceFrame {
        let view = texture.texture.create_view(&TextureViewDescriptor::default());
        SurfaceFrame {
            texture,
            view,
            reconfigure_after_present,
        }
    }

    fn configure(&self, context: &Context) {
        self.surface.configure(context.device(), &self.config);
    }
}

fn supports(capabilities: &wgpu::SurfaceCapabilities, config: &SurfaceConfiguration) -> bool {
    capabilities.formats.contains(&config.format)
        && capabilities.alpha_modes.contains(&config.alpha_mode)
        && capabilities.present_modes.contains(&config.present_mode)
        && capabilities.usages.contains(config.usage)
        && config
            .view_formats
            .iter()
            .all(|format| capabilities.formats.contains(format))
}

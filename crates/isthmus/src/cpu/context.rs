use core::{error::Error, fmt};
use wgpu::{
    Adapter, AdapterInfo, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, Limits,
    LoadOp, MemoryHints, Operations, PowerPreference, Queue, RenderPass, RenderPassColorAttachment,
    RenderPassDescriptor, RequestAdapterError, RequestAdapterOptions, RequestDeviceError, StoreOp,
    Surface, TextureView,
};

#[derive(Debug)]
pub enum SetupError {
    Adapter(RequestAdapterError),
    Device(RequestDeviceError),
    Validation(wgpu::Error),
    UnsupportedSurface,
    IncompatibleSurface,
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Adapter(error) => error.fmt(f),
            Self::Device(error) => error.fmt(f),
            Self::Validation(error) => error.fmt(f),
            Self::UnsupportedSurface => f.write_str("surface is unsupported"),
            Self::IncompatibleSurface => f.write_str("replacement surface is incompatible"),
        }
    }
}

impl Error for SetupError {}

/// An ordered unit of GPU preparation and drawing.
pub trait Render {
    fn prepare(&mut self);
    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>);
}

impl<T: Render> Render for Option<T> {
    fn prepare(&mut self) {
        if let Some(value) = self {
            value.prepare();
        }
    }

    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>) {
        if let Some(value) = self {
            value.draw(pass);
        }
    }
}

pub struct Context {
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Context {
    /// Creates a WGPU context.
    ///
    /// # Errors
    /// Returns an error when adapter or device setup fails.
    pub async fn new(
        instance: &Instance,
        compatible_surface: Option<&Surface<'_>>,
        power_preference: PowerPreference,
    ) -> Result<(Self, AdapterInfo), SetupError> {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference,
                compatible_surface,
                ..Default::default()
            })
            .await
            .map_err(SetupError::Adapter)?;
        let info = adapter.get_info();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                required_limits: Limits::default().using_resolution(adapter.limits()),
                memory_hints: MemoryHints::MemoryUsage,
                ..Default::default()
            })
            .await
            .map_err(SetupError::Device)?;
        Ok((
            Self {
                adapter,
                device,
                queue,
            },
            info,
        ))
    }

    pub const fn device(&self) -> &Device {
        &self.device
    }

    pub const fn queue(&self) -> &Queue {
        &self.queue
    }

    pub const fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub(crate) fn draw(&self, target: &TextureView, clear: Color, render: &impl Render) {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(clear),
                        store: StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            render.draw(&mut pass);
        }
        self.queue.submit([encoder.finish()]);
    }
}

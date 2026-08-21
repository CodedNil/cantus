use ash::{Entry, Instance, khr, vk};
use core::{error::Error, fmt};
use std::{
    ffi::CStr,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use super::surface::Surface;

#[derive(Debug)]
pub enum SetupError {
    Loader(ash::LoadingError),
    Vulkan(vk::Result),
    UnsupportedSurface,
    IncompatibleSurface,
    InvalidShader,
    MissingFeature(&'static str),
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Loader(error) => error.fmt(f),
            Self::Vulkan(error) => write!(f, "Vulkan error: {error:?}"),
            Self::UnsupportedSurface => f.write_str("surface is unsupported"),
            Self::IncompatibleSurface => f.write_str("replacement surface is incompatible"),
            Self::InvalidShader => f.write_str("shader is not valid SPIR-V"),
            Self::MissingFeature(feature) => write!(f, "Vulkan device is missing required feature `{feature}`"),
        }
    }
}

impl Error for SetupError {}
impl From<vk::Result> for SetupError {
    fn from(value: vk::Result) -> Self {
        Self::Vulkan(value)
    }
}

pub struct Inner {
    pub entry: Entry,
    pub instance: Instance,
    pub surface_loader: khr::surface::Instance,
    pub wayland_loader: khr::wayland_surface::Instance,
    pub physical: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_family: u32,
    pub queue: vk::Queue,
    pub command_pool: vk::CommandPool,
    pub descriptor_pool: vk::DescriptorPool,
    pub sampler: vk::Sampler,
    pub timeline: vk::Semaphore,
    next_timeline: AtomicU64,
    pub properties: vk::PhysicalDeviceProperties,
    pub memory: vk::PhysicalDeviceMemoryProperties,
}

pub struct Context(pub(crate) Arc<Inner>);

impl Context {
    /// Creates a Vulkan 1.3 device capable of presenting to `surface`.
    ///
    /// # Errors
    /// Returns an error when Vulkan loading, instance/device creation, or
    /// graphics/presentation queue selection fails.
    pub fn new(surface: &Surface) -> Result<Self, SetupError> {
        let entry = unsafe { Entry::load() }.map_err(SetupError::Loader)?;
        let name = c"isthmus";
        let app = vk::ApplicationInfo::default().application_name(name).engine_name(name).api_version(vk::API_VERSION_1_3);
        let extensions = [khr::surface::NAME.as_ptr(), khr::wayland_surface::NAME.as_ptr()];
        let create = vk::InstanceCreateInfo::default().application_info(&app).enabled_extension_names(&extensions);
        let instance = unsafe { entry.create_instance(&create, None) }?;
        let surface_loader = khr::surface::Instance::new(&entry, &instance);
        let wayland_loader = khr::wayland_surface::Instance::new(&entry, &instance);
        let raw_surface = unsafe { wayland_loader.create_wayland_surface(&surface.create_info(), None) }?;
        let selected = unsafe { instance.enumerate_physical_devices() }?.into_iter().find_map(|physical| {
            let properties = unsafe { instance.get_physical_device_properties(physical) };
            if properties.api_version < vk::API_VERSION_1_3 {
                return None;
            }
            unsafe { instance.get_physical_device_queue_family_properties(physical) }
                .iter()
                .enumerate()
                .find_map(|(index, family)| {
                    let graphics = family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                    let present = unsafe { surface_loader.get_physical_device_surface_support(physical, index as u32, raw_surface) }.unwrap_or(false);
                    (graphics && present).then_some((physical, index as u32, properties))
                })
        });
        unsafe { surface_loader.destroy_surface(raw_surface, None) };
        let (physical, queue_family, properties) = selected.ok_or(SetupError::UnsupportedSurface)?;
        let priorities = [1.0];
        let queue_info = [vk::DeviceQueueCreateInfo::default().queue_family_index(queue_family).queue_priorities(&priorities)];
        let device_extensions = [khr::swapchain::NAME.as_ptr()];
        let mut supported12 = vk::PhysicalDeviceVulkan12Features::default();
        let mut supported13 = vk::PhysicalDeviceVulkan13Features::default();
        let mut supported = vk::PhysicalDeviceFeatures2::default().push_next(&mut supported12).push_next(&mut supported13);
        unsafe { instance.get_physical_device_features2(physical, &mut supported) };
        for (enabled, name) in [
            (supported12.timeline_semaphore != 0, "timelineSemaphore"),
            (supported12.scalar_block_layout != 0, "scalarBlockLayout"),
            (supported13.dynamic_rendering != 0, "dynamicRendering"),
            (supported13.synchronization2 != 0, "synchronization2"),
        ] {
            if !enabled {
                return Err(SetupError::MissingFeature(name));
            }
        }
        let mut features12 = vk::PhysicalDeviceVulkan12Features::default().timeline_semaphore(true).scalar_block_layout(true);
        let mut features13 = vk::PhysicalDeviceVulkan13Features::default().dynamic_rendering(true).synchronization2(true);
        let device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&device_extensions)
            .push_next(&mut features12)
            .push_next(&mut features13);
        let device = unsafe { instance.create_device(physical, &device_info, None) }?;
        let queue = unsafe { device.get_device_queue(queue_family, 0) };
        let command_pool = unsafe {
            device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .queue_family_index(queue_family)
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
                None,
            )
        }?;
        let descriptor_sizes = [
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: 128,
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::SAMPLED_IMAGE,
                descriptor_count: 64,
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::SAMPLER,
                descriptor_count: 64,
            },
        ];
        let descriptor_pool = unsafe {
            device.create_descriptor_pool(
                &vk::DescriptorPoolCreateInfo::default()
                    .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
                    .max_sets(64)
                    .pool_sizes(&descriptor_sizes),
                None,
            )
        }?;
        let mut timeline_type = vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::TIMELINE).initial_value(0);
        let timeline = unsafe { device.create_semaphore(&vk::SemaphoreCreateInfo::default().push_next(&mut timeline_type), None) }?;
        let sampler = unsafe {
            device.create_sampler(
                &vk::SamplerCreateInfo::default()
                    .mag_filter(vk::Filter::LINEAR)
                    .min_filter(vk::Filter::LINEAR)
                    .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
                    .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE),
                None,
            )
        }?;
        let memory = unsafe { instance.get_physical_device_memory_properties(physical) };
        Ok(Self(Arc::new(Inner {
            entry,
            instance,
            surface_loader,
            wayland_loader,
            physical,
            device,
            queue_family,
            queue,
            command_pool,
            descriptor_pool,
            sampler,
            timeline,
            next_timeline: AtomicU64::new(1),
            properties,
            memory,
        })))
    }

    pub(crate) fn inner(&self) -> Arc<Inner> {
        self.0.clone()
    }
    pub fn device_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.properties.device_name.as_ptr()) }
            .to_str()
            .unwrap_or("unknown Vulkan device")
    }
    pub(crate) fn memory_type(&self, bits: u32, flags: vk::MemoryPropertyFlags) -> Result<u32, SetupError> {
        (0..self.0.memory.memory_type_count)
            .find(|index| bits & (1 << index) != 0 && self.0.memory.memory_types[*index as usize].property_flags.contains(flags))
            .ok_or(SetupError::UnsupportedSurface)
    }
    pub(crate) fn immediate(&self, record: impl FnOnce(vk::CommandBuffer)) -> Result<(), SetupError> {
        let inner = &self.0;
        let buffers = unsafe {
            inner.device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(inner.command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1),
            )
        }?;
        let command = buffers[0];
        unsafe {
            inner
                .device
                .begin_command_buffer(command, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT))
        }?;
        record(command);
        unsafe {
            inner.device.end_command_buffer(command)?;
            let value = inner.reserve_timeline();
            let command_info = [vk::CommandBufferSubmitInfo::default().command_buffer(command)];
            let signal = [vk::SemaphoreSubmitInfo::default()
                .semaphore(inner.timeline)
                .value(value)
                .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];
            inner.device.queue_submit2(
                inner.queue,
                &[vk::SubmitInfo2::default().command_buffer_infos(&command_info).signal_semaphore_infos(&signal)],
                vk::Fence::null(),
            )?;
            inner.wait_timeline(value)?;
            inner.device.free_command_buffers(inner.command_pool, &[command]);
        }
        Ok(())
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            self.device.destroy_sampler(self.sampler, None);
            self.device.destroy_descriptor_pool(self.descriptor_pool, None);
            self.device.destroy_semaphore(self.timeline, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl Inner {
    pub(crate) fn reserve_timeline(&self) -> u64 {
        self.next_timeline.fetch_add(1, Ordering::Relaxed)
    }

    pub(crate) fn wait_timeline(&self, value: u64) -> Result<(), vk::Result> {
        if value == 0 {
            return Ok(());
        }
        unsafe {
            self.device
                .wait_semaphores(&vk::SemaphoreWaitInfo::default().semaphores(&[self.timeline]).values(&[value]), u64::MAX)
        }
    }
}

pub(crate) unsafe fn transition_image(device: &ash::Device, command: vk::CommandBuffer, image: vk::Image, old: vk::ImageLayout, new: vk::ImageLayout, layers: u32) {
    let (src_stage, src_access) = match old {
        vk::ImageLayout::UNDEFINED | vk::ImageLayout::PRESENT_SRC_KHR => (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL => (vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT, vk::AccessFlags2::COLOR_ATTACHMENT_WRITE),
        vk::ImageLayout::TRANSFER_DST_OPTIMAL => (vk::PipelineStageFlags2::TRANSFER, vk::AccessFlags2::TRANSFER_WRITE),
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL => (vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::AccessFlags2::SHADER_SAMPLED_READ),
        _ => (vk::PipelineStageFlags2::ALL_COMMANDS, vk::AccessFlags2::MEMORY_READ | vk::AccessFlags2::MEMORY_WRITE),
    };
    let (dst_stage, dst_access) = match new {
        vk::ImageLayout::PRESENT_SRC_KHR => (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL => (vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT, vk::AccessFlags2::COLOR_ATTACHMENT_WRITE),
        vk::ImageLayout::TRANSFER_DST_OPTIMAL => (vk::PipelineStageFlags2::TRANSFER, vk::AccessFlags2::TRANSFER_WRITE),
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL => (vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::AccessFlags2::SHADER_SAMPLED_READ),
        _ => (vk::PipelineStageFlags2::ALL_COMMANDS, vk::AccessFlags2::MEMORY_READ | vk::AccessFlags2::MEMORY_WRITE),
    };
    let barrier = [vk::ImageMemoryBarrier2::default()
        .src_stage_mask(src_stage)
        .src_access_mask(src_access)
        .dst_stage_mask(dst_stage)
        .dst_access_mask(dst_access)
        .old_layout(old)
        .new_layout(new)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            level_count: 1,
            layer_count: layers,
            ..Default::default()
        })];
    unsafe { device.cmd_pipeline_barrier2(command, &vk::DependencyInfo::default().image_memory_barriers(&barrier)) };
}

pub trait Render {
    fn prepare(&mut self);
    fn draw(&self, frame: &RenderFrame<'_>);
}

pub struct RenderFrame<'a> {
    command: vk::CommandBuffer,
    shared: &'a [u8],
}

impl<'a> RenderFrame<'a> {
    pub(crate) const fn new(command: vk::CommandBuffer, shared: &'a [u8]) -> Self {
        Self { command, shared }
    }

    pub(crate) const fn command(&self) -> vk::CommandBuffer {
        self.command
    }

    pub(crate) const fn shared(&self) -> &[u8] {
        self.shared
    }

    pub fn draw(&self, render: &impl Render) {
        render.draw(self);
    }
}

impl<T: Render> Render for Option<T> {
    fn prepare(&mut self) {
        if let Some(value) = self {
            value.prepare();
        }
    }
    fn draw(&self, frame: &RenderFrame<'_>) {
        if let Some(value) = self {
            value.draw(frame);
        }
    }
}

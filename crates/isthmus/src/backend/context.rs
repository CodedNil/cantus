use crate::data::ShaderData;
use ash::{Entry, Instance, khr, vk};
use core::{error::Error, fmt, mem};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    rc::Rc,
    vec::Vec,
};

use super::{
    buffer::{BufferRange, StaticBuffer, UploadRing},
    image::{ImageAllocator, ImageUpload},
    surface::create_surface,
};

#[derive(Debug)]
pub enum SetupError {
    Loader(ash::LoadingError),
    Handle(raw_window_handle::HandleError),
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
            Self::Handle(error) => error.fmt(f),
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

impl From<raw_window_handle::HandleError> for SetupError {
    fn from(value: raw_window_handle::HandleError) -> Self {
        Self::Handle(value)
    }
}

pub(super) struct Inner {
    pub(super) entry: Entry,
    pub(super) instance: Instance,
    pub(super) surface_loader: khr::surface::Instance,
    pub(super) swapchain_loader: khr::swapchain::Device,
    pub(super) physical: vk::PhysicalDevice,
    pub(super) device: ash::Device,
    pub(super) queue: vk::Queue,
    pub(super) queue_family: u32,
    pub(super) command_pool: vk::CommandPool,
    pub(super) push_descriptors: khr::push_descriptor::Device,
    pub(super) sampler: vk::Sampler,
    pub(super) timeline: vk::Semaphore,
    next_timeline: Cell<u64>,
    pub(super) properties: vk::PhysicalDeviceProperties,
    pub(super) memory: vk::PhysicalDeviceMemoryProperties,
    uploads: RefCell<UploadRing>,
    static_buffer: RefCell<StaticBuffer>,
    image_memory: RefCell<ImageAllocator>,
    image_uploads: RefCell<Vec<ImageUpload>>,
}

#[derive(Clone)]
pub struct Context(pub(super) Rc<Inner>);

impl Context {
    /// Creates a Vulkan 1.4 device capable of presenting to `surface`.
    ///
    /// # Errors
    /// Returns an error when Vulkan loading, instance/device creation, or
    /// graphics/presentation queue selection fails.
    pub(super) fn new(surface: &(impl HasDisplayHandle + HasWindowHandle)) -> Result<(Self, vk::SurfaceKHR), SetupError> {
        let entry = unsafe { Entry::load() }.map_err(SetupError::Loader)?;
        let name = c"isthmus";
        let api_version = vk::make_api_version(0, 1, 4, 0);
        let app = vk::ApplicationInfo::default().application_name(name).engine_name(name).api_version(api_version);
        let extensions = ash_window::enumerate_required_extensions(surface.display_handle()?.as_raw())?;
        let create = vk::InstanceCreateInfo::default().application_info(&app).enabled_extension_names(extensions);
        let instance = unsafe { entry.create_instance(&create, None) }?;
        let surface_loader = khr::surface::Instance::new(&entry, &instance);
        let raw_surface = create_surface(surface, &entry, &instance)?;
        let (physical, queue_family, properties) = select_physical(&instance, &surface_loader, raw_surface, api_version)?;
        let device = create_device(&instance, physical, queue_family)?;
        let push_descriptors = khr::push_descriptor::Device::new(&instance, &device);
        let swapchain_loader = khr::swapchain::Device::new(&instance, &device);
        let queue = unsafe { device.get_device_queue(queue_family, 0) };
        let command_pool = unsafe {
            device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .queue_family_index(queue_family)
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
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
        let uploads = RefCell::new(UploadRing::new(device.clone(), &memory, properties.limits.min_storage_buffer_offset_alignment as usize)?);
        let static_buffer = RefCell::new(StaticBuffer::new(device.clone(), &memory, properties.limits.min_storage_buffer_offset_alignment as usize)?);
        Ok((
            Self(Rc::new(Inner {
                entry,
                instance,
                surface_loader,
                swapchain_loader,
                physical,
                device,
                queue,
                queue_family,
                command_pool,
                push_descriptors,
                sampler,
                timeline,
                next_timeline: Cell::new(1),
                properties,
                memory,
                uploads,
                static_buffer,
                image_memory: RefCell::new(ImageAllocator::default()),
                image_uploads: RefCell::new(Vec::new()),
            })),
            raw_surface,
        ))
    }

    pub(super) fn device_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.properties.device_name.as_ptr()) }
            .to_str()
            .unwrap_or("unknown Vulkan device")
    }
    pub(super) fn memory_type(&self, bits: u32, flags: vk::MemoryPropertyFlags) -> Result<u32, SetupError> {
        (0..self.0.memory.memory_type_count)
            .find(|index| bits & (1 << index) != 0 && self.0.memory.memory_types[*index as usize].property_flags.contains(flags))
            .ok_or(SetupError::UnsupportedSurface)
    }
    pub(super) fn allocate_image_memory(&self, kind: u32, requirements: vk::MemoryRequirements) -> Result<(vk::DeviceMemory, u64), SetupError> {
        self.0.image_memory.borrow_mut().allocate(&self.0.device, kind, requirements).map_err(SetupError::Vulkan)
    }
    pub(super) fn begin_uploads(&self) -> Result<(), SetupError> {
        let timeline = self.0.uploads.borrow_mut().next();
        self.0.wait_timeline(timeline)?;
        Ok(())
    }

    pub(crate) fn upload<T: ShaderData>(&self, values: &[T]) -> BufferRange {
        self.0.uploads.borrow_mut().write(values)
    }

    pub(super) fn upload_bytes(&self, values: &[u8]) -> BufferRange {
        self.0.uploads.borrow_mut().write_bytes(values, 4)
    }

    pub(super) fn queue_image(&self, upload: ImageUpload) {
        self.0.image_uploads.borrow_mut().push(upload);
    }

    pub(super) fn record_uploads(&self, command: vk::CommandBuffer) {
        self.0.static_buffer.borrow_mut().record(command, &mut self.0.uploads.borrow_mut());
        let mut uploads = mem::take(&mut *self.0.image_uploads.borrow_mut());
        uploads.sort_by_key(|upload| upload.image.as_ptr() as usize);
        for group in uploads.chunk_by(|left, right| left.image.ptr_eq(&right.image)) {
            if let Some(image) = group[0].image.upgrade() {
                transition_image(
                    &self.0.device,
                    command,
                    image.raw,
                    if image.initialized.replace(true) {
                        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                    } else {
                        vk::ImageLayout::UNDEFINED
                    },
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    image.layers,
                );
                let mut copies = Vec::with_capacity(group.len());
                let mut buffer = vk::Buffer::null();
                for upload in group {
                    let staging = self.upload_bytes(upload.pixels.as_ref());
                    buffer = staging.raw;
                    copies.push(
                        vk::BufferImageCopy::default()
                            .buffer_offset(staging.offset)
                            .image_subresource(vk::ImageSubresourceLayers {
                                aspect_mask: vk::ImageAspectFlags::COLOR,
                                mip_level: 0,
                                base_array_layer: upload.origin[2],
                                layer_count: 1,
                            })
                            .image_offset(vk::Offset3D {
                                x: upload.origin[0] as i32,
                                y: upload.origin[1] as i32,
                                z: 0,
                            })
                            .image_extent(vk::Extent3D {
                                width: upload.size[0],
                                height: upload.size[1],
                                depth: 1,
                            }),
                    );
                }
                unsafe {
                    self.0
                        .device
                        .cmd_copy_buffer_to_image(command, buffer, image.raw, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &copies);
                    transition_image(
                        &self.0.device,
                        command,
                        image.raw,
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                        image.layers,
                    );
                }
            }
        }
        uploads.clear();
        *self.0.image_uploads.borrow_mut() = uploads;
    }

    pub(crate) fn upload_static<T: ShaderData>(&self, values: &[T]) -> BufferRange {
        self.0.static_buffer.borrow_mut().write(values)
    }
}

fn select_physical(
    instance: &Instance,
    surface_loader: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
    api_version: u32,
) -> Result<(vk::PhysicalDevice, u32, vk::PhysicalDeviceProperties), SetupError> {
    unsafe { instance.enumerate_physical_devices() }?
        .into_iter()
        .find_map(|physical| {
            let properties = unsafe { instance.get_physical_device_properties(physical) };
            if properties.api_version < api_version {
                return None;
            }
            unsafe { instance.get_physical_device_queue_family_properties(physical) }
                .iter()
                .enumerate()
                .find_map(|(index, family)| {
                    let graphics = family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                    let present = unsafe { surface_loader.get_physical_device_surface_support(physical, index as u32, surface) }.unwrap_or(false);
                    (graphics && present).then_some((physical, index as u32, properties))
                })
        })
        .ok_or(SetupError::UnsupportedSurface)
}

fn create_device(instance: &Instance, physical: vk::PhysicalDevice, queue_family: u32) -> Result<ash::Device, SetupError> {
    let priorities = [1.0];
    let queue_info = [vk::DeviceQueueCreateInfo::default().queue_family_index(queue_family).queue_priorities(&priorities)];
    let extensions = unsafe { instance.enumerate_device_extension_properties(physical) }?;
    if !extensions
        .iter()
        .any(|extension| unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) } == khr::push_descriptor::NAME)
    {
        return Err(SetupError::MissingFeature("VK_KHR_push_descriptor"));
    }
    let device_extensions = [khr::swapchain::NAME.as_ptr(), khr::push_descriptor::NAME.as_ptr()];
    let mut supported12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut supported13 = vk::PhysicalDeviceVulkan13Features::default();
    let mut supported_maintenance5 = vk::PhysicalDeviceMaintenance5FeaturesKHR::default();
    let mut supported = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut supported12)
        .push_next(&mut supported13)
        .push_next(&mut supported_maintenance5);
    unsafe { instance.get_physical_device_features2(physical, &mut supported) };
    for (enabled, name) in [
        (supported12.timeline_semaphore != 0, "timelineSemaphore"),
        (supported12.scalar_block_layout != 0, "scalarBlockLayout"),
        (supported12.runtime_descriptor_array != 0, "runtimeDescriptorArray"),
        (
            supported12.shader_sampled_image_array_non_uniform_indexing != 0,
            "shaderSampledImageArrayNonUniformIndexing",
        ),
        (supported12.descriptor_binding_partially_bound != 0, "descriptorBindingPartiallyBound"),
        (supported13.dynamic_rendering != 0, "dynamicRendering"),
        (supported13.synchronization2 != 0, "synchronization2"),
        (supported_maintenance5.maintenance5 != 0, "maintenance5"),
    ] {
        if !enabled {
            return Err(SetupError::MissingFeature(name));
        }
    }
    let mut features12 = vk::PhysicalDeviceVulkan12Features::default()
        .timeline_semaphore(true)
        .scalar_block_layout(true)
        .runtime_descriptor_array(true)
        .shader_sampled_image_array_non_uniform_indexing(true)
        .descriptor_binding_partially_bound(true);
    let mut features13 = vk::PhysicalDeviceVulkan13Features::default().dynamic_rendering(true).synchronization2(true);
    let mut maintenance5 = vk::PhysicalDeviceMaintenance5FeaturesKHR::default().maintenance5(true);
    let device_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&device_extensions)
        .push_next(&mut features12)
        .push_next(&mut features13)
        .push_next(&mut maintenance5);
    Ok(unsafe { instance.create_device(physical, &device_info, None) }?)
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            self.device.destroy_sampler(self.sampler, None);
            self.device.destroy_semaphore(self.timeline, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.uploads.get_mut().destroy();
            self.static_buffer.get_mut().destroy();
            self.image_memory.get_mut().destroy(&self.device);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl Inner {
    pub(super) fn reserve_timeline(&self) -> u64 {
        let timeline = self.next_timeline.get();
        self.next_timeline.set(timeline + 1);
        timeline
    }

    pub(super) fn uploads_submitted(&self, timeline: u64) {
        self.uploads.borrow_mut().submitted(timeline);
    }

    pub(super) fn wait_timeline(&self, value: u64) -> Result<(), vk::Result> {
        if value == 0 {
            return Ok(());
        }
        unsafe {
            self.device
                .wait_semaphores(&vk::SemaphoreWaitInfo::default().semaphores(&[self.timeline]).values(&[value]), u64::MAX)
        }
    }
}

pub(super) fn transition_image(device: &ash::Device, command: vk::CommandBuffer, image: vk::Image, old: vk::ImageLayout, new: vk::ImageLayout, layers: u32) {
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

pub(super) struct RenderFrame<'a> {
    pub(super) command: vk::CommandBuffer,
    pub(super) shared: &'a [u8],
}

impl<'a> RenderFrame<'a> {
    pub(crate) const fn new(command: vk::CommandBuffer, shared: &'a [u8]) -> Self {
        Self { command, shared }
    }
}

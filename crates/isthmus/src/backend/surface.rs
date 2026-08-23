use ash::{Entry, Instance, vk};
use core::array::from_fn;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{rc::Rc, vec, vec::Vec};

use super::context::{Context, Inner, SetupError};

pub(super) fn create_surface(source: &(impl HasDisplayHandle + HasWindowHandle), entry: &Entry, instance: &Instance) -> Result<vk::SurfaceKHR, SetupError> {
    Ok(unsafe { ash_window::create_surface(entry, instance, source.display_handle()?.as_raw(), source.window_handle()?.as_raw(), None) }?)
}

pub(super) enum Present {
    Rendered,
    Unavailable,
    Lost,
    Validation,
}

pub(super) struct SurfaceFrame {
    pub(super) image: vk::Image,
    pub(super) view: vk::ImageView,
    pub(super) index: u32,
    pub(super) command: vk::CommandBuffer,
    pub(super) acquire: vk::Semaphore,
    pub(super) complete: vk::Semaphore,
    pub(super) old_layout: vk::ImageLayout,
    slot: usize,
}

struct FrameSlot {
    command: vk::CommandBuffer,
    acquire: vk::Semaphore,
    complete: vk::Semaphore,
    timeline: u64,
}

pub(super) struct SurfaceTarget {
    inner: Rc<Inner>,
    surface: vk::SurfaceKHR,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    views: Vec<vk::ImageView>,
    initialized: Vec<bool>,
    pub(super) format: vk::Format,
    pub(super) extent: vk::Extent2D,
    frames: [FrameSlot; 2],
    next_frame: usize,
}

impl SurfaceTarget {
    pub(crate) fn new(context: &Context, surface: &(impl HasDisplayHandle + HasWindowHandle), width: u32, height: u32) -> Result<Self, SetupError> {
        let surface = create_surface(surface, &context.0.entry, &context.0.instance)?;
        Self::from_raw(context, surface, width, height)
    }

    pub(super) fn from_raw(context: &Context, surface: vk::SurfaceKHR, width: u32, height: u32) -> Result<Self, SetupError> {
        let inner = context.0.clone();
        let supported = unsafe { inner.surface_loader.get_physical_device_surface_support(inner.physical, inner.queue_family, surface) }?;
        if !supported {
            unsafe { inner.surface_loader.destroy_surface(surface, None) };
            return Err(SetupError::IncompatibleSurface);
        }
        let commands = unsafe {
            inner.device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(inner.command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(2),
            )
        }?;
        let frames = from_fn(|index| FrameSlot {
            command: commands[index],
            acquire: unsafe { inner.device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None) }.expect("create acquire semaphore"),
            complete: unsafe { inner.device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None) }.expect("create completion semaphore"),
            timeline: 0,
        });
        let mut target = Self {
            inner,
            surface,
            swapchain: vk::SwapchainKHR::null(),
            images: Vec::new(),
            views: Vec::new(),
            initialized: Vec::new(),
            format: vk::Format::UNDEFINED,
            extent: vk::Extent2D::default(),
            frames,
            next_frame: 0,
        };
        target.configure(width, height)?;
        Ok(target)
    }

    pub(super) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 && self.extent != (vk::Extent2D { width, height }) {
            let _ = self.configure(width, height);
        }
    }
    fn configure(&mut self, width: u32, height: u32) -> Result<(), SetupError> {
        unsafe {
            self.inner.device.queue_wait_idle(self.inner.queue)?;
        }
        let caps = unsafe { self.inner.surface_loader.get_physical_device_surface_capabilities(self.inner.physical, self.surface) }?;
        let formats = unsafe { self.inner.surface_loader.get_physical_device_surface_formats(self.inner.physical, self.surface) }?;
        let chosen = formats
            .iter()
            .copied()
            .find(|format| format.format == vk::Format::B8G8R8A8_UNORM)
            .or_else(|| formats.iter().copied().find(|format| format.format == vk::Format::R8G8B8A8_UNORM))
            .or_else(|| formats.first().copied())
            .ok_or(SetupError::UnsupportedSurface)?;
        let extent = if caps.current_extent.width == u32::MAX {
            vk::Extent2D {
                width: width.clamp(caps.min_image_extent.width, caps.max_image_extent.width),
                height: height.clamp(caps.min_image_extent.height, caps.max_image_extent.height),
            }
        } else {
            caps.current_extent
        };
        let count = (caps.min_image_count + 1).min(if caps.max_image_count == 0 { u32::MAX } else { caps.max_image_count });
        let alpha = [
            vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
            vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
            vk::CompositeAlphaFlagsKHR::OPAQUE,
        ]
        .into_iter()
        .find(|mode| caps.supported_composite_alpha.contains(*mode))
        .ok_or(SetupError::UnsupportedSurface)?;
        let old = self.swapchain;
        let info = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface)
            .min_image_count(count)
            .image_format(chosen.format)
            .image_color_space(chosen.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(caps.current_transform)
            .composite_alpha(alpha)
            .present_mode(vk::PresentModeKHR::FIFO)
            .clipped(true)
            .old_swapchain(old);
        let swapchain = unsafe { self.inner.swapchain_loader.create_swapchain(&info, None) }?;
        let images = unsafe { self.inner.swapchain_loader.get_swapchain_images(swapchain) }?;
        let views = images
            .iter()
            .map(|image| unsafe {
                self.inner.device.create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(*image)
                        .view_type(vk::ImageViewType::TYPE_2D)
                        .format(chosen.format)
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            level_count: 1,
                            layer_count: 1,
                            ..Default::default()
                        }),
                    None,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        unsafe {
            for view in self.views.drain(..) {
                self.inner.device.destroy_image_view(view, None);
            }
            if old != vk::SwapchainKHR::null() {
                self.inner.swapchain_loader.destroy_swapchain(old, None);
            }
        }
        self.swapchain = swapchain;
        self.images = images;
        self.views = views;
        self.initialized = vec![false; self.images.len()];
        self.format = chosen.format;
        self.extent = extent;
        Ok(())
    }

    pub(crate) fn acquire(&mut self) -> Result<SurfaceFrame, Present> {
        let slot_index = self.next_frame;
        self.next_frame = (self.next_frame + 1) % self.frames.len();
        let slot = &self.frames[slot_index];
        if self.inner.wait_timeline(slot.timeline).is_err() {
            return Err(Present::Validation);
        }
        unsafe { self.inner.device.reset_command_buffer(slot.command, vk::CommandBufferResetFlags::empty()) }.map_err(|_| Present::Validation)?;
        match unsafe { self.inner.swapchain_loader.acquire_next_image(self.swapchain, u64::MAX, slot.acquire, vk::Fence::null()) } {
            Ok((index, _)) => Ok(SurfaceFrame {
                image: self.images[index as usize],
                view: self.views[index as usize],
                index,
                command: slot.command,
                acquire: slot.acquire,
                complete: slot.complete,
                old_layout: if self.initialized[index as usize] {
                    vk::ImageLayout::PRESENT_SRC_KHR
                } else {
                    vk::ImageLayout::UNDEFINED
                },
                slot: slot_index,
            }),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Err(Present::Unavailable),
            Err(vk::Result::ERROR_SURFACE_LOST_KHR) => Err(Present::Lost),
            Err(_) => Err(Present::Validation),
        }
    }
    pub(crate) fn submitted(&mut self, frame: &SurfaceFrame, timeline: u64) {
        self.frames[frame.slot].timeline = timeline;
        self.initialized[frame.index as usize] = true;
    }
    pub(crate) fn present(&self, frame: &SurfaceFrame) -> Present {
        let result = unsafe {
            self.inner.swapchain_loader.queue_present(
                self.inner.queue,
                &vk::PresentInfoKHR::default()
                    .wait_semaphores(&[frame.complete])
                    .swapchains(&[self.swapchain])
                    .image_indices(&[frame.index]),
            )
        };
        match result {
            Ok(_) => Present::Rendered,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Present::Unavailable,
            Err(vk::Result::ERROR_SURFACE_LOST_KHR) => Present::Lost,
            Err(_) => Present::Validation,
        }
    }
}

impl Drop for SurfaceTarget {
    fn drop(&mut self) {
        unsafe {
            let _ = self.inner.device.queue_wait_idle(self.inner.queue);
            for view in self.views.drain(..) {
                self.inner.device.destroy_image_view(view, None);
            }
            if self.swapchain != vk::SwapchainKHR::null() {
                self.inner.swapchain_loader.destroy_swapchain(self.swapchain, None);
            }
            for frame in &self.frames {
                self.inner.device.destroy_semaphore(frame.acquire, None);
                self.inner.device.destroy_semaphore(frame.complete, None);
            }
            let commands = self.frames.iter().map(|frame| frame.command).collect::<Vec<_>>();
            self.inner.device.free_command_buffers(self.inner.command_pool, &commands);
            self.inner.surface_loader.destroy_surface(self.surface, None);
        }
    }
}

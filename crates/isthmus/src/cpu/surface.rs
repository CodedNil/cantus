use ash::{khr, vk};
use core::{array::from_fn, ffi::c_void, ptr::NonNull};
use std::{sync::Arc, vec, vec::Vec};

use super::context::{Context, Inner, SetupError};

#[derive(Clone, Copy)]
pub struct Surface {
    display: NonNull<c_void>,
    surface: NonNull<c_void>,
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

impl Surface {
    /// Wraps live Wayland display and surface pointers for Vulkan WSI.
    ///
    /// # Safety
    /// Both pointers must remain valid until every [`SurfaceTarget`] created
    /// from this value has been dropped.
    pub const unsafe fn wayland(display: NonNull<c_void>, surface: NonNull<c_void>) -> Self {
        Self { display, surface }
    }
    pub(crate) fn create_info(&self) -> vk::WaylandSurfaceCreateInfoKHR<'static> {
        vk::WaylandSurfaceCreateInfoKHR::default()
            .display(self.display.as_ptr().cast())
            .surface(self.surface.as_ptr().cast())
    }
}

pub enum Present {
    Rendered,
    Unavailable,
    Lost,
    Validation,
}

pub(crate) struct SurfaceFrame {
    pub image: vk::Image,
    pub view: vk::ImageView,
    pub index: u32,
    pub command: vk::CommandBuffer,
    pub acquire: vk::Semaphore,
    pub complete: vk::Semaphore,
    pub old_layout: vk::ImageLayout,
    slot: usize,
}

struct FrameSlot {
    command: vk::CommandBuffer,
    acquire: vk::Semaphore,
    complete: vk::Semaphore,
    timeline: u64,
}

pub struct SurfaceTarget {
    inner: Arc<Inner>,
    loader: khr::swapchain::Device,
    surface: vk::SurfaceKHR,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    views: Vec<vk::ImageView>,
    initialized: Vec<bool>,
    format: vk::Format,
    extent: vk::Extent2D,
    frames: [FrameSlot; 3],
    next_frame: usize,
}

impl SurfaceTarget {
    pub(crate) fn new(context: &Context, surface: Surface, width: u32, height: u32) -> Result<Self, SetupError> {
        let inner = context.inner();
        let loader = khr::swapchain::Device::new(&inner.instance, &inner.device);
        let surface = unsafe { inner.wayland_loader.create_wayland_surface(&surface.create_info(), None) }?;
        let commands = unsafe {
            inner.device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(inner.command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(3),
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
            loader,
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

    pub const fn format(&self) -> vk::Format {
        self.format
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 && self.extent != (vk::Extent2D { width, height }) {
            let _ = self.configure(width, height);
        }
    }
    fn configure(&mut self, width: u32, height: u32) -> Result<(), SetupError> {
        unsafe {
            self.inner.device.device_wait_idle()?;
        }
        let caps = unsafe { self.inner.surface_loader.get_physical_device_surface_capabilities(self.inner.physical, self.surface) }?;
        let formats = unsafe { self.inner.surface_loader.get_physical_device_surface_formats(self.inner.physical, self.surface) }?;
        let chosen = formats
            .iter()
            .copied()
            .find(|format| matches!(format.format, vk::Format::B8G8R8A8_UNORM | vk::Format::R8G8B8A8_UNORM))
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
        let swapchain = unsafe { self.loader.create_swapchain(&info, None) }?;
        let images = unsafe { self.loader.get_swapchain_images(swapchain) }?;
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
                self.loader.destroy_swapchain(old, None);
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
        match unsafe { self.loader.acquire_next_image(self.swapchain, u64::MAX, slot.acquire, vk::Fence::null()) } {
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
    pub(crate) const fn extent(&self) -> vk::Extent2D {
        self.extent
    }
    pub(crate) fn submitted(&mut self, frame: &SurfaceFrame, timeline: u64) {
        self.frames[frame.slot].timeline = timeline;
        self.initialized[frame.index as usize] = true;
    }
    pub(crate) fn present(&self, frame: &SurfaceFrame) -> Present {
        let result = unsafe {
            self.loader.queue_present(
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
            let _ = self.inner.device.device_wait_idle();
            for view in self.views.drain(..) {
                self.inner.device.destroy_image_view(view, None);
            }
            if self.swapchain != vk::SwapchainKHR::null() {
                self.loader.destroy_swapchain(self.swapchain, None);
            }
            for frame in &self.frames {
                self.inner.device.destroy_semaphore(frame.acquire, None);
                self.inner.device.destroy_semaphore(frame.complete, None);
            }
            let commands: [vk::CommandBuffer; 3] = from_fn(|index| self.frames[index].command);
            self.inner.device.free_command_buffers(self.inner.command_pool, &commands);
            self.inner.surface_loader.destroy_surface(self.surface, None);
        }
    }
}

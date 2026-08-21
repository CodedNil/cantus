use super::context::{Context, Inner, SetupError};
use crate::data::ShaderData;
use ash::vk;
use core::{
    marker::PhantomData,
    mem::size_of,
    ptr::{self, NonNull},
};
use std::{rc::Rc, sync::Arc};

pub struct Buffer {
    inner: Arc<Inner>,
    raw: vk::Buffer,
    memory: vk::DeviceMemory,
    mapped: NonNull<u8>,
    pub size: usize,
}
impl Buffer {
    pub fn new(context: &Context, size: usize, usage: vk::BufferUsageFlags) -> Result<Self, SetupError> {
        let inner = context.inner();
        let raw = unsafe {
            inner.device.create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(size.max(4) as u64)
                    .usage(usage)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None,
            )
        }?;
        let requirements = unsafe { inner.device.get_buffer_memory_requirements(raw) };
        let kind = context.memory_type(
            requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;
        let memory = unsafe {
            inner
                .device
                .allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(kind), None)
        }?;
        unsafe {
            inner.device.bind_buffer_memory(raw, memory, 0)?;
        }
        let mapped = unsafe { inner.device.map_memory(memory, 0, requirements.size, vk::MemoryMapFlags::empty()) }?;
        Ok(Self {
            inner,
            raw,
            memory,
            mapped: NonNull::new(mapped.cast()).expect("Vulkan returned a null mapped pointer"),
            size: size.max(4),
        })
    }
    pub const fn raw(&self) -> vk::Buffer {
        self.raw
    }
    pub fn write(&self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }
        assert!(bytes.len() <= self.size, "buffer write exceeds allocation");
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), self.mapped.as_ptr(), bytes.len()) };
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.inner.device.unmap_memory(self.memory);
            self.inner.device.destroy_buffer(self.raw, None);
            self.inner.device.free_memory(self.memory, None);
        }
    }
}

pub struct DataBuffer<T> {
    context: Context,
    raw: Rc<Buffer>,
    marker: PhantomData<T>,
}
impl<T: ShaderData> DataBuffer<T> {
    pub fn new(context: &Context, _label: &str, capacity: usize) -> Self {
        Self {
            context: context.clone(),
            raw: Rc::new(Buffer::new(context, size_of::<T>() * capacity.max(1), vk::BufferUsageFlags::STORAGE_BUFFER).expect("create Vulkan storage buffer")),
            marker: PhantomData,
        }
    }
    pub fn upload(&self, data: &[T]) {
        self.raw.write(bytemuck::cast_slice(data));
    }
    pub fn grow(&mut self, capacity: usize) -> bool {
        if capacity * size_of::<T>() <= self.raw.size {
            return false;
        }
        let capacity = capacity.next_power_of_two();
        self.raw = Rc::new(Buffer::new(&self.context, size_of::<T>() * capacity, vk::BufferUsageFlags::STORAGE_BUFFER).expect("grow Vulkan storage buffer"));
        true
    }
    pub const fn raw(&self) -> &Rc<Buffer> {
        &self.raw
    }
}

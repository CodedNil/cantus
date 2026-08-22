use super::context::SetupError;
use crate::data::ShaderData;
use ash::vk;
use core::{cell::Cell, mem::align_of, ptr};
const STATIC_BYTES: usize = 4 * 1024 * 1024;
const FRAME_BYTES: usize = 3 * 1024 * 1024;
const FRAMES_IN_FLIGHT: usize = 2;

#[derive(Clone, Copy, Default)]
pub struct BufferRange {
    pub raw: vk::Buffer,
    pub offset: u64,
    pub size: u64,
}

pub trait BufferBinding {
    fn range(&self) -> BufferRange;
}

pub struct UploadRing {
    device: ash::Device,
    raw: vk::Buffer,
    memory: vk::DeviceMemory,
    mapped: usize,
    alignment: usize,
    slot: usize,
    static_cursor: usize,
    cursor: usize,
    timelines: [u64; FRAMES_IN_FLIGHT],
}

impl UploadRing {
    pub(super) fn new(device: ash::Device, memory: &vk::PhysicalDeviceMemoryProperties, alignment: usize) -> Result<Self, SetupError> {
        let size = STATIC_BYTES + FRAME_BYTES * FRAMES_IN_FLIGHT;
        let raw = unsafe {
            device.create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(size as u64)
                    .usage(vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None,
            )
        }?;
        let requirements = unsafe { device.get_buffer_memory_requirements(raw) };
        let kind = memory_type(
            memory,
            requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )
        .ok_or(SetupError::UnsupportedSurface)?;
        let allocation = unsafe { device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(kind), None) }?;
        unsafe { device.bind_buffer_memory(raw, allocation, 0) }?;
        let mapped = unsafe { device.map_memory(allocation, 0, requirements.size, vk::MemoryMapFlags::empty()) }? as usize;
        assert_ne!(mapped, 0, "Vulkan returned a null mapped pointer");
        Ok(Self {
            device,
            raw,
            memory: allocation,
            mapped,
            alignment,
            slot: FRAMES_IN_FLIGHT - 1,
            static_cursor: 0,
            cursor: 0,
            timelines: [0; FRAMES_IN_FLIGHT],
        })
    }

    pub(super) const fn next(&mut self) -> u64 {
        self.slot = (self.slot + 1) % FRAMES_IN_FLIGHT;
        self.cursor = 0;
        self.timelines[self.slot]
    }

    pub(super) const fn submitted(&mut self, timeline: u64) {
        self.timelines[self.slot] = timeline;
    }

    pub(super) fn write<T: ShaderData>(&mut self, values: &[T]) -> BufferRange {
        let bytes = bytemuck::cast_slice(values);
        self.write_bytes(bytes, align_of::<T>())
    }

    pub(super) fn write_bytes(&mut self, bytes: &[u8], alignment: usize) -> BufferRange {
        let size = bytes.len().max(4);
        let base = STATIC_BYTES + self.slot * FRAME_BYTES;
        let offset = self.cursor.next_multiple_of(alignment.max(self.alignment));
        assert!(offset + size <= FRAME_BYTES, "frame upload arena exhausted");
        if !bytes.is_empty() {
            unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), (self.mapped as *mut u8).add(base + offset), bytes.len()) };
        }
        self.cursor = offset + size;
        BufferRange {
            raw: self.raw,
            offset: (base + offset) as u64,
            size: size as u64,
        }
    }

    pub(super) fn write_static<T: ShaderData>(&mut self, values: &[T]) -> BufferRange {
        let bytes = bytemuck::cast_slice(values);
        let size = bytes.len().max(4);
        let offset = self.static_cursor.next_multiple_of(align_of::<T>().max(self.alignment));
        assert!(offset + size <= STATIC_BYTES, "static buffer arena exhausted");
        if !bytes.is_empty() {
            unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), (self.mapped as *mut u8).add(offset), bytes.len()) };
        }
        self.static_cursor = offset + size;
        BufferRange {
            raw: self.raw,
            offset: offset as u64,
            size: size as u64,
        }
    }

    pub(super) fn destroy(&mut self) {
        unsafe {
            self.device.unmap_memory(self.memory);
            self.device.destroy_buffer(self.raw, None);
            self.device.free_memory(self.memory, None);
        }
    }
}

fn memory_type(memory: &vk::PhysicalDeviceMemoryProperties, bits: u32, flags: vk::MemoryPropertyFlags) -> Option<u32> {
    (0..memory.memory_type_count)
        .filter(|index| bits & (1 << index) != 0 && memory.memory_types[*index as usize].property_flags.contains(flags))
        .max_by_key(|index| memory.memory_types[*index as usize].property_flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL))
}

pub struct StreamBinding(Cell<BufferRange>);

impl Default for StreamBinding {
    fn default() -> Self {
        Self(Cell::new(BufferRange::default()))
    }
}

impl StreamBinding {
    pub fn update(&self, range: BufferRange) {
        self.0.set(range);
    }
}

impl BufferBinding for StreamBinding {
    fn range(&self) -> BufferRange {
        self.0.get()
    }
}

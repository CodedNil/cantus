use super::context::SetupError;
use crate::data::ShaderData;
use ash::vk;
use core::{mem::align_of, ptr};
use std::vec::Vec;

const STATIC_BYTES: usize = 4 * 1024 * 1024;
const FRAME_BYTES: usize = 3 * 1024 * 1024;
const FRAMES_IN_FLIGHT: usize = 2;

#[derive(Clone, Copy, Default)]
pub struct BufferRange {
    pub raw: vk::Buffer,
    pub offset: u64,
    pub size: u64,
}

pub(super) struct UploadRing {
    device: ash::Device,
    raw: vk::Buffer,
    memory: vk::DeviceMemory,
    mapped: usize,
    alignment: usize,
    slot: usize,
    cursor: usize,
    timelines: [u64; FRAMES_IN_FLIGHT],
}

impl UploadRing {
    pub(super) fn new(device: ash::Device, memory: &vk::PhysicalDeviceMemoryProperties, alignment: usize) -> Result<Self, SetupError> {
        let size = FRAME_BYTES * FRAMES_IN_FLIGHT;
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
        let base = self.slot * FRAME_BYTES;
        let offset = self.cursor.next_multiple_of(alignment.max(self.alignment));
        assert!(offset + size <= FRAME_BYTES, "frame upload arena exhausted");
        self.cursor = offset + size;
        self.copy(bytes, base + offset, size)
    }

    const fn copy(&self, bytes: &[u8], offset: usize, size: usize) -> BufferRange {
        if !bytes.is_empty() {
            unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), (self.mapped as *mut u8).add(offset), bytes.len()) };
        }
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

pub(super) struct StaticBuffer {
    device: ash::Device,
    raw: vk::Buffer,
    memory: vk::DeviceMemory,
    alignment: usize,
    cursor: usize,
    pending: Vec<StaticUpload>,
}

struct StaticUpload {
    offset: u64,
    bytes: Vec<u8>,
}

impl StaticBuffer {
    pub(super) fn new(device: ash::Device, memory: &vk::PhysicalDeviceMemoryProperties, alignment: usize) -> Result<Self, SetupError> {
        let raw = unsafe {
            device.create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(STATIC_BYTES as u64)
                    .usage(vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None,
            )
        }?;
        let requirements = unsafe { device.get_buffer_memory_requirements(raw) };
        let kind = memory_type(memory, requirements.memory_type_bits, vk::MemoryPropertyFlags::DEVICE_LOCAL).ok_or(SetupError::UnsupportedSurface)?;
        let allocation = unsafe { device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(kind), None) }?;
        unsafe { device.bind_buffer_memory(raw, allocation, 0) }?;
        Ok(Self {
            device,
            raw,
            memory: allocation,
            alignment,
            cursor: 0,
            pending: Vec::new(),
        })
    }

    pub(super) fn write<T: ShaderData>(&mut self, values: &[T]) -> BufferRange {
        let bytes = bytemuck::cast_slice(values);
        let size = bytes.len().max(4);
        let offset = self.cursor.next_multiple_of(align_of::<T>().max(self.alignment));
        assert!(offset + size <= STATIC_BYTES, "static buffer arena exhausted");
        self.cursor = offset + size;
        self.pending.push(StaticUpload {
            offset: offset as u64,
            bytes: bytes.to_vec(),
        });
        BufferRange {
            raw: self.raw,
            offset: offset as u64,
            size: size as u64,
        }
    }

    pub(super) fn record(&mut self, command: vk::CommandBuffer, uploads: &mut UploadRing) {
        if self.pending.is_empty() {
            return;
        }
        for upload in self.pending.drain(..) {
            let staging = uploads.write_bytes(&upload.bytes, 4);
            unsafe {
                self.device.cmd_copy_buffer(
                    command,
                    staging.raw,
                    self.raw,
                    &[vk::BufferCopy::default().src_offset(staging.offset).dst_offset(upload.offset).size(staging.size)],
                );
            }
        }
        let barrier = [vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::ALL_GRAPHICS)
            .dst_access_mask(vk::AccessFlags2::SHADER_STORAGE_READ)
            .buffer(self.raw)
            .size(self.cursor as u64)];
        unsafe { self.device.cmd_pipeline_barrier2(command, &vk::DependencyInfo::default().buffer_memory_barriers(&barrier)) };
    }

    pub(super) fn destroy(&mut self) {
        unsafe {
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

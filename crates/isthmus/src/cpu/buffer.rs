use crate::BufferData;
use std::{string::String, vec::Vec};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

pub(super) fn storage<T: BufferData>(device: &Device, queue: &Queue, label: &str, data: &[T]) -> Buffer {
    let mut words = Vec::new();
    encode(data, &mut words);
    let raw = create::<T>(device, label, data.len());
    if !words.is_empty() {
        queue.write_buffer(&raw, 0, bytemuck::cast_slice(&words));
    }
    raw
}

fn create<T: BufferData>(device: &Device, label: &str, capacity: usize) -> Buffer {
    let () = T::ASSERT_LAYOUT;
    device.create_buffer(&BufferDescriptor {
        label: Some(label),
        size: (T::BUFFER_STRIDE * capacity.max(1)) as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn encode<T: BufferData>(data: &[T], words: &mut Vec<u32>) {
    words.clear();
    words.resize(data.len() * T::BUFFER_WORDS, 0);
    for (index, item) in data.iter().enumerate() {
        item.write(words, index);
    }
}

pub(super) struct DataBuffer {
    device: Device,
    queue: Queue,
    label: String,
    raw: Buffer,
    capacity: usize,
    words: Vec<u32>,
}

impl DataBuffer {
    pub fn new<T: BufferData>(device: &Device, queue: &Queue, label: &str, capacity: usize) -> Self {
        Self {
            device: device.clone(),
            queue: queue.clone(),
            label: label.into(),
            raw: create::<T>(device, label, capacity),
            capacity,
            words: Vec::with_capacity(T::BUFFER_WORDS * capacity),
        }
    }

    pub fn upload<T: BufferData>(&mut self, data: &[T]) {
        assert!(data.len() <= self.capacity, "buffer capacity exceeded");
        encode(data, &mut self.words);
        if !self.words.is_empty() {
            self.queue
                .write_buffer(&self.raw, 0, bytemuck::cast_slice(&self.words));
        }
    }

    pub fn grow<T: BufferData>(&mut self, capacity: usize) -> bool {
        if capacity <= self.capacity {
            return false;
        }
        self.capacity = capacity.next_power_of_two();
        self.raw = create::<T>(&self.device, &self.label, self.capacity);
        self.words
            .reserve((T::BUFFER_WORDS * self.capacity).saturating_sub(self.words.capacity()));
        true
    }

    pub const fn raw(&self) -> &Buffer {
        &self.raw
    }

    pub const fn device(&self) -> &Device {
        &self.device
    }
}

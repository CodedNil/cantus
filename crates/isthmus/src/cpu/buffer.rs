use crate::data::BufferData;
use core::marker::PhantomData;
use std::{string::String, vec::Vec};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

fn create<T: BufferData>(device: &Device, label: &str, capacity: usize) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: Some(label),
        size: (T::BUFFER_STRIDE * capacity.max(1)).max(4) as u64,
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

pub(super) struct DataBuffer<T> {
    device: Device,
    queue: Queue,
    label: String,
    raw: Buffer,
    words: Vec<u32>,
    marker: PhantomData<T>,
}

impl<T: BufferData> DataBuffer<T> {
    pub fn new(device: &Device, queue: &Queue, label: &str, capacity: usize) -> Self {
        Self {
            device: device.clone(),
            queue: queue.clone(),
            label: label.into(),
            raw: create::<T>(device, label, capacity),
            words: Vec::with_capacity(T::BUFFER_WORDS * capacity),
            marker: PhantomData,
        }
    }

    pub fn upload(&mut self, data: &[T]) {
        assert!(data.len() * T::BUFFER_STRIDE <= self.raw.size() as usize, "buffer capacity exceeded");
        encode(data, &mut self.words);
        if !self.words.is_empty() {
            self.queue.write_buffer(&self.raw, 0, bytemuck::cast_slice(&self.words));
        }
    }

    pub fn grow(&mut self, capacity: usize) -> bool {
        if capacity * T::BUFFER_STRIDE <= self.raw.size() as usize {
            return false;
        }
        let capacity = capacity.next_power_of_two();
        self.raw = create::<T>(&self.device, &self.label, capacity);
        self.words.reserve((T::BUFFER_WORDS * capacity).saturating_sub(self.words.capacity()));
        true
    }

    pub const fn raw(&self) -> &Buffer {
        &self.raw
    }

    pub const fn device(&self) -> &Device {
        &self.device
    }
}

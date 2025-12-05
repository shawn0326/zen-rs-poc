use crate::{BufferHandle, ResourceKey, Resources};
use slotmap::SecondaryMap;
use wgpu::util::DeviceExt;

pub struct InternalBuffer {
    buffer: wgpu::Buffer,
    ver: u64,
}

impl InternalBuffer {
    pub fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

pub struct Buffers {
    pool: SecondaryMap<ResourceKey, InternalBuffer>,
}

impl Buffers {
    pub fn new() -> Self {
        Self {
            pool: SecondaryMap::new(),
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        resources: &Resources,
        handle: &BufferHandle,
    ) -> &InternalBuffer {
        let entry = self
            .pool
            .entry(handle.raw())
            .expect("BufferHandle has been removed from resources.");

        let buffer = resources
            .get_buffer(handle)
            .expect("BufferHandle has been removed from resources.");

        let internal_buffer = entry.or_insert_with(|| {
            let wgpu_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: buffer.raw(),
                usage: buffer.usage(),
            });
            InternalBuffer {
                buffer: wgpu_buffer,
                ver: buffer.ver().as_u64(),
            }
        });

        if internal_buffer.ver != buffer.ver().as_u64() {
            queue.write_buffer(&internal_buffer.buffer, 0, buffer.raw());
            internal_buffer.ver = buffer.ver().as_u64();
        }

        internal_buffer
    }

    pub fn get_internal_buffer(&self, handle: &BufferHandle) -> &InternalBuffer {
        self.pool.get(handle.raw()).unwrap()
    }

    pub fn get_internal_buffer_by_key(&self, key: ResourceKey) -> &InternalBuffer {
        self.pool.get(key).unwrap()
    }

    pub fn destroy_internal_buffer(&mut self, key: ResourceKey) {
        self.pool.remove(key);
    }
}

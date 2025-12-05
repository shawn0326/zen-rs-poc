use crate::{BufferHandle, GeometryHandle, ResourceKey, Resources, Symbol, symbol};
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

    pub fn destroy_internal_buffer(&mut self, key: ResourceKey) {
        self.pool.remove(key);
    }

    pub fn set_buffers_to_render_pass(
        &self,
        resources: &Resources,
        render_pass: &mut wgpu::RenderPass,
        geometry_handle: &GeometryHandle,
    ) {
        const NAMES: [Symbol; 3] = [
            symbol!("positions"),
            symbol!("tex_coords"),
            symbol!("colors"),
        ];

        let geometry = resources
            .get_geometry(geometry_handle)
            .expect("GeometryHandle has been removed from resources.");

        let mut location = 0;

        for name in &NAMES {
            let attr = geometry.get_attribute(*name).unwrap();

            let buffer_handle = &attr.vertex_buffer.buffer_slice.buffer;

            let internal_buffer = self.get_internal_buffer(buffer_handle);

            render_pass.set_vertex_buffer(
                location,
                internal_buffer
                    .wgpu_buffer()
                    .slice(attr.vertex_buffer.buffer_slice.range_u64()),
            );

            location += 1;
        }

        if let Some(index_buffer) = geometry.indices() {
            let buffer_handle = &index_buffer.buffer_slice.buffer;

            let internal_buffer = self.get_internal_buffer(buffer_handle);

            render_pass.set_index_buffer(
                internal_buffer
                    .wgpu_buffer()
                    .slice(index_buffer.buffer_slice.range_u64()),
                index_buffer.format,
            );
        }
    }
}

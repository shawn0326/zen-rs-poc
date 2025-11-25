use crate::{BufferHandle, GeometryHandle, ResourceKey, Resources, Symbol};
use slotmap::SecondaryMap;
use wgpu::util::DeviceExt;

pub struct InnerBuffer {
    wgpu_buffer: wgpu::Buffer,
}

impl InnerBuffer {
    pub fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.wgpu_buffer
    }
}

pub struct Buffers {
    pool: SecondaryMap<ResourceKey, InnerBuffer>,
}

impl Buffers {
    pub fn new() -> Self {
        Self {
            pool: SecondaryMap::new(),
        }
    }

    pub fn prepare_inner_buffer(
        &mut self,
        device: &wgpu::Device,
        resources: &Resources,
        handle: &BufferHandle,
    ) -> &InnerBuffer {
        let entry = self
            .pool
            .entry(handle.raw())
            .expect("BufferHandle has been removed from resources.");

        let buffer = resources
            .get_buffer(handle)
            .expect("BufferHandle has been removed from resources.");

        entry.or_insert_with(|| {
            let wgpu_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: buffer.raw(),
                usage: buffer.usage(),
            });
            InnerBuffer { wgpu_buffer }
        })
    }

    pub fn get_inner_buffer(&self, handle: &BufferHandle) -> &InnerBuffer {
        self.pool.get(handle.raw()).unwrap()
    }

    pub fn destroy_inner_buffer(&mut self, key: ResourceKey) {
        self.pool.remove(key);
    }

    pub fn prepare_geometry_buffer(
        &mut self,
        device: &wgpu::Device,
        resources: &Resources,
        handle: &GeometryHandle,
    ) {
        const NAMES: [Symbol; 3] = [
            symbol!("positions"),
            symbol!("tex_coords"),
            symbol!("colors"),
        ];

        let geometry = resources
            .get_geometry(handle)
            .expect("GeometryHandle has been removed from resources.");

        for name in &NAMES {
            let attr = geometry.get_attribute(*name).unwrap();

            let buffer_handle = &attr.vertex_buffer.buffer_slice.buffer;

            self.prepare_inner_buffer(device, resources, buffer_handle);
        }

        if let Some(index_buffer) = geometry.indices() {
            let buffer_handle = &index_buffer.buffer_slice.buffer;

            self.prepare_inner_buffer(device, resources, buffer_handle);
        }
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

            let inner_buffer = self.get_inner_buffer(buffer_handle);

            render_pass.set_vertex_buffer(
                location,
                inner_buffer
                    .wgpu_buffer()
                    .slice(attr.vertex_buffer.buffer_slice.range_u64()),
            );

            location += 1;
        }

        if let Some(index_buffer) = geometry.indices() {
            let buffer_handle = &index_buffer.buffer_slice.buffer;

            let inner_buffer = self.get_inner_buffer(buffer_handle);

            render_pass.set_index_buffer(
                inner_buffer
                    .wgpu_buffer()
                    .slice(index_buffer.buffer_slice.range_u64()),
                index_buffer.format,
            );
        }
    }
}

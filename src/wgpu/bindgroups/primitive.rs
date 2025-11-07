use crate::math::Mat4;

pub(in super::super) struct GpuPrimitiveBindGroup {
    buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    matrix_capacity: usize,
}

impl GpuPrimitiveBindGroup {
    pub fn new(device: &wgpu::Device) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Primitive Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let initial_capacity = 10_000;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Primitive Buffer"),
            size: (initial_capacity as u64) * 64, // 64 bytes per Mat4
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Primitive Bind Group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            layout,
            bind_group,
            matrix_capacity: initial_capacity,
        }
    }

    pub fn ensure_capacity(&mut self, device: &wgpu::Device, needed: usize) {
        if needed <= self.matrix_capacity {
            return;
        }

        let mut new_cap = self.matrix_capacity;
        while new_cap < needed {
            new_cap *= 2;
        }
        let new_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Primitive Buffer (Resized)"),
            size: (new_cap as u64) * 64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let new_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Primitive Bind Group (Resized)"),
            layout: &self.layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: new_buffer.as_entire_binding(),
            }],
        });

        self.buffer = new_buffer;
        self.bind_group = new_bind_group;
        self.matrix_capacity = new_cap;
    }

    pub fn upload_matrices(&self, queue: &wgpu::Queue, matrices: &[Mat4]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(matrices));
    }
}

use crate::math::Mat4;

fn create_primitive_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Primitive Buffer"),
        size: (capacity as u64) * 64, // 64 bytes per Mat4
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_primitive_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Primitive Bind Group"),
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    })
}

pub(in super::super) struct PrimitiveBindGroup {
    capacity: usize,
    cpu_memory: Vec<Mat4>,
    gpu_buffer: wgpu::Buffer,
    gpu_layout: wgpu::BindGroupLayout,
    gpu_bind_group: wgpu::BindGroup,
}

impl PrimitiveBindGroup {
    pub fn new(device: &wgpu::Device, initial_capacity: usize) -> Self {
        let gpu_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let gpu_buffer = create_primitive_buffer(device, initial_capacity);
        let gpu_bind_group = create_primitive_bind_group(device, &gpu_layout, &gpu_buffer);

        Self {
            capacity: initial_capacity,
            cpu_memory: Vec::with_capacity(initial_capacity),
            gpu_buffer: gpu_buffer,
            gpu_layout,
            gpu_bind_group,
        }
    }

    pub fn prepare(&mut self, device: &wgpu::Device, needed: usize) -> &mut Self {
        self.cpu_memory.clear();

        if needed <= self.capacity {
            return self;
        }

        let mut new_cap = self.capacity;
        while new_cap < needed {
            new_cap *= 2;
        }

        self.cpu_memory.reserve(new_cap - self.capacity);

        let new_buffer = create_primitive_buffer(device, new_cap);
        let new_bind_group = create_primitive_bind_group(device, &self.gpu_layout, &new_buffer);

        self.capacity = new_cap;
        self.gpu_buffer = new_buffer;
        self.gpu_bind_group = new_bind_group;

        self
    }

    pub fn gpu_layout(&self) -> &wgpu::BindGroupLayout {
        &self.gpu_layout
    }

    pub fn gpu_bind_group(&self) -> &wgpu::BindGroup {
        &self.gpu_bind_group
    }

    pub fn push_data(&mut self, matrix: &Mat4) {
        self.cpu_memory.push(*matrix);
    }

    pub fn flush(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.gpu_buffer, 0, bytemuck::cast_slice(&self.cpu_memory));
    }
}

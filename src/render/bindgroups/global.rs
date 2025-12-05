use crate::camera::Camera;

pub struct GlobalBindGroup {
    gpu_buffer: wgpu::Buffer,
    gpu_layout: wgpu::BindGroupLayout,
    gpu_bind_group: wgpu::BindGroup,
}

impl GlobalBindGroup {
    pub fn new(device: &wgpu::Device) -> Self {
        let gpu_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let gpu_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Global Buffer"),
            size: 64_u64, // size of Mat4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let gpu_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &gpu_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: gpu_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            gpu_layout,
            gpu_buffer,
            gpu_bind_group,
        }
    }

    pub fn gpu_layout(&self) -> &wgpu::BindGroupLayout {
        &self.gpu_layout
    }

    pub fn gpu_bind_group(&self) -> &wgpu::BindGroup {
        &self.gpu_bind_group
    }

    pub fn upload(&self, queue: &wgpu::Queue, camera: &Camera) -> &Self {
        let data = camera.view_projection().to_cols_array_2d();
        queue.write_buffer(&self.gpu_buffer, 0, bytemuck::bytes_of(&data));
        self
    }
}

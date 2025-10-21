use crate::scene::Camera;
use wgpu::util::DeviceExt;

pub(in super::super) struct GpuGlobalBindGroup {
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl GpuGlobalBindGroup {
    pub fn new(device: &wgpu::Device) -> Self {
        let empty_data = glam::Mat4::IDENTITY.to_cols_array_2d();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&empty_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) -> &Self {
        let data = camera.build_view_projection_matrix().to_cols_array_2d();
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&data));
        self
    }
}

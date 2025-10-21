use super::super::textures::Textures;
use crate::graphics::Material;
use std::vec;
use wgpu::util::DeviceExt;

pub(in super::super) struct GpuMaterialBindGroup {
    // pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl GpuMaterialBindGroup {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        textures: &mut Textures,
        material: &Material,
    ) -> Self {
        let empty_data = glam::Vec3::ONE.to_array();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::bytes_of(&empty_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mut layout_entries = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }];

        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }];

        if let Some(texture) = material.texture() {
            let gpu_texture = textures.get_gpu_texture(device, queue, &*texture.borrow());

            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: gpu_texture.create_binding_type(),
                count: None,
            });

            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(gpu_texture.get_sampler_binding_type()),
                count: None,
            });

            entries.push(wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&gpu_texture.view),
            });

            entries.push(wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&gpu_texture.sampler),
            });
        }

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &layout_entries,
            label: Some("material_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &entries,
            label: Some("material_bind_group"),
        });

        Self {
            // buffer,
            bind_group_layout,
            bind_group,
        }
    }
}

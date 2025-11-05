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

        let layout_entries = material.bindgroup_layout_entries();

        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }];

        let gpu_texture;
        if let Some(texture) = material.texture() {
            gpu_texture = textures.get_gpu_texture(device, queue, &*texture.borrow());
        } else {
            gpu_texture = textures.get_default_gpu_texture();
        }
        entries.push(wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(&gpu_texture.sampler),
        });

        entries.push(wgpu::BindGroupEntry {
            binding: 2,
            resource: wgpu::BindingResource::TextureView(&gpu_texture.view),
        });

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

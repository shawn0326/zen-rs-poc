use super::textures::Textures;
use crate::{
    graphics::{Material, MaterialId},
    scene::Camera,
};
use std::{
    collections::{HashMap, hash_map::Entry},
    vec,
};
use wgpu::util::DeviceExt;

pub(super) struct BindGroups {
    gpu_global_bind_group: GpuGlobalBindGroup,
    map: HashMap<MaterialId, GpuMaterialBindGroup>,
}

impl BindGroups {
    pub fn new(device: &wgpu::Device) -> Self {
        let gpu_global_bind_group = GpuGlobalBindGroup::new(device);

        Self {
            gpu_global_bind_group,
            map: HashMap::new(),
        }
    }

    pub fn get_global_bind_group(&self) -> &GpuGlobalBindGroup {
        &self.gpu_global_bind_group
    }

    pub fn get_material_bind_group(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        material: &Material,
        textures: &mut Textures,
    ) -> &GpuMaterialBindGroup {
        let material_id = material.id();

        match self.map.entry(material_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let gpu_material_bind_group =
                    GpuMaterialBindGroup::new(device, queue, textures, &*material);

                v.insert(gpu_material_bind_group)
            }
        }
    }

    pub fn get_material_bind_group_by_id(&self, material_id: &MaterialId) -> &GpuMaterialBindGroup {
        self.map.get(material_id).unwrap()
    }
}

pub struct GpuGlobalBindGroup {
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl GpuGlobalBindGroup {
    fn new(device: &wgpu::Device) -> Self {
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

pub(super) struct GpuMaterialBindGroup {
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

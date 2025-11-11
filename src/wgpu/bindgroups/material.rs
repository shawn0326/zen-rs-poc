use super::super::textures::Textures;
use crate::material::Material;
use crate::shader::BindingType;
use std::vec;
use wgpu::util::DeviceExt;

pub(in super::super) struct GpuMaterialBindGroup {
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
        let shader = material.shader();

        let mut layout_entries = vec![];

        for binding_entry in shader.binding_schema() {
            match &binding_entry.ty {
                BindingType::UniformBuffer { .. } => {
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: binding_entry.slot,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    });
                }
                BindingType::Texture => {
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: binding_entry.slot,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    });
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: binding_entry.slot + 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    });
                }
            }
        }

        let data = material.bindings()[0].expect_uniform_buffer();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: &data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }];

        let gpu_texture;
        if let Some(texture) = material.bindings()[1].expect_texture() {
            gpu_texture = textures.get_gpu_texture(device, queue, &*texture.borrow());
        } else {
            gpu_texture = textures.get_default_gpu_texture();
        }

        entries.push(wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::TextureView(&gpu_texture.view),
        });
        entries.push(wgpu::BindGroupEntry {
            binding: 2,
            resource: wgpu::BindingResource::Sampler(&gpu_texture.sampler),
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
            bind_group_layout,
            bind_group,
        }
    }
}

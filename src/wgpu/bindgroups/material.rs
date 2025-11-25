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
        resources: &crate::Resources,
    ) -> Self {
        let shader = material.shader();

        let mut buffer = None;

        // Prepare gpu buffer and textures
        {
            for (index, binding_entry) in shader.binding_schema().iter().enumerate() {
                match &binding_entry.ty {
                    BindingType::UniformBuffer { .. } => {
                        let data = material.bindings()[index].expect_uniform_buffer();

                        buffer = Some(device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Material Buffer"),
                                contents: &data,
                                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                            },
                        ));
                    }
                    BindingType::Texture => {
                        if let Some(texture_handle) = material.bindings()[index].expect_texture() {
                            textures.get_gpu_texture(
                                device,
                                queue,
                                resources.get_texture(texture_handle).unwrap(),
                                texture_handle,
                                resources,
                            );
                        }
                    }
                }
            }
        }

        let mut layout_entries = vec![];
        let mut entries = vec![];

        for (index, binding_entry) in shader.binding_schema().iter().enumerate() {
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

                    if let Some(buffer) = &buffer {
                        entries.push(wgpu::BindGroupEntry {
                            binding: binding_entry.slot,
                            resource: buffer.as_entire_binding(),
                        });
                    }
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

                    let gpu_texture;
                    if let Some(texture_handle) = material.bindings()[index].expect_texture() {
                        gpu_texture = textures.get_gpu_texture_by_id(texture_handle).unwrap();
                    } else {
                        gpu_texture = textures.get_default_gpu_texture();
                    }

                    entries.push(wgpu::BindGroupEntry {
                        binding: binding_entry.slot,
                        resource: wgpu::BindingResource::TextureView(&gpu_texture.view),
                    });
                    entries.push(wgpu::BindGroupEntry {
                        binding: binding_entry.slot + 1,
                        resource: wgpu::BindingResource::Sampler(&gpu_texture.sampler),
                    });
                }
            }
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
            bind_group_layout,
            bind_group,
        }
    }
}

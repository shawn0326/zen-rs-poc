use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use super::textures::Textures;

use crate::{
    graphics::{Material, MaterialId},
    scene::Camera,
};

pub(super) struct BindGroups {
    map: HashMap<MaterialId, GpuBindGroup>,
}

impl BindGroups {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set_bindgroup(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        material: &Rc<RefCell<Material>>,
        textures: &mut Textures,
        _camera: &Camera,
    ) -> &GpuBindGroup {
        let material = material.borrow();

        let material_id = material.id();

        match self.map.entry(material_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let gpu_bindgroup: GpuBindGroup;

                if let Some(texture) = material.texture() {
                    textures.set_texture(device, queue, &texture);
                    let gpu_texture = textures.get_texture(&texture);

                    let texture_bind_group_layout =
                        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            entries: &[
                                wgpu::BindGroupLayoutEntry {
                                    binding: 0,
                                    visibility: wgpu::ShaderStages::FRAGMENT,
                                    ty: wgpu::BindingType::Texture {
                                        multisampled: false,
                                        view_dimension: wgpu::TextureViewDimension::D2,
                                        sample_type: wgpu::TextureSampleType::Float {
                                            filterable: true,
                                        },
                                    },
                                    count: None,
                                },
                                wgpu::BindGroupLayoutEntry {
                                    binding: 1,
                                    visibility: wgpu::ShaderStages::FRAGMENT,
                                    // This should match the filterable field of the
                                    // corresponding Texture entry above.
                                    ty: wgpu::BindingType::Sampler(
                                        wgpu::SamplerBindingType::Filtering,
                                    ),
                                    count: None,
                                },
                            ],
                            label: Some("texture_bind_group_layout"),
                        });

                    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &texture_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(
                                    &gpu_texture
                                        .texture
                                        .create_view(&wgpu::TextureViewDescriptor::default()),
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Sampler(&gpu_texture.sampler),
                            },
                        ],
                        label: Some("diffuse_bind_group"),
                    });

                    gpu_bindgroup = GpuBindGroup {
                        bind_group: diffuse_bind_group,
                        layout: texture_bind_group_layout,
                    };
                } else {
                    let bind_group_layout =
                        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            entries: &[],
                            label: Some("bind_group_layout"),
                        });

                    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[],
                        label: Some("diffuse_bind_group"),
                    });

                    gpu_bindgroup = GpuBindGroup {
                        bind_group,
                        layout: bind_group_layout,
                    };
                }

                println!("Created new bind group");

                v.insert(gpu_bindgroup)
            }
        }
    }

    // pub fn get_bindgroup(&self, material: &Rc<RefCell<Material>>) -> Option<&GpuBindGroup> {
    //     self.map.get(&material.borrow().id())
    // }
}

pub(super) struct GpuBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

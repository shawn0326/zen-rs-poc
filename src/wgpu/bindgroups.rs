use super::textures::Textures;
use crate::{
    graphics::{Material, MaterialId},
    scene::Camera,
};
use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};
use wgpu::util::DeviceExt;

pub(super) struct BindGroups {
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bindgroup: GpuBindGroup,
    map: HashMap<MaterialId, GpuBindGroup>,
}

impl BindGroups {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera_uniform = CameraUniform::new();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bindgroup_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX, // 1
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false, // 2
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            camera_uniform,
            camera_buffer,
            camera_bindgroup: GpuBindGroup {
                bindgroup: camera_bindgroup,
                layout: camera_bindgroup_layout,
            },
            map: HashMap::new(),
        }
    }

    pub fn update_camera(&mut self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera_uniform.update_view_proj(camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::bytes_of(&self.camera_uniform),
        );
    }

    pub fn get_camera_bindgroup(&self) -> &GpuBindGroup {
        &self.camera_bindgroup
    }

    pub fn set_bindgroup(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        material: &Rc<RefCell<Material>>,
        textures: &mut Textures,
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

                    let texture_bindgroup_layout =
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

                    let diffuse_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &texture_bindgroup_layout,
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
                        bindgroup: diffuse_bindgroup,
                        layout: texture_bindgroup_layout,
                    };
                } else {
                    let bind_group_layout =
                        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            entries: &[],
                            label: Some("bind_group_layout"),
                        });

                    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[],
                        label: Some("diffuse_bind_group"),
                    });

                    gpu_bindgroup = GpuBindGroup {
                        bindgroup,
                        layout: bind_group_layout,
                    };
                }

                println!("Created new bind group");

                v.insert(gpu_bindgroup)
            }
        }
    }

    pub fn get_bindgroup(&self, material: &Rc<RefCell<Material>>) -> &GpuBindGroup {
        self.map.get(&material.borrow().id()).unwrap()
    }
}

pub(super) struct GpuBindGroup {
    pub bindgroup: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

// 此属性标注数据的内存布局兼容 C-ABI，令其可用于着色器
#[repr(C)]
// derive 属性自动导入的这些 trait，令其可被存入缓冲区
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    // glam 的数据类型不能直接用于 bytemuck
    // 需要先将 Matrix4 矩阵转为一个 4x4 的浮点数数组
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

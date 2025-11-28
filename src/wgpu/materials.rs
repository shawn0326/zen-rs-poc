use super::samplers::Samplers;
use super::textures::Textures;
use crate::material::{Material, MaterialParameter};
use crate::{MaterialHandle, ResourceKey};
use crate::{Resources, shader::*};
use slotmap::SecondaryMap;

pub enum BindingCache {
    UniformBuffer {
        slot: u32,
        buf: Box<wgpu::Buffer>,
        ver: u64,
    },
    Texture {
        slot: u32,
        ver: u64,
    },
    Sampler {
        slot: u32,
        ver: u64,
    },
}

fn bind_group(
    device: &wgpu::Device,
    textures: &Textures,
    samplers: &Samplers,
    bindings_cache: &[BindingCache],
    bind_group_layout: &wgpu::BindGroupLayout,
    material: &Material,
) -> wgpu::BindGroup {
    let entries = bindings_cache
        .iter()
        .zip(material.parameters().iter())
        .map(|(binding_cache, param)| match (binding_cache, param) {
            (
                BindingCache::UniformBuffer { slot, buf, .. },
                MaterialParameter::UniformBuffer { .. },
            ) => wgpu::BindGroupEntry {
                binding: *slot,
                resource: buf.as_entire_binding(),
            },
            (BindingCache::Texture { slot, .. }, MaterialParameter::Texture { val, .. }) => {
                let texture_gpu = if let Some(texture_handle) = val {
                    textures.get_gpu_texture_by_id(texture_handle).unwrap()
                } else {
                    textures.get_default_gpu_texture()
                };
                wgpu::BindGroupEntry {
                    binding: *slot,
                    resource: wgpu::BindingResource::TextureView(&texture_gpu.view),
                }
            }
            (BindingCache::Sampler { slot, .. }, MaterialParameter::Sampler { val, .. }) => {
                let sampler = if let Some(sampler_box) = val {
                    samplers.get_gpu_sampler(sampler_box).unwrap()
                } else {
                    samplers.get_default_gpu_sampler()
                };
                wgpu::BindGroupEntry {
                    binding: *slot,
                    resource: wgpu::BindingResource::Sampler(sampler),
                }
            }
            _ => {
                panic!("BindingCache and MaterialParameter do not match or parameter missing!")
            }
        })
        .collect::<Vec<_>>();

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Material Bind Group"),
        layout: &bind_group_layout,
        entries: &entries,
    })
}

pub struct InternalMaterial {
    pub bindings_cache: Vec<BindingCache>,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl InternalMaterial {
    pub fn new(
        device: &wgpu::Device,
        textures: &Textures,
        samplers: &Samplers,
        material: &Material,
    ) -> Self {
        let shader = material.shader();

        let bindings_cache: Vec<BindingCache> = shader
            .binding_schema()
            .iter()
            .map(|binding_entry| match &binding_entry.ty {
                BindingType::UniformBuffer { total_size, .. } => {
                    let buf = device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some(&binding_entry.name),
                        size: *total_size as u64,
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });
                    BindingCache::UniformBuffer {
                        slot: binding_entry.slot,
                        ver: u64::MAX,
                        buf: Box::new(buf),
                    }
                }
                BindingType::Texture => BindingCache::Texture {
                    slot: binding_entry.slot,
                    ver: u64::MAX,
                },
                BindingType::Sampler => BindingCache::Sampler {
                    slot: binding_entry.slot,
                    ver: u64::MAX,
                },
            })
            .collect();

        let layout_entries = shader
            .binding_schema()
            .iter()
            .map(|binding_entry| match &binding_entry.ty {
                BindingType::UniformBuffer { .. } => wgpu::BindGroupLayoutEntry {
                    binding: binding_entry.slot,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindingType::Texture => wgpu::BindGroupLayoutEntry {
                    binding: binding_entry.slot,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindingType::Sampler => wgpu::BindGroupLayoutEntry {
                    binding: binding_entry.slot,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            })
            .collect::<Vec<_>>();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Material Bind Group Layout"),
            entries: &layout_entries,
        });

        let bind_group = bind_group(
            device,
            textures,
            samplers,
            &bindings_cache,
            &bind_group_layout,
            material,
        );

        Self {
            bindings_cache,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn ensure_bind_group(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        textures: &Textures,
        samplers: &Samplers,
        material: &Material,
    ) -> bool {
        let mut needs_update = false;

        for (binding_cache, param) in self
            .bindings_cache
            .iter_mut()
            .zip(material.parameters().iter())
        {
            match (binding_cache, param) {
                (
                    BindingCache::Texture { ver: pver, .. },
                    MaterialParameter::Texture { ver, .. },
                ) => {
                    if ver.as_u64() != *pver {
                        *pver = ver.as_u64();
                        needs_update = true;
                    }
                }
                (
                    BindingCache::Sampler { ver: pver, .. },
                    MaterialParameter::Sampler { ver, .. },
                ) => {
                    if ver.as_u64() != *pver {
                        *pver = ver.as_u64();
                        needs_update = true;
                    }
                }
                (
                    BindingCache::UniformBuffer { ver: pver, buf, .. },
                    MaterialParameter::UniformBuffer { ver, val },
                ) => {
                    if ver.as_u64() != *pver {
                        *pver = ver.as_u64();
                        queue.write_buffer(buf, 0, val.as_ref());
                    }
                }
                _ => {
                    panic!("BindingCache and MaterialParameter do not match or parameter missing!");
                }
            }
        }

        if needs_update {
            self.bind_group = bind_group(
                device,
                textures,
                samplers,
                &self.bindings_cache,
                &self.bind_group_layout,
                material,
            );
        }

        needs_update
    }
}

pub struct Materials {
    map: SecondaryMap<ResourceKey, InternalMaterial>,
}

impl Materials {
    pub fn new() -> Self {
        Self {
            map: SecondaryMap::new(),
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        resources: &Resources,
        textures: &mut Textures,
        samplers: &mut Samplers,
        material_handle: &MaterialHandle,
    ) -> &InternalMaterial {
        let entry = self
            .map
            .entry(material_handle.raw())
            .expect("MaterialHandle has been removed from resources.");

        let internal_material = entry.or_insert_with(|| {
            let material = resources.get_material(material_handle).unwrap();
            InternalMaterial::new(device, textures, samplers, material)
        });

        internal_material.ensure_bind_group(
            device,
            queue,
            textures,
            samplers,
            resources.get_material(material_handle).unwrap(),
        );

        internal_material
    }

    pub fn get_internal_material(&self, material_handle: &MaterialHandle) -> &InternalMaterial {
        self.map
            .get(material_handle.raw())
            .expect("Material GPU lost.")
    }
}

use crate::sampler::Sampler;
use std::collections::HashMap;

pub struct Samplers {
    default_sampler: Sampler,
    hash_map: HashMap<Sampler, wgpu::Sampler>,
}

impl Samplers {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut samplers = Self {
            default_sampler: Sampler::default(),
            hash_map: HashMap::new(),
        };
        samplers.prepare(device, samplers.default_sampler);
        samplers
    }

    pub fn prepare(&mut self, device: &wgpu::Device, sampler_desc: Sampler) -> &wgpu::Sampler {
        self.hash_map.entry(sampler_desc).or_insert_with(|| {
            device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: sampler_desc.address_mode_u,
                address_mode_v: sampler_desc.address_mode_v,
                address_mode_w: sampler_desc.address_mode_w,
                mag_filter: sampler_desc.mag_filter,
                min_filter: sampler_desc.min_filter,
                mipmap_filter: sampler_desc.mipmap_filter,
                lod_min_clamp: sampler_desc.lod_min_clamp,
                lod_max_clamp: sampler_desc.lod_max_clamp,
                compare: sampler_desc.compare,
                anisotropy_clamp: sampler_desc.anisotropy_clamp,
                border_color: sampler_desc.border_color,
                ..Default::default()
            })
        })
    }

    pub fn get_gpu_sampler(&self, sampler: &Sampler) -> Option<&wgpu::Sampler> {
        self.hash_map.get(sampler)
    }

    pub fn get_default_gpu_sampler(&self) -> &wgpu::Sampler {
        self.hash_map.get(&self.default_sampler).unwrap()
    }
}

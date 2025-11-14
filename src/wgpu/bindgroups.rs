mod global;
mod material;
mod primitive;
use super::textures::Textures;
use crate::material::{Material, MaterialId};
pub(super) use global::GlobalBindGroup;
pub(super) use material::GpuMaterialBindGroup;
pub(super) use primitive::PrimitiveBindGroup;

use std::collections::{HashMap, hash_map::Entry};

pub(super) struct MaterialBindGroups {
    map: HashMap<MaterialId, GpuMaterialBindGroup>,
}

impl MaterialBindGroups {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
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
}

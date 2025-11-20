mod global;
mod material;
mod primitive;
use super::textures::Textures;
use crate::MaterialHandle;
pub(super) use global::GlobalBindGroup;
pub(super) use material::GpuMaterialBindGroup;
pub(super) use primitive::PrimitiveBindGroup;

use std::collections::{HashMap, hash_map::Entry};

pub(super) struct MaterialBindGroups {
    map: HashMap<MaterialHandle, GpuMaterialBindGroup>,
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
        material_handle: MaterialHandle,
        textures: &mut Textures,
        resources: &crate::Resources,
    ) -> &GpuMaterialBindGroup {
        let material = resources.get_material(material_handle).unwrap();

        match self.map.entry(material_handle) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let gpu_material_bind_group =
                    GpuMaterialBindGroup::new(device, queue, textures, &*material, resources);

                v.insert(gpu_material_bind_group)
            }
        }
    }
}

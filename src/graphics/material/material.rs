use super::TextureRef;
pub use super::schema::{MaterialBindingType, MaterialSchema};
use crate::Symbol;
use std::collections::HashMap;

enum MaterialResource {
    UniformBuffer(Vec<u8>),
    Texture(Option<TextureRef>),
}

struct Material {
    schema: &'static MaterialSchema,
    resources: HashMap<u32, MaterialResource>,
}

impl Material {
    pub fn new(schema: &'static MaterialSchema) -> Self {
        // Initialize resources according to schema
        let mut resources = HashMap::new();
        schema.bindings().iter().for_each(|entry| {
            let resource = match &entry.ty {
                MaterialBindingType::Buffer {
                    total_size,
                    members: _,
                } => MaterialResource::UniformBuffer(vec![0u8; *total_size]),
                MaterialBindingType::Texture => MaterialResource::Texture(None),
            };
            resources.insert(entry.binding, resource);
        });

        Self { schema, resources }
    }

    fn set_vec4(&mut self, key: &Symbol, value: [f32; 4]) {
        let location = self.schema.find_uniform_location(key);

        if let Some((binding, offset)) = location {
            if let Some(MaterialResource::UniformBuffer(buffer)) = self.resources.get_mut(&binding)
            {
                let bytes = bytemuck::cast_slice(&value);
                buffer[offset..offset + bytes.len()].copy_from_slice(bytes);
            }
        }
    }

    fn get_vec4(&self, key: &Symbol) -> Option<[f32; 4]> {
        let location = self.schema.find_uniform_location(key);

        if let Some((binding, offset)) = location {
            if let Some(MaterialResource::UniformBuffer(buffer)) = self.resources.get(&binding) {
                let bytes = &buffer[offset..offset + std::mem::size_of::<[f32; 4]>()];
                let value: [f32; 4] = bytemuck::cast_slice(bytes)
                    .try_into()
                    .expect("Failed to cast bytes to [f32; 4]");
                return Some(value);
            }
        }

        None
    }

    fn set_texture(&mut self, key: &Symbol, texture: TextureRef) {
        if let Some(binding) = self.schema.find_texture_location(key) {
            if let Some(MaterialResource::Texture(tex_option)) = self.resources.get_mut(&binding) {
                *tex_option = Some(texture);
            }
        }
    }
}

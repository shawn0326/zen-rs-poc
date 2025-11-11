pub use zen_material_derive::*;

pub trait UniformBuffer {
    fn std140_size(&self) -> u64;
    fn to_std140_bytes(&self) -> Vec<u8>;
}

pub enum MaterialBindingKind {
    Buffer,
    Texture,
    Sampler,
}

pub struct MaterialBindingDesc {
    pub name: &'static str,
    pub binding: u32,
    pub kind: MaterialBindingKind,
}

pub trait Material {
    type Texture;
    type Sampler;

    fn schema(&self) -> &'static [MaterialBindingDesc];
    fn uniform_buffer(&self, _binding: u32) -> Option<&dyn UniformBuffer> {
        None
    }
    fn uniform_texture(&self, _binding: u32) -> Option<&Self::Texture> {
        None
    }
    fn uniform_sampler(&self, _binding: u32) -> Option<&Self::Sampler> {
        None
    }
}

#[cfg(test)]
mod tests {
    pub use crate as zen_material;
    use crate::{Material, UniformBuffer};

    #[allow(unused)]
    #[derive(UniformBuffer, Copy, Clone, Debug)]
    pub struct TestUniforms {
        albedo_factor: [f32; 4],
    }

    #[allow(unused)]
    #[derive(Material, Debug)]
    pub struct TestMaterial {
        #[buffer(binding = 0)]
        uniforms: TestUniforms,
        #[texture(binding = 1)]
        albedo_texture: u32, // Placeholder type
        #[sampler(binding = 2)]
        albedo_sampler: u32, // Placeholder type
    }

    #[test]
    fn test_material() {
        let mut material = TestMaterial {
            uniforms: TestUniforms {
                albedo_factor: [1.0, 0.0, 0.0, 1.0],
            },
            albedo_texture: 42,
            albedo_sampler: 42,
        };

        material.uniforms.albedo_factor[0] = 0.5;

        if let Some(a) = material.uniform_buffer(0) {
            println!("Sampler: {}", a.std140_size());
        }

        if let Some(a) = material.uniform_sampler(2) {
            println!("Sampler: {}", a);
        }

        println!("Material: {:?}", material);
    }
}

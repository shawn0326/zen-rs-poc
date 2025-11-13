use super::graphics::TextureRef;
use super::shader::*;
use crate::Symbol;
use crate::math::Vec4;
use std::rc::Rc;

enum MaterialResource {
    UniformBuffer(Box<[u8]>),
    Texture(Option<TextureRef>),
}

impl MaterialResource {
    #[inline(always)]
    fn expect_uniform_buffer(&self) -> &[u8] {
        match self {
            MaterialResource::UniformBuffer(b) => &b[..],
            _ => panic!("expected UniformBuffer at index"),
        }
    }
    #[inline(always)]
    fn expect_uniform_buffer_mut(&mut self) -> &mut [u8] {
        match self {
            MaterialResource::UniformBuffer(b) => &mut b[..],
            _ => panic!("expected UniformBuffer at index"),
        }
    }
    #[inline(always)]
    fn expect_texture(&self) -> &Option<TextureRef> {
        match self {
            MaterialResource::Texture(t) => t,
            _ => panic!("expected Texture at index"),
        }
    }
    #[inline(always)]
    fn expect_texture_mut(&mut self) -> &mut Option<TextureRef> {
        match self {
            MaterialResource::Texture(t) => t,
            _ => panic!("expected Texture at index"),
        }
    }
}

struct Material {
    shader: Rc<Shader>,
    resources: Box<[MaterialResource]>,
}

impl Material {
    pub fn from_shader(shader: &Rc<Shader>) -> Self {
        let resources = build_material_resources(shader);
        Self {
            shader: Rc::clone(shader),
            resources,
        }
    }

    pub fn set_vec4(&mut self, key: &Symbol, value: &[f32; 4]) {
        const LEN_VEC4: usize = core::mem::size_of::<[f32; 4]>();

        let (idx, offset) = self
            .shader
            .find_uniform_location(key)
            .expect("unknown uniform key");

        let buf = self.resources[idx].expect_uniform_buffer_mut();

        let end = offset + LEN_VEC4;
        let bytes = bytemuck::cast_slice(value);
        buf[offset..end].copy_from_slice(bytes);
    }

    pub fn get_vec4(&self, key: &Symbol) -> Vec4 {
        const LEN_VEC4: usize = core::mem::size_of::<[f32; 4]>();

        let (idx, offset) = self
            .shader
            .find_uniform_location(key)
            .expect("unknown uniform key");

        let buf = self.resources[idx].expect_uniform_buffer();

        let end = offset + LEN_VEC4;
        let bytes = &buf[offset..end];
        let value: [f32; 4] = bytemuck::cast_slice(bytes)
            .try_into()
            .expect("Failed to cast bytes to [f32; 4]");
        value.into()
    }
}

fn build_material_resources(shader: &Shader) -> Box<[MaterialResource]> {
    let mut resources = Vec::new();
    for entry in shader.binding_schema.iter() {
        let resource = match &entry.ty {
            BindingType::UniformBuffer { total_size, .. } => {
                MaterialResource::UniformBuffer(vec![0u8; *total_size].into_boxed_slice())
            }
            BindingType::Texture => MaterialResource::Texture(None),
        };
        resources.push(resource);
    }
    resources.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_material_resources() {
        let shader = ShaderBuilder::new()
            .source("shader code here")
            .buffer(symbol!("uniforms"), 0)
            .vec4(symbol!("albedo_factor"))
            .float(symbol!("roughness"))
            .finish()
            .texture(symbol!("albedo_texture"), 1)
            .vertex_attr(symbol!("position"), 0)
            .build()
            .into_rc();

        let mut material = Material::from_shader(&shader);

        assert_eq!(material.resources.len(), 2);

        match &material.resources[0] {
            MaterialResource::UniformBuffer(buffer) => {
                assert_eq!(buffer.len(), 32); // std140: vec4 + float
            }
            _ => panic!("Expected UniformBuffer"),
        }

        match &material.resources[1] {
            MaterialResource::Texture(_) => {}
            _ => panic!("Expected Texture"),
        }

        material.set_vec4(&symbol!("albedo_factor"), Vec4::ZERO.as_ref());
        let albedo: Vec4 = material.get_vec4(&symbol!("albedo_factor"));
        assert_eq!(albedo, Vec4::ZERO);
    }
}

use super::graphics::TextureRef;
use super::shader::*;
use crate::Symbol;
use crate::math::Vec4;
use std::cell::RefCell;
use std::rc::Rc;

define_id!(MaterialId);

pub(crate) enum MaterialResource {
    UniformBuffer(Box<[u8]>),
    Texture(Option<TextureRef>),
}

impl MaterialResource {
    #[inline(always)]
    pub(crate) fn expect_uniform_buffer(&self) -> &[u8] {
        match self {
            MaterialResource::UniformBuffer(b) => &b[..],
            _ => panic!("expected UniformBuffer at index"),
        }
    }
    #[inline(always)]
    pub(crate) fn expect_uniform_buffer_mut(&mut self) -> &mut [u8] {
        match self {
            MaterialResource::UniformBuffer(b) => &mut b[..],
            _ => panic!("expected UniformBuffer at index"),
        }
    }
    #[inline(always)]
    pub(crate) fn expect_texture(&self) -> &Option<TextureRef> {
        match self {
            MaterialResource::Texture(t) => t,
            _ => panic!("expected Texture at index"),
        }
    }
    #[inline(always)]
    pub(crate) fn expect_texture_mut(&mut self) -> &mut Option<TextureRef> {
        match self {
            MaterialResource::Texture(t) => t,
            _ => panic!("expected Texture at index"),
        }
    }
}

fn build_material_resources(shader: &Shader) -> Box<[MaterialResource]> {
    let mut resources = Vec::new();
    for entry in shader.binding_schema() {
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

pub type MaterialRcCell = Rc<RefCell<Material>>;

pub struct Material {
    id: MaterialId,
    shader: ShaderRc,
    resources: Box<[MaterialResource]>,
}

impl Material {
    pub fn into_rc_cell(self) -> MaterialRcCell {
        Rc::new(RefCell::new(self))
    }

    pub fn from_shader(shader: &ShaderRc) -> Self {
        let resources = build_material_resources(shader);
        Self {
            id: MaterialId::new(),
            shader: Rc::clone(shader),
            resources,
        }
    }

    pub(crate) fn id(&self) -> MaterialId {
        self.id
    }

    pub fn shader(&self) -> &ShaderRc {
        &self.shader
    }

    pub(crate) fn resources(&self) -> &Box<[MaterialResource]> {
        &self.resources
    }

    pub fn set_param_f(&mut self, key: &Symbol, value: f32) -> &mut Self {
        const LEN_FLOAT: usize = core::mem::size_of::<f32>();
        let meta = self
            .shader
            .uniform_field_meta(key)
            .expect("unknown uniform key");
        let buf = self.resources[meta.index].expect_uniform_buffer_mut();
        let end = meta.offset + LEN_FLOAT;
        let bytes = bytemuck::bytes_of(&value);
        buf[meta.offset..end].copy_from_slice(bytes);

        self
    }

    pub fn get_param_f(&self, key: &Symbol) -> f32 {
        const LEN_FLOAT: usize = core::mem::size_of::<f32>();
        let meta = self
            .shader
            .uniform_field_meta(key)
            .expect("unknown uniform key");
        let buf = self.resources[meta.index].expect_uniform_buffer();
        let end = meta.offset + LEN_FLOAT;
        let bytes = &buf[meta.offset..end];
        let value: [f32; 1] = bytemuck::cast_slice(bytes)
            .try_into()
            .expect("Failed to cast bytes to f32");
        value[0]
    }

    pub fn set_param_4fv(&mut self, key: &Symbol, value: &[f32; 4]) -> &mut Self {
        const LEN_VEC4: usize = core::mem::size_of::<[f32; 4]>();

        let meta = self
            .shader
            .uniform_field_meta(key)
            .expect("unknown uniform key");

        let buf = self.resources[meta.index].expect_uniform_buffer_mut();

        let end = meta.offset + LEN_VEC4;
        let bytes = bytemuck::cast_slice(value);
        buf[meta.offset..end].copy_from_slice(bytes);

        self
    }

    pub fn get_param_4fv(&self, key: &Symbol) -> Vec4 {
        const LEN_VEC4: usize = core::mem::size_of::<[f32; 4]>();

        let meta = self
            .shader
            .uniform_field_meta(key)
            .expect("unknown uniform key");

        let buf = self.resources[meta.index].expect_uniform_buffer();

        let end = meta.offset + LEN_VEC4;
        let bytes = &buf[meta.offset..end];
        let value: [f32; 4] = bytemuck::cast_slice(bytes)
            .try_into()
            .expect("Failed to cast bytes to [f32; 4]");
        value.into()
    }

    pub fn set_param_t(&mut self, key: &Symbol, texture: TextureRef) {
        let meta = self.shader.texture_meta(key).expect("unknown texture key");

        let tex_option = self.resources[meta.index].expect_texture_mut();
        *tex_option = Some(texture);
    }

    pub fn get_param_t(&self, key: &Symbol) -> Option<&TextureRef> {
        let meta = self.shader.texture_meta(key).expect("unknown texture key");

        let tex_option = self.resources[meta.index].expect_texture();
        tex_option.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shader::pbr_shader;

    #[test]
    fn test_build_material_resources() {
        let shader = pbr_shader();

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

        material.set_param_4fv(&symbol!("albedo_factor"), Vec4::ZERO.as_ref());
        let albedo: Vec4 = material.get_param_4fv(&symbol!("albedo_factor"));
        assert_eq!(albedo, Vec4::ZERO);

        material.set_param_f(&symbol!("roughness"), 0.5);
        let roughness = material.get_param_f(&symbol!("roughness"));
        assert_eq!(roughness, 0.5);
    }
}

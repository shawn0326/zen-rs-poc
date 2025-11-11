use super::TextureRef;
use std::cell::RefCell;
use std::rc::Rc;

define_id!(MaterialId);

pub type MaterialRef = Rc<RefCell<Material>>;

#[derive(zen_macro::Uniforms)]
pub struct Material {
    id: MaterialId,
    #[uniform]
    albedo_factor: [f32; 4],
    #[uniform]
    metallic: f32,
    #[uniform]
    roughness: f32,
    #[uniform]
    albedo_texture: Option<TextureRef>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            id: MaterialId::new(),
            albedo_factor: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 1.0,
            albedo_texture: None,
        }
    }

    pub fn to_ref(self) -> MaterialRef {
        Rc::new(RefCell::new(self))
    }

    pub(crate) fn id(&self) -> MaterialId {
        self.id
    }

    pub fn set_albedo_factor(&mut self, color: [f32; 4]) -> &mut Self {
        self.albedo_factor = color;
        self
    }

    pub fn set_texture(&mut self, texture: TextureRef) -> &mut Self {
        self.albedo_texture = Some(texture);
        self
    }

    pub fn texture(&self) -> Option<&TextureRef> {
        self.albedo_texture.as_ref()
    }

    pub fn shader_source(&self) -> &'static str {
        include_str!("shader.wgsl")
    }
}

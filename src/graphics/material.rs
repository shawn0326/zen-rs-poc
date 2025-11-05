use super::TextureRef;
use std::cell::RefCell;
use std::rc::Rc;

define_id!(MaterialId);

pub type MaterialRef = Rc<RefCell<Material>>;

#[derive(zen_macro::Uniforms)]
pub struct Material {
    id: MaterialId,
    #[uniform]
    albedo_color: [f32; 4],
    #[uniform]
    metallic: f32,
    #[uniform]
    roughness: f32,
    #[uniform]
    texture: Option<TextureRef>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            id: MaterialId::new(),
            albedo_color: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 1.0,
            texture: None,
        }
    }

    pub fn to_ref(self) -> MaterialRef {
        Rc::new(RefCell::new(self))
    }

    pub(crate) fn id(&self) -> MaterialId {
        self.id
    }

    pub fn set_texture(&mut self, texture: TextureRef) -> &mut Self {
        self.texture = Some(texture);
        self
    }

    pub fn texture(&self) -> Option<&TextureRef> {
        self.texture.as_ref()
    }
}

use super::TextureRef;
use std::cell::RefCell;
use std::rc::Rc;

define_id!(MaterialId);

pub type MaterialRef = Rc<RefCell<Material>>;

pub struct Material {
    id: MaterialId,
    texture: Option<TextureRef>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            id: MaterialId::new(),
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

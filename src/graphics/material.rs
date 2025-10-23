use super::TextureRef;
use std::cell::RefCell;
use std::rc::Rc;

define_id!(MaterialId);

#[non_exhaustive]
pub struct Material {
    id: MaterialId,
    texture: Option<TextureRef>,
}

impl Material {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id: MaterialId::new(),
            texture: None,
        }))
    }

    pub(crate) fn id(&self) -> MaterialId {
        self.id
    }

    pub fn set_texture(&mut self, texture: TextureRef) {
        self.texture = Some(texture);
    }

    pub fn texture(&self) -> Option<TextureRef> {
        self.texture.as_ref().cloned()
    }
}

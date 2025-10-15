use super::Texture;
use std::cell::RefCell;
use std::rc::Rc;

define_id!(MaterialId);

#[non_exhaustive]
pub struct Material {
    id: MaterialId,
    texture: Option<Rc<RefCell<Texture>>>,
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

    pub fn set_texture(&mut self, texture: Rc<RefCell<Texture>>) {
        self.texture = Some(texture);
    }

    pub fn texture(&self) -> Option<Rc<RefCell<Texture>>> {
        self.texture.as_ref().cloned()
    }
}

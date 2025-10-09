use super::Object3D;
use std::rc::Rc;

pub struct Scene {
    pub root: Rc<Object3D>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            root: Object3D::new(),
        }
    }

    pub fn add(&self, child: &Rc<Object3D>) -> bool {
        Object3D::add(&self.root, child)
    }

    pub fn remove(&self, child: &Rc<Object3D>) -> bool {
        Object3D::remove(&self.root, child)
    }

    pub fn update_world_matrix(&self) {
        self.root.update_world_matrix();
    }
}

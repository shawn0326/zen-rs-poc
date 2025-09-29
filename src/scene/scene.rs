use super::Object3D;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Scene {
    pub root: Rc<RefCell<Object3D>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            root: Object3D::new(),
        }
    }

    pub fn add(&self, child: &Rc<RefCell<Object3D>>) -> bool {
        Object3D::add(&self.root, child)
    }

    pub fn remove(&self, child: &Rc<RefCell<Object3D>>) -> bool {
        Object3D::remove(&self.root, child)
    }

    pub fn update_world_matrix(&self) {
        self.root.borrow_mut().update_world_matrix();
    }
}

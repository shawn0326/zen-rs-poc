use crate::math::{Matrix4, Quaternion, Vector3};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Object3D {
    pub name: String,
    pub position: Vector3,
    pub scale: Vector3,
    pub euler: Vector3,
    pub quaternion: Quaternion,
    pub matrix: Matrix4,
    pub world_matrix: Matrix4,
    pub children: Vec<Rc<RefCell<Object3D>>>,
    parent: Weak<RefCell<Object3D>>,
}

impl Object3D {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Object3D {
            name: String::new(),
            position: Vector3::new(),
            scale: Vector3::one(),
            euler: Vector3::new(),
            quaternion: Quaternion::new(),
            matrix: Matrix4::new(),
            world_matrix: Matrix4::new(),
            children: Vec::new(),
            parent: Weak::new(),
        }))
    }

    pub fn add(parent: &Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
        child.borrow_mut().parent = Rc::downgrade(parent);
        parent.borrow_mut().children.push(child);
    }

    pub fn remove(
        parent: &Rc<RefCell<Self>>,
        child: &Rc<RefCell<Self>>,
    ) -> Option<Rc<RefCell<Self>>> {
        let mut obj = parent.borrow_mut();
        if let Some(pos) = obj.children.iter().position(|x| Rc::ptr_eq(x, child)) {
            let removed = obj.children.remove(pos);
            removed.borrow_mut().parent = Weak::new();
            Some(removed)
        } else {
            None
        }
    }

    pub fn get_parent(obj: &Rc<RefCell<Self>>) -> Option<Rc<RefCell<Self>>> {
        obj.borrow().parent.upgrade()
    }

    pub fn update_matrix(&mut self) {
        self.matrix
            .compose(&self.position, &self.quaternion, &self.scale);
    }

    pub fn update_world_matrix(&mut self) {
        self.update_matrix();

        if let Some(parent) = self.parent.upgrade() {
            self.world_matrix
                .multiply_matrices(&parent.borrow().world_matrix, &self.matrix);
        } else {
            self.world_matrix.copy(&self.matrix);
        }

        for child in &self.children {
            child.borrow_mut().update_world_matrix();
        }
    }
}

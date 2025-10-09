use crate::graphics::Primitive;
use crate::math::{Matrix4, Quaternion, Vector3};
use std::cell::{Cell, Ref, RefCell};
use std::ptr;
use std::rc::{Rc, Weak};

pub struct Object3D {
    pub name: String,
    pub position: Cell<Vector3>,
    pub scale: Cell<Vector3>,
    pub euler: Cell<Vector3>,
    pub quaternion: Cell<Quaternion>,
    pub matrix: Cell<Matrix4>,
    pub world_matrix: Cell<Matrix4>,
    children: RefCell<Vec<Rc<Object3D>>>,
    parent: RefCell<Weak<Object3D>>,
    pub primitives: RefCell<Vec<Primitive>>,
}

impl Object3D {
    pub fn new() -> Rc<Self> {
        Rc::new(Object3D {
            name: String::new(),
            position: Cell::new(Vector3::new()),
            scale: Cell::new(Vector3::one()),
            euler: Cell::new(Vector3::new()),
            quaternion: Cell::new(Quaternion::new()),
            matrix: Cell::new(Matrix4::new()),
            world_matrix: Cell::new(Matrix4::new()),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            primitives: RefCell::new(Vec::new()),
        })
    }

    pub fn add(parent: &Rc<Self>, child: &Rc<Self>) -> bool {
        // Prevent adding self as a child
        if Rc::ptr_eq(parent, child) {
            return false;
        }

        // Prevent adding the same child multiple times
        if parent.children().iter().any(|c| Rc::ptr_eq(c, child)) {
            return false;
        }

        // Prevent creating cycles in the hierarchy
        if child.is_child_of(parent) {
            return false;
        }

        // If the child already has a parent, remove it from that parent first
        if let Some(old_parent) = child.parent() {
            Self::remove(&old_parent, child);
        }

        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(Rc::clone(child));
        true
    }

    pub fn remove(parent: &Rc<Self>, child: &Rc<Self>) -> bool {
        if let Some(pos) = parent.children().iter().position(|x| Rc::ptr_eq(x, child)) {
            let removed = parent.children.borrow_mut().remove(pos);
            removed.parent.replace(Weak::new());
            true
        } else {
            false
        }
    }

    pub fn children(&self) -> Ref<'_, Vec<Rc<Self>>> {
        self.children.borrow()
    }

    pub fn parent(&self) -> Option<Rc<Self>> {
        self.parent.borrow().upgrade()
    }

    pub fn is_child_of(&self, potential_ancestor: &Rc<Self>) -> bool {
        let mut current = self.parent();

        while let Some(parent) = current {
            if Rc::ptr_eq(&parent, potential_ancestor) {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    pub fn is_ancestor_of(&self, potential_ancestor: &Rc<Self>) -> bool {
        let mut current = potential_ancestor.parent();
        while let Some(parent) = current {
            if ptr::eq(&*parent, self) {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    pub fn traverse<F>(root: &Rc<Self>, callback: &F)
    where
        F: Fn(&Rc<Self>),
    {
        callback(root);

        for child in root.children().iter() {
            Self::traverse(child, callback);
        }
    }

    pub fn update_matrix(&self) {
        let mut matrix = self.matrix.get();
        matrix.compose(
            &self.position.get(),
            &self.quaternion.get(),
            &self.scale.get(),
        );
        self.matrix.set(matrix);
    }

    pub fn update_world_matrix(&self) {
        self.update_matrix();

        if let Some(parent) = self.parent() {
            let world_matrix = &parent.world_matrix.get() * &self.matrix.get();
            self.world_matrix.set(world_matrix);
        } else {
            let world_matrix = self.matrix.get();
            self.world_matrix.set(world_matrix);
        }

        for child in self.children().iter() {
            child.update_world_matrix();
        }
    }
}

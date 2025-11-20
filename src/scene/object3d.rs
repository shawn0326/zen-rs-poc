use crate::math::{EulerRot, Mat4, Quat, Vec3};
use crate::primitive::Primitive;
use std::cell::{Cell, Ref, RefCell};
use std::ptr;
use std::rc::{Rc, Weak};

pub struct Object3D {
    pub name: String,
    pub position: Cell<Vec3>,
    pub scale: Cell<Vec3>,
    pub euler: Cell<EulerRot>,
    pub quaternion: Cell<Quat>,
    pub matrix: Cell<Mat4>,
    pub world_matrix: Cell<Mat4>,
    children: RefCell<Vec<Rc<Object3D>>>,
    parent: RefCell<Weak<Object3D>>,
    pub primitives: RefCell<Vec<Primitive>>,
}

impl Object3D {
    pub fn new() -> Rc<Self> {
        Rc::new(Object3D {
            name: String::new(),
            position: Cell::new(Vec3::ZERO),
            scale: Cell::new(Vec3::ONE),
            euler: Cell::new(EulerRot::XYZ),
            quaternion: Cell::new(Quat::IDENTITY),
            matrix: Cell::new(Mat4::IDENTITY),
            world_matrix: Cell::new(Mat4::IDENTITY),
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

    pub fn traverse(root: &Rc<Self>) -> Traversal {
        Traversal::new(root)
    }

    pub fn update_matrix(&self) {
        let matrix = Mat4::from_scale_rotation_translation(
            self.scale.get(),
            self.quaternion.get(),
            self.position.get(),
        );

        self.matrix.set(matrix);
    }

    pub fn update_world_matrix(&self) {
        self.update_matrix();

        if let Some(parent) = self.parent() {
            self.world_matrix
                .set(parent.world_matrix.get() * self.matrix.get());
        } else {
            self.world_matrix.set(self.matrix.get());
        }

        for child in self.children().iter() {
            child.update_world_matrix();
        }
    }
}

pub struct Traversal {
    stack: Vec<Rc<Object3D>>,
}

impl Traversal {
    pub fn new(root: &Rc<Object3D>) -> Self {
        Self {
            stack: vec![Rc::clone(root)],
        }
    }
}

impl Iterator for Traversal {
    type Item = Rc<Object3D>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            // Push children in reverse order to maintain original order during traversal
            for child in node.children().iter().rev() {
                self.stack.push(Rc::clone(child));
            }
            Some(node)
        } else {
            None
        }
    }
}

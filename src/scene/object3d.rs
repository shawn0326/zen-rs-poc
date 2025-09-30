use crate::graphics::Primitive;
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
    children: Vec<Rc<RefCell<Object3D>>>,
    parent: Weak<RefCell<Object3D>>,
    pub primitives: Vec<Primitive>,
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
            primitives: Vec::new(),
        }))
    }

    pub fn add(parent: &Rc<RefCell<Self>>, child: &Rc<RefCell<Self>>) -> bool {
        // Prevent adding self as a child
        if Rc::ptr_eq(parent, child) {
            return false;
        }

        // Prevent adding the same child multiple times
        if parent
            .borrow()
            .children
            .iter()
            .any(|c| Rc::ptr_eq(c, child))
        {
            return false;
        }

        // Prevent creating cycles in the hierarchy
        if Self::is_ancestor_of(child, parent) {
            return false;
        }

        // If the child already has a parent, remove it from that parent first
        if let Some(old_parent) = Self::parent(child) {
            Self::remove(&old_parent, child);
        }

        child.borrow_mut().parent = Rc::downgrade(parent);
        parent.borrow_mut().children.push(Rc::clone(child));
        true
    }

    pub fn remove(parent: &Rc<RefCell<Self>>, child: &Rc<RefCell<Self>>) -> bool {
        let mut obj = parent.borrow_mut();
        if let Some(pos) = obj.children.iter().position(|x| Rc::ptr_eq(x, child)) {
            let removed = obj.children.remove(pos);
            removed.borrow_mut().parent = Weak::new();
            true
        } else {
            false
        }
    }

    pub fn is_ancestor_of(
        potential_ancestor: &Rc<RefCell<Self>>,
        potential_descendant: &Rc<RefCell<Self>>,
    ) -> bool {
        let mut current = Self::parent(potential_descendant);

        while let Some(parent) = current {
            if Rc::ptr_eq(&parent, potential_ancestor) {
                return true;
            }
            current = Self::parent(&parent);
        }
        false
    }

    pub fn traverse<F>(root: &Rc<RefCell<Self>>, callback: &F)
    where
        F: Fn(&Rc<RefCell<Self>>),
    {
        callback(root);

        // clone children to avoid borrow conflicts
        let children = &root.borrow().children.clone();

        for child in children {
            Self::traverse(child, callback);
        }
    }

    pub fn children(&self) -> &Vec<Rc<RefCell<Self>>> {
        &self.children
    }

    pub fn parent(obj: &Rc<RefCell<Self>>) -> Option<Rc<RefCell<Self>>> {
        obj.borrow().parent.upgrade()
    }

    pub fn update_matrix(&mut self) {
        self.matrix
            .compose(&self.position, &self.quaternion, &self.scale);
    }

    pub fn update_world_matrix(root: &Rc<RefCell<Self>>) {
        {
            let mut current_mut = root.borrow_mut();

            current_mut.update_matrix();

            let matrix = &current_mut.matrix.clone();

            if let Some(parent) = current_mut.parent.upgrade() {
                current_mut
                    .world_matrix
                    .multiply_matrices(&parent.borrow().world_matrix, matrix);
            } else {
                current_mut.world_matrix.copy(matrix);
            }
        }

        let children = &root.borrow().children.clone();
        for child in children {
            Self::update_world_matrix(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_add_child_reference_counts() {
        let parent = Object3D::new();
        let child = Object3D::new();

        // Initial state
        assert_eq!(Rc::strong_count(&parent), 1);
        assert_eq!(Rc::weak_count(&parent), 0);
        assert_eq!(Rc::strong_count(&child), 1);

        // Add child
        Object3D::add(&parent, &child);

        // After add: parent gets +1 weak ref, child gets +1 strong ref
        assert_eq!(Rc::strong_count(&parent), 1);
        assert_eq!(Rc::weak_count(&parent), 1);
        assert_eq!(Rc::strong_count(&child), 2);
        assert_eq!(parent.borrow().children.len(), 1);
    }

    #[test]
    fn test_remove_child_reference_counts() {
        let parent = Object3D::new();
        let child = Object3D::new();

        // Add then remove
        Object3D::add(&parent, &child);
        let removed = Object3D::remove(&parent, &child);

        // After remove: counts should return to original state
        assert!(removed);
        assert_eq!(Rc::strong_count(&parent), 1);
        assert_eq!(Rc::weak_count(&parent), 0);
        assert_eq!(Rc::strong_count(&child), 1);
        assert_eq!(parent.borrow().children.len(), 0);
    }

    #[test]
    fn test_parent_child_relationship() {
        let parent = Object3D::new();
        let child = Object3D::new();

        // Before add: no parent
        assert!(Object3D::parent(&child).is_none());

        // After add: has parent
        Object3D::add(&parent, &child);
        assert!(Object3D::parent(&child).is_some());
        assert!(Rc::ptr_eq(&Object3D::parent(&child).unwrap(), &parent));

        // After remove: no parent again
        Object3D::remove(&parent, &child);
        assert!(Object3D::parent(&child).is_none());
    }

    #[test]
    fn test_multiple_children() {
        let parent = Object3D::new();
        let child1 = Object3D::new();
        let child2 = Object3D::new();

        Object3D::add(&parent, &child1);
        Object3D::add(&parent, &child2);

        assert_eq!(Rc::weak_count(&parent), 2);
        assert_eq!(parent.borrow().children.len(), 2);

        Object3D::remove(&parent, &child1);

        assert_eq!(Rc::weak_count(&parent), 1);
        assert_eq!(parent.borrow().children.len(), 1);
    }

    #[test]
    fn test_remove_nonexistent_child() {
        let parent = Object3D::new();
        let child = Object3D::new();
        let other_child = Object3D::new();

        Object3D::add(&parent, &child);
        let result = Object3D::remove(&parent, &other_child);

        assert!(!result);
        assert_eq!(parent.borrow().children.len(), 1);
    }

    #[test]
    fn test_add_self_reference_fails() {
        let obj = Object3D::new();
        obj.borrow_mut().name = "SelfRef".to_string();

        // Try to add object to itself
        let result = Object3D::add(&obj, &obj);

        assert!(!result, "Adding object to itself should fail");
        assert_eq!(
            obj.borrow().children.len(),
            0,
            "Object should have no children"
        );
        assert!(
            Object3D::parent(&obj).is_none(),
            "Object should have no parent"
        );
    }

    #[test]
    fn test_add_duplicate_child_fails() {
        let parent = Object3D::new();
        let child = Object3D::new();

        parent.borrow_mut().name = "Parent".to_string();
        child.borrow_mut().name = "Child".to_string();

        // First add should succeed
        assert!(Object3D::add(&parent, &child), "First add should succeed");
        assert_eq!(parent.borrow().children.len(), 1);
        assert_eq!(Rc::weak_count(&parent), 1);

        // Second add of same child should fail
        let result = Object3D::add(&parent, &child);
        assert!(!result, "Adding same child twice should fail");

        // State should remain unchanged
        assert_eq!(
            parent.borrow().children.len(),
            1,
            "Should still have only 1 child"
        );
        assert_eq!(Rc::weak_count(&parent), 1, "Weak count should remain 1");
        assert_eq!(
            Rc::strong_count(&child),
            2,
            "Child strong count should remain 2"
        );
    }

    #[test]
    fn test_add_creates_cycle_fails() {
        let grandparent = Object3D::new();
        let parent = Object3D::new();
        let child = Object3D::new();

        grandparent.borrow_mut().name = "Grandparent".to_string();
        parent.borrow_mut().name = "Parent".to_string();
        child.borrow_mut().name = "Child".to_string();

        // Build hierarchy: grandparent -> parent -> child
        assert!(Object3D::add(&grandparent, &parent));
        assert!(Object3D::add(&parent, &child));

        // Try to create cycle: child -> grandparent (should fail)
        let result = Object3D::add(&child, &grandparent);
        assert!(!result, "Creating cycle should fail");

        // Try to create cycle: child -> parent (should fail)
        let result = Object3D::add(&child, &parent);
        assert!(!result, "Creating cycle should fail");

        // Verify original hierarchy is intact
        assert_eq!(grandparent.borrow().children.len(), 1);
        assert_eq!(parent.borrow().children.len(), 1);
        assert_eq!(child.borrow().children.len(), 0);

        assert!(Rc::ptr_eq(
            &Object3D::parent(&parent).unwrap(),
            &grandparent
        ));
        assert!(Rc::ptr_eq(&Object3D::parent(&child).unwrap(), &parent));
    }

    #[test]
    fn test_add_direct_cycle_fails() {
        let parent = Object3D::new();
        let child = Object3D::new();

        parent.borrow_mut().name = "Parent".to_string();
        child.borrow_mut().name = "Child".to_string();

        // Create parent -> child relationship
        assert!(Object3D::add(&parent, &child));

        // Try to create direct cycle: child -> parent
        let result = Object3D::add(&child, &parent);
        assert!(!result, "Direct cycle should fail");

        // Verify original relationship is intact
        assert_eq!(parent.borrow().children.len(), 1);
        assert_eq!(child.borrow().children.len(), 0);
        assert!(Rc::ptr_eq(&Object3D::parent(&child).unwrap(), &parent));
        assert!(Object3D::parent(&parent).is_none());
    }

    #[test]
    fn test_add_deep_cycle_fails() {
        let root = Object3D::new();
        let level1 = Object3D::new();
        let level2 = Object3D::new();
        let level3 = Object3D::new();
        let level4 = Object3D::new();

        // Create deep hierarchy
        assert!(Object3D::add(&root, &level1));
        assert!(Object3D::add(&level1, &level2));
        assert!(Object3D::add(&level2, &level3));
        assert!(Object3D::add(&level3, &level4));

        // Try to create cycle at any level
        assert!(
            !Object3D::add(&level4, &root),
            "Deep cycle to root should fail"
        );
        assert!(
            !Object3D::add(&level4, &level1),
            "Deep cycle to level1 should fail"
        );
        assert!(
            !Object3D::add(&level4, &level2),
            "Deep cycle to level2 should fail"
        );
        assert!(
            !Object3D::add(&level4, &level3),
            "Deep cycle to level3 should fail"
        );

        // Verify hierarchy is still intact
        assert_eq!(root.borrow().children.len(), 1);
        assert_eq!(level1.borrow().children.len(), 1);
        assert_eq!(level2.borrow().children.len(), 1);
        assert_eq!(level3.borrow().children.len(), 1);
        assert_eq!(level4.borrow().children.len(), 0);
    }

    #[test]
    fn test_add_failures_dont_affect_reference_counts() {
        let parent = Object3D::new();
        let child = Object3D::new();

        // Record initial counts
        let initial_parent_strong = Rc::strong_count(&parent);
        let initial_parent_weak = Rc::weak_count(&parent);
        let initial_child_strong = Rc::strong_count(&child);

        // Test self-reference failure
        Object3D::add(&parent, &parent);
        assert_eq!(Rc::strong_count(&parent), initial_parent_strong);
        assert_eq!(Rc::weak_count(&parent), initial_parent_weak);
        assert_eq!(Rc::strong_count(&child), initial_child_strong);

        // Add child successfully first
        Object3D::add(&parent, &child);

        // Record counts after successful add
        let after_add_parent_strong = Rc::strong_count(&parent);
        let after_add_parent_weak = Rc::weak_count(&parent);
        let after_add_child_strong = Rc::strong_count(&child);

        // Test duplicate add failure
        Object3D::add(&parent, &child);
        assert_eq!(Rc::strong_count(&parent), after_add_parent_strong);
        assert_eq!(Rc::weak_count(&parent), after_add_parent_weak);
        assert_eq!(Rc::strong_count(&child), after_add_child_strong);

        // Test cycle creation failure
        Object3D::add(&child, &parent);
        assert_eq!(Rc::strong_count(&parent), after_add_parent_strong);
        assert_eq!(Rc::weak_count(&parent), after_add_parent_weak);
        assert_eq!(Rc::strong_count(&child), after_add_child_strong);
    }

    #[test]
    fn test_combined_failure_scenarios() {
        let obj1 = Object3D::new();
        let obj2 = Object3D::new();

        obj1.borrow_mut().name = "Obj1".to_string();
        obj2.borrow_mut().name = "Obj2".to_string();

        // Create obj1 -> obj2
        assert!(Object3D::add(&obj1, &obj2));

        // Test multiple failure scenarios in sequence
        assert!(!Object3D::add(&obj1, &obj1), "Self-reference should fail");
        assert!(!Object3D::add(&obj1, &obj2), "Duplicate should fail");
        assert!(!Object3D::add(&obj2, &obj1), "Cycle should fail");

        // Verify state is still correct after all failures
        assert_eq!(obj1.borrow().children.len(), 1);
        assert_eq!(obj2.borrow().children.len(), 0);
        assert!(Rc::ptr_eq(&Object3D::parent(&obj2).unwrap(), &obj1));
        assert!(Object3D::parent(&obj1).is_none());
    }
}

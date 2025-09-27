use crate::math::{Matrix4, Quaternion, Vector3};

pub struct Object3D {
    pub name: String,
    pub position: Vector3,
    pub scale: Vector3,
    pub euler: Vector3,
    pub quaternion: Quaternion,
    pub matrix: Matrix4,
    pub world_matrix: Matrix4,
    pub children: Vec<Object3D>,
}

impl Object3D {
    pub fn new() -> Self {
        Object3D {
            name: String::new(),
            position: Vector3::new(),
            scale: Vector3::one(),
            euler: Vector3::new(),
            quaternion: Quaternion::new(),
            matrix: Matrix4::new(),
            world_matrix: Matrix4::new(),
            children: Vec::new(),
            // parent: None, ???
        }
    }

    pub fn add(&mut self, child: Object3D) {
        self.children.push(child);
    }

    pub fn remove(&mut self, index: usize) -> Option<Object3D> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    pub fn update_matrix(&mut self) {
        self.matrix
            .compose(&self.position, &self.quaternion, &self.scale);
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::{
    graphics::{Geometry, Material},
    math::Mat4,
    scene::{Object3D, Scene},
};

pub struct RenderCollector {}

impl RenderCollector {
    pub fn collect(&self, scene: &Scene) -> Vec<RenderItem> {
        let mut result = Vec::new();
        for obj in Object3D::traverse(&scene.root) {
            let primitives = obj.primitives.borrow();
            for primitive in primitives.iter() {
                let render_item = RenderItem {
                    world_matrix: obj.world_matrix.get(),
                    geometry: primitive.geometry().clone(),
                    material: primitive.material().clone(),
                };
                result.push(render_item);
            }
        }
        result
    }
}

pub struct RenderItem {
    pub world_matrix: Mat4,
    pub geometry: Rc<RefCell<Geometry>>,
    pub material: Rc<RefCell<Material>>,
}

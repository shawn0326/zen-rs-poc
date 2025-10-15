use std::{cell::RefCell, rc::Rc};

use crate::{
    graphics::{Geometry, Material},
    math::Mat4,
    scene::{Object3D, Scene},
};

pub struct RenderCollector {}

impl RenderCollector {
    pub fn collect(&self, scene: &Scene) -> Vec<RenderItem> {
        let result = RefCell::new(Vec::new());
        Object3D::traverse(&scene.root, &|o| {
            let primitives = o.primitives.borrow();
            for primitive in primitives.iter() {
                let render_item = RenderItem {
                    world_matrix: o.world_matrix.get(),
                    geometry: primitive.geometry().clone(),
                    material: primitive.material().clone(),
                };
                result.borrow_mut().push(render_item);
            }
        });
        result.into_inner()
    }
}

pub struct RenderItem {
    pub world_matrix: Mat4,
    pub geometry: Rc<RefCell<Geometry>>,
    pub material: Rc<RefCell<Material>>,
}

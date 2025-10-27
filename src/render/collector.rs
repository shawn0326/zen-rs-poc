use crate::{
    graphics::{GeometryRef, MaterialRef},
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

        // sort by material and geometry to minimize bind group changes
        result.sort_by_key(|item| (item.material.borrow().id(), item.geometry.borrow().id()));

        result
    }
}

pub struct RenderItem {
    pub world_matrix: Mat4,
    pub geometry: GeometryRef,
    pub material: MaterialRef,
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    graphics::{Material, MaterialId},
    scene::Camera,
};

pub(super) struct BindGroups {
    map: HashMap<MaterialId, ()>,
}

impl BindGroups {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn update(&mut self, material: &Rc<RefCell<Material>>, _camera: &Camera) {
        let material = material.borrow();

        let material_id = material.id();

        if self.map.contains_key(&material_id) {
            return;
        } else {
            // do something to
            // create bind group
            // create bind group layout
        }
    }
}

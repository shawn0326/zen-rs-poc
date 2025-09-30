use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub struct Primitive {
    geometry: Rc<RefCell<super::Geometry>>,
    material: Rc<RefCell<super::Material>>,
}

impl Primitive {
    pub fn new(
        geometry: &Rc<RefCell<super::Geometry>>,
        material: &Rc<RefCell<super::Material>>,
    ) -> Self {
        Self {
            geometry: Rc::clone(geometry),
            material: Rc::clone(material),
        }
    }

    pub fn geometry(&self) -> Rc<RefCell<super::Geometry>> {
        Rc::clone(&self.geometry)
    }

    pub fn material(&self) -> Rc<RefCell<super::Material>> {
        Rc::clone(&self.material)
    }

    pub fn set_geometry(&mut self, geometry: &Rc<RefCell<super::Geometry>>) {
        self.geometry = Rc::clone(geometry);
    }

    pub fn set_material(&mut self, material: &Rc<RefCell<super::Material>>) {
        self.material = Rc::clone(material);
    }
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Primitive")
            .field("geometry", &Rc::as_ptr(&self.geometry))
            .field("material", &Rc::as_ptr(&self.material))
            .finish()
    }
}

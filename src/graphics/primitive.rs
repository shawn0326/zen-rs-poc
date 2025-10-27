use super::{GeometryRef, MaterialRef};

#[derive(Clone)]
pub struct Primitive {
    geometry: GeometryRef,
    material: MaterialRef,
}

impl Primitive {
    pub fn new(geometry: GeometryRef, material: MaterialRef) -> Self {
        Self { geometry, material }
    }

    pub fn set_geometry(&mut self, geometry: GeometryRef) {
        self.geometry = geometry;
    }

    pub fn geometry(&self) -> &GeometryRef {
        &self.geometry
    }

    pub fn set_material(&mut self, material: MaterialRef) {
        self.material = material;
    }

    pub fn material(&self) -> &MaterialRef {
        &self.material
    }
}

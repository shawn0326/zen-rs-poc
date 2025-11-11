use super::GeometryRef;
use crate::material::MaterialRcCell;

#[derive(Clone)]
pub struct Primitive {
    geometry: GeometryRef,
    material: MaterialRcCell,
}

impl Primitive {
    pub fn new(geometry: GeometryRef, material: MaterialRcCell) -> Self {
        Self { geometry, material }
    }

    pub fn set_geometry(&mut self, geometry: GeometryRef) {
        self.geometry = geometry;
    }

    pub fn geometry(&self) -> &GeometryRef {
        &self.geometry
    }

    pub fn set_material(&mut self, material: MaterialRcCell) {
        self.material = material;
    }

    pub fn material(&self) -> &MaterialRcCell {
        &self.material
    }
}

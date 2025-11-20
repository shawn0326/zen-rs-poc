use crate::GeometryHandle;
use crate::MaterialHandle;

#[derive(Clone)]
pub struct Primitive {
    geometry: GeometryHandle,
    material: MaterialHandle,
}

impl Primitive {
    pub fn new(geometry: GeometryHandle, material: MaterialHandle) -> Self {
        Self { geometry, material }
    }

    pub fn set_geometry(&mut self, geometry: GeometryHandle) {
        self.geometry = geometry;
    }

    pub fn geometry(&self) -> GeometryHandle {
        self.geometry
    }

    pub fn set_material(&mut self, material: MaterialHandle) {
        self.material = material;
    }

    pub fn material(&self) -> MaterialHandle {
        self.material
    }
}

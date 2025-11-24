use crate::GeometryHandle;
use crate::MaterialHandle;
use crate::math::Mat4;

#[derive(Clone, Debug)]
pub struct Primitive {
    transform: Mat4,
    geometry: GeometryHandle,
    material: MaterialHandle,
}

impl Primitive {
    pub fn new(geometry: GeometryHandle, material: MaterialHandle) -> Self {
        Self {
            transform: Mat4::IDENTITY,
            geometry,
            material,
        }
    }

    #[inline]
    pub fn set_transform(&mut self, transform: Mat4) -> &mut Self {
        self.transform = transform;
        self
    }

    #[inline]
    pub fn transform(&self) -> Mat4 {
        self.transform
    }

    #[inline]
    pub fn set_geometry(&mut self, geometry: GeometryHandle) -> &mut Self {
        self.geometry = geometry;
        self
    }

    #[inline]
    pub fn geometry(&self) -> &GeometryHandle {
        &self.geometry
    }

    #[inline]
    pub fn set_material(&mut self, material: MaterialHandle) -> &mut Self {
        self.material = material;
        self
    }

    #[inline]
    pub fn material(&self) -> &MaterialHandle {
        &self.material
    }
}

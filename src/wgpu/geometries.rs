use crate::{GeometryHandle, ResourceKey, geometry::Geometry, shader::Shader};
use slotmap::SecondaryMap;

pub struct InternalGeometry {
    pub ver: u64,
}

impl InternalGeometry {
    pub fn new(geometry: &Geometry) -> Self {
        Self {
            ver: geometry.ver(),
        }
    }

    pub fn ensure(&mut self, geometry: &Geometry) -> &Self {
        if self.ver != geometry.ver() {
            self.ver = geometry.ver();

            // todo: update internal geometry layout
        }
        self
    }

    #[allow(dead_code)]
    pub fn get_or_create_layout(&self, shader: &Shader) -> u64 {
        shader.vertex_schema_hash()
        // get vertex layout by shader vertex_schema
    }
}

pub struct Geometries {
    pool: SecondaryMap<ResourceKey, InternalGeometry>,
}

impl Geometries {
    pub fn new() -> Self {
        Self {
            pool: SecondaryMap::new(),
        }
    }

    pub fn prepare(
        &mut self,
        geometry: &Geometry,
        geometry_handle: &GeometryHandle,
    ) -> &InternalGeometry {
        let entry = self
            .pool
            .entry(geometry_handle.raw())
            .expect("GeometryHandle has been removed from pool.");

        let internal_geometry = entry.or_insert_with(|| InternalGeometry::new(&geometry));

        internal_geometry.ensure(geometry)
    }

    #[allow(dead_code)]
    pub fn get_internal_geometry(&self, geometry_handle: &GeometryHandle) -> &InternalGeometry {
        self.pool
            .get(geometry_handle.raw())
            .expect("Internal Geometry lost.")
    }
}

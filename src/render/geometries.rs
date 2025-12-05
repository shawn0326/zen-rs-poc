use crate::{
    GeometryHandle, ResourceKey,
    geometry::{Geometry, VertexBuffer},
    shader::Shader,
};
use slotmap::SecondaryMap;
use std::collections::HashMap;
use std::ops::Range;

pub struct VertexBufferDesc {
    pub array_stride: wgpu::BufferAddress,
    pub step_mode: wgpu::VertexStepMode,
    pub attributes: Vec<VertexAttributeDesc>,
}

pub struct VertexAttributeDesc {
    pub format: wgpu::VertexFormat,
    pub offset: wgpu::BufferAddress,
    pub shader_location: u32,
}

pub struct GeometryShaderDesc {
    layouts: Vec<VertexBufferDesc>,
    entries: Vec<(ResourceKey, Range<u64>)>,
}

impl GeometryShaderDesc {
    pub fn new(geometry: &Geometry, shader: &Shader) -> Self {
        println!("Creating GeometryShaderDesc");

        let mut layouts: Vec<VertexBufferDesc> = Vec::new();
        let mut entries: Vec<(ResourceKey, Range<u64>)> = Vec::new();

        let mut vb_index: HashMap<VertexBuffer, usize> = HashMap::new();

        for entry in shader.vertex_schema().iter() {
            let attribute = geometry
                .get_attribute(entry.key)
                .expect(&format!("Attribute {} not found", entry.name));
            let vb = &attribute.vertex_buffer;

            let idx = if let Some(&i) = vb_index.get(vb) {
                i
            } else {
                let layout = VertexBufferDesc {
                    array_stride: vb.stride,
                    step_mode: vb.step_mode,
                    attributes: Vec::new(),
                };

                layouts.push(layout);
                entries.push((vb.buffer_slice.buffer.raw(), vb.buffer_slice.range_u64()));

                let new_index = layouts.len() - 1;
                vb_index.insert(vb.clone(), new_index);
                new_index
            };

            layouts[idx].attributes.push(VertexAttributeDesc {
                format: attribute.format,
                offset: attribute.byte_offset,
                shader_location: entry.location,
            });
        }

        Self { layouts, entries }
    }

    #[inline]
    pub fn layouts(&self) -> &[VertexBufferDesc] {
        &self.layouts
    }

    #[inline]
    pub fn entries(&self) -> &[(ResourceKey, Range<u64>)] {
        &self.entries
    }
}

pub struct InternalGeometry {
    desc_map: HashMap<u64, GeometryShaderDesc>,
    ver: u64,
}

impl InternalGeometry {
    pub fn new(geometry: &Geometry) -> Self {
        Self {
            desc_map: HashMap::new(),
            ver: geometry.ver(),
        }
    }

    pub fn ensure(&mut self, geometry: &Geometry) -> &Self {
        if self.ver != geometry.ver() {
            self.ver = geometry.ver();
            self.desc_map.clear();
        }
        self
    }

    pub fn link_shader(&mut self, geometry: &Geometry, shader: &Shader) -> &GeometryShaderDesc {
        self.desc_map
            .entry(shader.vertex_schema_hash())
            .or_insert_with(|| GeometryShaderDesc::new(geometry, shader))
    }

    pub fn get_desc(&self, shader: &Shader) -> &GeometryShaderDesc {
        self.desc_map
            .get(&shader.vertex_schema_hash())
            .expect("GeometryShaderDesc not found for given shader.")
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

    pub fn link_shader(
        &mut self,
        geometry: &Geometry,
        geometry_handle: &GeometryHandle,
        shader: &Shader,
    ) -> &GeometryShaderDesc {
        let internal_geometry = self
            .pool
            .get_mut(geometry_handle.raw())
            .expect("Internal Geometry lost.");
        internal_geometry.link_shader(geometry, shader)
    }

    #[allow(dead_code)]
    pub fn get_internal_geometry(&self, geometry_handle: &GeometryHandle) -> &InternalGeometry {
        self.pool
            .get(geometry_handle.raw())
            .expect("Internal Geometry lost.")
    }
}

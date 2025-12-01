mod factory;

use crate::{BufferHandle, GeometryHandle, Resource, Resources, Symbol, buffer::BufferSlice};
use std::collections::HashMap;
use wgpu::{IndexFormat, VertexFormat};

#[derive(Clone, Debug)]
pub struct VertexBuffer {
    pub buffer_slice: BufferSlice,
    pub stride: u64,
    pub step_mode: wgpu::VertexStepMode,
}

#[derive(Debug, Clone)]
pub struct VertexAttribute {
    pub vertex_buffer: VertexBuffer,
    pub byte_offset: u64,
    pub format: VertexFormat,
}

#[derive(Debug, Clone)]
pub struct IndexBuffer {
    pub buffer_slice: BufferSlice,
    pub format: IndexFormat,
}

impl IndexBuffer {
    pub fn index_count(&self) -> u32 {
        (self.buffer_slice.size / self.format.byte_size()) as u32
    }
}

#[derive(Debug, Clone)]
pub struct Geometry {
    attributes: HashMap<Symbol, VertexAttribute>,
    indices: Option<IndexBuffer>,
}

impl Resource for Geometry {}

impl Geometry {
    pub fn new() -> Self {
        Geometry {
            attributes: HashMap::new(),
            indices: None,
        }
    }

    pub fn into_handle(self, resources: &mut Resources) -> GeometryHandle {
        resources.insert_geometry(self)
    }

    pub fn with_attribute(mut self, key: Symbol, attr: VertexAttribute) -> Self {
        self.attributes.insert(key, attr);
        self
    }

    pub fn with_indices(mut self, indices: IndexBuffer) -> Self {
        self.indices = Some(indices);
        self
    }

    pub fn set_attribute(&mut self, key: Symbol, attr: VertexAttribute) -> &mut Self {
        self.attributes.insert(key, attr);
        self
    }

    pub fn remove_attribute(&mut self, key: Symbol) -> &mut Self {
        self.attributes.remove(&key);
        self
    }

    pub fn get_attribute(&self, key: Symbol) -> Option<&VertexAttribute> {
        self.attributes.get(&key)
    }

    pub fn set_indices(&mut self, idx: IndexBuffer) -> &mut Self {
        self.indices = Some(idx);
        self
    }

    pub fn remove_indices(&mut self) -> &mut Self {
        self.indices = None;
        self
    }

    pub fn indices(&self) -> Option<&IndexBuffer> {
        self.indices.as_ref()
    }
}

impl Geometry {
    pub(crate) fn buffers(&self) -> impl Iterator<Item = &BufferHandle> {
        self.attributes
            .values()
            .map(|attr| &attr.vertex_buffer.buffer_slice.buffer)
            .chain(
                self.indices
                    .iter()
                    .map(|idx_buf| &idx_buf.buffer_slice.buffer),
            )
    }
}

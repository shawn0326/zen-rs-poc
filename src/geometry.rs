mod attribute;
mod factory;
mod vertex_buffer;

pub use attribute::{Attribute, AttributeKey};
pub use vertex_buffer::{VertexBuffer, VertexBufferRef};

use crate::{GeometryHandle, Resources};
use std::collections::HashMap;

pub struct Geometry {
    attributes: HashMap<AttributeKey, Attribute>,
    indices: Option<Vec<u32>>,
}

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

    pub fn with_attribute(mut self, key: impl Into<AttributeKey>, attr: Attribute) -> Self {
        self.attributes.insert(key.into(), attr);
        self
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    pub fn set_attribute(&mut self, key: impl Into<AttributeKey>, attr: Attribute) -> &mut Self {
        self.attributes.insert(key.into(), attr);
        self
    }

    pub fn remove_attribute(&mut self, key: impl Into<AttributeKey>) -> &mut Self {
        self.attributes.remove(&key.into());
        self
    }

    pub fn get_attribute(&self, key: impl Into<AttributeKey>) -> Option<&Attribute> {
        self.attributes.get(&key.into())
    }

    pub fn set_indices(&mut self, idx: Vec<u32>) -> &mut Self {
        self.indices = Some(idx);
        self
    }

    pub fn remove_indices(&mut self) -> &mut Self {
        self.indices = None;
        self
    }

    pub fn indices(&self) -> Option<&[u32]> {
        self.indices.as_deref()
    }
}

impl Clone for Geometry {
    fn clone(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
            indices: self.indices.clone(),
        }
    }
}

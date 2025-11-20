use crate::{Resources, VertexBufferHandle};
use std::{cell::RefCell, rc::Rc};

pub type VertexBufferRef = Rc<RefCell<VertexBuffer>>;

pub struct VertexBuffer {
    data: Vec<f32>,
    stride: u8,
}

impl VertexBuffer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            stride: 0,
        }
    }

    pub fn into_handle(self, resources: &mut Resources) -> VertexBufferHandle {
        resources.insert_vertex_buffer(self)
    }

    pub fn with_data(mut self, data: Vec<f32>) -> Self {
        self.data = data;
        self
    }

    pub fn with_stride(mut self, stride: u8) -> Self {
        self.stride = stride;
        self
    }

    pub fn set_data(&mut self, data: Vec<f32>) -> &mut Self {
        self.data = data;
        self
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn set_stride(&mut self, stride: u8) -> &mut Self {
        self.stride = stride;
        self
    }

    pub fn stride(&self) -> u8 {
        self.stride
    }

    pub fn vertex_count(&self) -> usize {
        if self.stride == 0 {
            0
        } else {
            self.data.len() / self.stride as usize
        }
    }
}

impl Clone for VertexBuffer {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            stride: self.stride,
        }
    }
}

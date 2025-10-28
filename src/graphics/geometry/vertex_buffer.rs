use std::{cell::RefCell, rc::Rc};

define_id!(VertexBufferId);

pub type VertexBufferRef = Rc<RefCell<VertexBuffer>>;

pub struct VertexBuffer {
    id: VertexBufferId,
    data: Vec<f32>,
    stride: u8,
}

impl VertexBuffer {
    pub fn new() -> Self {
        Self {
            id: VertexBufferId::new(),
            data: Vec::new(),
            stride: 0,
        }
    }

    pub fn with_data(mut self, data: Vec<f32>) -> Self {
        self.data = data;
        self
    }

    pub fn with_stride(mut self, stride: u8) -> Self {
        self.stride = stride;
        self
    }

    pub fn into_ref(self) -> VertexBufferRef {
        Rc::new(RefCell::new(self))
    }

    #[allow(dead_code)]
    pub(crate) fn id(&self) -> VertexBufferId {
        self.id
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
            id: self.id,
            data: self.data.clone(),
            stride: self.stride,
        }
    }
}

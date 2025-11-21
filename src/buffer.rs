use crate::{BufferHandle, Resources};

#[derive(Clone)]
pub struct Buffer {
    pub data: Vec<f32>,
}

impl Buffer {
    pub fn into_handle(self, resources: &mut Resources) -> BufferHandle {
        resources.insert_buffer(self)
    }
}

impl Buffer {
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Buffer {{ len: {}, data: {:?} }}",
            self.data.len(),
            &self.data.get(0..std::cmp::min(8, self.data.len()))
        )
    }
}

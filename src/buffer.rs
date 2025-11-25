mod slice;

use crate::{BufferHandle, Resource, Resources};
use std::fmt;
use wgpu::BufferUsages;

pub use slice::BufferSlice;

#[derive(Clone)]
pub struct Buffer {
    raw: Vec<u8>,
    usage: BufferUsages,
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_len = 16;
        let display_len = self.raw.len().min(max_len);
        write!(f, "Buffer {{ raw: [{} bytes: ", self.raw.len())?;
        for (i, byte) in self.raw.iter().take(display_len).enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:#04x}", byte)?;
        }
        if self.raw.len() > max_len {
            write!(f, ", ...")?;
        }
        write!(f, "], usage: {:?} }}", self.usage)
    }
}

impl Resource for Buffer {}

impl Buffer {
    pub fn into_handle(self, resources: &mut Resources) -> BufferHandle {
        resources.insert_buffer(self)
    }
}

impl Buffer {
    #[inline]
    pub fn new(vec: Vec<u8>, usage: BufferUsages) -> Self {
        Self { raw: vec, usage }
    }

    #[inline]
    pub fn for_vertex(vec: Vec<u8>) -> Self {
        Self {
            raw: vec,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        }
    }

    #[inline]
    pub fn for_index(vec: Vec<u8>) -> Self {
        Self {
            raw: vec,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        }
    }

    #[inline]
    pub fn for_copy(vec: Vec<u8>) -> Self {
        Self {
            raw: vec,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        }
    }
}

impl Buffer {
    #[inline]
    pub fn set_raw(&mut self, raw: Vec<u8>) -> &mut Self {
        self.raw = raw;
        self
    }

    #[inline]
    pub fn raw(&self) -> &Vec<u8> {
        &self.raw
    }

    #[inline]
    pub fn raw_mut(&mut self) -> &mut Vec<u8> {
        &mut self.raw
    }

    #[inline]
    pub fn set_usage(&mut self, usage: BufferUsages) -> &mut Self {
        self.usage = usage;
        self
    }

    #[inline]
    pub fn usage(&self) -> BufferUsages {
        self.usage
    }

    #[inline]
    pub fn byte_length(&self) -> usize {
        self.raw.len()
    }
}

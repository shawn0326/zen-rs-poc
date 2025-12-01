mod slice;

use crate::{BufferHandle, DirtyVersion, Resource, Resources};
use std::fmt;
use wgpu::BufferUsages;

pub use slice::BufferSlice;

#[derive(Clone)]
pub struct Buffer {
    raw: Box<[u8]>,
    usage: BufferUsages,
    ver: DirtyVersion,
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
    pub fn new(raw: impl Into<Box<[u8]>>, usage: BufferUsages) -> Self {
        Self {
            raw: raw.into(),
            usage,
            ver: DirtyVersion::new(),
        }
    }

    #[inline]
    pub fn for_vertex(raw: impl Into<Box<[u8]>>) -> Self {
        Self::new(raw, BufferUsages::VERTEX | BufferUsages::COPY_DST)
    }

    #[inline]
    pub fn for_index(raw: impl Into<Box<[u8]>>) -> Self {
        Self::new(raw, BufferUsages::INDEX | BufferUsages::COPY_DST)
    }

    #[inline]
    pub fn for_copy(raw: impl Into<Box<[u8]>>) -> Self {
        Self::new(raw, BufferUsages::COPY_SRC | BufferUsages::COPY_DST)
    }
}

impl Buffer {
    #[inline]
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }

    #[inline]
    pub fn raw_mut(&mut self) -> &mut [u8] {
        &mut self.raw
    }

    #[inline]
    pub fn mark_dirty(&mut self) {
        self.ver.bump();
    }

    #[inline]
    pub fn usage(&self) -> BufferUsages {
        self.usage
    }

    #[inline]
    pub fn byte_len(&self) -> usize {
        self.raw.len()
    }
}

impl Buffer {
    #[inline]
    pub(crate) fn ver(&self) -> &DirtyVersion {
        &self.ver
    }
}

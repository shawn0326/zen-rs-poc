use crate::{BufferHandle, Resources};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct BufferSlice {
    pub buffer: BufferHandle,
    pub offset: usize,
    pub size: usize,
}

impl BufferSlice {
    pub fn from_entire_buffer(resources: &Resources, buffer_handle: BufferHandle) -> Self {
        let buffer = resources.get_buffer(&buffer_handle).unwrap();
        Self {
            buffer: buffer_handle,
            offset: 0,
            size: buffer.byte_len(),
        }
    }
}

impl BufferSlice {
    pub fn range(&self) -> Range<usize> {
        self.offset..(self.offset + self.size)
    }

    pub(crate) fn range_u64(&self) -> Range<u64> {
        self.offset as u64..(self.offset + self.size) as u64
    }
}

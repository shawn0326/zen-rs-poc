pub struct BufferHandle {
    raw: u64,
}

pub struct Buffer {
    raw: Vec<u8>,
}

pub struct BufferView {
    buffer: BufferHandle,
    byte_offset: usize,
    byte_length: usize,
    byte_stride: usize,
}

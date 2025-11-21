use crate::BufferHandle;

#[derive(Copy, Clone, Debug)]
pub struct VertexBuffer {
    pub buffer: BufferHandle,
    pub stride: u8,
    pub len: usize,
}

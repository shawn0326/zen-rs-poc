use super::vertex_buffer::VertexBuffer;

#[derive(Copy, Clone, Debug)]
pub struct Attribute {
    pub vertex_buffer: VertexBuffer,
    pub offset: u64,
    pub component: u8,
}

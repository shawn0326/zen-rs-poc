use crate::VertexBufferHandle;

#[derive(Clone)]
pub struct Attribute {
    buffer: VertexBufferHandle,
    offset: u8,
    components: u8,
}

impl Attribute {
    pub fn from_buffer(buffer: VertexBufferHandle) -> Self {
        Self {
            buffer,
            offset: 0,
            components: 0,
        }
    }

    pub fn with_buffer(mut self, buffer: VertexBufferHandle) -> Self {
        self.buffer = buffer;
        self
    }

    pub fn with_offset(mut self, offset: u8) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_components(mut self, components: u8) -> Self {
        self.components = components;
        self
    }

    pub fn set_buffer(&mut self, buffer: VertexBufferHandle) -> &mut Self {
        self.buffer = buffer;
        self
    }

    pub fn buffer(&self) -> &VertexBufferHandle {
        &self.buffer
    }

    pub fn set_offset(&mut self, offset: u8) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn offset(&self) -> u8 {
        self.offset
    }

    pub fn set_components(&mut self, components: u8) -> &mut Self {
        self.components = components;
        self
    }

    pub fn components(&self) -> u8 {
        self.components
    }
}

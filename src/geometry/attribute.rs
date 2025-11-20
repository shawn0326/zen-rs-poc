use crate::VertexBufferHandle;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeKey {
    Positions,
    Normals,
    TexCoords,
    Colors,
    Custom(String),
}

impl From<&str> for AttributeKey {
    fn from(s: &str) -> Self {
        match s {
            "positions" => AttributeKey::Positions,
            "normals" => AttributeKey::Normals,
            "texcoords" => AttributeKey::TexCoords,
            "colors" => AttributeKey::Colors,
            other => AttributeKey::Custom(other.to_owned()),
        }
    }
}

impl ToString for AttributeKey {
    fn to_string(&self) -> String {
        match self {
            AttributeKey::Positions => "positions".into(),
            AttributeKey::Normals => "normals".into(),
            AttributeKey::TexCoords => "texcoords".into(),
            AttributeKey::Colors => "colors".into(),
            AttributeKey::Custom(s) => s.clone(),
        }
    }
}

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

    pub fn buffer(&self) -> VertexBufferHandle {
        self.buffer
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

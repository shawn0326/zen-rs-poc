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
    data: Vec<f32>,
    components: u8,
}

impl Attribute {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            components: 0,
        }
    }

    pub fn with_data(mut self, data: Vec<f32>) -> Self {
        self.data = data;
        self
    }

    pub fn with_components(mut self, components: u8) -> Self {
        self.components = components;
        self
    }

    pub fn set_data(&mut self, data: Vec<f32>) -> &mut Self {
        self.data = data;
        self
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn set_components(&mut self, components: u8) -> &mut Self {
        self.components = components;
        self
    }

    pub fn components(&self) -> u8 {
        self.components
    }

    pub fn vertex_count(&self) -> usize {
        if self.components == 0 {
            0
        } else {
            self.data.len() / self.components as usize
        }
    }
}

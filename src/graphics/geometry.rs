use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

define_id!(GeometryId);

#[non_exhaustive]
pub struct Geometry {
    id: GeometryId,
    attributes: HashMap<AttributeKey, Attribute>,
    indices: Vec<u32>,
}

impl Geometry {
    pub fn new(
        attributes: HashMap<AttributeKey, Attribute>,
        indices: Vec<u32>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Geometry {
            id: GeometryId::new(),
            attributes,
            indices,
        }))
    }

    pub fn empty() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Geometry {
            id: GeometryId::new(),
            attributes: HashMap::new(),
            indices: Vec::new(),
        }))
    }

    pub(crate) fn id(&self) -> GeometryId {
        self.id
    }

    pub fn set_attribute(&mut self, key: impl Into<AttributeKey>, attr: Attribute) -> &mut Self {
        self.attributes.insert(key.into(), attr);
        self
    }

    pub fn set_indices(&mut self, idx: Vec<u32>) -> &mut Self {
        self.indices = idx;
        self
    }

    pub fn get_attribute(&self, key: &AttributeKey) -> Option<&Attribute> {
        self.attributes.get(key)
    }

    pub fn get_attribute_by_str(&self, key: &str) -> Option<&Attribute> {
        self.attributes.get(&AttributeKey::from(key))
    }

    pub fn get_indices(&self) -> &[u32] {
        &self.indices
    }
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub data: Vec<f32>,
    pub components: u8,
}

impl Attribute {
    pub fn new(components: u8) -> Self {
        Self {
            data: Vec::new(),
            components,
        }
    }

    pub fn with_data(components: u8, data: Vec<f32>) -> Self {
        Self { data, components }
    }

    pub fn vertex_count(&self) -> usize {
        if self.components == 0 {
            0
        } else {
            self.data.len() / self.components as usize
        }
    }
}

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

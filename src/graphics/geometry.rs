use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

define_id!(GeometryId);

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

pub type GeometryRef = Rc<RefCell<Geometry>>;

pub struct Geometry {
    id: GeometryId,
    attributes: HashMap<AttributeKey, Attribute>,
    indices: Option<Vec<u32>>,
}

impl Geometry {
    pub fn new() -> Self {
        Geometry {
            id: GeometryId::new(),
            attributes: HashMap::new(),
            indices: None,
        }
    }

    pub fn with_attribute(mut self, key: impl Into<AttributeKey>, attr: Attribute) -> Self {
        self.attributes.insert(key.into(), attr);
        self
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    pub fn into_ref(self) -> GeometryRef {
        Rc::new(RefCell::new(self))
    }

    pub(crate) fn id(&self) -> GeometryId {
        self.id
    }

    pub fn set_attribute(&mut self, key: impl Into<AttributeKey>, attr: Attribute) -> &mut Self {
        self.attributes.insert(key.into(), attr);
        self
    }

    pub fn remove_attribute(&mut self, key: impl Into<AttributeKey>) -> &mut Self {
        self.attributes.remove(&key.into());
        self
    }

    pub fn get_attribute(&self, key: impl Into<AttributeKey>) -> Option<&Attribute> {
        self.attributes.get(&key.into())
    }

    pub fn set_indices(&mut self, idx: Vec<u32>) -> &mut Self {
        self.indices = Some(idx);
        self
    }

    pub fn remove_indices(&mut self) -> &mut Self {
        self.indices = None;
        self
    }

    pub fn indices(&self) -> Option<&[u32]> {
        self.indices.as_deref()
    }
}

impl Clone for Geometry {
    fn clone(&self) -> Self {
        Self {
            id: GeometryId::new(),
            attributes: self.attributes.clone(),
            indices: self.indices.clone(),
        }
    }
}

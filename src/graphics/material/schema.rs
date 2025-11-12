use crate::Symbol;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct UniformMemberDesc {
    pub key: Symbol,
    pub offset: usize,
}

pub enum MaterialBindingType {
    Buffer {
        total_size: usize,
        members: Vec<UniformMemberDesc>,
    },
    Texture,
}

pub struct MaterialBindingEntry {
    pub key: Symbol,
    pub binding: u32,
    pub ty: MaterialBindingType,
}

pub struct MaterialSchema {
    bindings: Vec<MaterialBindingEntry>,
    uniform_lut: OnceLock<HashMap<Symbol, (u32, usize)>>,
    texture_lut: OnceLock<HashMap<Symbol, u32>>,
}

impl MaterialSchema {
    pub fn new(bindings: Vec<MaterialBindingEntry>) -> Self {
        Self {
            bindings,
            uniform_lut: OnceLock::new(),
            texture_lut: OnceLock::new(),
        }
    }

    fn uniform_map(&self) -> &HashMap<Symbol, (u32, usize)> {
        self.uniform_lut.get_or_init(|| {
            let mut map = HashMap::new();
            for entry in &self.bindings {
                if let MaterialBindingType::Buffer { members, .. } = &entry.ty {
                    for m in members {
                        map.insert(m.key, (entry.binding, m.offset));
                    }
                }
            }
            map
        })
    }

    fn texture_map(&self) -> &HashMap<Symbol, u32> {
        self.texture_lut.get_or_init(|| {
            let mut map = HashMap::new();
            for entry in &self.bindings {
                if let MaterialBindingType::Texture = entry.ty {
                    map.insert(entry.key, entry.binding);
                }
            }
            map
        })
    }

    pub fn bindings(&self) -> &Vec<MaterialBindingEntry> {
        &self.bindings
    }

    pub fn find_uniform_location(&self, key: &Symbol) -> Option<(u32, usize)> {
        self.uniform_map().get(key).copied()
    }

    pub fn find_texture_location(&self, key: &Symbol) -> Option<u32> {
        self.texture_map().get(key).copied()
    }
}

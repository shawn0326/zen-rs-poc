mod builder;
pub mod builtins;
mod schema;
mod types;

pub use builder::*;
pub(crate) use schema::*;
pub use types::*;

use crate::Symbol;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::OnceLock;

/// Metadata describing a single uniform field within one uniform-buffer binding.
/// Fields:
/// - index: index into `binding_schema` pointing to the UniformBuffer entry that owns this field.
/// - offset: byte offset inside that buffer (already alignment‑adjusted).
/// - size: logical byte size of the field (e.g. vec3 = 12 even though alignment is 16).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct UniformFieldMeta {
    pub(crate) index: usize,
    pub(crate) offset: usize,
    pub(crate) size: usize,
}

/// Metadata for a texture binding.
/// Fields:
/// - index: index into `binding_schema` pointing to the texture BindingEntry.
/// Future additions may include sample type, view dimension, array/slice info, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TextureMeta {
    pub(crate) index: usize,
}

pub type ShaderRc = Rc<Shader>;

/// Holds WGSL source and the immutable shader interface schema:
/// - binding schema: uniform buffers and textures (for bind group layout generation and material writes)
/// - vertex schema: vertex attributes required by the shader (for pipeline vertex state)
///
/// The schema is metadata used to:
/// - allocate and address material uniform storage (offset/size),
/// - build fast lookup tables (symbol → locations),
/// - help the backend create wgpu layouts and pipelines.
/// This type does not compile or own any GPU objects by itself.
pub struct Shader {
    source: Cow<'static, str>,
    binding_schema: Box<[BindingEntry]>,
    vertex_schema: Box<[VertexEntry]>,
    uniform_lut: OnceLock<HashMap<Symbol, UniformFieldMeta>>,
    texture_lut: OnceLock<HashMap<Symbol, TextureMeta>>,
}

impl Shader {
    /// Constructs a Shader from source and precomputed schemas.
    /// No GPU compilation happens here; backends consume the metadata later.
    #[inline]
    pub(crate) fn new(
        source: Cow<'static, str>,
        binding_schema: Box<[BindingEntry]>,
        vertex_schema: Box<[VertexEntry]>,
    ) -> Self {
        Shader {
            source,
            binding_schema,
            vertex_schema,
            uniform_lut: OnceLock::new(),
            texture_lut: OnceLock::new(),
        }
    }

    /// Returns the shader source as `&str` (typically WGSL).
    #[inline]
    pub(crate) fn source(&self) -> &str {
        &self.source
    }

    /// Read-only access to the binding schema (uniform buffers/textures).
    #[inline]
    pub(crate) fn binding_schema(&self) -> &[BindingEntry] {
        &self.binding_schema
    }

    /// Read-only access to the vertex attribute schema.
    #[inline]
    pub(crate) fn vertex_schema(&self) -> &[VertexEntry] {
        &self.vertex_schema
    }

    /// Wraps this Shader in `Rc` for shared ownership across materials.
    #[inline]
    pub fn into_rc(self) -> ShaderRc {
        Rc::new(self)
    }

    /// Builds (once) and caches the map: uniform symbol → field metadata.
    /// Subsequent calls are O(1) lookups into the cached map.
    fn uniform_map(&self) -> &HashMap<Symbol, UniformFieldMeta> {
        self.uniform_lut.get_or_init(|| {
            let mut map = HashMap::new();
            for (i, entry) in self.binding_schema.iter().enumerate() {
                if let BindingType::UniformBuffer { members, .. } = &entry.ty {
                    for m in members {
                        if map
                            .insert(
                                m.key,
                                UniformFieldMeta {
                                    index: i,
                                    offset: m.offset,
                                    size: m.size,
                                },
                            )
                            .is_some()
                        {
                            panic!("duplicate uniform key: {:?}", m.key);
                        }
                    }
                }
            }
            map
        })
    }

    /// Builds (once) and caches the map: texture symbol → binding metadata.
    fn texture_map(&self) -> &HashMap<Symbol, TextureMeta> {
        self.texture_lut.get_or_init(|| {
            let mut map = HashMap::new();
            for (i, entry) in self.binding_schema.iter().enumerate() {
                if let BindingType::Texture = entry.ty {
                    if map.insert(entry.key, TextureMeta { index: i }).is_some() {
                        panic!("duplicate texture key: {:?}", entry.key);
                    }
                }
            }
            map
        })
    }

    /// Fast lookup of uniform field metadata by symbol.
    #[inline]
    pub(crate) fn uniform_field_meta(&self, key: Symbol) -> Option<UniformFieldMeta> {
        self.uniform_map().get(&key).copied()
    }

    /// Fast lookup of texture binding metadata by symbol.
    #[inline]
    pub(crate) fn texture_meta(&self, key: Symbol) -> Option<TextureMeta> {
        self.texture_map().get(&key).copied()
    }
}

impl fmt::Debug for Shader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let preview_len = self.source.len().min(64);
        let preview = &self.source[..preview_len];

        let alternate = f.alternate();

        let mut ds = f.debug_struct("Shader");
        ds.field("source_len", &self.source.len())
            .field("source_preview", &preview)
            .field("bindings_len", &self.binding_schema.len())
            .field("vertex_attrs_len", &self.vertex_schema.len())
            .field("uniform_cache_init", &self.uniform_lut.get().is_some())
            .field("texture_cache_init", &self.texture_lut.get().is_some());

        if alternate {
            ds.field("binding_schema", &self.binding_schema)
                .field("vertex_schema", &self.vertex_schema);
        }

        ds.finish()
    }
}

impl Clone for Shader {
    fn clone(&self) -> Self {
        Shader {
            source: self.source.clone(),
            binding_schema: self.binding_schema.clone(),
            vertex_schema: self.vertex_schema.clone(),
            // Do not copy caches, reinitialize
            uniform_lut: OnceLock::new(),
            texture_lut: OnceLock::new(),
        }
    }
}

impl PartialEq for Shader {
    fn eq(&self, other: &Self) -> bool {
        self.source.as_ref() == other.source.as_ref()
            && self.binding_schema == other.binding_schema
            && self.vertex_schema == other.vertex_schema
    }
}
impl Eq for Shader {}

impl std::hash::Hash for Shader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.as_ref().hash(state);
        self.binding_schema.hash(state);
        self.vertex_schema.hash(state);
    }
}

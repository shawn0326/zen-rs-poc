use crate::Symbol;

/// Describes a single member inside a uniform buffer.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct UniformDesc {
    /// Symbol key (stable ID for lookups)
    pub(crate) key: Symbol,
    /// Human-readable name (for logs/debugging)
    pub(crate) name: Box<str>,
    /// Byte offset within the UBO
    pub(crate) offset: usize,
    /// Byte size of the member payload
    /// Can be used to validate the length of data written to the member
    pub(crate) size: usize,
}

/// Describes the type of a binding entry in the shader's binding schema.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum BindingType {
    /// Uniform buffer binding.
    /// total_size: final byte size of the buffer after alignment padding (struct size rounded up).
    /// members: ordered metadata for each declared member (offset + size for validation / writes).
    UniformBuffer {
        total_size: usize,
        members: Box<[UniformDesc]>,
    },
    /// Texture binding (currently a generic 2D texture slot).
    /// Future extension ideas:
    /// - Distinguish between 2D, Cube, Array, 3D, and Storage textures.
    /// Note: Samplers are now a separate binding type; future extensions here should focus solely on texture type distinctions.
    Texture,

    /// Sampler binding.
    Sampler,
}

/// A single binding entry in the shader's binding schema.
/// Maps a logical resource (by symbol/name) to a concrete binding slot and its type.
/// Note:
/// - `slot` corresponds to the WGSL annotation `@binding(n)`; it is not necessarily
///   contiguous nor equal to the index in the schema array.
/// - `key` is a stable, interned identifier for fast lookups; `name` is for human‑readable logs.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BindingEntry {
    /// Stable interned key used for lookups and caches.
    pub(crate) key: Symbol,
    /// Human‑readable name as written in the shader source.
    pub(crate) name: Box<str>,
    /// The binding slot number (WGSL `@binding(n)` value).
    pub(crate) slot: u32,
    /// Binding kind and associated metadata (e.g. UBO layout, texture kind).
    pub(crate) ty: BindingType,
}

/// Describes a single vertex attribute required by the shader.
///
/// This type captures only the shader interface (which attribute and where
/// it is read from), and deliberately omits buffer layout details such as
/// stride, and component format. That information belongs to the
/// geometry/vertex-buffer layer, keeping the shader interface decoupled from
/// how vertex data is packed in memory.
///
/// Notes:
/// - `location` maps directly to the WGSL annotation `@location(n)`.
/// - `key` is a stable, interned identifier for fast lookups; `name` is the
///   human‑readable attribute name for logs and tooling.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VertexEntry {
    /// Stable interned key used for lookups and caches.
    pub(crate) key: Symbol,
    /// Human‑readable attribute name as written in shader/source tools.
    pub(crate) name: Box<str>,
    /// Shader location (WGSL `@location(n)` value).
    pub(crate) location: u32,
}

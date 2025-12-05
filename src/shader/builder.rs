use super::*;
use crate::symbol;
use std::borrow::Cow;

/// Rounds `v` up to the next multiple of `a` (where `a` is a power of two).
/// Used to compute std140-like alignment for uniform buffer members.
#[inline]
fn align_up(v: usize, a: usize) -> usize {
    debug_assert!(a.is_power_of_two());
    (v + (a - 1)) & !(a - 1)
}

/// Generates convenience methods on `UniformBufferBuilder` that forward to
/// `uniform(name, UniformValueType::<Ty>)`, keeping the builder chainable.
///
/// Each generated method:
/// - takes an attribute name,
/// - appends a field of the given type to the current UBO,
/// - returns `Self` to continue the chain.
macro_rules! impl_uniform_methods {
    ( $( $(#[$meta:meta])* ($fn:ident, $ty:ident) ),* $(,)? ) => {
        $(
            $(#[$meta])*
            #[inline]
            pub fn $fn(self, name: &str) -> Self {
                self.uniform(name, UniformValueType::$ty)
            }
        )*
    };
}

/// Fluent builder for one uniform-buffer binding.
/// Collects members, tracks alignment/cursor, and emits a `BindingEntry`
/// back into the parent `ShaderBuilder` on `finish()`.
#[must_use = "continue chaining or call `.finish()` to emit the uniform buffer"]
pub struct UniformBufferBuilder {
    /// Builder to return to when this UBO is finished.
    parent: ShaderBuilder,
    /// Human‑readable binding name (also used to derive a stable key).
    name: Box<str>,
    /// WGSL `@binding(n)` slot of this uniform buffer.
    binding_slot: u32,
    /// Collected uniform members (offset/size computed at insert time).
    members: Vec<UniformDesc>,
    /// Current byte cursor (before final 16‑byte padding of the UBO).
    cursor: usize,
}

impl UniformBufferBuilder {
    /// Appends a uniform field to this buffer.
    /// Computes the aligned byte offset (std140‑like), records its size,
    /// advances the cursor, and returns the updated builder.
    pub fn uniform(mut self, name: &str, ty: UniformValueType) -> Self {
        let offset = align_up(self.cursor, ty.align());
        self.members.push(UniformDesc {
            key: symbol!(name),
            name: name.into(),
            offset,
            size: ty.size(),
        });
        self.cursor = offset + ty.size();
        self
    }

    impl_uniform_methods! {
        /// Add a f32 uniform
        (float,   Float),
        /// Add an i32 uniform
        (int,     Int),
        /// Add a u32 uniform
        (uint,    Uint),

        /// Add a vec2<f32> uniform
        (vec2f,   Vec2Float),
        /// Add a vec3<f32> uniform
        (vec3f,   Vec3Float),
        /// Add a vec4<f32> uniform
        (vec4f,   Vec4Float),

        /// Add a vec2<i32> uniform
        (vec2i,   Vec2Int),
        /// Add a vec3<i32> uniform
        (vec3i,   Vec3Int),
        /// Add a vec4<i32> uniform
        (vec4i,   Vec4Int),

        /// Add a vec2<u32> uniform
        (vec2u,   Vec2Uint),
        /// Add a vec3<u32> uniform
        (vec3u,   Vec3Uint),
        /// Add a vec4<u32> uniform
        (vec4u,   Vec4Uint),

        /// Add a mat4x4<f32> uniform
        (mat4f,   Mat4Float),
    }

    /// Finalizes this uniform buffer:
    /// - pads total size to 16 bytes,
    /// - pushes a `BindingEntry` into the parent,
    /// - returns the parent `ShaderBuilder` to continue building.
    pub fn finish(mut self) -> ShaderBuilder {
        let total_size = align_up(self.cursor, 16);
        self.parent.binding_schema.push(BindingEntry {
            key: symbol!(&self.name),
            name: self.name,
            slot: self.binding_slot,
            ty: BindingType::UniformBuffer {
                total_size,
                members: self.members.into_boxed_slice(),
            },
        });
        self.parent
    }
}

/// Builder that assembles a `Shader` from WGSL source and a declarative
/// interface schema (bindings and vertex attributes).
///
/// Usage:
/// - call `source(...)` to set WGSL,
/// - add one or more bindings via `uniform_buffer(...).finish()` and `texture(...)`,
/// - add vertex attributes via `vertex_attr(...)`,
/// - call `build()` to validate and produce an immutable `Shader`.
pub struct ShaderBuilder {
    /// WGSL source code (borrowed static or owned)
    source: Cow<'static, str>,
    /// Accumulated binding entries (UBOs and textures).
    binding_schema: Vec<BindingEntry>,
    /// Required vertex attributes (locations only; no buffer layout).
    vertex_schema: Vec<VertexEntry>,
}

impl ShaderBuilder {
    /// Creates an empty builder with no schema and an empty source string.
    pub fn new() -> Self {
        Self {
            source: Cow::Borrowed(""),
            binding_schema: Vec::new(),
            vertex_schema: Vec::new(),
        }
    }

    /// Sets the WGSL source.
    pub fn source(mut self, src: impl Into<Cow<'static, str>>) -> Self {
        self.source = src.into();
        self
    }

    /// Starts defining a uniform buffer at the given WGSL `@binding(slot)`.
    /// Returns a scoped `UniformBufferBuilder`; call `.finish()` to return here.
    pub fn uniform_buffer(self, name: &str, slot: u32) -> UniformBufferBuilder {
        UniformBufferBuilder {
            parent: self,
            name: name.into(),
            binding_slot: slot,
            members: Vec::new(),
            cursor: 0,
        }
    }

    /// Adds a texture binding at the given WGSL `@binding(slot)`.
    pub fn texture(mut self, name: &str, slot: u32) -> Self {
        self.binding_schema.push(BindingEntry {
            key: symbol!(name),
            name: name.into(),
            slot,
            ty: BindingType::Texture,
        });
        self
    }

    /// Adds a sampler binding at the given WGSL `@binding(slot)`.
    pub fn sampler(mut self, name: &str, slot: u32) -> Self {
        self.binding_schema.push(BindingEntry {
            key: symbol!(name),
            name: name.into(),
            slot,
            ty: BindingType::Sampler,
        });
        self
    }

    /// Registers a vertex attribute read by the shader at WGSL `@location(n)`.
    /// Only captures the shader interface; buffer layout is specified elsewhere.
    pub fn vertex_attr(
        mut self,
        name: &str,
        location: u32,
        format: wgpu::VertexFormat,
        step_mode: wgpu::VertexStepMode,
    ) -> Self {
        self.vertex_schema.push(VertexEntry {
            key: symbol!(name),
            name: name.into(),
            location,
            format,
            step_mode,
        });
        self
    }

    /// Validates the accumulated schemas and returns an immutable `Shader`.
    /// Panics on invalid schemas. Consider a `try_build` variant for fallible use.
    pub fn build(self) -> Shader {
        validate::assert_valid_schemas(&self.binding_schema, &self.vertex_schema);

        Shader::new(
            self.source,
            self.binding_schema.into_boxed_slice(),
            self.vertex_schema.into_boxed_slice(),
        )
    }
}

mod validate {
    //! Schema validation helpers for `ShaderBuilder`.
    //!
    //! Scope:
    //! - This module is private to `builder.rs` and not exported; only items
    //!   marked `pub(super)` are visible to the parent module.
    //! Responsibilities:
    //! - Ensure uniqueness of binding keys and slots across the binding schema.
    //! - Ensure uniqueness of uniform member keys within each uniform buffer.
    //! - Ensure uniqueness of vertex attribute locations and keys.
    //!
    //! Notes:
    //! - Layout/offset correctness (alignment, padding, non‑overlap) is enforced
    //!   by the builder when computing member offsets, so checks here focus on
    //!   naming/slot/location consistency.
    //! - All checks are O(n) using HashSet and run once during `build()`.

    use super::*;
    use std::collections::HashSet;

    /// Enumerates all schema validation failures we detect.
    /// Each variant carries enough context for a helpful error message.
    #[derive(Debug)]
    pub(super) enum SchemaError {
        /// Duplicate binding name/key detected across all bindings.
        DuplicateBindingKey { name: Box<str> },
        /// Two bindings share the same WGSL `@binding(slot)`.
        DuplicateBindingSlot { slot: u32 },
        /// A uniform buffer contains duplicate member keys.
        DuplicateUniformMember { ub_name: Box<str>, member: Box<str> },
        /// Two vertex attributes share the same `@location(n)`.
        DuplicateVertexLocation { location: u32 },
        /// Duplicate vertex attribute name/key detected.
        DuplicateVertexKey { name: Box<str> },
    }

    /// Human‑readable formatting used in panic messages and tests.
    impl fmt::Display for SchemaError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SchemaError::DuplicateBindingKey { name } => {
                    write!(f, "duplicate binding key: {}", name)
                }
                SchemaError::DuplicateBindingSlot { slot } => {
                    write!(f, "duplicate binding slot: {}", slot)
                }
                SchemaError::DuplicateUniformMember { ub_name, member } => write!(
                    f,
                    "duplicate uniform member '{}' in UBO '{}'",
                    member, ub_name
                ),
                SchemaError::DuplicateVertexLocation { location } => {
                    write!(f, "duplicate vertex location: {}", location)
                }
                SchemaError::DuplicateVertexKey { name } => {
                    write!(f, "duplicate vertex attribute key: {}", name)
                }
            }
        }
    }

    /// Validates both binding and vertex schemas and returns either `Ok(())`
    /// or a list of `SchemaError` describing all issues found.
    ///
    /// Use this when you want to surface errors without panicking.
    pub(super) fn validate_schemas(
        binding_schema: &[BindingEntry],
        vertex_schema: &[VertexEntry],
    ) -> Result<(), Vec<SchemaError>> {
        let mut errs = Vec::new();
        validate_bindings(binding_schema, &mut errs);
        validate_vertex_attrs(vertex_schema, &mut errs);
        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }

    /// Convenience wrapper that validates schemas and panics with a concise,
    /// multi‑line summary if any errors are found.
    ///
    /// Used by `ShaderBuilder::build()` to fail fast during development.
    pub(super) fn assert_valid_schemas(
        binding_schema: &[BindingEntry],
        vertex_schema: &[VertexEntry],
    ) {
        if let Err(errs) = validate_schemas(binding_schema, vertex_schema) {
            let msg = errs
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n  - ");
            panic!("invalid shader schema:\n  - {msg}");
        }
    }

    /// Binding‑level checks:
    /// - no duplicate binding keys,
    /// - no duplicate binding slots,
    /// - within each UBO, no duplicate uniform member keys.
    fn validate_bindings(binding_schema: &[BindingEntry], errs: &mut Vec<SchemaError>) {
        let mut binding_keys = HashSet::new();
        let mut binding_slots = HashSet::new();

        for be in binding_schema {
            if !binding_keys.insert(be.key) {
                errs.push(SchemaError::DuplicateBindingKey {
                    name: be.name.clone(),
                });
            }
            if !binding_slots.insert(be.slot) {
                errs.push(SchemaError::DuplicateBindingSlot { slot: be.slot });
            }
            if let BindingType::UniformBuffer { members, .. } = &be.ty {
                let mut member_keys = HashSet::new();
                for m in members.iter() {
                    if !member_keys.insert(m.key) {
                        errs.push(SchemaError::DuplicateUniformMember {
                            ub_name: be.name.clone(),
                            member: m.name.clone(),
                        });
                    }
                }
            }
        }
    }

    /// Vertex‑level checks:
    /// - no duplicate `@location(n)` values,
    /// - no duplicate vertex attribute keys.
    fn validate_vertex_attrs(vertex_schema: &[VertexEntry], errs: &mut Vec<SchemaError>) {
        let mut locs = HashSet::new();
        let mut va_keys = HashSet::new();

        for va in vertex_schema {
            if !locs.insert(va.location) {
                errs.push(SchemaError::DuplicateVertexLocation {
                    location: va.location,
                });
            }
            if !va_keys.insert(va.key) {
                errs.push(SchemaError::DuplicateVertexKey {
                    name: va.name.clone(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use wgpu::VertexFormat::*;
    use wgpu::VertexStepMode::*;

    #[test]
    fn test_shader_builder_basic() {
        let shader = ShaderBuilder::new()
            .source("shader code here")
            .uniform_buffer("uniforms", 0)
            .vec4f("albedo_factor")
            .float("roughness")
            .finish()
            .texture("albedo_texture", 1)
            .vertex_attr("position", 0, Float32x3, Vertex)
            .build();

        assert_eq!(shader.source(), "shader code here");
        assert_eq!(shader.binding_schema().len(), 2);
        assert_eq!(shader.vertex_schema().len(), 1);

        let meta = shader.uniform_field_meta(symbol!("albedo_factor")).unwrap();
        assert_eq!(meta.index, 0);
        assert_eq!(meta.offset, 0);

        let meta = shader.uniform_field_meta(symbol!("roughness")).unwrap();
        assert_eq!(meta.index, 0);
        assert_eq!(meta.offset, 16);

        let tmeta = shader.texture_meta(symbol!("albedo_texture")).unwrap();
        assert_eq!(tmeta.index, 1);
    }

    #[test]
    fn test_uniform_alignment_and_total_size() {
        // float (align 4) -> vec3f (align 16) -> vec4f (align 16)
        // offsets: 0, 16, 32; total_size = align_up(48, 16) = 48
        let shader = ShaderBuilder::new()
            .source("s")
            .uniform_buffer("ubo", 0)
            .float("f")
            .vec3f("v3")
            .vec4f("v4")
            .finish()
            .build();

        // Validate member offsets/sizes
        let be0 = &shader.binding_schema()[0];
        match &be0.ty {
            BindingType::UniformBuffer {
                total_size,
                members,
            } => {
                assert_eq!(*total_size, 48);
                assert_eq!(members.len(), 3);

                let m0 = &members[0];
                assert_eq!(&*m0.name, "f");
                assert_eq!(m0.offset, 0);
                assert_eq!(m0.size, 4);

                let m1 = &members[1];
                assert_eq!(&*m1.name, "v3");
                assert_eq!(m1.offset, 16);
                assert_eq!(m1.size, 12);

                let m2 = &members[2];
                assert_eq!(&*m2.name, "v4");
                assert_eq!(m2.offset, 32);
                assert_eq!(m2.size, 16);
            }
            _ => panic!("expected UniformBuffer"),
        }
    }

    #[test]
    fn test_texture_meta_indices() {
        // schema indices: [0]=ubo, [1]=texA(slot=5), [2]=texB(slot=7)
        let shader = ShaderBuilder::new()
            .source("s")
            .uniform_buffer("ubo", 0)
            .float("a")
            .finish()
            .texture("texA", 5)
            .texture("texB", 7)
            .build();

        assert_eq!(shader.binding_schema().len(), 3);

        let a = shader.texture_meta(symbol!("texA")).unwrap();
        let b = shader.texture_meta(symbol!("texB")).unwrap();
        assert_eq!(a.index, 1);
        assert_eq!(b.index, 2);

        // uniform 的 index 仍为 0
        let u = shader.uniform_field_meta(symbol!("a")).unwrap();
        assert_eq!(u.index, 0);
    }

    #[test]
    fn test_duplicate_binding_slot_panics() {
        let res = panic::catch_unwind(|| {
            let _ = ShaderBuilder::new()
                .source("s")
                .texture("t0", 1)
                .texture("t1", 1) // duplicate slot
                .build();
        });
        assert!(res.is_err());
    }

    #[test]
    fn test_duplicate_binding_key_panics() {
        let res = panic::catch_unwind(|| {
            let _ = ShaderBuilder::new()
                .source("s")
                .uniform_buffer("dup", 0)
                .float("a")
                .finish()
                .texture("dup", 1) // duplicate binding name/key
                .build();
        });
        assert!(res.is_err());
    }

    #[test]
    fn test_duplicate_uniform_member_panics() {
        let res = panic::catch_unwind(|| {
            let _ = ShaderBuilder::new()
                .source("s")
                .uniform_buffer("ubo", 0)
                .float("x")
                .float("x") // duplicate member name/key within the same UBO
                .finish()
                .build();
        });
        assert!(res.is_err());
    }

    #[test]
    fn test_duplicate_vertex_location_panics() {
        let res = panic::catch_unwind(|| {
            let _ = ShaderBuilder::new()
                .source("s")
                .vertex_attr("pos", 0, Float32x3, Vertex)
                .vertex_attr("uv", 0, Float32x2, Vertex) // duplicate location
                .build();
        });
        assert!(res.is_err());
    }

    #[test]
    fn test_various_uniform_types_offsets() {
        // Cover more types; sanity‑check that offsets increase and 16‑byte alignment across boundaries
        let shader = ShaderBuilder::new()
            .source("s")
            .uniform_buffer("ubo", 0)
            .int("i")
            .vec2u("u2")
            .vec4i("i4")
            .mat4f("m4")
            .finish()
            .build();

        let be0 = &shader.binding_schema()[0];
        match &be0.ty {
            BindingType::UniformBuffer { members, .. } => {
                assert_eq!(members[0].name.as_ref(), "i");
                assert_eq!(members[0].offset % 4, 0);

                assert_eq!(members[1].name.as_ref(), "u2");
                assert!(members[1].offset >= members[0].offset + members[0].size);

                assert_eq!(members[2].name.as_ref(), "i4");
                assert_eq!(members[2].offset % 16, 0);

                assert_eq!(members[3].name.as_ref(), "m4");
                assert_eq!(members[3].offset % 16, 0);
            }
            _ => panic!("expected UniformBuffer"),
        }
    }
}

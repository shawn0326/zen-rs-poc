//! Material data container.
//!
//! This module defines `Material`, which stores per-instance binding data that
//! aligns one-to-one with a `Shader`'s binding schema.
//!
//! Invariants:
//! - The order of `bindings` exactly matches `shader.binding_schema()`.
//! - Uniform buffers are laid out and sized by the shader builder (std140-like).
//! - All uniform bytes are zero-initialized; textures start as `None`.

mod binding_data;

pub(crate) use binding_data::*;

use crate::Symbol;
use crate::TextureHandle;
use crate::math::*;
use crate::shader::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Builds binding storage for a given shader:
/// - Uniform buffers are allocated with zeroed bytes sized by `total_size`.
/// - Texture bindings are initialized as `None`.
/// - The output order matches `shader.binding_schema()`.
fn build_material_bindings(shader: &Shader) -> Box<[MaterialBindingData]> {
    let mut bindings = Vec::new();
    for entry in shader.binding_schema() {
        let resource = match &entry.ty {
            BindingType::UniformBuffer { total_size, .. } => {
                MaterialBindingData::UniformBuffer(vec![0u8; *total_size].into_boxed_slice())
            }
            BindingType::Texture => MaterialBindingData::Texture(None),
        };
        bindings.push(resource);
    }
    bindings.into_boxed_slice()
}

/// Generates typed uniform accessors (setter/getter) with consistent docs.
/// Each setter accepts any type convertible into `$ty` via `Into`,
/// and each getter returns `$ty` by value.
macro_rules! impl_uniform_accessors {
    ($( ($set_fn:ident, $get_fn:ident, $ty:ty, $desc:literal) ),* $(,)?) => {
        $(
            #[doc = concat!("Sets a ", $desc, " uniform value.")]
            #[doc = "Notes:"]
            #[doc = concat!("- Accepts types convertible into `", stringify!($ty), "` via `Into`.")]
            #[doc = "- For borrowed data (slices), use [`set_param_raw`]."]
            #[inline]
            pub fn $set_fn<V>(&mut self, key: Symbol, value: V) -> &mut Self
            where
                V: Into<$ty>,
            {
                let v: $ty = value.into();
                self.write_uniform(key, &v)
            }

            #[doc = concat!("Gets a ", $desc, " uniform value.")]
            #[inline]
            pub fn $get_fn(&self, key: Symbol) -> $ty {
                self.read_uniform::<$ty>(key)
            }
        )*
    };
}

define_id!(MaterialId);

/// Shared, interior-mutable handle to a `Material` (`Rc<RefCell<...>>`).
pub type MaterialRcCell = Rc<RefCell<Material>>;

/// Per-instance material data aligned with a `Shader`'s binding schema.
///
/// - `shader`: the shader this material adheres to (layout/metadata source).
/// - `bindings`: per-binding storage:
///   - UniformBuffer: raw bytes sized and aligned by the builder
///   - Texture: optional texture handle
#[derive(Clone)]
pub struct Material {
    id: MaterialId,
    shader: ShaderRc,
    bindings: Box<[MaterialBindingData]>,
}

impl Material {
    /// Constructs a new material from a shader handle.
    ///
    /// - Uniform bytes are zero-initialized.
    /// - Texture slots start as `None`.
    /// - Consumes `shader` (clone at callsite if you want to keep it).
    pub fn new(shader: ShaderRc) -> Self {
        let bindings = build_material_bindings(&shader);
        Self {
            id: MaterialId::new(),
            shader,
            bindings,
        }
    }

    /// Convenience alias of [`Material::new`].
    #[inline]
    pub fn from_shader(shader: ShaderRc) -> Self {
        Self::new(shader)
    }

    /// Converts this material into `Rc<RefCell<Material>>` for shared ownership.
    #[inline]
    pub fn into_rc_cell(self) -> MaterialRcCell {
        Rc::new(RefCell::new(self))
    }

    /// Returns the material id.
    #[inline]
    pub(crate) fn id(&self) -> MaterialId {
        self.id
    }

    /// Returns the underlying shader handle.
    #[inline]
    pub fn shader(&self) -> &ShaderRc {
        &self.shader
    }

    /// Internal accessor to the binding storage (schema-aligned).
    #[inline]
    pub(crate) fn bindings(&self) -> &Box<[MaterialBindingData]> {
        &self.bindings
    }

    /// Writes a POD uniform value into its byte range.
    ///
    /// Safety/assumptions:
    /// - `key` must be a known uniform symbol in the shader.
    /// - In debug builds we assert the expected size matches `T`.
    fn write_uniform<T: bytemuck::Pod>(&mut self, key: Symbol, value: &T) -> &mut Self {
        let meta = self
            .shader
            .uniform_field_meta(key)
            .expect("unknown uniform key");

        let buf = self.bindings[meta.index].expect_uniform_buffer_mut();
        let bytes = bytemuck::bytes_of(value);
        let end = meta.offset + bytes.len();

        debug_assert!(meta.size == bytes.len());

        buf[meta.offset..end].copy_from_slice(bytes);

        self
    }

    /// Reads a POD uniform value from its byte range (by value).
    ///
    /// Safety/assumptions:
    /// - `key` must be a known uniform symbol in the shader.
    /// - In debug builds we assert the expected size matches `T`.
    fn read_uniform<T: bytemuck::Pod>(&self, key: Symbol) -> T {
        let meta = self
            .shader
            .uniform_field_meta(key)
            .expect("unknown uniform key");

        let buf = self.bindings[meta.index].expect_uniform_buffer();
        let size = core::mem::size_of::<T>();
        let end = meta.offset + size;

        debug_assert!(meta.size == size);

        bytemuck::from_bytes::<T>(&buf[meta.offset..end]).to_owned()
    }

    /// Writes a uniform as raw POD bytes (escape hatch for uncommon types).
    #[inline]
    pub fn set_param_raw<T: bytemuck::Pod>(&mut self, key: Symbol, value: &T) -> &mut Self {
        self.write_uniform(key, value)
    }

    /// Reads a uniform as raw POD bytes (escape hatch for uncommon types).
    #[inline]
    pub fn get_param_raw<T: bytemuck::Pod>(&self, key: Symbol) -> T {
        self.read_uniform::<T>(key)
    }

    impl_uniform_accessors! {
        (set_param_f, get_param_f, f32, "float"),
        (set_param_i, get_param_i, i32, "integer"),
        (set_param_u, get_param_u, u32, "unsigned integer"),
        (set_param_vec2f, get_param_vec2f, Vec2, "vec2<f32>"),
        (set_param_vec3f, get_param_vec3f, Vec3, "vec3<f32>"),
        (set_param_vec4f, get_param_vec4f, Vec4, "vec4<f32>"),
        (set_param_vec2i, get_param_vec2i, IVec2, "vec2<i32>"),
        (set_param_vec3i, get_param_vec3i, IVec3, "vec3<i32>"),
        (set_param_vec4i, get_param_vec4i, IVec4, "vec4<i32>"),
        (set_param_vec2u, get_param_vec2u, UVec2, "vec2<u32>"),
        (set_param_vec3u, get_param_vec3u, UVec3, "vec3<u32>"),
        (set_param_vec4u, get_param_vec4u, UVec4, "vec4<u32>"),
        (set_param_mat4f, get_param_mat4f, Mat4, "mat4x4<f32>"),
        (set_param_col3, get_param_col3, Color3, "Color3"),
        (set_param_col4, get_param_col4, Color4, "Color4"),
    }

    /// Assigns a texture to the specified texture binding key.
    #[inline]
    pub fn set_param_t(&mut self, key: Symbol, texture: TextureHandle) -> &mut Self {
        let meta = self.shader.texture_meta(key).expect("unknown texture key");
        if !matches!(
            self.shader.binding_schema()[meta.index].ty,
            BindingType::Texture
        ) {
            panic!("binding key is not a texture");
        }
        self.bindings[meta.index] = MaterialBindingData::Texture(Some(texture));
        self
    }

    /// Returns the texture handle stored at the binding (if any).
    #[inline]
    pub fn get_param_t(&self, key: Symbol) -> &Option<TextureHandle> {
        let meta = self.shader.texture_meta(key).expect("unknown texture key");
        self.bindings[meta.index].expect_texture()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shader::builtins::pbr_shader;

    #[test]
    fn test_build_material_resources() {
        let shader = pbr_shader();

        let mut material = Material::from_shader(shader.clone());

        assert_eq!(material.bindings.len(), 2);

        match &material.bindings[0] {
            MaterialBindingData::UniformBuffer(buffer) => {
                assert_eq!(buffer.len(), 32); // std140: vec4 + float
            }
            _ => panic!("Expected UniformBuffer"),
        }

        match &material.bindings[1] {
            MaterialBindingData::Texture(_) => {}
            _ => panic!("Expected Texture"),
        }

        material.set_param_vec4f(symbol!("albedo_factor"), Vec4::ZERO);
        let albedo: Vec4 = material.get_param_vec4f(symbol!("albedo_factor"));
        assert_eq!(albedo, Vec4::ZERO);

        material.set_param_f(symbol!("roughness"), 0.5);
        let roughness = material.get_param_f(symbol!("roughness"));
        assert_eq!(roughness, 0.5);
    }
}

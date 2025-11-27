//! Internal binding storage used by `Material`.
//!
//! This module defines per-binding payloads that align one-to-one with a
//! `Shader`'s binding schema. It is not exposed publicly; callers access it
//! indirectly through `Material`.

use crate::shader::*;
use crate::{DirtyVersion, TextureHandle, sampler::Sampler};

/// Per-binding data stored by a `Material`.
///
/// Variants mirror the shader binding types:
/// - `UniformBuffer`: raw bytes sized by the layout computed in the builder
/// - `Texture`: an optional texture handle (`None` means unbound)
#[derive(Clone)]
pub enum MaterialParameter {
    /// Raw bytes that back a uniform-buffer binding.
    /// The length equals the total size computed by the layout.
    UniformBuffer(Box<[u8]>),

    /// Texture binding stored as an optional handle.
    /// `None` indicates the texture is currently unbound.
    Texture {
        val: Option<TextureHandle>,
        ver: DirtyVersion,
    },

    /// Sampler binding stored as an optional sampler.
    /// `None` indicates the sampler is currently unbound.
    Sampler {
        val: Option<Box<Sampler>>,
        ver: DirtyVersion,
    },
}

impl MaterialParameter {
    #[inline]
    pub fn uniform_buffer<T: AsRef<[u8]>>(val: T) -> Self {
        MaterialParameter::UniformBuffer(val.as_ref().into())
    }

    #[inline]
    pub fn texture(val: Option<TextureHandle>) -> Self {
        MaterialParameter::Texture {
            val,
            ver: DirtyVersion::new(),
        }
    }

    #[inline]
    pub fn sampler(val: Option<Box<Sampler>>) -> Self {
        MaterialParameter::Sampler {
            val,
            ver: DirtyVersion::new(),
        }
    }

    /// Builds binding storage for a given shader:
    /// - Uniform buffers are allocated with zeroed bytes sized by `total_size`.
    /// - Texture bindings are initialized as `None`.
    /// - The output order matches `shader.binding_schema()`.
    pub fn from_shader(shader: &Shader) -> Box<[MaterialParameter]> {
        let mut bindings = Vec::new();
        for entry in shader.binding_schema() {
            let resource = match &entry.ty {
                BindingType::UniformBuffer { total_size, .. } => {
                    MaterialParameter::uniform_buffer(vec![0u8; *total_size])
                }
                BindingType::Texture => MaterialParameter::texture(None),
                BindingType::Sampler => MaterialParameter::sampler(None),
            };
            bindings.push(resource);
        }
        bindings.into_boxed_slice()
    }
}

impl MaterialParameter {
    /// Returns an immutable view of the uniform-buffer bytes.
    ///
    /// Panics
    /// - If this binding is not `UniformBuffer`.
    #[inline(always)]
    pub fn expect_uniform_buffer(&self) -> &[u8] {
        match self {
            MaterialParameter::UniformBuffer(b) => &b[..],
            _ => panic!("expected UniformBuffer at index"),
        }
    }

    /// Returns a mutable view of the uniform-buffer bytes.
    ///
    /// Panics
    /// - If this binding is not `UniformBuffer`.
    #[inline(always)]
    pub fn expect_uniform_buffer_mut(&mut self) -> &mut [u8] {
        match self {
            MaterialParameter::UniformBuffer(b) => &mut b[..],
            _ => panic!("expected UniformBuffer at index"),
        }
    }

    /// Returns the optional texture handle for this binding.
    ///
    /// Panics
    /// - If this binding is not `Texture`.
    #[inline(always)]
    pub fn expect_texture(&self) -> &Option<TextureHandle> {
        match self {
            MaterialParameter::Texture { val, .. } => val,
            _ => panic!("expected Texture at index"),
        }
    }

    /// Returns the optional sampler for this binding.
    ///
    /// Panics
    /// - If this binding is not `Sampler`.
    #[inline(always)]
    pub fn expect_sampler(&self) -> &Option<Box<Sampler>> {
        match self {
            MaterialParameter::Sampler { val, .. } => val,
            _ => panic!("expected Sampler at index"),
        }
    }
}

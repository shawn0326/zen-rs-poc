//! Internal binding storage used by `Material`.
//!
//! This module defines per-binding payloads that align one-to-one with a
//! `Shader`'s binding schema. It is not exposed publicly; callers access it
//! indirectly through `Material`.

use crate::{TextureHandle, sampler::Sampler};

/// Per-binding data stored by a `Material`.
///
/// Variants mirror the shader binding types:
/// - `UniformBuffer`: raw bytes sized by the layout computed in the builder
/// - `Texture`: an optional texture handle (`None` means unbound)
#[derive(Clone)]
pub enum MaterialBindingData {
    /// Raw bytes that back a uniform-buffer binding.
    /// The length equals the total size computed by the layout.
    UniformBuffer(Box<[u8]>),

    /// Texture binding stored as an optional handle.
    /// `None` indicates the texture is currently unbound.
    Texture(Option<TextureHandle>),

    /// Sampler binding stored as an optional sampler.
    /// `None` indicates the sampler is currently unbound.
    Sampler(Option<Box<Sampler>>),
}

impl MaterialBindingData {
    /// Returns an immutable view of the uniform-buffer bytes.
    ///
    /// Panics
    /// - If this binding is not `UniformBuffer`.
    #[inline(always)]
    pub fn expect_uniform_buffer(&self) -> &[u8] {
        match self {
            MaterialBindingData::UniformBuffer(b) => &b[..],
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
            MaterialBindingData::UniformBuffer(b) => &mut b[..],
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
            MaterialBindingData::Texture(t) => t,
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
            MaterialBindingData::Sampler(s) => s,
            _ => panic!("expected Sampler at index"),
        }
    }
}

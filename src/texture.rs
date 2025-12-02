mod source;

pub use source::TextureSource;

use crate::{Resource, Resources, TextureHandle};

/// Represents a CPU-side texture resource.
///
/// Stores texture source data and format information.
/// Provides constructors, builder-style configuration, and getter/setter methods.
///
/// Typical usage:
/// - Create a Texture with `Texture::new` or `Texture::default`.
/// - Configure with `with_source` / `with_format` or setters.
/// - Insert into a resource pool with `into_handle`.
#[derive(Clone, Debug)]
pub struct Texture {
    source: TextureSource,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
}

impl Resource for Texture {}

impl Default for Texture {
    fn default() -> Self {
        Self {
            source: TextureSource::default(),
            format: wgpu::TextureFormat::Rgba8UnormSrgb, // default srgb format
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
        }
    }
}

impl Texture {
    /// Consumes the texture and inserts it into the resource pool, returning a handle.
    pub fn into_handle(self, resources: &mut Resources) -> TextureHandle {
        resources.insert_texture(self)
    }
}

impl Texture {
    /// Creates a new texture with the given source and format.
    #[inline]
    pub fn new(source: TextureSource, format: wgpu::TextureFormat) -> Self {
        Self {
            source,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
        }
    }

    /// Sets the texture source in builder style.
    #[inline]
    pub fn with_source(mut self, source: TextureSource) -> Self {
        self.source = source;
        self
    }

    /// Sets the texture format in builder style.
    #[inline]
    pub fn with_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = format;
        self
    }
}

impl Texture {
    /// Sets the texture source.
    #[inline]
    pub fn set_source(&mut self, source: TextureSource) -> &mut Self {
        self.source = source;
        self
    }

    /// Returns a reference to the texture source.
    #[inline]
    pub fn source(&self) -> &TextureSource {
        &self.source
    }

    /// Sets the texture format.
    #[inline]
    pub fn set_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        self.format = format;
        self
    }

    /// Returns the texture format.
    #[inline]
    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    #[inline]
    pub fn usage(&self) -> wgpu::TextureUsages {
        self.usage
    }
}

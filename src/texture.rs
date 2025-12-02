mod data;
mod kind;

pub use data::TextureData;
pub use kind::TextureKind;

use crate::{DirtyVersion, Resource, Resources, SurfaceKey, TextureHandle};

#[derive(Clone, Debug)]
pub struct Texture {
    kind: TextureKind,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
    version: DirtyVersion,
}

impl Resource for Texture {}

impl Default for Texture {
    fn default() -> Self {
        Self {
            kind: TextureKind::default(),
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            version: DirtyVersion::new(),
        }
    }
}

impl Texture {
    pub fn into_handle(self, resources: &mut Resources) -> TextureHandle {
        resources.insert_texture(self)
    }
}

impl Texture {
    #[inline]
    pub fn new(kind: TextureKind, format: wgpu::TextureFormat, usage: wgpu::TextureUsages) -> Self {
        Self {
            kind,
            format,
            usage,
            version: DirtyVersion::new(),
        }
    }

    pub fn d2_texture(data: impl Into<Box<[u8]>>, width: u32, height: u32) -> Self {
        Self {
            kind: TextureKind::D2 {
                data: TextureData::from_bytes(data),
                width,
                height,
            },
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            version: DirtyVersion::new(),
        }
    }

    pub fn surface_texture(surface_key: SurfaceKey, width: u32, height: u32) -> Self {
        Self {
            kind: TextureKind::Surface {
                surface_key,
                width,
                height,
            },
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            version: DirtyVersion::new(),
        }
    }

    pub fn render_texture(width: u32, height: u32) -> Self {
        Self {
            kind: TextureKind::Render { width, height },
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            version: DirtyVersion::new(),
        }
    }
}

impl Texture {
    pub fn set_kind(&mut self, kind: TextureKind) -> &mut Self {
        if !self.kind.same_features(&kind) {
            self.version.bump();
        }

        self.kind = kind;

        self
    }

    #[inline]
    pub fn kind(&self) -> &TextureKind {
        &self.kind
    }

    pub fn set_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        if self.format != format {
            self.version.bump();
            self.format = format;
        }
        self
    }

    #[inline]
    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    pub fn set_usage(&mut self, usage: wgpu::TextureUsages) -> &mut Self {
        if self.usage != usage {
            self.version.bump();
            self.usage = usage;
        }
        self
    }

    #[inline]
    pub fn usage(&self) -> wgpu::TextureUsages {
        self.usage
    }
}

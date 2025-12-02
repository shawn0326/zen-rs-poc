use super::{
    Operations, RenderTarget, RenderTargetColorAttachment, RenderTargetDepthStencilAttachment,
};
use crate::texture::{Texture, TextureKind};
use crate::{Resources, SurfaceKey};

pub struct RenderTargetBuilder {
    name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    color_textures: Vec<Texture>,
    depth_stencil_texture: Option<Texture>,
}

impl RenderTargetBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            width: None,
            height: None,
            color_textures: Vec::new(),
            depth_stencil_texture: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn attach_surface(mut self, surface_key: SurfaceKey) -> Self {
        let width = self.width.expect("Width must be set");
        let height = self.height.expect("Height must be set");
        let tex = Texture::surface_texture(surface_key, width, height);
        self.color_textures.push(tex);
        self
    }

    pub fn attach_color(mut self, kind: TextureKind, format: wgpu::TextureFormat) -> Self {
        let tex = Texture::new(
            kind,
            format,
            wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
        );
        self.color_textures.push(tex);
        self
    }

    pub fn attach_depth24(mut self) -> Self {
        let width = self.width.expect("Width must be set");
        let height = self.height.expect("Height must be set");
        let tex = Texture::new(
            TextureKind::Render { width, height },
            wgpu::TextureFormat::Depth24Plus,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        self.depth_stencil_texture = Some(tex);
        self
    }

    pub fn build(self, resources: &mut Resources) -> RenderTarget {
        let width = self.width.expect("Width must be set");
        let height = self.height.expect("Height must be set");

        let color_attachments = self
            .color_textures
            .into_iter()
            .map(|tex| {
                let handle = resources.insert_texture(tex);
                RenderTargetColorAttachment {
                    texture: handle,
                    ops: Operations::default(),
                }
            })
            .collect();

        let depth_stencil_attachment = self.depth_stencil_texture.map(|tex| {
            let handle = resources.insert_texture(tex);
            RenderTargetDepthStencilAttachment {
                texture: handle,
                depth_ops: Operations::default(),
                stencil_ops: Operations::default(),
            }
        });

        let mut render_target = RenderTarget::new(
            self.name.unwrap_or_else(|| "Unnamed".to_string()),
            width,
            height,
        );

        render_target
            .set_color_attachments(color_attachments)
            .set_depth_stencil_attachment(depth_stencil_attachment);

        render_target
    }
}

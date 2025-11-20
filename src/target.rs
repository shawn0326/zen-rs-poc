mod types;

pub use types::*;

use crate::{Resources, texture::*};

#[derive(Clone)]
pub struct RenderTarget {
    pub name: String,
    width: u32,
    height: u32,
    pub color_attachments: Vec<RenderTargetColorAttachment>,
    pub depth_stencil_attachment: Option<RenderTargetDepthStencilAttachment>,
}

impl RenderTarget {
    pub fn from_surface(
        resources: &mut Resources,
        surface_id: u32,
        width: u32,
        height: u32,
    ) -> Self {
        let texture = Texture::new()
            .with_source(TextureSource::Surface {
                surface_id,
                width,
                height,
            })
            .with_format(TextureFormat::Rgba8UnormSrgb);
        let texture_handle = resources.insert_texture(texture);
        Self {
            name: format!("RT_Surface_{}", surface_id),
            width,
            height,
            color_attachments: vec![RenderTargetColorAttachment {
                texture: texture_handle,
                ops: Operations::default(),
            }],
            depth_stencil_attachment: None,
        }
    }

    pub fn with_depth24(&mut self, resources: &mut Resources) -> &mut Self {
        let depth_texture = Texture::new()
            .with_source(TextureSource::Render {
                width: self.width,
                height: self.height,
            })
            .with_format(TextureFormat::Depth24Plus);
        self.depth_stencil_attachment = Some(RenderTargetDepthStencilAttachment {
            texture: resources.insert_texture(depth_texture),
            depth_ops: Operations::default(),
            stencil_ops: Operations::default(),
        });
        self
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn resize(&mut self, resources: &mut Resources, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;

        if let Some(depth_stencil_attachment) = &self.depth_stencil_attachment {
            resources
                .get_texture_mut(depth_stencil_attachment.texture)
                .unwrap()
                .set_source(TextureSource::Render {
                    width: new_width,
                    height: new_height,
                });
        }
    }
}

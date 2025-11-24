mod builder;
mod types;

pub use builder::*;
pub use types::*;

use crate::{Resources, texture::*};

#[derive(Clone)]
pub struct RenderTarget {
    name: String,
    width: u32,
    height: u32,
    color_attachments: Vec<RenderTargetColorAttachment>,
    depth_stencil_attachment: Option<RenderTargetDepthStencilAttachment>,
}

impl RenderTarget {
    #[inline]
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            name: name.into(),
            width,
            height,
            color_attachments: Vec::new(),
            depth_stencil_attachment: None,
        }
    }
}

impl RenderTarget {
    #[inline]
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_size(
        &mut self,
        resources: &mut Resources,
        new_width: u32,
        new_height: u32,
    ) -> &mut Self {
        self.width = new_width;
        self.height = new_height;

        // todo: update textures' sources

        if let Some(depth_stencil_attachment) = &self.depth_stencil_attachment {
            resources
                .get_texture_mut(&depth_stencil_attachment.texture)
                .unwrap()
                .set_source(TextureSource::Render {
                    width: new_width,
                    height: new_height,
                });
        }

        self
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    pub fn set_color_attachments(
        &mut self,
        color_attachments: Vec<RenderTargetColorAttachment>,
    ) -> &mut Self {
        self.color_attachments = color_attachments;
        self
    }

    #[inline]
    pub fn color_attachments(&self) -> &Vec<RenderTargetColorAttachment> {
        &self.color_attachments
    }

    #[inline]
    pub fn color_attachments_mut(&mut self) -> &mut Vec<RenderTargetColorAttachment> {
        &mut self.color_attachments
    }

    #[inline]
    pub fn set_depth_stencil_attachment(
        &mut self,
        depth_stencil_attachment: Option<RenderTargetDepthStencilAttachment>,
    ) -> &mut Self {
        self.depth_stencil_attachment = depth_stencil_attachment;
        self
    }

    #[inline]
    pub fn depth_stencil_attachment(&self) -> Option<&RenderTargetDepthStencilAttachment> {
        self.depth_stencil_attachment.as_ref()
    }

    #[inline]
    pub fn depth_stencil_attachment_mut(
        &mut self,
    ) -> Option<&mut RenderTargetDepthStencilAttachment> {
        self.depth_stencil_attachment.as_mut()
    }
}

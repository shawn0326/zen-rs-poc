use std::fmt::Debug;

use super::traits::RenderTargetLike;

pub struct OffscreenRenderTarget {
    pub width: u32,
    pub height: u32,
    color_attachments: Vec<u32>,
    depth_stencil_attachment: Option<u32>,
}

impl OffscreenRenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color_attachments: Vec::new(),
            depth_stencil_attachment: None,
        }
    }

    pub fn color_attachments(&self) -> &Vec<u32> {
        &self.color_attachments
    }

    pub fn depth_stencil_attachment(&self) -> Option<u32> {
        self.depth_stencil_attachment
    }
}

impl RenderTargetLike for OffscreenRenderTarget {
    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

impl Debug for OffscreenRenderTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OffscreenRenderTarget")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("color_attachments", &self.color_attachments)
            .field("depth_stencil_attachment", &self.depth_stencil_attachment)
            .finish()
    }
}

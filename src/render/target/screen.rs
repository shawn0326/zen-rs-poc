use std::fmt::Debug;

use super::traits::ScreenSurfaceLike;

use super::traits::RenderTargetLike;

pub struct ScreenRenderTarget {
    pub surface: Box<dyn ScreenSurfaceLike>,
    pub width: u32,
    pub height: u32,
}

impl ScreenRenderTarget {
    pub fn new(surface: Box<dyn ScreenSurfaceLike>, width: u32, height: u32) -> Self {
        Self {
            surface,
            width,
            height,
        }
    }
}

impl RenderTargetLike for ScreenRenderTarget {
    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

impl Debug for ScreenRenderTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScreenRenderTarget")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

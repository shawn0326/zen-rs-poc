use super::traits::RenderTargetLike;

pub struct ScreenRenderTarget {
    pub(super) width: u32,
    pub(super) height: u32,
}

impl ScreenRenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl RenderTargetLike for ScreenRenderTarget {
    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

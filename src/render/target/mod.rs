mod offscreen;
mod screen;
mod traits;

use std::fmt::Debug;

use offscreen::OffscreenRenderTarget;
use screen::ScreenRenderTarget;
use traits::RenderTargetLike;

pub enum RenderTarget {
    Screen(ScreenRenderTarget),
    Offscreen(OffscreenRenderTarget),
}

impl RenderTarget {
    pub fn screen(width: u32, height: u32) -> Self {
        RenderTarget::Screen(ScreenRenderTarget::new(width, height))
    }

    pub fn offscreen(width: u32, height: u32) -> Self {
        RenderTarget::Offscreen(OffscreenRenderTarget::new(width, height))
    }

    pub fn width(&self) -> u32 {
        match self {
            RenderTarget::Screen(target) => target.width,
            RenderTarget::Offscreen(target) => target.width,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            RenderTarget::Screen(target) => target.height,
            RenderTarget::Offscreen(target) => target.height,
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        match self {
            RenderTarget::Screen(target) => target.set_size(width, height),
            RenderTarget::Offscreen(target) => target.set_size(width, height),
        }
    }
}

impl Debug for RenderTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderTarget::Screen(target) => {
                f.debug_tuple("RenderTarget::Screen").field(target).finish()
            }
            RenderTarget::Offscreen(target) => f
                .debug_tuple("RenderTarget::Offscreen")
                .field(target)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize() {
        let mut target = RenderTarget::screen(800, 600);
        assert_eq!(target.width(), 800);
        assert_eq!(target.height(), 600);

        target.set_size(1024, 768);
        assert_eq!(target.width(), 1024);
        assert_eq!(target.height(), 768);

        let mut offscreen_target = RenderTarget::offscreen(640, 480);
        assert_eq!(offscreen_target.width(), 640);
        assert_eq!(offscreen_target.height(), 480);

        offscreen_target.set_size(1280, 720);
        assert_eq!(offscreen_target.width(), 1280);
        assert_eq!(offscreen_target.height(), 720);
    }
}

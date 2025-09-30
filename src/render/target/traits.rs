pub(super) trait RenderTargetLike {
    fn set_size(&mut self, width: u32, height: u32);
}

pub trait ScreenSurfaceLike {
    fn get_size(&self) -> (u32, u32);
}

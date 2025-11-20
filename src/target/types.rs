use crate::{TextureHandle, math::Color4};

#[derive(Copy, Clone, Debug)]
pub enum LoadOp<V> {
    Clear(V),
    Load,
}

impl Default for LoadOp<Color4> {
    fn default() -> Self {
        Self::Clear(Color4::new(0.0, 0.0, 0.0, 1.0))
    }
}

impl Default for LoadOp<f32> {
    fn default() -> Self {
        Self::Clear(1.0)
    }
}

impl Default for LoadOp<u32> {
    fn default() -> Self {
        Self::Clear(0)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum StoreOp {
    #[default]
    Store,
    Discard,
}

#[derive(Copy, Clone, Debug)]
pub struct Operations<T> {
    pub load: LoadOp<T>,
    pub store: StoreOp,
}

impl<T> Default for Operations<T>
where
    LoadOp<T>: Default,
{
    fn default() -> Self {
        Self {
            load: LoadOp::default(),
            store: StoreOp::default(),
        }
    }
}

#[derive(Clone)]
pub struct RenderTargetColorAttachment {
    pub texture: TextureHandle,
    pub ops: Operations<Color4>,
}

#[derive(Clone)]
pub struct RenderTargetDepthStencilAttachment {
    pub texture: TextureHandle,
    pub depth_ops: Operations<f32>,
    pub stencil_ops: Operations<u32>,
}

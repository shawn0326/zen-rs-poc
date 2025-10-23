use crate::{
    graphics::{Texture, TextureRef},
    math::Color4,
};

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
    pub texture: TextureRef,
    pub ops: Operations<Color4>,
}

#[derive(Clone)]
pub struct RenderTargetDepthStencilAttachment {
    pub texture: TextureRef,
    pub depth_ops: Operations<f32>,
    pub stencil_ops: Operations<u32>,
}

#[derive(Clone)]
pub struct RenderTarget {
    pub name: String,
    width: u32,
    height: u32,
    pub color_attachments: Vec<RenderTargetColorAttachment>,
    pub depth_stencil_attachment: Option<RenderTargetDepthStencilAttachment>,
}

impl RenderTarget {
    pub fn from_surface(surface_id: u32, width: u32, height: u32) -> Self {
        let texture = Texture::from_surface(surface_id, width, height);
        Self {
            name: format!("RT_Surface_{}", surface_id),
            width,
            height,
            color_attachments: vec![RenderTargetColorAttachment {
                texture,
                ops: Operations::default(),
            }],
            depth_stencil_attachment: None,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

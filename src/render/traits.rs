use super::RenderTarget;
use crate::graphics::Primitive;

pub trait RendererLike {
    fn begin_render(target: RenderTarget);
    fn render_primitive(primitive: &Primitive);
    fn end_render();
}

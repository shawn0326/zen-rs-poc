use crate::render::RenderTarget;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn init(&self) {
        // Initialization code for wgpu would go here
    }

    pub fn begin_render(&self, _target: RenderTarget) {
        // Code to begin rendering
    }

    pub fn render_primitive(&self) {
        // Code to render a primitive
    }

    pub fn end_render(&self) {
        // Code to end rendering
    }
}

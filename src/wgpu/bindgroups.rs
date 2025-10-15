use crate::scene::Camera;

pub(super) struct BindGroups {}

impl BindGroups {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _camera: &Camera) {}
}

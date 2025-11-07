mod projection;
use crate::math::{Mat4, Vec3};
pub use projection::{Orthographic, Perspective, Projection};

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub proj: Mat4,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        return self.proj * view;
    }

    #[inline]
    pub fn set_projection<P: Projection + ?Sized>(&mut self, proj: &P) {
        self.proj = proj.to_mat4();
    }
}

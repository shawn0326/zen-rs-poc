use crate::math::Mat4;

pub trait Projection {
    fn to_mat4(&self) -> Mat4;
}

impl Projection for Mat4 {
    #[inline]
    fn to_mat4(&self) -> Mat4 {
        *self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Perspective {
    pub fovy_deg: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Perspective {
    #[inline]
    pub fn new(fovy_deg: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fovy_deg,
            aspect,
            near,
            far,
        }
    }
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            fovy_deg: 60.0,
            aspect: 1.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl Projection for Perspective {
    #[inline]
    fn to_mat4(&self) -> Mat4 {
        Mat4::perspective_rh(self.fovy_deg.to_radians(), self.aspect, self.near, self.far)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orthographic {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl Orthographic {
    #[inline]
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }
    }

    #[inline]
    pub fn from_width_height(width: f32, height: f32, near: f32, far: f32) -> Self {
        let hw = width * 0.5;
        let hh = height * 0.5;
        Self {
            left: -hw,
            right: hw,
            bottom: -hh,
            top: hh,
            near,
            far,
        }
    }
}

impl Default for Orthographic {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1.0,
        }
    }
}

impl Projection for Orthographic {
    #[inline]
    fn to_mat4(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
    }
}

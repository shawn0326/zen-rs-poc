use crate::math::Mat4;

/// Perspective projection parameters for 3D rendering.
///
/// Stores vertical field of view (in degrees), aspect ratio, near and far plane distances.
/// Use [`to_mat4`] to generate a perspective projection matrix (right-handed).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerspectiveProjection {
    /// Vertical field of view in degrees.
    pub fovy_deg: f32,
    /// Aspect ratio (width / height).
    pub aspect: f32,
    /// Near plane distance.
    pub near: f32,
    /// Far plane distance.
    pub far: f32,
}

impl PerspectiveProjection {
    /// Creates a new perspective projection.
    ///
    /// - `fovy_deg`: vertical field of view in degrees.
    /// - `aspect`: aspect ratio (width / height).
    /// - `near`: near plane distance.
    /// - `far`: far plane distance.
    #[inline]
    pub fn new(fovy_deg: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fovy_deg,
            aspect,
            near,
            far,
        }
    }

    /// Converts parameters to a perspective projection matrix (right-handed).
    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::perspective_rh(self.fovy_deg.to_radians(), self.aspect, self.near, self.far)
    }
}

impl Default for PerspectiveProjection {
    /// Returns a typical perspective projection:
    /// - fovy_deg: 60.0
    /// - aspect: 1.0
    /// - near: 0.1
    /// - far: 1000.0
    fn default() -> Self {
        Self {
            fovy_deg: 60.0,
            aspect: 1.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

/// Orthographic projection parameters for 2D/3D rendering.
///
/// Stores left/right/bottom/top bounds and near/far plane distances.
/// Use [`to_mat4`] to generate an orthographic projection matrix (right-handed).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OrthographicProjection {
    /// Left bound of the view volume.
    pub left: f32,
    /// Right bound of the view volume.
    pub right: f32,
    /// Bottom bound of the view volume.
    pub bottom: f32,
    /// Top bound of the view volume.
    pub top: f32,
    /// Near plane distance.
    pub near: f32,
    /// Far plane distance.
    pub far: f32,
}

impl OrthographicProjection {
    /// Creates a new orthographic projection.
    ///
    /// - `left`, `right`, `bottom`, `top`: bounds of the view volume.
    /// - `near`, `far`: near and far plane distances.
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

    /// Creates an orthographic projection centered at origin, given width and height.
    ///
    /// - `width`, `height`: size of the view volume.
    /// - `near`, `far`: near and far plane distances.
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

    /// Converts parameters to an orthographic projection matrix (right-handed).
    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
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

impl Default for OrthographicProjection {
    /// Returns a typical orthographic projection:
    /// - left: -1.0, right: 1.0
    /// - bottom: -1.0, top: 1.0
    /// - near: 0.0, far: 1.0
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

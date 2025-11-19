mod projection;

pub use projection::*;

use crate::math::Mat4;

/// Camera data for rendering.
///
/// Stores world transform and projection matrix.
/// Provides methods to set/get transform, projection, view, and view-projection matrices.
///
/// - `transform`: camera position and orientation in world space.
/// - `projection`: projection matrix (perspective or orthographic).
///
/// Typical usage: set transform and projection, then query view/view-projection for rendering.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Camera {
    transform: Mat4,
    projection: Mat4,
}

impl Camera {
    /// Creates a camera from transform and projection matrices.
    #[inline]
    pub fn new(transform: Mat4, projection: Mat4) -> Self {
        Self {
            transform,
            projection,
        }
    }

    /// Creates a camera with identity transform and given projection matrix.
    #[inline]
    pub fn from_projection(projection: Mat4) -> Self {
        Self {
            transform: Mat4::IDENTITY,
            projection,
        }
    }

    /// Sets the world transform (camera position/orientation).
    #[inline]
    pub fn set_transform(&mut self, transform: Mat4) -> &mut Self {
        self.transform = transform;
        self
    }

    /// Returns the world transform matrix.
    #[inline]
    pub fn transform(&self) -> Mat4 {
        self.transform
    }

    /// Sets the projection matrix.
    #[inline]
    pub fn set_projection(&mut self, projection: Mat4) -> &mut Self {
        self.projection = projection;
        self
    }

    /// Returns the projection matrix.
    #[inline]
    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    /// Sets the view matrix (camera-to-world inverse).
    /// Internally, stores the inverse as the transform.
    #[inline]
    pub fn set_view(&mut self, view: Mat4) -> &mut Self {
        self.transform = view.inverse();
        self
    }

    /// Returns the view matrix (world-to-camera).
    #[inline]
    pub fn view(&self) -> Mat4 {
        self.transform.inverse()
    }

    /// Returns the combined projection * view matrix.
    #[inline]
    pub fn view_projection(&self) -> Mat4 {
        self.projection * self.view()
    }
}

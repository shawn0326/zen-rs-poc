pub type Vec3 = glam::Vec3;
pub type Quat = glam::Quat;
pub type EulerRot = glam::EulerRot;
pub type Mat4 = glam::Mat4;

#[derive(Copy, Clone, Debug, Default)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color3 {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color4 {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

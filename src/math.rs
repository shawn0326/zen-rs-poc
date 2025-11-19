use bytemuck::{Pod, Zeroable};

pub type Vec2 = glam::Vec2;
pub type Vec3 = glam::Vec3;
pub type Vec4 = glam::Vec4;
pub type IVec2 = glam::IVec2;
pub type IVec3 = glam::IVec3;
pub type IVec4 = glam::IVec4;
pub type UVec2 = glam::UVec2;
pub type UVec3 = glam::UVec3;
pub type UVec4 = glam::UVec4;
pub type Mat4 = glam::Mat4;
pub type Quat = glam::Quat;
pub type EulerRot = glam::EulerRot;

#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
#[repr(C)]
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

#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
#[repr(C)]
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

pub trait ToMat4 {
    fn to_mat4(&self) -> Mat4;
}

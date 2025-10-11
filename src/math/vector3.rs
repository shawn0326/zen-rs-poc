use std::ops::Add;

#[derive(Default, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;
    #[inline]
    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vector3> for &Vector3 {
    type Output = Vector3;
    #[inline]
    fn add(self, other: Vector3) -> Vector3 {
        self + &other
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Self;
    #[inline]
    fn add(self, other: &Vector3) -> Self {
        &self + other
    }
}

impl Add for Vector3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        &self + &other
    }
}

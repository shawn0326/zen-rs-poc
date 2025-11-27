#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Symbol(pub u64);

pub const fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u64;
        h = h.wrapping_mul(0x100000001b3);
        i += 1;
    }
    h
}

#[macro_export]
macro_rules! symbol {
    ($lit:literal) => {
        $crate::Symbol($crate::fnv1a64($lit.as_bytes()))
    };
    ($s:expr) => {{ $crate::Symbol($crate::fnv1a64($s.as_bytes())) }};
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
pub struct DirtyVersion(u64);

impl DirtyVersion {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn bump(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

pub mod buffer;
pub mod camera;
pub mod geometry;
pub mod material;
pub mod math;
pub mod primitive;
pub mod sampler;
pub mod shader;
pub mod target;
pub mod texture;
pub mod wgpu;

mod resource;
pub use resource::*;

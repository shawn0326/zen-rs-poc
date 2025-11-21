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

pub mod buffer;
pub mod camera;
pub mod geometry;
pub mod material;
pub mod math;
pub mod primitive;
pub mod shader;
pub mod target;
pub mod texture;
pub mod wgpu;

mod resource;
pub use resource::*;

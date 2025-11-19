macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(crate) struct $name(u64);

        impl $name {
            pub fn new() -> Self {
                use std::sync::atomic::{AtomicU64, Ordering};
                static COUNTER: AtomicU64 = AtomicU64::new(1);
                Self(COUNTER.fetch_add(1, Ordering::Relaxed))
            }
        }
    };
}

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

pub mod camera;
pub mod graphics;
pub mod material;
pub mod math;
pub mod render;
pub mod scene;
pub mod shader;
pub mod wgpu;

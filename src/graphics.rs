macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) struct $name(u64);

        impl $name {
            fn new() -> Self {
                use std::sync::atomic::{AtomicU64, Ordering};
                static COUNTER: AtomicU64 = AtomicU64::new(1);
                Self(COUNTER.fetch_add(1, Ordering::Relaxed))
            }
        }
    };
}

mod geometry;
mod geometry_factory;
mod material;
mod primitive;
mod texture;
pub(crate) use geometry::GeometryId;
pub use geometry::{Attribute, AttributeKey, Geometry};
pub use material::Material;
pub(crate) use material::MaterialId;
pub use primitive::Primitive;
pub(crate) use texture::TextureId;
pub use texture::{ImageSource, Texture};

mod pool;

use crate::{
    geometry::{Geometry, VertexBuffer},
    material::Material,
    texture::Texture,
};
use pool::{ResourceHandle, ResourcePool};

pub(crate) use pool::{Resource, ResourceKey};

pub type TextureHandle = ResourceHandle<Texture>;
pub type MaterialHandle = ResourceHandle<Material>;
pub type GeometryHandle = ResourceHandle<Geometry>;
pub type VertexBufferHandle = ResourceHandle<VertexBuffer>;

#[derive(Debug, Default)]
pub struct Resources {
    pub(crate) textures: ResourcePool<Texture>,
    pub(crate) materials: ResourcePool<Material>,
    pub(crate) geometries: ResourcePool<Geometry>,
    pub(crate) vertex_buffers: ResourcePool<VertexBuffer>,
}

impl Resources {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            textures: ResourcePool::with_capacity(capacity),
            materials: ResourcePool::with_capacity(capacity),
            geometries: ResourcePool::with_capacity(capacity),
            vertex_buffers: ResourcePool::with_capacity(capacity),
        }
    }
}

macro_rules! resource_methods {
    ($ty:ident, $handle:ident, $field:ident) => {
        paste::paste! {
            #[inline]
            pub fn [<insert_ $ty:snake>](&mut self, value: $ty) -> $handle {
                self.$field.insert(value)
            }

            #[inline]
            pub fn [<get_ $ty:snake>](&self, handle: &$handle) -> Option<&$ty> {
                self.$field.get(handle)
            }

            #[inline]
            pub fn [<get_ $ty:snake _mut>](&mut self, handle: &$handle) -> Option<&mut $ty> {
                self.$field.get_mut(handle)
            }

            #[inline]
            pub fn [<remove_ $ty:snake>](&mut self, handle: $handle) -> Option<$ty> {
                self.$field.remove(handle)
            }

            #[inline]
            pub fn [<$ty:snake _len>](&self) -> usize {
                self.$field.len()
            }

            #[inline]
            pub fn [<$ty:snake _free_len>](&self) -> usize {
                self.$field.free_len()
            }
        }
    };
}

impl Resources {
    resource_methods!(Texture, TextureHandle, textures);
    resource_methods!(Material, MaterialHandle, materials);
    resource_methods!(Geometry, GeometryHandle, geometries);
    resource_methods!(VertexBuffer, VertexBufferHandle, vertex_buffers);
}

impl Resources {
    pub fn collect_garbage(&mut self) {
        if self.textures.free_len() > 0 {
            self.textures.collect_garbage();
        }
        if self.materials.free_len() > 0 {
            self.materials.collect_garbage();
        }
        if self.geometries.free_len() > 0 {
            self.geometries.collect_garbage();
        }
        if self.vertex_buffers.free_len() > 0 {
            self.vertex_buffers.collect_garbage();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Resources;
    use crate::texture::*;

    #[test]
    fn test_texture_pool() {
        let mut resources = Resources::default();
        let texture = Texture::default().with_format(TextureFormat::Rgba8UnormSrgb);
        let handle = resources.insert_texture(texture);

        let retrieved = resources.get_texture(&handle).unwrap();
        assert_eq!(retrieved.format(), TextureFormat::Rgba8UnormSrgb);

        let removed = resources.remove_texture(handle);

        assert!(removed.is_some());
        assert!(resources.textures.len() == 0);
        assert!(resources.textures.free_len() == 1);
    }
}

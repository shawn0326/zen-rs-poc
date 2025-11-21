use crate::{buffer::Buffer, geometry::Geometry, material::Material, texture::Texture};
use slotmap::{Key, SlotMap, new_key_type};

pub struct Pool<K: Key, V> {
    inner: SlotMap<K, V>,
}

impl<K: Key, V> Pool<K, V> {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: SlotMap::with_capacity_and_key(capacity),
        }
    }

    #[inline]
    pub fn insert(&mut self, value: V) -> K {
        self.inner.insert(value)
    }

    #[inline]
    pub fn get(&self, key: K) -> Option<&V> {
        self.inner.get(key)
    }

    #[inline]
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    #[inline]
    pub fn remove(&mut self, key: K) -> Option<V> {
        self.inner.remove(key)
    }
}

new_key_type! { pub struct TextureHandle; }
new_key_type! { pub struct MaterialHandle; }
new_key_type! { pub struct GeometryHandle; }
new_key_type! { pub struct BufferHandle; }

pub struct Resources {
    textures: Pool<TextureHandle, Texture>,
    materials: Pool<MaterialHandle, Material>,
    geometries: Pool<GeometryHandle, Geometry>,
    buffers: Pool<BufferHandle, Buffer>,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            textures: Pool::new(32),
            materials: Pool::new(32),
            geometries: Pool::new(32),
            buffers: Pool::new(32),
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
            pub fn [<get_ $ty:snake>](&self, handle: $handle) -> Option<&$ty> {
                self.$field.get(handle)
            }

            #[inline]
            pub fn [<get_ $ty:snake _mut>](&mut self, handle: $handle) -> Option<&mut $ty> {
                self.$field.get_mut(handle)
            }

            #[inline]
            pub fn [<remove_ $ty:snake>](&mut self, handle: $handle) -> Option<$ty> {
                self.$field.remove(handle)
            }
        }
    };
}

impl Resources {
    resource_methods!(Texture, TextureHandle, textures);
    resource_methods!(Material, MaterialHandle, materials);
    resource_methods!(Geometry, GeometryHandle, geometries);
    resource_methods!(Buffer, BufferHandle, buffers);
}

#[cfg(test)]
mod tests {
    use super::Resources;
    use crate::texture::*;

    #[test]
    fn test_pool_cpacity() {
        let pool: super::Pool<super::TextureHandle, u32> = super::Pool::new(128);
        assert_eq!(pool.inner.capacity(), 128);
    }

    #[test]
    fn test_texture_pool() {
        let mut world = Resources::default();
        let texture = Texture::default().with_format(TextureFormat::Rgba8UnormSrgb);
        let handle = world.insert_texture(texture);

        let retrieved = world.get_texture(handle).unwrap();
        assert_eq!(retrieved.format(), TextureFormat::Rgba8UnormSrgb);

        let removed = world.remove_texture(handle);
        assert!(removed.is_some());
        assert!(world.textures.get(handle).is_none());
    }
}

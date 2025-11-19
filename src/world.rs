//! WIP: World module to manage global resources like textures.

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
    pub fn get_unwrapped(&self, key: K) -> &V {
        &self.inner[key]
    }

    #[inline]
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    #[inline]
    pub fn get_mut_unwrapped(&mut self, key: K) -> &mut V {
        &mut self.inner[key]
    }

    #[inline]
    pub fn remove(&mut self, key: K) -> Option<V> {
        self.inner.remove(key)
    }
}

new_key_type! { pub struct TextureHandle; }

pub struct World {
    pub textures: Pool<TextureHandle, crate::graphics::Texture>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            textures: Pool::new(64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn test_pool_cpacity() {
        let pool: super::Pool<super::TextureHandle, u32> = super::Pool::new(128);
        assert_eq!(pool.inner.capacity(), 128);
    }

    #[test]
    fn test_texture_pool() {
        let mut world = World::default();
        let texture = crate::graphics::Texture::new()
            .with_format(crate::graphics::TextureFormat::Rgba8UnormSrgb);
        let handle = world.textures.insert(texture);

        let retrieved = world.textures.get_unwrapped(handle);
        assert_eq!(
            retrieved.format(),
            crate::graphics::TextureFormat::Rgba8UnormSrgb
        );

        let removed = world.textures.remove(handle);
        assert!(removed.is_some());
        assert!(world.textures.get(handle).is_none());
    }
}

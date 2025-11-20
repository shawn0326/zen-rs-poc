use crate::texture::Texture;
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

pub struct Resources {
    textures: Pool<TextureHandle, Texture>,
}

impl Resources {
    #[inline]
    pub fn textures(&self) -> &Pool<TextureHandle, Texture> {
        &self.textures
    }

    #[inline]
    pub fn textures_mut(&mut self) -> &mut Pool<TextureHandle, Texture> {
        &mut self.textures
    }

    #[inline]
    pub fn insert_texture(&mut self, texture: Texture) -> TextureHandle {
        self.textures.insert(texture)
    }

    #[inline]
    pub fn get_texture(&self, handle: TextureHandle) -> Option<&Texture> {
        self.textures.get(handle)
    }

    #[inline]
    pub fn get_texture_mut(&mut self, handle: TextureHandle) -> Option<&mut Texture> {
        self.textures.get_mut(handle)
    }

    #[inline]
    pub fn remove_texture(&mut self, handle: TextureHandle) -> Option<Texture> {
        self.textures.remove(handle)
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            textures: Pool::new(64),
        }
    }
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
        let texture = Texture::new().with_format(TextureFormat::Rgba8UnormSrgb);
        let handle = world.insert_texture(texture);

        let retrieved = world.get_texture(handle).unwrap();
        assert_eq!(retrieved.format(), TextureFormat::Rgba8UnormSrgb);

        let removed = world.remove_texture(handle);
        assert!(removed.is_some());
        assert!(world.textures.get(handle).is_none());
    }
}

use crate::{
    geometry::{Geometry, VertexBuffer},
    material::Material,
    texture::Texture,
};
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
new_key_type! { pub struct VertexBufferHandle; }

pub struct Resources {
    textures: Pool<TextureHandle, Texture>,
    materials: Pool<MaterialHandle, Material>,
    geometries: Pool<GeometryHandle, Geometry>,
    vertex_buffers: Pool<VertexBufferHandle, VertexBuffer>,
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

impl Resources {
    #[inline]
    pub fn materials(&self) -> &Pool<MaterialHandle, Material> {
        &self.materials
    }

    #[inline]
    pub fn materials_mut(&mut self) -> &mut Pool<MaterialHandle, Material> {
        &mut self.materials
    }

    #[inline]
    pub fn insert_material(&mut self, material: Material) -> MaterialHandle {
        self.materials.insert(material)
    }

    #[inline]
    pub fn get_material(&self, handle: MaterialHandle) -> Option<&Material> {
        self.materials.get(handle)
    }

    #[inline]
    pub fn get_material_mut(&mut self, handle: MaterialHandle) -> Option<&mut Material> {
        self.materials.get_mut(handle)
    }

    #[inline]
    pub fn remove_material(&mut self, handle: MaterialHandle) -> Option<Material> {
        self.materials.remove(handle)
    }
}

impl Resources {
    #[inline]
    pub fn geometries(&self) -> &Pool<GeometryHandle, Geometry> {
        &self.geometries
    }

    #[inline]
    pub fn geometries_mut(&mut self) -> &mut Pool<GeometryHandle, Geometry> {
        &mut self.geometries
    }

    #[inline]
    pub fn insert_geometry(&mut self, geometry: Geometry) -> GeometryHandle {
        self.geometries.insert(geometry)
    }

    #[inline]
    pub fn get_geometry(&self, handle: GeometryHandle) -> Option<&Geometry> {
        self.geometries.get(handle)
    }

    #[inline]
    pub fn get_geometry_mut(&mut self, handle: GeometryHandle) -> Option<&mut Geometry> {
        self.geometries.get_mut(handle)
    }

    #[inline]
    pub fn remove_geometry(&mut self, handle: GeometryHandle) -> Option<Geometry> {
        self.geometries.remove(handle)
    }
}

impl Resources {
    #[inline]
    pub fn vertex_buffers(&self) -> &Pool<VertexBufferHandle, VertexBuffer> {
        &self.vertex_buffers
    }

    #[inline]
    pub fn vertex_buffers_mut(&mut self) -> &mut Pool<VertexBufferHandle, VertexBuffer> {
        &mut self.vertex_buffers
    }

    #[inline]
    pub fn insert_vertex_buffer(&mut self, vertex_buffer: VertexBuffer) -> VertexBufferHandle {
        self.vertex_buffers.insert(vertex_buffer)
    }

    #[inline]
    pub fn get_vertex_buffer(&self, handle: VertexBufferHandle) -> Option<&VertexBuffer> {
        self.vertex_buffers.get(handle)
    }

    #[inline]
    pub fn get_vertex_buffer_mut(
        &mut self,
        handle: VertexBufferHandle,
    ) -> Option<&mut VertexBuffer> {
        self.vertex_buffers.get_mut(handle)
    }

    #[inline]
    pub fn remove_vertex_buffer(&mut self, handle: VertexBufferHandle) -> Option<VertexBuffer> {
        self.vertex_buffers.remove(handle)
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            textures: Pool::new(32),
            materials: Pool::new(32),
            geometries: Pool::new(32),
            vertex_buffers: Pool::new(32),
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

use slotmap::{SlotMap, new_key_type};
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

new_key_type! { pub struct ResourceKey; }

pub trait Resource {}

pub struct ResourceHandle<R: Resource> {
    raw: ResourceKey,
    refcount: Arc<dyn Fn(ResourceKey) + Send + Sync>,
    _phantom: PhantomData<R>,
}

impl<R: Resource> ResourceHandle<R> {
    #[inline]
    pub fn raw(&self) -> ResourceKey {
        self.raw
    }
}

impl<R: Resource> Drop for ResourceHandle<R> {
    fn drop(&mut self) {
        if Arc::strong_count(&self.refcount) == 1 {
            (self.refcount)(self.raw);
        }
    }
}

impl<R: Resource> Clone for ResourceHandle<R> {
    fn clone(&self) -> Self {
        Self {
            refcount: self.refcount.clone(),
            raw: self.raw,
            _phantom: self._phantom,
        }
    }
}

impl<R: Resource> PartialEq for ResourceHandle<R> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<R: Resource> Eq for ResourceHandle<R> {}

impl<R: Resource> Hash for ResourceHandle<R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<R: Resource> Debug for ResourceHandle<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ResourceHandle")
            .field("raw", &self.raw)
            .finish()
    }
}

pub struct ResourcePool<R: Resource> {
    inner: SlotMap<ResourceKey, R>,
    free_list: Arc<Mutex<Vec<ResourceKey>>>,
}

impl<R: Resource> Debug for ResourcePool<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ResourcePool")
            .field("len", &self.inner.len())
            .field("free_len", &self.free_list.lock().unwrap().len())
            .finish()
    }
}

impl<R: Resource> Default for ResourcePool<R> {
    fn default() -> Self {
        Self {
            inner: SlotMap::with_key(),
            free_list: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<R: Resource> ResourcePool<R> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: SlotMap::with_capacity_and_key(capacity),
            free_list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn insert(&mut self, value: R) -> ResourceHandle<R> {
        let free_list = self.free_list.clone();
        ResourceHandle {
            raw: self.inner.insert(value),
            refcount: Arc::new(move |raw: ResourceKey| {
                free_list.lock().unwrap().push(raw);
            }),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self, handle: &ResourceHandle<R>) -> Option<&R> {
        self.inner.get(handle.raw())
    }

    #[inline]
    pub fn get_mut(&mut self, handle: &ResourceHandle<R>) -> Option<&mut R> {
        self.inner.get_mut(handle.raw())
    }

    #[inline]
    pub fn remove(&mut self, handle: ResourceHandle<R>) -> Option<R> {
        self.inner.remove(handle.raw())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn free_len(&self) -> usize {
        self.free_list.lock().unwrap().len()
    }

    pub fn for_each_free<F: FnMut(ResourceKey)>(&self, mut f: F) {
        for &key in self.free_list.lock().unwrap().iter() {
            f(key);
        }
    }

    pub fn collect_garbage(&mut self) {
        let mut free_list = self.free_list.lock().unwrap();
        for key in free_list.drain(..) {
            self.inner.remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Resource, ResourcePool};

    struct TestResource(u32);
    impl Resource for TestResource {}

    #[test]
    fn test_pool() {
        let mut pool: ResourcePool<TestResource> = ResourcePool::with_capacity(128);

        assert_eq!(pool.inner.capacity(), 128);

        {
            let handle = pool.insert(TestResource(0));

            assert_eq!(pool.len(), 1);
            assert_eq!(pool.free_len(), 0);

            let res = pool.get(&handle).unwrap();
            assert_eq!(res.0, 0);
        }

        assert_eq!(pool.len(), 1);
        assert_eq!(pool.free_len(), 1);

        pool.collect_garbage();

        assert_eq!(pool.len(), 0);
        assert_eq!(pool.free_len(), 0);
    }
}

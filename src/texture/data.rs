use crate::DirtyVersion;
use std::fmt;

#[derive(Clone)]
pub struct TextureData {
    bytes: Box<[u8]>,
    version: DirtyVersion,
}

impl fmt::Debug for TextureData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TextureData {{ bytes_len: {} }}", self.bytes.len())
    }
}

impl TextureData {
    pub fn from_bytes(bytes: impl Into<Box<[u8]>>) -> Self {
        Self {
            bytes: bytes.into(),
            version: DirtyVersion::new(),
        }
    }
}

impl TextureData {
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    #[inline]
    pub fn bytes_len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub fn mark_dirty(&mut self) {
        self.version.bump();
    }
}

impl TextureData {
    #[inline]
    pub(crate) fn ver(&self) -> u64 {
        self.version.as_u64()
    }
}

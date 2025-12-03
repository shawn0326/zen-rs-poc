use crate::SurfaceKey;
use crate::texture::TextureData;
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub enum TextureKind {
    Empty,
    D1 {
        data: TextureData,
        width: u32,
    },
    D2 {
        data: TextureData,
        width: u32,
        height: u32,
    },
    D3 {
        data: TextureData,
        width: u32,
        height: u32,
        depth: u32,
    },
    Cube {
        data: TextureData,
        size: u32,
    },
    Surface {
        surface_key: SurfaceKey,
        width: u32,
        height: u32,
    },
    Render {
        width: u32,
        height: u32,
    },
}

impl Default for TextureKind {
    fn default() -> Self {
        TextureKind::Empty
    }
}

impl TextureKind {
    pub fn dimensions(&self) -> (u32, u32, u32) {
        use TextureKind::*;
        match self {
            D1 { width, .. } => (*width, 1, 1),
            D2 { width, height, .. } => (*width, *height, 1),
            D3 {
                width,
                height,
                depth,
                ..
            } => (*width, *height, *depth),
            Cube { size, .. } => (*size, *size, 1),
            Surface { width, height, .. } => (*width, *height, 1),
            Render { width, height } => (*width, *height, 1),
            Empty => (0, 0, 0),
        }
    }

    pub fn data_mut(&mut self) -> Option<&mut TextureData> {
        use TextureKind::*;
        match self {
            D1 { data, .. } | D2 { data, .. } | D3 { data, .. } | Cube { data, .. } => Some(data),
            _ => None,
        }
    }
}

impl TextureKind {
    pub(crate) fn features(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        use TextureKind::*;
        match self {
            Empty => {
                0u8.hash(&mut hasher);
            }
            D1 { data, width } => {
                1u8.hash(&mut hasher);
                data.bytes_len().hash(&mut hasher);
                width.hash(&mut hasher);
            }
            D2 {
                data,
                width,
                height,
            } => {
                2u8.hash(&mut hasher);
                data.bytes_len().hash(&mut hasher);
                width.hash(&mut hasher);
                height.hash(&mut hasher);
            }
            D3 {
                data,
                width,
                height,
                depth,
            } => {
                3u8.hash(&mut hasher);
                data.bytes_len().hash(&mut hasher);
                width.hash(&mut hasher);
                height.hash(&mut hasher);
                depth.hash(&mut hasher);
            }
            Cube { data, size } => {
                4u8.hash(&mut hasher);
                data.bytes_len().hash(&mut hasher);
                size.hash(&mut hasher);
            }
            Surface {
                surface_key,
                width,
                height,
            } => {
                5u8.hash(&mut hasher);
                surface_key.hash(&mut hasher);
                width.hash(&mut hasher);
                height.hash(&mut hasher);
            }
            Render { width, height } => {
                6u8.hash(&mut hasher);
                width.hash(&mut hasher);
                height.hash(&mut hasher);
            }
        }

        hasher.finish()
    }

    pub(crate) fn same_features(&self, other: &Self) -> bool {
        self.features() == other.features()
    }
}

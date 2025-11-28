use crate::{SurfaceKey, buffer::BufferSlice};

/// Describes the source data or allocation type for a texture.
///
/// Variants cover common texture types:
/// - `D1`, `D2`, `D3`: 1D, 2D, and 3D textures with raw data and dimensions.
/// - `Cube`: Cubemap texture with raw data and size.
/// - `Surface`: Texture sourced from a window surface (for swapchain).
/// - `Render`: Texture allocated for GPU rendering (no initial data).
/// - `Empty`: Uninitialized or placeholder texture.
///
/// Used to specify how a texture should be created or uploaded.
#[derive(Clone, Debug)]
pub enum TextureSource {
    D1 {
        buffer_slice: BufferSlice,
        width: u32,
    },
    D2 {
        buffer_slice: BufferSlice,
        width: u32,
        height: u32,
    },
    D3 {
        buffer_slice: BufferSlice,
        width: u32,
        height: u32,
        depth: u32,
    },
    Cube {
        buffer_slice: BufferSlice,
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
    Empty,
}

impl Default for TextureSource {
    fn default() -> Self {
        TextureSource::Empty
    }
}

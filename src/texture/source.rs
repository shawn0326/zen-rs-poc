use std::fmt;

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
#[derive(Clone)]
pub enum TextureSource {
    D1 {
        data: Vec<u8>,
        width: u32,
    },
    D2 {
        data: Vec<u8>,
        width: u32,
        height: u32,
    },
    D3 {
        data: Vec<u8>,
        width: u32,
        height: u32,
        depth: u32,
    },
    Cube {
        data: Vec<u8>,
        size: u32,
    },
    Surface {
        surface_id: u32,
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

impl fmt::Debug for TextureSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureSource::D1 { width, data } => {
                write!(
                    f,
                    "D1 {{ width: {}, data: [{} bytes] ... }}",
                    width,
                    data.len()
                )
            }
            TextureSource::D2 {
                width,
                height,
                data,
            } => {
                write!(
                    f,
                    "D2 {{ width: {}, height: {}, data: [{} bytes] ... }}",
                    width,
                    height,
                    data.len()
                )
            }
            TextureSource::D3 {
                width,
                height,
                depth,
                data,
            } => {
                write!(
                    f,
                    "D3 {{ width: {}, height: {}, depth: {}, data: [{} bytes] ... }}",
                    width,
                    height,
                    depth,
                    data.len()
                )
            }
            TextureSource::Cube { size, data } => {
                write!(
                    f,
                    "Cube {{ size: {}, data: [{} bytes] ... }}",
                    size,
                    data.len()
                )
            }
            TextureSource::Surface {
                surface_id,
                width,
                height,
            } => {
                write!(
                    f,
                    "Surface {{ surface_id: {}, width: {}, height: {} }}",
                    surface_id, width, height
                )
            }
            TextureSource::Render { width, height } => {
                write!(f, "Render {{ width: {}, height: {} }}", width, height)
            }
            TextureSource::Empty => write!(f, "Empty"),
        }
    }
}

use crate::SurfaceKey;

#[derive(Clone, Debug)]
pub enum TextureSource {
    D1 {
        bytes: Box<[u8]>,
        width: u32,
    },
    D2 {
        bytes: Box<[u8]>,
        width: u32,
        height: u32,
    },
    D3 {
        bytes: Box<[u8]>,
        width: u32,
        height: u32,
        depth: u32,
    },
    Cube {
        bytes: Box<[u8]>,
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

impl TextureSource {
    pub fn size(&self) -> (u32, u32, u32) {
        use TextureSource::*;
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
}

use std::cell::RefCell;
use std::rc::Rc;

define_id!(TextureId);

pub type TextureRef = Rc<RefCell<Texture>>;

#[derive(Clone)]
pub struct Texture1DData {
    pub data: Vec<u8>,
}

#[derive(Clone)]
pub struct Texture2DData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct Texture3DData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Clone)]
pub struct TextureCubeData {
    pub data: Vec<u8>,
    pub size: u32,
}

#[derive(Clone)]
pub struct SurfaceTextureRef {
    pub surface_id: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub enum TextureSource {
    D1(Texture1DData),
    D2(Texture2DData),
    D3(Texture3DData),
    Cube(TextureCubeData),
    Surface(SurfaceTextureRef),
    Empty,
}

pub struct Texture {
    id: TextureId,
    source: TextureSource,
}

impl Texture {
    pub fn new() -> TextureRef {
        Rc::new(RefCell::new(Self {
            id: TextureId::new(),
            source: TextureSource::Empty,
        }))
    }

    pub fn from_2d_data(data: Vec<u8>, width: u32, height: u32) -> TextureRef {
        Rc::new(RefCell::new(Self {
            id: TextureId::new(),
            source: TextureSource::D2(Texture2DData {
                data,
                width,
                height,
            }),
        }))
    }

    pub fn from_surface(surface_id: u32, width: u32, height: u32) -> TextureRef {
        Rc::new(RefCell::new(Self {
            id: TextureId::new(),
            source: TextureSource::Surface(SurfaceTextureRef {
                surface_id,
                width,
                height,
            }),
        }))
    }

    pub(crate) fn id(&self) -> TextureId {
        self.id
    }

    pub fn source(&self) -> &TextureSource {
        &self.source
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self {
            id: TextureId::new(),
            source: self.source.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_clone() {
        let tex1 = Texture::from_2d_data(vec![255; 16 * 16 * 4], 16, 16);
        let tex1_id = tex1.borrow().id();
        let tex2 = tex1.borrow().clone();
        let tex2_id = tex2.id();

        assert_ne!(tex1_id, tex2_id, "Cloned texture should have a new ID");
    }
}

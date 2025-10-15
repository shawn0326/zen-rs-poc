use std::cell::RefCell;
use std::rc::Rc;

define_id!(TextureId);

#[non_exhaustive]
pub struct Texture {
    id: TextureId,
    source: Option<ImageSource>,
}

impl Texture {
    pub fn from_data(data: Vec<u8>, (width, height): (u32, u32)) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id: TextureId::new(),
            source: Some(ImageSource {
                data,
                width,
                height,
            }),
        }))
    }

    pub(crate) fn id(&self) -> TextureId {
        self.id
    }

    pub fn source(&self) -> Option<&ImageSource> {
        self.source.as_ref()
    }
}

pub struct ImageSource {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

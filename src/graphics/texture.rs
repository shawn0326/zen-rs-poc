mod format;
mod source;
pub use format::TextureFormat;
pub use source::TextureSource;
use std::cell::RefCell;
use std::rc::Rc;

define_id!(TextureId);

pub type TextureRef = Rc<RefCell<Texture>>;

pub struct Texture {
    id: TextureId,
    source: TextureSource,
    format: TextureFormat,
}

impl Texture {
    pub fn new() -> Self {
        Self {
            id: TextureId::new(),
            source: TextureSource::Empty,
            format: TextureFormat::Rgba8UnormSrgb,
        }
    }

    pub fn with_source(mut self, source: TextureSource) -> Self {
        self.source = source;
        self
    }

    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.format = format;
        self
    }

    pub fn into_ref(self) -> TextureRef {
        Rc::new(RefCell::new(self))
    }

    pub(crate) fn id(&self) -> TextureId {
        self.id
    }

    pub fn set_source(&mut self, source: TextureSource) -> &mut Self {
        self.source = source;
        self
    }

    pub fn source(&self) -> &TextureSource {
        &self.source
    }

    pub fn set_format(&mut self, format: TextureFormat) -> &mut Self {
        self.format = format;
        self
    }

    pub fn format(&self) -> TextureFormat {
        self.format
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self {
            id: TextureId::new(),
            source: self.source.clone(),
            format: self.format,
        }
    }
}

mod format;
mod source;
pub use format::TextureFormat;
pub use source::TextureSource;

pub struct Texture {
    source: TextureSource,
    format: TextureFormat,
}

impl Texture {
    pub fn new() -> Self {
        Self {
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
            source: self.source.clone(),
            format: self.format,
        }
    }
}

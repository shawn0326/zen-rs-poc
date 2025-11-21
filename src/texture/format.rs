/// Enumerates supported texture formats for GPU resources.
///
/// Used to specify how texture data is interpreted and stored on the GPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureFormat {
    Bgra8UnormSrgb,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Rgba8Unorm,
    Depth24Plus,
    Depth24PlusStencil8,
}

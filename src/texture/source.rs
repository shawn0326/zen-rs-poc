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

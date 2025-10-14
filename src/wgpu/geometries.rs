use wgpu::util::DeviceExt;

const POSITIONS: &[f32] = &[
    -0.0868241,
    0.49240386,
    0.0,
    -0.49513406,
    0.06958647,
    0.0,
    -0.21918549,
    -0.44939706,
    0.0,
    0.35966998,
    -0.3473291,
    0.0,
    0.44147372,
    0.2347359,
    0.0,
];

const COLORS: &[f32] = &[
    0.5, 0.0, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5,
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub(super) struct Geometries {
    pub positions_buffer: wgpu::Buffer,
    pub colors_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Geometries {
    pub fn new(device: &wgpu::Device) -> Self {
        let positions_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(POSITIONS),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let colors_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(COLORS),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        Self {
            positions_buffer,
            colors_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn desc<'a>() -> [wgpu::VertexBufferLayout<'a>; 2] {
        [
            // Buffer 0: positions
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                }],
            },
            // Buffer 1: colors
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }],
            },
        ]
    }
}

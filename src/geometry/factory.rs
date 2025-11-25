use wgpu::{BufferUsages, IndexFormat, VertexFormat, VertexStepMode};

use super::{Geometry, IndexBuffer, VertexAttribute, VertexBuffer};
use crate::{
    Resources,
    buffer::{Buffer, BufferSlice},
    symbol,
};

fn create_buffer<T: bytemuck::Pod>(data: Vec<T>, usage: BufferUsages) -> Buffer {
    let byte_data = bytemuck::cast_slice(&data);
    Buffer::new(byte_data.to_vec(), usage)
}

fn create_vertex_attribute_from_f32v(
    resources: &mut Resources,
    data: Vec<f32>,
    format: VertexFormat,
) -> VertexAttribute {
    let buffer_handle =
        create_buffer(data, BufferUsages::VERTEX | BufferUsages::COPY_DST).into_handle(resources);
    let buffer_slice = BufferSlice::from_entire_buffer(resources, buffer_handle);
    VertexAttribute {
        vertex_buffer: VertexBuffer {
            buffer_slice,
            stride: format.size() as u64,
            step_mode: VertexStepMode::Vertex,
        },
        byte_offset: 0,
        format,
    }
}

fn create_index_buffer_from_u32v(data: Vec<u32>, resources: &mut Resources) -> IndexBuffer {
    let buffer_handle =
        create_buffer(data, BufferUsages::INDEX | BufferUsages::COPY_DST).into_handle(resources);
    let buffer_slice = BufferSlice::from_entire_buffer(resources, buffer_handle);
    IndexBuffer {
        buffer_slice,
        format: IndexFormat::Uint32,
    }
}

impl Geometry {
    pub fn create_unit_quad(resources: &mut Resources) -> Geometry {
        let positions_attr = create_vertex_attribute_from_f32v(
            resources,
            vec![
                -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
            ],
            VertexFormat::Float32x3,
        );

        let tex_coords_attr = create_vertex_attribute_from_f32v(
            resources,
            vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            VertexFormat::Float32x2,
        );

        let color_attr = create_vertex_attribute_from_f32v(
            resources,
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            VertexFormat::Float32x3,
        );

        let index_buffer = create_index_buffer_from_u32v(vec![0, 1, 2, 2, 3, 0], resources);

        Self::new()
            .with_attribute(symbol!("positions"), positions_attr)
            .with_attribute(symbol!("tex_coords"), tex_coords_attr)
            .with_attribute(symbol!("colors"), color_attr)
            .with_indices(index_buffer)
    }

    pub fn create_unit_cube(resources: &mut crate::Resources) -> Geometry {
        Self::create_box(resources, (1.0, 1.0, 1.0))
    }

    pub fn create_box(
        resources: &mut crate::Resources,
        (width, height, depth): (f32, f32, f32),
    ) -> Geometry {
        let hw = width * 0.5;
        let hh = height * 0.5;
        let hd = depth * 0.5;

        let positions = vec![
            -hw, -hh, hd, hw, -hh, hd, hw, hh, hd, -hw, hh, hd, // Front (+Z)
            hw, -hh, -hd, -hw, -hh, -hd, -hw, hh, -hd, hw, hh, -hd, // Back (-Z)
            -hw, -hh, -hd, -hw, -hh, hd, -hw, hh, hd, -hw, hh, -hd, // Left (-X)
            hw, -hh, hd, hw, -hh, -hd, hw, hh, -hd, hw, hh, hd, // Right (+X)
            -hw, hh, hd, hw, hh, hd, hw, hh, -hd, -hw, hh, -hd, // Top (+Y)
            -hw, -hh, -hd, hw, -hh, -hd, hw, -hh, hd, -hw, -hh, hd, // Bottom (-Y)
        ];

        let mut tex_coords = Vec::with_capacity(6 * 4 * 2);
        for _ in 0..6 {
            tex_coords.extend_from_slice(&[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0]);
        }

        let mut colors = Vec::with_capacity(6 * 4 * 3);
        for _ in 0..(6 * 4) {
            colors.extend_from_slice(&[1.0, 1.0, 1.0]);
        }

        let mut indices = Vec::with_capacity(6 * 6);
        for face in 0..6 {
            let base = face * 4;
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }

        let positions_attr =
            create_vertex_attribute_from_f32v(resources, positions, VertexFormat::Float32x3);
        let tex_coords_attr =
            create_vertex_attribute_from_f32v(resources, tex_coords, VertexFormat::Float32x2);
        let colors_attr =
            create_vertex_attribute_from_f32v(resources, colors, VertexFormat::Float32x3);
        let indices = create_index_buffer_from_u32v(indices, resources);

        Self::new()
            .with_attribute(symbol!("positions"), positions_attr)
            .with_attribute(symbol!("tex_coords"), tex_coords_attr)
            .with_attribute(symbol!("colors"), colors_attr)
            .with_indices(indices)
    }
}

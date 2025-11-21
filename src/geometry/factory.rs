use super::{Attribute, Geometry, VertexBuffer};
use crate::{buffer::Buffer, symbol};

impl Geometry {
    pub fn create_unit_quad(resources: &mut crate::Resources) -> Geometry {
        let buffer = Buffer {
            data: vec![
                -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
            ],
        };
        let len = buffer.len();

        let positions_attr = Attribute {
            vertex_buffer: VertexBuffer {
                buffer: buffer.into_handle(resources),
                stride: 3,
                len,
            },
            offset: 0,
            component: 3,
        };

        let buffer = Buffer {
            data: vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
        };
        let len = buffer.len();

        let tex_coords_attr = Attribute {
            vertex_buffer: VertexBuffer {
                buffer: buffer.into_handle(resources),
                stride: 2,
                len,
            },
            offset: 0,
            component: 2,
        };

        let buffer = Buffer {
            data: vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        };
        let len = buffer.len();

        let colors_attr = Attribute {
            vertex_buffer: VertexBuffer {
                buffer: buffer.into_handle(resources),
                stride: 3,
                len,
            },
            offset: 0,
            component: 3,
        };

        Self::new()
            .with_attribute(symbol!("positions"), positions_attr)
            .with_attribute(symbol!("tex_coords"), tex_coords_attr)
            .with_attribute(symbol!("colors"), colors_attr)
            .with_indices(vec![0, 1, 2, 2, 3, 0])
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

        let buffer = Buffer { data: positions };
        let len = buffer.len();

        let positions_attr = Attribute {
            vertex_buffer: VertexBuffer {
                buffer: buffer.into_handle(resources),
                stride: 3,
                len,
            },
            offset: 0,
            component: 3,
        };

        let buffer = Buffer { data: tex_coords };
        let len = buffer.len();

        let tex_coords_attr = Attribute {
            vertex_buffer: VertexBuffer {
                buffer: buffer.into_handle(resources),
                stride: 2,
                len,
            },
            offset: 0,
            component: 2,
        };

        let buffer = Buffer { data: colors };
        let len = buffer.len();

        let colors_attr = Attribute {
            vertex_buffer: VertexBuffer {
                buffer: buffer.into_handle(resources),
                stride: 3,
                len,
            },
            offset: 0,
            component: 3,
        };

        Self::new()
            .with_attribute(symbol!("positions"), positions_attr)
            .with_attribute(symbol!("tex_coords"), tex_coords_attr)
            .with_attribute(symbol!("colors"), colors_attr)
            .with_indices(indices)
    }
}

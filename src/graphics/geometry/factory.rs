use super::{Attribute, AttributeKey, Geometry, GeometryRef, VertexBuffer};

impl Geometry {
    pub fn create_unit_quad() -> GeometryRef {
        let positions_buffer = VertexBuffer::new()
            .with_data(vec![
                -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
            ])
            .with_stride(3)
            .into_ref();
        let positions = Attribute::new()
            .with_buffer(positions_buffer)
            .with_components(3);

        let tex_coords_buffer = VertexBuffer::new()
            .with_data(vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0])
            .with_stride(2)
            .into_ref();
        let tex_coords = Attribute::new()
            .with_buffer(tex_coords_buffer)
            .with_components(2);

        let colors_buffer = VertexBuffer::new()
            .with_data(vec![
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            ])
            .with_stride(3)
            .into_ref();
        let colors = Attribute::new()
            .with_buffer(colors_buffer)
            .with_components(3);

        Self::new()
            .with_attribute(AttributeKey::Positions, positions)
            .with_attribute(AttributeKey::TexCoords, tex_coords)
            .with_attribute(AttributeKey::Colors, colors)
            .with_indices(vec![0, 1, 2, 2, 3, 0])
            .into_ref()
    }

    pub fn create_unit_cube() -> GeometryRef {
        Self::create_box((1.0, 1.0, 1.0))
    }

    pub fn create_box((width, height, depth): (f32, f32, f32)) -> GeometryRef {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let hd = depth / 2.0;

        let positions_buffer = VertexBuffer::new()
            .with_data(vec![
                -hw, -hh, -hd, hw, -hh, -hd, hw, hh, -hd, -hw, hh, -hd, // Back face
                -hw, -hh, hd, hw, -hh, hd, hw, hh, hd, -hw, hh, hd, // Front face
                -hw, hh, -hd, hw, hh, -hd, hw, hh, hd, -hw, hh, hd, // Top face
                -hw, -hh, -hd, hw, -hh, -hd, hw, -hh, hd, -hw, -hh, hd, // Bottom face
                -hw, -hh, -hd, -hw, hh, -hd, -hw, hh, hd, -hw, -hh, hd, // Left face
                hw, -hh, -hd, hw, hh, -hd, hw, hh, hd, hw, -hh, hd, // Right face
            ])
            .with_stride(3)
            .into_ref();
        let positions = Attribute::new()
            .with_buffer(positions_buffer)
            .with_components(3);

        let tex_coords_buffer = VertexBuffer::new()
            .with_data(vec![
                0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
            ])
            .with_stride(2)
            .into_ref();
        let tex_coords = Attribute::new()
            .with_buffer(tex_coords_buffer)
            .with_components(2);

        let colors_buffer = VertexBuffer::new()
            .with_data(vec![
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            ])
            .with_stride(3)
            .into_ref();
        let colors = Attribute::new()
            .with_buffer(colors_buffer)
            .with_components(3);

        let indices = vec![
            0, 1, 2, 2, 3, 0, // Back face
            4, 5, 6, 6, 7, 4, // Front face
            8, 9, 10, 10, 11, 8, // Top face
            12, 13, 14, 14, 15, 12, // Bottom face
            16, 17, 18, 18, 19, 16, // Left face
            20, 21, 22, 22, 23, 20, // Right face
        ];

        Self::new()
            .with_attribute(AttributeKey::Positions, positions)
            .with_attribute(AttributeKey::TexCoords, tex_coords)
            .with_attribute(AttributeKey::Colors, colors)
            .with_indices(indices)
            .into_ref()
    }
}

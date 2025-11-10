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

        let positions_buffer = VertexBuffer::new()
            .with_data(positions)
            .with_stride(3)
            .into_ref();
        let tex_coords_buffer = VertexBuffer::new()
            .with_data(tex_coords)
            .with_stride(2)
            .into_ref();
        let colors_buffer = VertexBuffer::new()
            .with_data(colors)
            .with_stride(3)
            .into_ref();

        let positions_attr = Attribute::new()
            .with_buffer(positions_buffer)
            .with_components(3);
        let tex_coords_attr = Attribute::new()
            .with_buffer(tex_coords_buffer)
            .with_components(2);
        let colors_attr = Attribute::new()
            .with_buffer(colors_buffer)
            .with_components(3);

        Self::new()
            .with_attribute(AttributeKey::Positions, positions_attr)
            .with_attribute(AttributeKey::TexCoords, tex_coords_attr)
            .with_attribute(AttributeKey::Colors, colors_attr)
            .with_indices(indices)
            .into_ref()
    }
}

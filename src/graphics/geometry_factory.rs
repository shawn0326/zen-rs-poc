use super::{Attribute, AttributeKey, Geometry};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl Geometry {
    pub fn create_unit_quad() -> Rc<RefCell<Self>> {
        let attributes: HashMap<AttributeKey, Attribute> = HashMap::from([
            (
                AttributeKey::Positions,
                Attribute {
                    data: vec![
                        -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
                    ],
                    components: 3,
                },
            ),
            (
                AttributeKey::TexCoords,
                Attribute {
                    data: vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
                    components: 2,
                },
            ),
            (
                AttributeKey::Colors,
                Attribute {
                    data: vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                    components: 3,
                },
            ),
        ]);

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(attributes, indices)
    }

    pub fn create_unit_cube() -> Rc<RefCell<Self>> {
        Self::create_box((1.0, 1.0, 1.0))
    }

    pub fn create_box((width, height, depth): (f32, f32, f32)) -> Rc<RefCell<Self>> {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let hd = depth / 2.0;

        let attributes: HashMap<AttributeKey, Attribute> = HashMap::from([
            (
                AttributeKey::Positions,
                Attribute {
                    data: vec![
                        -hw, -hh, -hd, hw, -hh, -hd, hw, hh, -hd, -hw, hh, -hd, // Back face
                        -hw, -hh, hd, hw, -hh, hd, hw, hh, hd, -hw, hh, hd, // Front face
                        -hw, hh, -hd, hw, hh, -hd, hw, hh, hd, -hw, hh, hd, // Top face
                        -hw, -hh, -hd, hw, -hh, -hd, hw, -hh, hd, -hw, -hh, hd, // Bottom face
                        -hw, -hh, -hd, -hw, hh, -hd, -hw, hh, hd, -hw, -hh, hd, // Left face
                        hw, -hh, -hd, hw, hh, -hd, hw, hh, hd, hw, -hh, hd, // Right face
                    ],
                    components: 3,
                },
            ),
            (
                AttributeKey::TexCoords,
                Attribute {
                    data: vec![
                        0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0,
                        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
                        0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
                        0.0, 1.0,
                    ],
                    components: 2,
                },
            ),
            (
                AttributeKey::Colors,
                Attribute {
                    data: vec![
                        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                    ],
                    components: 3,
                },
            ),
        ]);

        let indices = vec![
            0, 1, 2, 2, 3, 0, // Back face
            4, 5, 6, 6, 7, 4, // Front face
            8, 9, 10, 10, 11, 8, // Top face
            12, 13, 14, 14, 15, 12, // Bottom face
            16, 17, 18, 18, 19, 16, // Left face
            20, 21, 22, 22, 23, 20, // Right face
        ];

        Self::new(attributes, indices)
    }

    pub fn create_test_shape() -> Rc<RefCell<Self>> {
        let attributes: HashMap<AttributeKey, Attribute> = HashMap::from([
            (
                AttributeKey::Positions,
                Attribute {
                    data: vec![
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
                    ],
                    components: 3,
                },
            ),
            (
                AttributeKey::TexCoords,
                Attribute {
                    data: vec![
                        0.4131759, 0.00759614, 0.00486594, 0.43041353, 0.28081452, 0.949397,
                        0.85966998, 0.8473291, 0.9414737, 0.2652641,
                    ],
                    components: 2,
                },
            ),
            (
                AttributeKey::Colors,
                Attribute {
                    data: vec![
                        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                    ],
                    components: 3,
                },
            ),
        ]);

        let indices = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];

        Self::new(attributes, indices)
    }
}

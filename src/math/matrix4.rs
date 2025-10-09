use std::ops::Mul;

use super::{Quaternion, Vector3};

#[derive(Clone, Copy)]
pub struct Matrix4 {
    pub elements: [f32; 16],
}

impl Matrix4 {
    #[rustfmt::skip]
    const IDENTITY: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    pub fn new() -> Self {
        Matrix4 {
            elements: Self::IDENTITY,
        }
    }

    pub fn identity(&mut self) -> &mut Self {
        self.elements = Self::IDENTITY;
        self
    }

    pub fn multiply(&mut self, m: &Matrix4) -> &mut Self {
        let self_copy = Matrix4 {
            elements: self.elements,
        };
        self.multiply_matrices(&self_copy, m)
    }

    pub fn premultiply(&mut self, m: &Matrix4) -> &mut Self {
        let self_copy = Matrix4 {
            elements: self.elements,
        };
        self.multiply_matrices(m, &self_copy)
    }

    pub fn multiply_matrices(&mut self, a: &Matrix4, b: &Matrix4) -> &mut Self {
        let ae = &a.elements;
        let be = &b.elements;

        let (a11, a12, a13, a14) = (ae[0], ae[4], ae[8], ae[12]);
        let (a21, a22, a23, a24) = (ae[1], ae[5], ae[9], ae[13]);
        let (a31, a32, a33, a34) = (ae[2], ae[6], ae[10], ae[14]);
        let (a41, a42, a43, a44) = (ae[3], ae[7], ae[11], ae[15]);
        let (b11, b12, b13, b14) = (be[0], be[4], be[8], be[12]);
        let (b21, b22, b23, b24) = (be[1], be[5], be[9], be[13]);
        let (b31, b32, b33, b34) = (be[2], be[6], be[10], be[14]);
        let (b41, b42, b43, b44) = (be[3], be[7], be[11], be[15]);

        self.elements[0] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
        self.elements[4] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
        self.elements[8] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
        self.elements[12] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;

        self.elements[1] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
        self.elements[5] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
        self.elements[9] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
        self.elements[13] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;

        self.elements[2] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
        self.elements[6] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
        self.elements[10] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
        self.elements[14] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;

        self.elements[3] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
        self.elements[7] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
        self.elements[11] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
        self.elements[15] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;

        self
    }

    pub fn compose(
        &mut self,
        position: &Vector3,
        quaternion: &Quaternion,
        scale: &Vector3,
    ) -> &mut Self {
        let te = &mut self.elements;

        let x = quaternion.x;
        let y = quaternion.y;
        let z = quaternion.z;
        let w = quaternion.w;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let sx = scale.x;
        let sy = scale.y;
        let sz = scale.z;

        te[0] = (1.0 - (yy + zz)) * sx;
        te[1] = (xy + wz) * sx;
        te[2] = (xz - wy) * sx;
        te[3] = 0.0;

        te[4] = (xy - wz) * sy;
        te[5] = (1.0 - (xx + zz)) * sy;
        te[6] = (yz + wx) * sy;
        te[7] = 0.0;

        te[8] = (xz + wy) * sz;
        te[9] = (yz - wx) * sz;
        te[10] = (1.0 - (xx + yy)) * sz;
        te[11] = 0.0;

        te[12] = position.x;
        te[13] = position.y;
        te[14] = position.z;
        te[15] = 1.0;

        self
    }

    pub fn decompose(
        &self,
        position: &mut Vector3,
        quaternion: &mut Quaternion,
        scale: &mut Vector3,
    ) -> &Self {
        let te = &self.elements;

        let mut sx = (te[0] * te[0] + te[1] * te[1] + te[2] * te[2]).sqrt();
        let sy = (te[4] * te[4] + te[5] * te[5] + te[6] * te[6]).sqrt();
        let sz = (te[8] * te[8] + te[9] * te[9] + te[10] * te[10]).sqrt();

        // if determine is negative, we need to invert one scale
        let det = self.determinant();
        if det < 0.0 {
            sx = -sx;
        }

        position.x = te[12];
        position.y = te[13];
        position.z = te[14];

        // scale the rotation part
        let mut mat4_temp = *self;

        let inv_sx = 1.0 / sx;
        let inv_sy = 1.0 / sy;
        let inv_sz = 1.0 / sz;

        mat4_temp.elements[0] *= inv_sx;
        mat4_temp.elements[1] *= inv_sx;
        mat4_temp.elements[2] *= inv_sx;

        mat4_temp.elements[4] *= inv_sy;
        mat4_temp.elements[5] *= inv_sy;
        mat4_temp.elements[6] *= inv_sy;

        mat4_temp.elements[8] *= inv_sz;
        mat4_temp.elements[9] *= inv_sz;
        mat4_temp.elements[10] *= inv_sz;

        quaternion.set_from_rotation_matrix(&mat4_temp);

        scale.x = sx;
        scale.y = sy;
        scale.z = sz;

        self
    }

    #[rustfmt::skip]
    pub fn determinant(&self) -> f32 {
        let te = &self.elements;

        let n11 = te[0];
        let n12 = te[4];
        let n13 = te[8];
        let n14 = te[12];
        let n21 = te[1];
        let n22 = te[5];
        let n23 = te[9];
        let n24 = te[13];
        let n31 = te[2];
        let n32 = te[6];
        let n33 = te[10];
        let n34 = te[14];
        let n41 = te[3];
        let n42 = te[7];
        let n43 = te[11];
        let n44 = te[15];

        n41 * (n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34 - n12 * n23 * n34) +
        n42 * (n11 * n23 * n34 - n11 * n24 * n33 + n14 * n21 * n33 - n13 * n21 * n34 + n13 * n24 * n31 - n14 * n23 * n31) +
        n43 * (n11 * n24 * n32 - n11 * n22 * n34 - n14 * n21 * n32 + n12 * n21 * n34 + n14 * n22 * n31 - n12 * n24 * n31) +
        n44 * (-n13 * n22 * n31 - n11 * n23 * n32 + n11 * n22 * n33 + n13 * n21 * n32 - n12 * n21 * n33 + n12 * n23 * n31)
    }
}

impl<'a> Mul<&'a Matrix4> for &'a Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: &'a Matrix4) -> Matrix4 {
        let mut result = Matrix4::new();
        result.multiply_matrices(self, rhs);
        result
    }
}

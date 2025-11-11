/// Describes uniform value kinds supported in WGSL uniform buffers.
/// Layout (std140–like):
/// - Scalar (f32/i32/u32): align 4, size 4
/// - vec2: align 8, size 8
/// - vec3: align 16, size 12 (next member starts at 16-aligned offset)
/// - vec4: align 16, size 16
/// - mat4x4<f32>: align 16, size 64 (4 columns of vec4<f32>)
/// Bool intentionally omitted; prefer Uint (0/1) for stability across backends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UniformValueType {
    Float,
    Int,
    Uint,
    Vec2Float,
    Vec3Float,
    Vec4Float,
    Vec2Int,
    Vec3Int,
    Vec4Int,
    Vec2Uint,
    Vec3Uint,
    Vec4Uint,
    Mat4Float,
}

impl UniformValueType {
    /// Returns the alignment (in bytes) required for this type.
    #[inline]
    pub const fn align(self) -> usize {
        use UniformValueType::*;
        match self {
            Float | Int | Uint => 4,
            Vec2Float | Vec2Int | Vec2Uint => 8,
            Vec3Float | Vec3Int | Vec3Uint | Vec4Float | Vec4Int | Vec4Uint | Mat4Float => 16,
        }
    }

    /// Returns the natural size (in bytes) of this type (without trailing struct padding).
    #[inline]
    pub const fn size(self) -> usize {
        use UniformValueType::*;
        match self {
            Float | Int | Uint => 4,
            Vec2Float | Vec2Int | Vec2Uint => 8,
            Vec3Float | Vec3Int | Vec3Uint => 12,
            Vec4Float | Vec4Int | Vec4Uint => 16,
            Mat4Float => 64,
        }
    }
}

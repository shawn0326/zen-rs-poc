use super::*;

/// std140 对齐规则
#[inline]
fn align_up(v: usize, a: usize) -> usize {
    debug_assert!(a.is_power_of_two());
    (v + (a - 1)) & !(a - 1)
}

pub struct BufferBuilder {
    parent: ShaderBuilder,
    name: Box<str>,
    binding_slot: u32,
    members: Vec<UniformDesc>,
    cursor: usize,
}

impl BufferBuilder {
    #[inline]
    pub fn uniform(mut self, name: &str, ty: UniformValueType) -> Self {
        let off = align_up(self.cursor, ty.align());
        self.members.push(UniformDesc {
            key: symbol!(name),
            name: name.into(),
            offset: off,
            size: ty.size(),
        });
        self.cursor = off + ty.size();
        self
    }

    // 便捷方法
    #[inline]
    pub fn float(self, name: &str) -> Self {
        self.uniform(name, UniformValueType::Float)
    }
    #[inline]
    pub fn vec4(self, name: &str) -> Self {
        self.uniform(name, UniformValueType::Vec4Float)
    }

    // 结束该 buffer，回到主 Builder
    pub fn finish(mut self) -> ShaderBuilder {
        let total_size = align_up(self.cursor, 16);
        self.parent.binding_schema.push(BindingEntry {
            key: symbol!(self.name),
            name: self.name,
            slot: self.binding_slot,
            ty: BindingType::UniformBuffer {
                total_size,
                members: self.members.into_boxed_slice(),
            },
        });
        self.parent
    }
}

pub struct ShaderBuilder {
    source: String,
    binding_schema: Vec<BindingEntry>,
    vertex_schema: Vec<VertexEntry>,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            binding_schema: Vec::new(),
            vertex_schema: Vec::new(),
        }
    }

    // 设置源码
    pub fn source(mut self, src: impl Into<String>) -> Self {
        self.source = src.into();
        self
    }

    // 开始一个 UniformBuffer 的定义（返回子 Builder 以链式添加成员）
    pub fn buffer(self, name: &str, slot: u32) -> BufferBuilder {
        BufferBuilder {
            parent: self,
            name: name.into(),
            binding_slot: slot,
            members: Vec::new(),
            cursor: 0,
        }
    }

    // 添加一个纹理绑定条目
    pub fn texture(mut self, name: &str, slot: u32) -> Self {
        self.binding_schema.push(BindingEntry {
            key: symbol!(name),
            name: name.into(),
            slot,
            ty: BindingType::Texture,
        });
        self
    }

    // 可选：添加一个顶点属性声明
    pub fn vertex_attr(mut self, name: &str, slot: u32) -> Self {
        self.vertex_schema.push(VertexEntry {
            key: symbol!(name),
            name: name.into(),
            slot,
        });
        self
    }

    // 生成最终 Shader
    pub fn build(self) -> Shader {
        Shader::new(
            self.source.into_boxed_str(),
            self.binding_schema.into_boxed_slice(),
            self.vertex_schema.into_boxed_slice(),
        )
    }
}

pub fn pbr_shader() -> ShaderRc {
    ShaderBuilder::new()
        .source(include_str!("wgsl/pbr.wgsl"))
        .buffer("uniforms", 0)
        .vec4("albedo_factor")
        .float("metallic")
        .float("roughness")
        .finish()
        .texture("albedo_texture", 1)
        .vertex_attr("position", 0)
        .vertex_attr("tex_coord", 1)
        .vertex_attr("color", 2)
        .build()
        .into_rc()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_builder() {
        let shader = ShaderBuilder::new()
            // set source
            .source("shader code here")
            // uniform buffer
            .buffer("uniforms", 0)
            .vec4("albedo_factor")
            .float("roughness")
            .finish()
            // texture
            .texture("albedo_texture", 1)
            // vertex attribute
            .vertex_attr("position", 0)
            // build
            .build();

        assert_eq!(shader.source.as_ref(), "shader code here");
        assert_eq!(shader.binding_schema.len(), 2);
        assert_eq!(shader.vertex_schema.len(), 1);

        let meta = shader.uniform_field_meta(&symbol!("albedo_factor"));
        assert!(meta.is_some());
        let meta = meta.unwrap();
        assert_eq!(meta.index, 0);
        assert_eq!(meta.offset, 0);

        let meta = shader.uniform_field_meta(&symbol!("roughness"));
        assert!(meta.is_some());
        let meta = meta.unwrap();
        assert_eq!(meta.index, 0);
        assert_eq!(meta.offset, 16);

        let meta = shader.texture_meta(&symbol!("albedo_texture"));
        assert!(meta.is_some());
        assert_eq!(meta.unwrap().index, 1);
    }
}

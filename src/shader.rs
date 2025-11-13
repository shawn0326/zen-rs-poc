use crate::Symbol;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

pub struct UniformBufferMemberDesc {
    pub key: Symbol,
    pub offset: usize,
}

pub enum BindingType {
    UniformBuffer {
        total_size: usize,
        members: Box<[UniformBufferMemberDesc]>,
    },
    // StorageBuffer
    Texture,
}

pub struct BindingEntry {
    pub key: Symbol,
    pub binding: u32,
    pub ty: BindingType,
}

/// 仅包含着色器所需的顶点输入信息，但不包含具体的顶点缓冲布局，以保持和
/// Geometry的解耦。比如：Geometry可以是Interleaved的，而Shader只关心
/// 每个属性的位置。
pub struct VertexEntry {
    pub key: Symbol,
    pub location: u32,
}

pub type ShaderRc = Rc<Shader>;

/// `Shader`持有着着色器程序源码以及相关的绑定与顶点输入描述信息。
/// 它作为材质的`元信息`，用于指导材质的初始化（开辟内存）
/// 以及GPU绑定资源和顶点输入布局的创建。
pub struct Shader {
    pub source: Box<str>,
    pub binding_schema: Box<[BindingEntry]>,
    pub vertex_schema: Box<[VertexEntry]>,
    uniform_lut: OnceLock<HashMap<Symbol, (usize, usize)>>,
    texture_lut: OnceLock<HashMap<Symbol, usize>>,
}

impl Shader {
    pub fn into_rc(self) -> ShaderRc {
        Rc::new(self)
    }

    fn uniform_map(&self) -> &HashMap<Symbol, (usize, usize)> {
        self.uniform_lut.get_or_init(|| {
            let mut map = HashMap::new();
            for (i, entry) in self.binding_schema.iter().enumerate() {
                if let BindingType::UniformBuffer { members, .. } = &entry.ty {
                    for m in members {
                        map.insert(m.key, (i, m.offset));
                    }
                }
            }
            map
        })
    }

    fn texture_map(&self) -> &HashMap<Symbol, usize> {
        self.texture_lut.get_or_init(|| {
            let mut map = HashMap::new();
            for (i, entry) in self.binding_schema.iter().enumerate() {
                if let BindingType::Texture = entry.ty {
                    map.insert(entry.key, i);
                }
            }
            map
        })
    }

    /// 查找指定uniform变量的位置，返回(index, offset)
    pub fn find_uniform_location(&self, key: &Symbol) -> Option<(usize, usize)> {
        self.uniform_map().get(key).copied()
    }

    /// 查找指定纹理的位置，返回index
    pub fn find_texture_location(&self, key: &Symbol) -> Option<usize> {
        self.texture_map().get(key).copied()
    }
}

////// Builders //////

/// std140 对齐规则
#[inline]
fn align_up(v: usize, a: usize) -> usize {
    debug_assert!(a.is_power_of_two());
    (v + (a - 1)) & !(a - 1)
}

#[derive(Clone, Copy)]
pub enum UniformType {
    Float,
    Vec4,
}

impl UniformType {
    #[inline]
    fn align(self) -> usize {
        match self {
            UniformType::Float => 4,
            UniformType::Vec4 => 16,
        }
    }
    #[inline]
    fn size(self) -> usize {
        match self {
            UniformType::Float => 4,
            UniformType::Vec4 => 16,
        }
    }
}

pub struct BufferBuilder {
    parent: ShaderBuilder,
    key: Symbol,
    binding: u32,
    members: Vec<UniformBufferMemberDesc>,
    cursor: usize,
}

impl BufferBuilder {
    #[inline]
    pub fn uniform(mut self, key: Symbol, ty: UniformType) -> Self {
        let off = align_up(self.cursor, ty.align());
        self.members
            .push(UniformBufferMemberDesc { key, offset: off });
        self.cursor = off + ty.size();
        self
    }

    // 便捷方法
    #[inline]
    pub fn float(self, key: Symbol) -> Self {
        self.uniform(key, UniformType::Float)
    }
    #[inline]
    pub fn vec4(self, key: Symbol) -> Self {
        self.uniform(key, UniformType::Vec4)
    }

    // 结束该 buffer，回到主 Builder
    pub fn finish(mut self) -> ShaderBuilder {
        let total_size = align_up(self.cursor, 16);
        self.parent.binding_schema.push(BindingEntry {
            key: self.key,
            binding: self.binding,
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
    pub fn buffer(self, key: Symbol, binding: u32) -> BufferBuilder {
        BufferBuilder {
            parent: self,
            key,
            binding,
            members: Vec::new(),
            cursor: 0,
        }
    }

    // 添加一个纹理绑定条目
    pub fn texture(mut self, key: Symbol, binding: u32) -> Self {
        self.binding_schema.push(BindingEntry {
            key,
            binding,
            ty: BindingType::Texture,
        });
        self
    }

    // 可选：添加一个顶点属性声明
    pub fn vertex_attr(mut self, key: Symbol, location: u32) -> Self {
        self.vertex_schema.push(VertexEntry { key, location });
        self
    }

    // 生成最终 Shader
    pub fn build(self) -> Shader {
        Shader {
            source: self.source.into_boxed_str(),
            binding_schema: self.binding_schema.into_boxed_slice(),
            vertex_schema: self.vertex_schema.into_boxed_slice(),
            uniform_lut: OnceLock::new(),
            texture_lut: OnceLock::new(),
        }
    }
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
            .buffer(symbol!("uniforms"), 0)
            .vec4(symbol!("albedo_factor"))
            .float(symbol!("roughness"))
            .finish()
            // texture
            .texture(symbol!("albedo_texture"), 1)
            // vertex attribute
            .vertex_attr(symbol!("position"), 0)
            // build
            .build();

        assert_eq!(shader.source.as_ref(), "shader code here");
        assert_eq!(shader.binding_schema.len(), 2);
        assert_eq!(shader.vertex_schema.len(), 1);

        let loc = shader.find_uniform_location(&symbol!("albedo_factor"));
        assert!(loc.is_some());
        let (index, offset) = loc.unwrap();
        assert_eq!(index, 0);
        assert_eq!(offset, 0);

        let loc = shader.find_uniform_location(&symbol!("roughness"));
        assert!(loc.is_some());
        let (index, offset) = loc.unwrap();
        assert_eq!(index, 0);
        assert_eq!(offset, 16);

        let tex_loc = shader.find_texture_location(&symbol!("albedo_texture"));
        assert!(tex_loc.is_some());
        assert_eq!(tex_loc.unwrap(), 1);
    }
}

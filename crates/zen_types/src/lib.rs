#[derive(Debug, Clone)]
pub enum ShaderBindingType {
    Uniform,
    Texture,
    Sampler,
}

#[derive(Debug, Clone)]
pub struct ShaderBindGroupLayoutEntry {
    pub binding: u32,
    pub name: &'static str,
    pub binding_type: ShaderBindingType,
}

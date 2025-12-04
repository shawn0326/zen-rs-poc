use super::*;
use wgpu::VertexFormat::*;
use wgpu::VertexStepMode::*;

pub fn unlit_shader() -> ShaderRc {
    ShaderBuilder::new()
        .source(include_str!("wgsl/unlit.wgsl"))
        .uniform_buffer("uniforms", 0)
        .vec4f("albedo_factor")
        .finish()
        .texture("albedo_texture", 1)
        .sampler("albedo_sampler", 2)
        .vertex_attr("position", 0, Float32x3, Vertex)
        .vertex_attr("tex_coord", 1, Float32x2, Vertex)
        .vertex_attr("color", 2, Float32x4, Vertex)
        .build()
        .into_rc()
}

pub fn pbr_shader() -> ShaderRc {
    ShaderBuilder::new()
        .source(include_str!("wgsl/pbr.wgsl"))
        .uniform_buffer("uniforms", 0)
        .vec4f("albedo_factor")
        .float("metallic")
        .float("roughness")
        .finish()
        .texture("albedo_texture", 1)
        .sampler("albedo_sampler", 2)
        .vertex_attr("position", 0, Float32x3, Vertex)
        .vertex_attr("tex_coord", 1, Float32x2, Vertex)
        .vertex_attr("color", 2, Float32x4, Vertex)
        .build()
        .into_rc()
}

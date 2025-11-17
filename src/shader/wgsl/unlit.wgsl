struct VertexInput {
    @location(0) position: vec3f,
    @location(1) tex_coord: vec2f,
    @location(2) color: vec3f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) tex_coord: vec2f,
    @location(1) color: vec3f,
};

struct CameraUniform {
    view_proj: mat4x4f,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<storage, read> model_matrices: array<mat4x4f>;

struct MaterialUniforms {
    albedo_factor: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> material: MaterialUniforms;
@group(2) @binding(1)
var albedo_texture: texture_2d<f32>;
@group(2) @binding(2)
var albedo_texture_sampler: sampler;

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) instance_idx: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = model.tex_coord;
    out.color = model.color;
    let model_matrix = model_matrices[instance_idx];
    out.clip_position = camera.view_proj * model_matrix * vec4f(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    var color = textureSample(albedo_texture, albedo_texture_sampler, in.tex_coord) * material.albedo_factor;
    return vec4f(color.rgb * in.color.rgb, color.a);
}
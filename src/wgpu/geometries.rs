use crate::graphics::{AttributeKey, Geometry, GeometryId};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

pub(super) struct Geometries {
    map: HashMap<GeometryId, GpuGeometry>,
}

impl Geometries {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_gpu_geometry(&mut self, device: &wgpu::Device, geometry: &Geometry) -> &GpuGeometry {
        let geometry_id = geometry.id();

        self.map.entry(geometry_id).or_insert_with(|| {
            println!("Creating GPU geometry for GeometryId {:?}", geometry_id);
            let gpu_geometry = GpuGeometry::new(device, geometry);
            gpu_geometry
        })
    }
}

pub(super) struct GpuGeometry {
    pub positions_buffer: wgpu::Buffer,
    pub tex_coords_buffer: wgpu::Buffer,
    pub colors_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub vertex_buffer_layouts: Vec<VertexBufferLayout>,
}

impl GpuGeometry {
    pub fn new(device: &wgpu::Device, geometry: &Geometry) -> Self {
        let positions = geometry
            .get_attribute(AttributeKey::Positions)
            .expect("Geometry must have positions");
        let tex_coords = geometry
            .get_attribute(AttributeKey::TexCoords)
            .expect("Geometry must have texture coordinates");
        let colors = geometry
            .get_attribute(AttributeKey::Colors)
            .expect("Geometry must have colors");
        let indices = geometry.indices().expect("Geometry must have indices");

        let positions_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&positions.buffer().borrow().data()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let tex_coords_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&tex_coords.buffer().borrow().data()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let colors_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&colors.buffer().borrow().data()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        let vertex_buffer_layouts = vec![
            // Buffer 0: positions
            VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: vec![wgpu::VertexAttribute {
                    offset: positions.offset() as wgpu::BufferAddress,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                }],
            },
            // Buffer 1: texture coordinates
            VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: vec![wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }],
            },
            // Buffer 2: colors
            VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: vec![wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                }],
            },
        ];

        Self {
            positions_buffer,
            tex_coords_buffer,
            colors_buffer,
            index_buffer,
            num_indices,
            vertex_buffer_layouts,
        }
    }

    pub fn vertex_buffer_layouts(&self) -> Vec<wgpu::VertexBufferLayout<'_>> {
        self.vertex_buffer_layouts
            .iter()
            .map(|vbl| vbl.as_wgpu_layout())
            .collect::<Vec<_>>()
    }

    pub fn set_buffers_to_render_pass(&self, render_pass: &mut wgpu::RenderPass) -> &Self {
        render_pass.set_vertex_buffer(0, self.positions_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.tex_coords_buffer.slice(..));
        render_pass.set_vertex_buffer(2, self.colors_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self
    }
}

pub(super) struct VertexBufferLayout {
    array_stride: wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode,
    attributes: Vec<wgpu::VertexAttribute>,
}

impl VertexBufferLayout {
    pub fn as_wgpu_layout(&self) -> wgpu::VertexBufferLayout<'_> {
        wgpu::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode,
            attributes: &self.attributes,
        }
    }
}

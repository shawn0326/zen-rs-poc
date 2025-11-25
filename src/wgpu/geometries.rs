use crate::{GeometryHandle, ResourceKey, Resources, Symbol, geometry::Geometry};
use std::{collections::HashMap, ops::Range};
use wgpu::util::DeviceExt;

pub(super) struct Geometries {
    map: HashMap<ResourceKey, GpuGeometry>,
}

impl Geometries {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_gpu_geometry(
        &mut self,
        device: &wgpu::Device,
        resources: &Resources,
        geometry_handle: &GeometryHandle,
    ) -> &GpuGeometry {
        let geometry = resources
            .get_geometry(geometry_handle)
            .expect("GeometryHandle is invalid");

        self.map.entry(geometry_handle.raw()).or_insert_with(|| {
            println!(
                "Creating GPU geometry for GeometryHandle {:?}",
                geometry_handle
            );
            let gpu_geometry = GpuGeometry::new(device, geometry, resources);
            gpu_geometry
        })
    }
}

pub(super) struct GpuGeometry {
    pub index_buffer: (wgpu::Buffer, Range<wgpu::BufferAddress>, u32),
    pub vertex_buffers: Vec<(wgpu::Buffer, Range<wgpu::BufferAddress>)>,
    pub vertex_buffer_layouts: Vec<VertexBufferLayout>,
}

impl GpuGeometry {
    pub fn new(device: &wgpu::Device, geometry: &Geometry, resources: &Resources) -> Self {
        const NAMES: [Symbol; 3] = [
            symbol!("positions"),
            symbol!("tex_coords"),
            symbol!("colors"),
        ];

        let mut vertex_buffers = vec![];

        for name in &NAMES {
            let attr = geometry
                .get_attribute(*name)
                .expect(&format!("Geometry must have attribute {:?}", name));

            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: resources
                    .get_buffer(&attr.vertex_buffer.buffer_slice.buffer)
                    .unwrap()
                    .raw(),
                usage: wgpu::BufferUsages::VERTEX,
            });

            vertex_buffers.push((buffer, attr.vertex_buffer.buffer_slice.range_u64()));
        }

        let index_buffer = {
            let indices = geometry.indices().expect("Geometry must have indices");

            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: resources
                    .get_buffer(&indices.buffer_slice.buffer)
                    .unwrap()
                    .raw(),
                usage: wgpu::BufferUsages::INDEX,
            });

            (
                buffer,
                indices.buffer_slice.range_u64(),
                (indices.buffer_slice.size / indices.format.byte_size()) as u32,
            )
        };

        let vertex_buffer_layouts = vec![
            // Buffer 0: positions
            VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: vec![wgpu::VertexAttribute {
                    offset: 0 as wgpu::BufferAddress,
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
            vertex_buffers,
            vertex_buffer_layouts,
            index_buffer,
        }
    }

    pub fn vertex_buffer_layouts(&self) -> Vec<wgpu::VertexBufferLayout<'_>> {
        self.vertex_buffer_layouts
            .iter()
            .map(|vbl| vbl.as_wgpu_layout())
            .collect::<Vec<_>>()
    }

    pub fn set_buffers_to_render_pass(&self, render_pass: &mut wgpu::RenderPass) -> &Self {
        for (i, (buffer, range)) in self.vertex_buffers.iter().enumerate() {
            render_pass.set_vertex_buffer(i as u32, buffer.slice(range.clone()));
        }
        let (buffer, range, _) = &self.index_buffer;
        render_pass.set_index_buffer(buffer.slice(range.clone()), wgpu::IndexFormat::Uint32);
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

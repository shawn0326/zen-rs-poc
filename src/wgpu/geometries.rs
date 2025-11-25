use crate::{GeometryHandle, ResourceKey};
use slotmap::SecondaryMap;

pub(super) struct Geometries {
    pool: SecondaryMap<ResourceKey, GpuGeometry>,
}

impl Geometries {
    pub fn new() -> Self {
        Self {
            pool: SecondaryMap::new(),
        }
    }

    pub fn prepare(
        &mut self,
        // device: &wgpu::Device,
        // resources: &Resources,
        handle: &GeometryHandle,
    ) -> &GpuGeometry {
        let entry = self
            .pool
            .entry(handle.raw())
            .expect("GeometryHandle has been removed from pool.");

        // let geometry = resources
        //     .get_geometry(handle)
        //     .expect("GeometryHandle has been removed from pool.");

        // todo: hot reload support

        entry.or_insert_with(|| GpuGeometry::new())
    }

    #[allow(dead_code)]
    pub fn get_gpu_geometry(&self, geometry_handle: &GeometryHandle) -> &GpuGeometry {
        self.pool
            .get(geometry_handle.raw())
            .expect("Geometry GPU lost.")
    }
}

pub(super) struct GpuGeometry {
    pub vertex_buffer_layouts: Vec<VertexBufferLayout>,
}

impl GpuGeometry {
    pub fn new() -> Self {
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
            vertex_buffer_layouts,
        }
    }

    pub fn vertex_buffer_layouts(&self) -> Vec<wgpu::VertexBufferLayout<'_>> {
        self.vertex_buffer_layouts
            .iter()
            .map(|vbl| vbl.as_wgpu_layout())
            .collect::<Vec<_>>()
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

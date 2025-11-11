use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::graphics::{Material, MaterialId};

pub(super) struct Pipelines {
    format: wgpu::TextureFormat,
    map: HashMap<MaterialId, wgpu::RenderPipeline>,
}

impl Pipelines {
    pub fn new(format: wgpu::TextureFormat) -> Self {
        Self {
            format,
            map: HashMap::new(),
        }
    }

    pub fn set_pipeline(
        &mut self,
        device: &wgpu::Device,
        material: &Rc<RefCell<Material>>,
        vertex_buffer_layout: &[wgpu::VertexBufferLayout],
        bindgroup_layout: &[&wgpu::BindGroupLayout],
    ) -> &wgpu::RenderPipeline {
        let material = material.borrow();

        let material_id = material.id();

        match self.map.entry(material_id) {
            std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Shader"),
                    source: wgpu::ShaderSource::Wgsl(material.shader_source().into()),
                });

                let pipeline_layout =
                    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: bindgroup_layout,
                        push_constant_ranges: &[],
                    });

                let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        compilation_options: Default::default(),
                        entry_point: Some("vs_main"),
                        buffers: vertex_buffer_layout,
                    },
                    fragment: Some(wgpu::FragmentState {
                        // 3.
                        module: &shader,
                        compilation_options: Default::default(),
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            // 4.
                            format: self.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw, // 2.
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24Plus,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState {
                            constant: 0,
                            slope_scale: 0.0,
                            clamp: 0.0,
                        },
                    }), // 1.
                    multisample: wgpu::MultisampleState {
                        count: 1,                         // 2.
                        mask: !0,                         // 3.
                        alpha_to_coverage_enabled: false, // 4.
                    },
                    multiview: None, // 5.
                    cache: None,
                });

                println!("Created new pipeline");

                v.insert(pipeline)
            }
        }
    }

    // pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {
    //     &self.pipeline
    // }
}

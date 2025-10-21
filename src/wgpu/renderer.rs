use super::{
    bindgroups::{GpuGlobalBindGroup, GpuPrimitiveBindGroup, MaterialBindGroups},
    geometries::Geometries,
    pipelines::Pipelines,
    targets::Targets,
    textures::Textures,
};
use crate::{
    graphics::{Geometry, Material},
    render::{RenderItem, RenderTarget},
    scene::Camera,
};

pub struct Renderer<'window> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    pipelines: Pipelines,
    targets: Targets,
    geometries: Geometries,
    global_bind_group: GpuGlobalBindGroup,
    primitive_bind_group: GpuPrimitiveBindGroup,
    material_bind_groups: MaterialBindGroups,
    textures: Textures,
}

impl<'window> Renderer<'window> {
    pub async fn new(
        instance: &wgpu::Instance,
        surface: wgpu::Surface<'window>,
        (width, height): (u32, u32),
    ) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        println!("{:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let caps = surface.get_capabilities(&adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let geometries = Geometries::new();
        let pipelines = Pipelines::new();
        let targets = Targets::new();
        let global_bind_group = GpuGlobalBindGroup::new(&device);
        let primitive_bind_group = GpuPrimitiveBindGroup::new(&device);
        let material_bind_groups = MaterialBindGroups::new();
        let textures = Textures::new(&device);

        Self {
            device,
            queue,
            surface,
            surface_config,
            pipelines,
            targets,
            geometries,
            global_bind_group,
            primitive_bind_group,
            material_bind_groups,
            textures,
        }
    }

    pub fn render(&mut self, render_list: &[RenderItem], camera: &Camera, target: &RenderTarget) {
        if let RenderTarget::Screen(screen_target) = target {
            if (screen_target.width != self.surface_config.width)
                || (screen_target.height != self.surface_config.height)
            {
                self.surface_config.width = screen_target.width.max(1);
                self.surface_config.height = screen_target.height.max(1);
                self.surface.configure(&self.device, &self.surface_config);
            }
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let surface_texture = self.surface.get_current_texture().unwrap();

        let mut current_material_ptr: Option<*const Material> = None;
        let mut current_geometry_ptr: Option<*const Geometry> = None;

        {
            let mut render_pass = self
                .targets
                .create_render_pass(&surface_texture, &mut encoder);

            render_pass.set_bind_group(0, &self.global_bind_group.bind_group, &[]);
            render_pass.set_bind_group(1, &self.primitive_bind_group.bind_group, &[]);

            let mut matrices = Vec::with_capacity(render_list.len() * 16);
            let mut batch_start = 0u32;
            let mut indices = 0..0;

            for (i, render_item) in render_list.iter().enumerate() {
                let geometry = render_item.geometry.borrow();
                let material = render_item.material.borrow();

                let geometry_ptr = (&*geometry) as *const Geometry;
                let material_ptr = (&*material) as *const Material;

                let geometry_changed = current_geometry_ptr != Some(geometry_ptr);
                let material_changed = current_material_ptr != Some(material_ptr);

                if geometry_changed || material_changed {
                    if i > 0 {
                        // flush previous batch
                        render_pass.draw_indexed(indices, 0, batch_start..(i as u32));
                    }

                    let gpu_geometry = self.geometries.get_gpu_geometry(&self.device, &*geometry);

                    let gpu_material_bind_group =
                        self.material_bind_groups.get_material_bind_group(
                            &self.device,
                            &self.queue,
                            &*material,
                            &mut self.textures,
                        );

                    let pipeline = self.pipelines.set_pipeline(
                        &self.device,
                        &render_item.material,
                        self.surface_config.format,
                        &gpu_geometry.vertex_buffer_layouts,
                        &[
                            &self.global_bind_group.bind_group_layout,
                            &self.primitive_bind_group.layout,
                            &gpu_material_bind_group.bind_group_layout,
                        ],
                    );

                    render_pass.set_pipeline(pipeline);

                    if material_changed {
                        render_pass.set_bind_group(2, &gpu_material_bind_group.bind_group, &[]);
                    }

                    if geometry_changed {
                        render_pass.set_vertex_buffer(0, gpu_geometry.positions_buffer.slice(..));
                        render_pass.set_vertex_buffer(1, gpu_geometry.tex_coords_buffer.slice(..));
                        render_pass.set_vertex_buffer(2, gpu_geometry.colors_buffer.slice(..));
                        render_pass.set_index_buffer(
                            gpu_geometry.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint32,
                        );
                    }

                    current_material_ptr = Some(material_ptr);
                    current_geometry_ptr = Some(geometry_ptr);

                    batch_start = i as u32;
                    indices = 0..gpu_geometry.num_indices;
                }

                matrices.extend_from_slice(render_item.world_matrix.to_cols_array().as_slice());
            }

            if !render_list.is_empty() {
                // flush last batch
                render_pass.draw_indexed(indices, 0, batch_start..(render_list.len() as u32));
            }

            self.global_bind_group.update_camera(&self.queue, camera);
            self.primitive_bind_group
                .upload_matrices(&self.queue, &matrices);
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

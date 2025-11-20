use super::{
    bindgroups::{GlobalBindGroup, MaterialBindGroups, PrimitiveBindGroup},
    geometries::Geometries,
    pipelines::Pipelines,
    surfaces::Surfaces,
    targets::Targets,
    textures::Textures,
};
use crate::{
    GeometryHandle, MaterialHandle, camera::Camera, render::RenderItem, target::RenderTarget,
};

pub struct Renderer<'surf> {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surfaces: Surfaces<'surf>,
    pipelines: Pipelines,
    targets: Targets,
    geometries: Geometries,
    global_bind_group: GlobalBindGroup,
    primitive_bind_group: PrimitiveBindGroup,
    material_bind_groups: MaterialBindGroups,
    textures: Textures,
}

impl<'surf> Renderer<'surf> {
    pub async fn new(instance: &wgpu::Instance, surface: wgpu::Surface<'surf>) -> Self {
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

        println!("{:?}", surface.get_capabilities(&adapter));

        let mut surfaces = Surfaces::new();
        let geometries = Geometries::new();
        let pipelines = Pipelines::new(surface.get_capabilities(&adapter).formats[0]);
        let targets = Targets::new();
        let global_bind_group = GlobalBindGroup::new(&device);
        let primitive_bind_group = PrimitiveBindGroup::new(&device, 10_000);
        let material_bind_groups = MaterialBindGroups::new();
        let textures = Textures::new(&device, &queue);

        surfaces.add_surface(surface);

        Self {
            adapter,
            device,
            queue,
            surfaces,
            pipelines,
            targets,
            geometries,
            global_bind_group,
            primitive_bind_group,
            material_bind_groups,
            textures,
        }
    }

    pub fn render(
        &mut self,
        render_list: &[RenderItem],
        camera: &Camera,
        target: &RenderTarget,
        resources: &crate::Resources,
    ) {
        let surface_textures =
            self.surfaces
                .get_surface_textures(&self.adapter, &self.device, target, resources);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = self.targets.create_render_pass(
                &self.device,
                &self.queue,
                &surface_textures,
                &mut encoder,
                &mut self.textures,
                target,
                resources,
            );

            let global_bind_group = &self.global_bind_group;
            let primitive_bind_group = self
                .primitive_bind_group
                .prepare(&self.device, render_list.len());

            render_pass.set_bind_group(0, global_bind_group.gpu_bind_group(), &[]);
            render_pass.set_bind_group(1, primitive_bind_group.gpu_bind_group(), &[]);

            let mut batch_start = 0u32;
            let mut indices = 0..0;

            let mut current_material_handle: Option<MaterialHandle> = None;
            let mut current_geometry_handle: Option<GeometryHandle> = None;

            for (i, render_item) in render_list.iter().enumerate() {
                let geometry_handle = render_item.geometry;
                let material_handle = render_item.material;

                let material_changed = match current_material_handle {
                    Some(handle) => handle != material_handle,
                    None => true,
                };

                let geometry_changed = match current_geometry_handle {
                    Some(handle) => handle != geometry_handle,
                    None => true,
                };

                if geometry_changed || material_changed {
                    if i > 0 {
                        // flush previous batch
                        render_pass.draw_indexed(indices, 0, batch_start..(i as u32));
                    }

                    let gpu_geometry =
                        self.geometries
                            .get_gpu_geometry(&self.device, resources, geometry_handle);

                    let gpu_material_bind_group =
                        self.material_bind_groups.get_material_bind_group(
                            &self.device,
                            &self.queue,
                            material_handle,
                            &mut self.textures,
                            resources,
                        );

                    let pipeline = self.pipelines.set_pipeline(
                        &self.device,
                        material_handle,
                        &gpu_geometry.vertex_buffer_layouts(),
                        &[
                            global_bind_group.gpu_layout(),
                            primitive_bind_group.gpu_layout(),
                            &gpu_material_bind_group.bind_group_layout,
                        ],
                        resources,
                    );

                    render_pass.set_pipeline(pipeline);

                    if material_changed {
                        render_pass.set_bind_group(2, &gpu_material_bind_group.bind_group, &[]);
                    }

                    if geometry_changed {
                        gpu_geometry.set_buffers_to_render_pass(&mut render_pass);
                    }

                    current_material_handle = Some(material_handle);
                    current_geometry_handle = Some(geometry_handle);

                    batch_start = i as u32;
                    indices = 0..gpu_geometry.num_indices;
                }

                primitive_bind_group.push_data(&render_item.world_matrix);
            }

            if !render_list.is_empty() {
                // flush last batch
                render_pass.draw_indexed(indices, 0, batch_start..(render_list.len() as u32));
            }

            global_bind_group.upload(&self.queue, camera);
            primitive_bind_group.flush(&self.queue);
        }

        self.queue.submit(Some(encoder.finish()));

        surface_textures.present();
    }
}

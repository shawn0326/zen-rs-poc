use super::{
    bindgroups::{GlobalBindGroup, PrimitiveBindGroup},
    buffers::Buffers,
    geometries::Geometries,
    materials::Materials,
    pipelines::Pipelines,
    samplers::Samplers,
    surfaces::Surfaces,
    targets::Targets,
    textures::Textures,
};
use crate::{ResourceKey, Resources, camera::Camera, primitive::Primitive, target::RenderTarget};
use std::collections::HashSet;

const GEOMETRY_CHANGED: u8 = 0b01;
const MATERIAL_CHANGED: u8 = 0b10;

pub struct Renderer {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surfaces: Surfaces,
    pipelines: Pipelines,
    targets: Targets,
    geometries: Geometries,
    global_bind_group: GlobalBindGroup,
    primitive_bind_group: PrimitiveBindGroup,
    textures: Textures,
    buffers: Buffers,
    samplers: Samplers,
    materials: Materials,
}

impl Renderer {
    pub async fn new(instance: &wgpu::Instance, surface: &wgpu::Surface<'_>) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
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

        let surfaces = Surfaces::new();
        let geometries = Geometries::new();
        let pipelines = Pipelines::new(surface.get_capabilities(&adapter).formats[0]);
        let targets = Targets::new();
        let global_bind_group = GlobalBindGroup::new(&device);
        let primitive_bind_group = PrimitiveBindGroup::new(&device, 10_000);
        let textures = Textures::new(&device, &queue);
        let buffers = Buffers::new();
        let samplers = Samplers::new(&device);
        let materials = Materials::new();

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
            textures,
            buffers,
            samplers,
            materials,
        }
    }

    pub fn render(
        &mut self,
        primitives: &[Primitive],
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

        let change_flags = {
            let mut change_flags = Vec::with_capacity(primitives.len());

            let mut last_geometry_handle = None;
            let mut last_material_handle = None;

            let mut geometry_handles = HashSet::new();
            let mut material_handles = HashSet::new();

            for primitive in primitives {
                let geometry_handle = primitive.geometry();
                let material_handle = primitive.material();

                let mut flag = 0u8;

                if Some(geometry_handle.raw()) != last_geometry_handle {
                    flag |= GEOMETRY_CHANGED;
                    last_geometry_handle = Some(geometry_handle.raw());
                    geometry_handles.insert(primitive.geometry());
                }
                if Some(material_handle.raw()) != last_material_handle {
                    flag |= MATERIAL_CHANGED;
                    last_material_handle = Some(material_handle.raw());
                    material_handles.insert(primitive.material());
                }

                change_flags.push(flag);
            }

            let geometris = geometry_handles
                .iter()
                .map(|handle| resources.get_geometry(handle).unwrap())
                .collect::<Vec<_>>();

            let materials = material_handles
                .iter()
                .map(|handle| resources.get_material(handle).unwrap())
                .collect::<Vec<_>>();

            let texture_handles = materials
                .iter()
                .flat_map(|material| material.textures())
                .collect::<Vec<_>>();

            let textures = texture_handles
                .iter()
                .filter_map(|handle| resources.get_texture(handle))
                .collect::<Vec<_>>();

            let buffer_handles = geometris
                .iter()
                .flat_map(|geometry| geometry.buffers())
                .chain(textures.iter().filter_map(|texture| texture.buffer()))
                .collect::<HashSet<_>>();

            buffer_handles.iter().for_each(|buffer_handle| {
                self.buffers
                    .prepare(&self.device, &self.queue, resources, buffer_handle);
            });

            texture_handles
                .iter()
                .zip(textures.iter())
                .for_each(|(handle, texture)| {
                    self.textures
                        .prepare(&self.device, &self.queue, resources, texture, handle);
                });

            material_handles
                .iter()
                .zip(materials.iter())
                .for_each(|(handle, material)| {
                    material.samplers().for_each(|sampler| {
                        self.samplers.prepare(&self.device, *sampler);
                    });

                    self.materials.prepare(
                        &self.device,
                        &self.queue,
                        resources,
                        &self.textures,
                        &self.samplers,
                        handle,
                    );
                });

            change_flags
        };

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
                .prepare(&self.device, primitives.len());

            render_pass.set_bind_group(0, global_bind_group.gpu_bind_group(), &[]);
            render_pass.set_bind_group(1, primitive_bind_group.gpu_bind_group(), &[]);

            let mut batch_start = 0u32;
            let mut indices = 0..0;

            for (i, primitive) in primitives.iter().enumerate() {
                let geometry_handle = primitive.geometry();
                let material_handle = primitive.material();

                let flag = change_flags[i];
                let geometry_changed = (flag & GEOMETRY_CHANGED) != 0;
                let material_changed = (flag & MATERIAL_CHANGED) != 0;

                if geometry_changed || material_changed {
                    if i > 0 {
                        // flush previous batch
                        render_pass.draw_indexed(indices, 0, batch_start..(i as u32));
                    }

                    let gpu_geometry = self.geometries.prepare(geometry_handle);

                    let internal_material = self.materials.get_internal_material(material_handle);

                    let pipeline = self.pipelines.set_pipeline(
                        &self.device,
                        material_handle,
                        &gpu_geometry,
                        &[
                            global_bind_group.gpu_layout(),
                            primitive_bind_group.gpu_layout(),
                            &internal_material.bind_group_layout,
                        ],
                        resources,
                    );

                    render_pass.set_pipeline(pipeline);

                    if material_changed {
                        render_pass.set_bind_group(2, &internal_material.bind_group, &[]);
                    }

                    if geometry_changed {
                        self.buffers.set_buffers_to_render_pass(
                            resources,
                            &mut render_pass,
                            geometry_handle,
                        );
                    }

                    batch_start = i as u32;

                    if let Some(index_buffer) =
                        resources.get_geometry(geometry_handle).unwrap().indices()
                    {
                        indices = 0..index_buffer.index_count();
                    } else {
                        indices = 0..0;
                    }
                }

                primitive_bind_group.push_data(&primitive.transform());
            }

            if !primitives.is_empty() {
                // flush last batch
                render_pass.draw_indexed(indices, 0, batch_start..(primitives.len() as u32));
            }

            global_bind_group.upload(&self.queue, camera);
            primitive_bind_group.flush(&self.queue);
        }

        self.queue.submit(Some(encoder.finish()));

        surface_textures.present();
    }

    pub fn destroy_texture_gpu(&mut self, _: ResourceKey) {
        // todo: implement
    }

    pub fn destroy_material_gpu(&mut self, _: ResourceKey) {
        // todo: implement
    }

    pub fn destroy_geometry_gpu(&mut self, _: ResourceKey) {
        // todo: implement
    }

    pub fn destroy_buffer_gpu(&mut self, key: ResourceKey) {
        self.buffers.destroy_internal_buffer(key);
    }

    pub fn destroy_garbage_gpu(&mut self, resources: &Resources) {
        if resources.textures.free_len() > 0 {
            resources.textures.for_each_free(|key| {
                self.destroy_texture_gpu(key);
            });
        }

        if resources.materials.free_len() > 0 {
            resources.materials.for_each_free(|key| {
                self.destroy_material_gpu(key);
            });
        }

        if resources.geometries.free_len() > 0 {
            resources.geometries.for_each_free(|key| {
                self.destroy_geometry_gpu(key);
            });
        }

        if resources.buffers.free_len() > 0 {
            resources.buffers.for_each_free(|key| {
                self.destroy_buffer_gpu(key);
            });
        }
    }
}

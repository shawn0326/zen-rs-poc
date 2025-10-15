use super::{
    bindgroups::BindGroups, geometries::Geometries, pipelines::Pipelines, targets::Targets,
    textures::Textures,
};
use crate::{
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
    bindgroups: BindGroups,
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

        let geometries = Geometries::new(&device);
        let pipelines = Pipelines::new(&device, surface_config.format, &Geometries::desc());
        let targets = Targets::new();
        let bindgroups = BindGroups::new();
        let textures = Textures::new();

        Self {
            device,
            queue,
            surface,
            surface_config,
            pipelines,
            targets,
            geometries,
            bindgroups,
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

        {
            let mut render_pass = self
                .targets
                .create_render_pass(&surface_texture, &mut encoder);

            #[allow(unused_variables)]
            for render_item in render_list.iter() {
                if let Some(texture) = render_item.material.borrow().texture() {
                    self.textures
                        .set_texture(&self.device, &self.queue, &texture);
                    if let Some(gpu_texture) = self.textures.get_texture(&texture) {
                        #[allow(unused_variables)]
                        let _ = &gpu_texture.texture;
                    };
                }

                self.bindgroups.update(&render_item.material, camera);

                let pipeline = self.pipelines.get_pipeline();
                render_pass.set_pipeline(pipeline);
                render_pass.set_vertex_buffer(0, self.geometries.positions_buffer.slice(..));
                render_pass.set_vertex_buffer(1, self.geometries.colors_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.geometries.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(0..self.geometries.num_indices, 0, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

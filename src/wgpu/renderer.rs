use std::{cell::RefCell, sync::Arc};

use super::pipelines::Pipelines;
use crate::{render::RenderTarget, wgpu::targets::Targets};

pub struct Renderer<'window> {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface: wgpu::Surface<'window>,
    surface_config: RefCell<wgpu::SurfaceConfiguration>,
    pipelines: Pipelines,
    targets: Targets,
}

impl<'window> Renderer<'window> {
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'window>>,
        width: u32,
        height: u32,
    ) -> Self {
        let context = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = context.create_surface(target).unwrap();

        let adapter = context
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

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
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let pipelines = Pipelines::new(&device, config.format);
        let targets = Targets::new();

        Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface,
            surface_config: RefCell::new(config),
            pipelines,
            targets,
        }
    }

    pub fn render(&self, target: &RenderTarget) {
        if let RenderTarget::Screen(screen_target) = target {
            if (screen_target.width != self.surface_config.borrow().width)
                || (screen_target.height != self.surface_config.borrow().height)
            {
                self.surface_config.borrow_mut().width = screen_target.width.max(1);
                self.surface_config.borrow_mut().height = screen_target.height.max(1);
                self.surface
                    .configure(&self.device, &self.surface_config.borrow());
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

            let pipeline = self.pipelines.get_pipeline();
            render_pass.set_pipeline(pipeline); // 2.
            render_pass.draw(0..3, 0..1); // 3.
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

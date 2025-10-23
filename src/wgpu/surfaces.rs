use crate::{graphics::TextureSource, render::RenderTarget};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

pub(super) struct ActiveSurfaceTextures(HashMap<u32, wgpu::SurfaceTexture>);

impl ActiveSurfaceTextures {
    pub fn get_surface_texture(&self, surface_id: u32) -> &wgpu::SurfaceTexture {
        self.0.get(&surface_id).unwrap()
    }

    pub fn present(self) {
        self.0.into_values().for_each(|st| {
            st.present();
        });
    }
}

struct SurfaceInfo<'surf> {
    surface: wgpu::Surface<'surf>,
    config: Option<wgpu::SurfaceConfiguration>,
}

pub(super) struct Surfaces<'surf> {
    map: HashMap<u32, SurfaceInfo<'surf>>,
    next_id: AtomicU32,
}

impl<'surf> Surfaces<'surf> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: AtomicU32::new(0),
        }
    }

    pub fn add_surface(&mut self, surface: wgpu::Surface<'surf>) -> u32 {
        let surface_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.map.insert(
            surface_id,
            SurfaceInfo {
                surface,
                config: None,
            },
        );
        surface_id
    }

    pub fn get_surface_textures(
        &mut self,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        target: &RenderTarget,
    ) -> ActiveSurfaceTextures {
        let mut surface_textures = ActiveSurfaceTextures(HashMap::new());

        for color_attachment in target.color_attachments.iter() {
            if let TextureSource::Surface { surface_id, .. } =
                color_attachment.texture.borrow().source()
            {
                let (width, height) = target.size();
                let surface = self.map.get_mut(&surface_id);

                if let Some(surface_info) = surface {
                    if let Some(config) = &mut surface_info.config {
                        if config.width != width || config.height != height {
                            config.width = width;
                            config.height = height;
                            surface_info.surface.configure(&device, &config);
                        }
                    } else {
                        let caps = surface_info.surface.get_capabilities(&adapter);
                        let config = wgpu::SurfaceConfiguration {
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            format: caps.formats[0],
                            width,
                            height,
                            present_mode: wgpu::PresentMode::Fifo,
                            alpha_mode: caps.alpha_modes[0],
                            view_formats: vec![],
                            desired_maximum_frame_latency: 2,
                        };
                        surface_info.surface.configure(&device, &config);
                        surface_info.config = Some(config);
                    }

                    surface_textures.0.insert(
                        *surface_id,
                        surface_info.surface.get_current_texture().unwrap(),
                    );
                }
            }
        }

        surface_textures
    }
}

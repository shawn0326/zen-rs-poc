use crate::target::RenderTarget;
use crate::texture::TextureKind;
use crate::{Resources, SurfaceKey};
use slotmap::SecondaryMap;
use std::collections::HashMap;

pub struct ActiveSurfaceTextures(HashMap<SurfaceKey, wgpu::SurfaceTexture>);

impl ActiveSurfaceTextures {
    pub fn get_surface_texture(&self, surface_key: SurfaceKey) -> &wgpu::SurfaceTexture {
        self.0.get(&surface_key).unwrap()
    }

    pub fn present(self) {
        self.0.into_values().for_each(|st| {
            st.present();
        });
    }
}

pub struct Surfaces {
    map: SecondaryMap<SurfaceKey, wgpu::SurfaceConfiguration>,
}

impl Surfaces {
    pub fn new() -> Self {
        Self {
            map: SecondaryMap::new(),
        }
    }

    pub fn get_surface_textures(
        &mut self,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        target: &RenderTarget,
        resources: &Resources,
    ) -> ActiveSurfaceTextures {
        let mut surface_textures = ActiveSurfaceTextures(HashMap::new());

        let (width, height) = target.size();

        for color_attachment in target.color_attachments().iter() {
            let texture_handle = &color_attachment.texture;
            let texture = resources.get_texture(texture_handle).unwrap();
            if let TextureKind::Surface { surface_key, .. } = texture.kind() {
                let surface = resources.get_surface(*surface_key).unwrap();
                let internal_surface = self.map.get_mut(*surface_key);

                if let Some(config) = internal_surface {
                    if config.width != width || config.height != height {
                        config.width = width;
                        config.height = height;
                        surface.configure(&device, &config);
                    }
                } else {
                    let caps = surface.get_capabilities(&adapter);
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
                    surface.configure(&device, &config);
                    self.map.insert(*surface_key, config);
                }

                surface_textures
                    .0
                    .insert(*surface_key, surface.get_current_texture().unwrap());
            }
        }

        surface_textures
    }
}

use crate::graphics::{Texture, TextureId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) struct Textures {
    default_gpu_texture: GpuTexture,
    map: HashMap<TextureId, GpuTexture>,
}

impl Textures {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            default_gpu_texture: GpuTexture {
                texture,
                sampler: diffuse_sampler,
            },
            map: HashMap::new(),
        }
    }

    pub fn set_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &Rc<RefCell<Texture>>,
    ) {
        let texture = texture.borrow();

        let texture_id = texture.id();

        if self.map.contains_key(&texture_id) {
            return;
        } else {
            if let Some(source) = texture.source() {
                let size = wgpu::Extent3d {
                    width: source.width,
                    height: source.height,
                    depth_or_array_layers: 1,
                };
                let wgpu_texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("Texture"),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });

                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &wgpu_texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &source.data,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * source.width),
                        rows_per_image: Some(source.height),
                    },
                    size,
                );

                let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                });

                self.map.insert(
                    texture_id,
                    GpuTexture {
                        texture: wgpu_texture,
                        sampler: diffuse_sampler,
                    },
                );
            }
        }
    }

    pub fn get_texture(&self, texture: &Rc<RefCell<Texture>>) -> &GpuTexture {
        if let Some(gpu_texture) = self.map.get(&texture.borrow().id()) {
            return gpu_texture;
        }
        &self.default_gpu_texture
    }
}

pub(super) struct GpuTexture {
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
}

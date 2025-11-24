use crate::texture::{Texture, TextureFormat, TextureSource};
use crate::{ResourceKey, TextureHandle};
use slotmap::SecondaryMap;

pub(super) struct Textures {
    default_gpu_texture: GpuTexture,
    pool: SecondaryMap<ResourceKey, GpuTexture>,
}

impl Textures {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let default_gpu_texture = GpuTexture::new(device, (1, 1), TextureFormat::Rgba8Unorm);

        default_gpu_texture.upload(queue, &vec![255, 255, 255, 255], 1, 1);

        Self {
            default_gpu_texture,
            pool: SecondaryMap::new(),
        }
    }

    pub fn get_default_gpu_texture(&self) -> &GpuTexture {
        &self.default_gpu_texture
    }

    pub fn get_gpu_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &Texture,
        texture_handle: &TextureHandle,
    ) -> &GpuTexture {
        match texture.source() {
            TextureSource::D2 {
                data,
                width,
                height,
            } => {
                let create_gpu_texture = || {
                    let gpu_texture = GpuTexture::new(device, (*width, *height), texture.format());
                    gpu_texture.upload(queue, data, *width, *height);
                    gpu_texture
                };

                match self.pool.entry(texture_handle.raw()) {
                    Some(entry) => entry.or_insert_with(create_gpu_texture),
                    None => panic!("Texture source is Render, but has been removed from pool."),
                }
            }
            TextureSource::Render { width, height } => {
                if let Some(existing) = self.pool.get(texture_handle.raw()) {
                    let desc_size = existing.descriptor.size;
                    if desc_size.width != *width || desc_size.height != *height {
                        self.pool.remove(texture_handle.raw());
                    }
                }

                match self.pool.entry(texture_handle.raw()) {
                    Some(entry) => entry.or_insert_with(|| {
                        GpuTexture::new(device, (*width, *height), texture.format())
                    }),
                    None => panic!("Texture source is Render, but has been removed from pool."),
                }
            }
            _ => &self.default_gpu_texture,
        }
    }

    pub fn get_gpu_texture_by_id(&self, texture_handle: &TextureHandle) -> Option<&GpuTexture> {
        self.pool.get(texture_handle.raw())
    }
}

pub(super) struct GpuTexture {
    pub descriptor: wgpu::TextureDescriptor<'static>,
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
    pub view: wgpu::TextureView,
}

impl GpuTexture {
    pub fn new(device: &wgpu::Device, (width, height): (u32, u32), format: TextureFormat) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let descriptor = wgpu::TextureDescriptor {
            label: Some("Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format_to_wgpu_format(format),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture = device.create_texture(&descriptor);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            descriptor,
            texture,
            sampler,
            view,
        }
    }

    pub fn upload(&self, queue: &wgpu::Queue, data: &Vec<u8>, width: u32, height: u32) -> &Self {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            self.descriptor.size,
        );
        &self
    }
}

fn texture_format_to_wgpu_format(format: TextureFormat) -> wgpu::TextureFormat {
    match format {
        TextureFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
        TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
        TextureFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
        TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        TextureFormat::Depth24Plus => wgpu::TextureFormat::Depth24Plus,
        TextureFormat::Depth24PlusStencil8 => wgpu::TextureFormat::Depth24PlusStencil8,
    }
}

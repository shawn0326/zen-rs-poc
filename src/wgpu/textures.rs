use crate::graphics::{ImageSource, Texture, TextureId};
use std::collections::HashMap;

pub(super) struct Textures {
    default_gpu_texture: GpuTexture,
    map: HashMap<TextureId, GpuTexture>,
}

impl Textures {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            default_gpu_texture: GpuTexture::new(device, (1, 1)),
            map: HashMap::new(),
        }
    }

    pub fn get_gpu_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &Texture,
    ) -> &GpuTexture {
        let texture_id = texture.id();

        if let Some(source) = texture.source() {
            self.map.entry(texture_id).or_insert_with(|| {
                let gpu_texture = GpuTexture::new(device, (source.width, source.height));
                gpu_texture.upload(queue, source);
                gpu_texture
            })
        } else {
            &self.default_gpu_texture
        }
    }
}

pub(super) struct GpuTexture {
    pub descriptor: wgpu::TextureDescriptor<'static>,
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
}

impl GpuTexture {
    pub fn new(device: &wgpu::Device, (width, height): (u32, u32)) -> Self {
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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
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

        Self {
            descriptor,
            texture,
            sampler,
        }
    }

    pub fn upload(&self, queue: &wgpu::Queue, source: &ImageSource) -> &Self {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
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
            self.descriptor.size,
        );
        &self
    }

    pub fn create_binding_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
        }
    }

    // todo: cache view
    pub fn create_view(&self) -> wgpu::TextureView {
        self.texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn get_sampler_binding_type(&self) -> wgpu::SamplerBindingType {
        wgpu::SamplerBindingType::Filtering
    }
}

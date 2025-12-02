use crate::texture::{Texture, TextureKind};
use crate::{ResourceKey, TextureHandle};
use slotmap::SecondaryMap;

pub(super) struct Textures {
    default_gpu_texture: GpuTexture,
    pool: SecondaryMap<ResourceKey, GpuTexture>,
}

impl Textures {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let default_gpu_texture = GpuTexture::new(
            device,
            (1, 1, 1),
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        );

        default_gpu_texture.upload(queue, &vec![255, 255, 255, 255], 1, 1);

        Self {
            default_gpu_texture,
            pool: SecondaryMap::new(),
        }
    }

    pub fn get_default_gpu_texture(&self) -> &GpuTexture {
        &self.default_gpu_texture
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &Texture,
        texture_handle: &TextureHandle,
    ) -> &GpuTexture {
        match texture.kind() {
            TextureKind::D2 {
                data,
                width,
                height,
            } => {
                let create_gpu_texture = || {
                    let gpu_texture = GpuTexture::new(
                        device,
                        texture.kind().dimensions(),
                        texture.format(),
                        texture.usage(),
                    );
                    gpu_texture.upload(queue, data.bytes(), *width, *height);
                    gpu_texture
                };

                match self.pool.entry(texture_handle.raw()) {
                    Some(entry) => entry.or_insert_with(create_gpu_texture),
                    None => panic!("Texture source is Render, but has been removed from pool."),
                }
            }
            TextureKind::Render { width, height } => {
                if let Some(existing) = self.pool.get(texture_handle.raw()) {
                    let desc_size = existing.descriptor.size;
                    if desc_size.width != *width || desc_size.height != *height {
                        self.pool.remove(texture_handle.raw());
                    }
                }

                match self.pool.entry(texture_handle.raw()) {
                    Some(entry) => entry.or_insert_with(|| {
                        GpuTexture::new(
                            device,
                            texture.kind().dimensions(),
                            texture.format(),
                            texture.usage(),
                        )
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
    pub view: wgpu::TextureView,
}

impl GpuTexture {
    pub fn new(
        device: &wgpu::Device,
        (width, height, depth_or_array_layers): (u32, u32, u32),
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
    ) -> Self {
        let size: wgpu::Extent3d = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers,
        };

        let descriptor = wgpu::TextureDescriptor {
            label: Some("Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        };

        let texture = device.create_texture(&descriptor);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            descriptor,
            texture,
            view,
        }
    }

    pub fn upload(&self, queue: &wgpu::Queue, data: &[u8], width: u32, height: u32) -> &Self {
        let format = self.descriptor.format;

        let layout = bytes_layout_for_level(format, width, height);

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
                bytes_per_row: layout.and_then(|(b, _)| Some(b)),
                rows_per_image: layout.and_then(|(_, r)| Some(r)),
            },
            self.descriptor.size,
        );
        &self
    }
}

fn div_ceil(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

pub fn bytes_layout_for_level(
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
) -> Option<(u32, u32)> {
    // returns (bytes_per_row, bytes_per_image, total_bytes) or None if format needs special-case
    let block_size = format.block_copy_size(None)?;
    let (bw, bh) = format.block_dimensions();
    let blocks_w = div_ceil(width, bw);
    let blocks_h = div_ceil(height, bh);
    let bytes_per_row = block_size * blocks_w;
    let rows_per_image = blocks_h;
    Some((bytes_per_row, rows_per_image))
}

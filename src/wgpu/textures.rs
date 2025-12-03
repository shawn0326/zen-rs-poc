use crate::texture::{Texture, TextureData, TextureKind};
use crate::{ResourceKey, TextureHandle};
use slotmap::SecondaryMap;
use std::u64;

fn create_texture(device: &wgpu::Device, texture: &Texture) -> (wgpu::Texture, wgpu::TextureView) {
    let (width, height, depth_or_array_layers) = texture.kind().dimensions();

    let descriptor = wgpu::TextureDescriptor {
        label: texture.name(),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: texture.format(),
        usage: texture.usage(),
        view_formats: &[],
    };

    let gpu_texture = device.create_texture(&descriptor);

    let view = gpu_texture.create_view(&wgpu::TextureViewDescriptor::default());

    (gpu_texture, view)
}

fn div_ceil(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

fn bytes_layout_for_level(
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

pub struct InternalTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub texture_ver: u64,
    pub data_ver: Option<u64>,
}

impl InternalTexture {
    pub fn new(device: &wgpu::Device, texture: &Texture) -> Self {
        let (gpu_texture, view) = create_texture(device, texture);

        Self {
            texture: gpu_texture,
            texture_ver: texture.ver(),
            data_ver: None,
            view,
        }
    }

    pub fn ensure_gpu_texture(&mut self, device: &wgpu::Device, texture: &Texture) -> &mut Self {
        if self.texture_ver == texture.ver() {
            return self;
        }

        let (gpu_texture, view) = create_texture(device, texture);

        self.texture = gpu_texture;
        self.view = view;
        self.texture_ver = texture.ver();
        self.data_ver = None;

        self
    }

    pub fn upload_if_dirty(&mut self, queue: &wgpu::Queue, texture: &Texture) -> &mut Self {
        match texture.kind() {
            TextureKind::D2 {
                data,
                width,
                height,
            } => {
                let current_data_ver = data.ver();
                if self.data_ver == Some(current_data_ver) {
                    return self;
                } else {
                    self.data_ver = Some(current_data_ver);
                }

                let layout = bytes_layout_for_level(texture.format(), *width, *height);

                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &self.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    data.bytes(),
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: layout.and_then(|(b, _)| Some(b)),
                        rows_per_image: layout.and_then(|(_, r)| Some(r)),
                    },
                    wgpu::Extent3d {
                        width: *width,
                        height: *height,
                        depth_or_array_layers: 1,
                    },
                );
            }
            _ => (),
        }

        self
    }
}

pub struct Textures {
    default_gpu_texture: InternalTexture,
    pool: SecondaryMap<ResourceKey, InternalTexture>,
}

impl Textures {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let default_texture = Texture::new(
            TextureKind::D2 {
                data: TextureData::from_bytes(vec![255, 255, 255, 255]),
                width: 1,
                height: 1,
            },
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        );

        let mut default_gpu_texture = InternalTexture::new(device, &default_texture);
        default_gpu_texture.upload_if_dirty(queue, &default_texture);

        Self {
            default_gpu_texture,
            pool: SecondaryMap::new(),
        }
    }

    pub fn get_default_gpu_texture(&self) -> &InternalTexture {
        &self.default_gpu_texture
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &Texture,
        texture_handle: &TextureHandle,
    ) -> &InternalTexture {
        if let TextureKind::Empty = texture.kind() {
            return &self.default_gpu_texture;
        }

        let internal_texture = match self.pool.entry(texture_handle.raw()) {
            Some(entry) => entry.or_insert_with(|| InternalTexture::new(device, texture)),
            None => panic!("Texture has been removed from pool."),
        };

        internal_texture
            .ensure_gpu_texture(device, texture)
            .upload_if_dirty(queue, texture)
    }

    pub fn get_internal_texture(&self, texture_handle: &TextureHandle) -> &InternalTexture {
        self.pool.get(texture_handle.raw()).unwrap()
    }
}

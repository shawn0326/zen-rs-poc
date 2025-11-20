use super::textures::Textures;
use crate::{
    target::{LoadOp, RenderTarget, StoreOp},
    texture::TextureSource,
    wgpu::surfaces::ActiveSurfaceTextures,
};

pub(super) struct Targets {}

impl Targets {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_render_pass<'a>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_textures: &ActiveSurfaceTextures,
        encoder: &'a mut wgpu::CommandEncoder,
        textures: &mut Textures,
        target: &RenderTarget,
        resources: &crate::Resources,
    ) -> wgpu::RenderPass<'a> {
        let views: Vec<wgpu::TextureView> = target
            .color_attachments
            .iter()
            .map(|color_attachment| {
                let texture_handle = color_attachment.texture;
                let texture = resources.get_texture(texture_handle).unwrap();

                let gpu_texture = match texture.source() {
                    TextureSource::Surface { surface_id, .. } => {
                        &surface_textures.get_surface_texture(*surface_id).texture
                    }
                    _ => {
                        &textures
                            .get_gpu_texture(device, queue, &*texture, texture_handle)
                            .texture
                    }
                };

                gpu_texture.create_view(&wgpu::TextureViewDescriptor::default())
            })
            .collect();

        let color_attachments = target
            .color_attachments
            .iter()
            .zip(views.iter())
            .map(|(color_attachment, view)| {
                Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: match color_attachment.ops.load {
                            LoadOp::Clear(value) => wgpu::LoadOp::Clear(wgpu::Color {
                                r: value.r as f64,
                                g: value.g as f64,
                                b: value.b as f64,
                                a: value.a as f64,
                            }),
                            LoadOp::Load => wgpu::LoadOp::Load,
                        },
                        store: match color_attachment.ops.store {
                            StoreOp::Store => wgpu::StoreOp::Store,
                            StoreOp::Discard => wgpu::StoreOp::Discard,
                        },
                    },
                })
            })
            .collect::<Vec<_>>();

        let depth_stencil_attachment = match &target.depth_stencil_attachment {
            Some(depth_stencil_attachment) => {
                let texture_handle = depth_stencil_attachment.texture;
                let texture = resources.get_texture(texture_handle).unwrap();
                let gpu_texture =
                    textures.get_gpu_texture(device, queue, &*texture, texture_handle);
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &gpu_texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                    depth_ops: Some(wgpu::Operations {
                        load: match depth_stencil_attachment.depth_ops.load {
                            LoadOp::Clear(value) => wgpu::LoadOp::Clear(value),
                            LoadOp::Load => wgpu::LoadOp::Load,
                        },
                        store: match depth_stencil_attachment.depth_ops.store {
                            StoreOp::Store => wgpu::StoreOp::Store,
                            StoreOp::Discard => wgpu::StoreOp::Discard,
                        },
                    }),
                    // stencil_ops: Some(wgpu::Operations {
                    //     load: match depth_stencil_attachment.stencil_ops.load {
                    //         LoadOp::Clear(value) => wgpu::LoadOp::Clear(value),
                    //         LoadOp::Load => wgpu::LoadOp::Load,
                    //     },
                    //     store: match depth_stencil_attachment.stencil_ops.store {
                    //         StoreOp::Store => wgpu::StoreOp::Store,
                    //         StoreOp::Discard => wgpu::StoreOp::Discard,
                    //     },
                    // }),
                    stencil_ops: None,
                })
            }
            None => None,
        };

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(target.name.as_str()),
            color_attachments: &color_attachments,
            depth_stencil_attachment,
            ..Default::default()
        })
    }
}

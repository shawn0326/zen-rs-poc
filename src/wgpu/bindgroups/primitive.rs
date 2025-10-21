pub(in super::super) struct GpuPrimitiveBindGroup {}

impl GpuPrimitiveBindGroup {
    pub fn new(device: &wgpu::Device) -> Self {
        // let align = device.limits().min_uniform_buffer_offset_alignment as u64;

        // let stride = ((std::mem::size_of::<[f32; 16]>() as u64 + align - 1) / align) * align;

        println!(
            "min_uniform_buffer_offset_alignment: {}, max_uniform_buffer_binding_size: {}",
            (std::mem::size_of::<[f32; 16]>() as u64),
            device.limits().max_uniform_buffer_binding_size
        );

        device.limits().max_storage_buffer_binding_size;
        device.limits().max_buffer_size;
        Self {}
    }
}

// pub(in super::super) struct GpuPrimitiveBuffer {}

// impl GpuPrimitiveBuffer {
//     pub fn new(device: &wgpu::Device) -> Self {
//         Self {}
//     }
// }

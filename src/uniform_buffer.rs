use wgpu::util::DeviceExt;

use crate::renderer::Configuration;

pub fn create_uniform_buffer(config: &Configuration, label: &str, contents: &[u8]) -> wgpu::Buffer {
    let buffer = config
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
    buffer
}

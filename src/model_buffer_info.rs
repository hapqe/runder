use getset::Getters;
use gltf::Gltf;
use wgpu::util::DeviceExt;

use crate::{
    renderer::Configuration,
    view::{ViewInfo, ViewType},
};

#[derive(Getters)]
pub struct ModelBufferIndo {
    #[getset(get = "pub")]
    vertex_buffer: wgpu::Buffer,
    #[getset(get = "pub")]
    index_buffer: wgpu::Buffer,
    #[getset(get = "pub")]
    views: Vec<ViewType>,
}

impl ModelBufferIndo {
    pub fn new(config: &Configuration, gltf: &Gltf, raw_buffer_data: &[u8]) -> Self {
        let mut vertex_buffer_data = Vec::new();
        let mut index_buffer_data = Vec::new();
        let mut views = Vec::new();
        let mut raw_buffer_index = 0;

        for view in gltf.views() {
            let target = view.target().expect("Buffer views need to have a target");

            let buffer_data = match target {
                gltf::buffer::Target::ArrayBuffer => &mut vertex_buffer_data,
                gltf::buffer::Target::ElementArrayBuffer => &mut index_buffer_data,
            };

            let start = raw_buffer_index;
            let len = view.length();
            let data = &raw_buffer_data[start..start + len];

            let view_info = ViewInfo::new(len as u64, buffer_data.len() as u64);

            match target {
                gltf::buffer::Target::ArrayBuffer => views.push(ViewType::Vertex(view_info)),
                gltf::buffer::Target::ElementArrayBuffer => views.push(ViewType::Index(view_info)),
            };

            buffer_data.append(&mut data.to_vec());
            raw_buffer_index += len;
        }

        let vertex_buffer = create_buffer(config, &vertex_buffer_data, wgpu::BufferUsages::VERTEX);
        let index_buffer = create_buffer(config, &index_buffer_data, wgpu::BufferUsages::INDEX);

        Self {
            vertex_buffer,
            index_buffer,
            views,
        }
    }
}

fn create_buffer(
    config: &Configuration,
    buffer_data: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    config
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("buffer with usage {:?}", usage)),
            contents: buffer_data,
            usage: usage,
        })
}

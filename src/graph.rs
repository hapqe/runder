use gltf::Gltf;
use wgpu::util::DeviceExt;

use crate::renderer::Configuration;

///Create a graph of all scenes by consuming a gltf.
pub fn create_graph(gltf: Gltf, config: &Configuration) {
    for buffer in gltf.buffers() {
        let data = match buffer.source() {
            gltf::buffer::Source::Bin => todo!(),
            gltf::buffer::Source::Uri(data) => data,
        };

        let vertex_buffer = config
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: data.as_bytes(),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX,
            });
    }

    // for scene in gltf.scenes() {
    //     for node in scene.nodes() {
    //         if let Some(mesh) = node.mesh() {
    //             mesh.
    //         }
    //     }
    // }
}

use std::io::Cursor;

use getset::Getters;
use gltf::Gltf;

use crate::{buffer_info::BufferInfo, mesh::Mesh, renderer::Configuration};

#[derive(Getters)]
pub struct Graph {
    #[getset(get = "pub")]
    buffer_info: BufferInfo,
    #[getset(get = "pub")]
    meshes: Vec<Mesh>,
}

impl Graph {
    ///Create a graph of all scenes
    pub fn create(config: &Configuration) -> Graph {
        let gltf = Gltf::from_reader(Cursor::new(include_bytes!("gltf/scenes.gltf")))
            .expect("failed to import gltf!");

        let raw_buffer_data = include_bytes!("gltf/scenes.bin").to_vec();

        let buffer_info = BufferInfo::new(config, &gltf, &raw_buffer_data);

        let mut meshes = Vec::new();

        let scene = gltf
            .default_scene()
            .expect("There needs to be a default scene!");

        for node in scene.nodes() {
            if let Some(mesh) = node.mesh() {
                let mesh = Mesh::new(config, mesh, &buffer_info, node);
                meshes.push(mesh);
            }
        }

        Graph {
            buffer_info,
            meshes,
        }
    }
}

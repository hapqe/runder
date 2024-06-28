use std::io::Cursor;

use getset::Getters;
use gltf::{camera, Gltf};

use crate::{
    camera::{Camera, CameraData},
    math::{transform::Transform, vec::Vec4},
    mesh::Mesh,
    model_buffer_info::ModelBufferIndo,
    renderer::Configuration,
};

#[derive(Getters)]
pub struct Graph {
    #[getset(get = "pub")]
    buffer_info: ModelBufferIndo,
    #[getset(get = "pub")]
    meshes: Vec<Mesh>,
    #[getset(get = "pub")]
    camera: Camera,
}

impl Graph {
    ///Create a graph of all scenes
    pub fn create(config: &Configuration) -> Graph {
        let gltf = Gltf::from_reader(Cursor::new(include_bytes!("gltf/scenes.gltf")))
            .expect("failed to import gltf!");

        let raw_buffer_data = include_bytes!("gltf/scenes.bin").to_vec();

        let buffer_info = ModelBufferIndo::new(config, &gltf, &raw_buffer_data);

        let mut meshes = Vec::new();

        let scene = gltf
            .default_scene()
            .expect("There needs to be a default scene!");

        let camera = gltf
            .cameras()
            .next()
            .expect("There needs to be at least one camera!");

        let projection = match camera.projection() {
            gltf::camera::Projection::Orthographic(_) => {
                todo!("Orthographic Cameras not yet supported")
            }
            gltf::camera::Projection::Perspective(p) => p,
        };
        let camera_data = CameraData::new(
            projection.znear(),
            projection
                .zfar()
                .expect("Perspective camera needs to have a z-far"),
            projection.yfov(),
            Default::default(),
        );

        let camera = Camera::new(config, camera_data, camera.name().unwrap_or("Camera"));

        for node in scene.nodes() {
            if let Some(mesh) = node.mesh() {
                let mesh = Mesh::new(config, &camera, mesh, node, &buffer_info);
                meshes.push(mesh);
            }
        }

        Graph {
            buffer_info,
            meshes,
            camera,
        }
    }
}

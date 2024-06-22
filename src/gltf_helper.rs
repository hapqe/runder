use std::io::Cursor;

use gltf::Gltf;
use log::info;
use wgpu::util::DeviceExt;

use crate::renderer::Configuration;

// modification of https://docs.rs/wgpu/latest/wgpu/macro.include_wgsl.html
#[macro_export]
macro_rules! include_gltf {
    ($($token:tt)*) => {
        {
            crate::gltf_helper::path_to_gltf(include_str!($($token)*), $($token)*)
        }
    };
}

/// intended to be used with `include_str!("model.gltf")`
pub fn path_to_gltf(file: &str, name: &'static str) -> Gltf {
    info!("loaded {}!", name);

    let cursor = Cursor::new(file);

    Gltf::from_reader(cursor).expect(&format!("Failed to import, {}", name))
}

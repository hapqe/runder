use getset::Getters;
use gltf::buffer;

use crate::{
    bindgroup::{create_bindgroup, BindGroupEntryInfo, BindGroupInfo},
    math::{mat::Mat4, transform::Transform, vec::Vec4},
    renderer::Configuration,
    uniform_buffer::create_uniform_buffer,
};

#[derive(Clone, Debug)]
pub struct CameraData {
    pub near: f32,
    pub far: f32,
    /// Field of view on the Y-Axis
    pub fov: f32,

    pub transform: Transform,
}

impl CameraData {
    pub fn new(near: f32, far: f32, fov: f32, transform: Transform) -> Self {
        Self {
            near,
            far,
            fov,
            transform,
        }
    }
}

#[derive(Getters)]
pub struct Camera {
    data: CameraData,
    buffer: wgpu::Buffer,
    #[getset(get = "pub")]
    bind_group: BindGroupInfo,
}

impl Camera {
    pub fn new(config: &Configuration, data: CameraData, label: &str) -> Self {
        let CameraData { near, far, fov, .. } = data;

        let y = (fov / 2.0).tan();
        let aspect = config.size.width as f32 / config.size.height as f32;

        let projection = Mat4::new(
            Vec4::new(1.0 / (y * aspect), 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0 / y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, far / (far - near), 1.0),
            Vec4::new(0.0, 0.0, (-far * near) / (far - near), 0.0),
        );

        // let projection = Mat4::new(
        //     Vec4::new(near, 0.0, 0.0, 0.0),
        //     Vec4::new(0.0, near, 0.0, 0.0),
        //     Vec4::new(0.0, 0.0, far / (far - near), 1.0),
        //     Vec4::new(0.0, 0.0, (-far * near) / (far - near), 0.0),
        // );

        let buffer = create_uniform_buffer(config, label, &projection.bytes());

        let bind_group = create_bindgroup(
            config,
            &[BindGroupEntryInfo::new(
                wgpu::ShaderStages::VERTEX,
                wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            )],
            label,
        );

        Self {
            data,
            buffer,
            bind_group,
        }
    }
}

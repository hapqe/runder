use gltf::Node;
use wgpu::util::DeviceExt;

use crate::{
    bindgroup::{self, BindGroupEntryInfo, BindGroupInfo},
    buffer_info::BufferInfo,
    math::{
        mat::Mat4,
        transform::{self, Transform},
        vec::Vec3,
    },
    primitive::Primitive,
    renderer::Configuration,
};

pub struct Mesh {
    primitives: Vec<Primitive>,
    transform_bind_group: BindGroupInfo,
    transform: Transform,
}

impl Mesh {
    pub fn new(
        config: &Configuration,
        mesh: gltf::Mesh,
        buffer_info: &BufferInfo,
        node: Node,
    ) -> Self {
        let decomposed = node.transform().decomposed();
        let position = decomposed.0.into();
        let rotation = Vec3::euler_from_quaternion_data(decomposed.1);
        let scale = decomposed.2.into();

        let transform = Transform::new(position, rotation, scale);

        let buffer = config
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: &Mat4::transform(&transform).data(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let transform_bind_group = bindgroup::create_bindgroup(
            config,
            &[BindGroupEntryInfo::new(
                wgpu::ShaderStages::VERTEX,
                wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            )],
            Some(&format!(
                "Transform bind group of '{}'",
                node.name().unwrap_or("Unnamed Node")
            )),
        );

        let primitives = mesh
            .primitives()
            .map(|primitive| {
                Primitive::new(
                    config,
                    &primitive,
                    &mesh,
                    &[&transform_bind_group.layout],
                    buffer_info,
                )
            })
            .collect();

        Self {
            primitives,
            transform_bind_group,
            transform,
        }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        buffer_info: &'a BufferInfo,
    ) {
        render_pass.set_bind_group(0, &self.transform_bind_group.group, &[]);

        for primitive in &self.primitives {
            primitive.render(render_pass, buffer_info);
        }
    }
}

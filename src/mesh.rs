use crate::{
    bindgroup::{self, BindGroupEntryInfo, BindGroupInfo},
    camera::Camera,
    math::{mat::Mat4, transform::Transform, vec::Vec3},
    model_buffer_info::ModelBufferIndo,
    primitive::Primitive,
    renderer::Configuration,
    uniform_buffer::create_uniform_buffer,
};

pub struct Mesh {
    primitives: Vec<Primitive>,
    transform_bind_group: BindGroupInfo,
    transform: Transform,
}

impl Mesh {
    pub fn new(
        config: &Configuration,
        camera: &Camera,
        mesh: gltf::Mesh,
        node: gltf::Node,
        buffer_info: &ModelBufferIndo,
    ) -> Self {
        let name = node.name().unwrap_or("Unnamed Mesh");

        let decomposed = node.transform().decomposed();
        let position = decomposed.0.into();
        let rotation = Vec3::euler_from_quaternion_data(decomposed.1);
        let scale = decomposed.2.into();

        let transform = Transform::new(position, rotation, scale);

        let buffer = create_uniform_buffer(
            config,
            &format!("Transform buffer for '{}'", name),
            &Mat4::transform(&transform).bytes(),
        );

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
            &format!("Transform bind group of '{}'", name),
        );

        let primitives = mesh
            .primitives()
            .map(|primitive| {
                Primitive::new(
                    config,
                    &primitive,
                    &mesh,
                    &[&transform_bind_group.layout, &camera.bind_group().layout],
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
        buffer_info: &'a ModelBufferIndo,
    ) {
        render_pass.set_bind_group(0, &self.transform_bind_group.group, &[]);

        for primitive in &self.primitives {
            primitive.render(render_pass, buffer_info);
        }
    }
}

use crate::{buffer_info::BufferInfo, pipeline, renderer::Configuration, view::ViewType};

pub struct Primitive {
    pipeline: wgpu::RenderPipeline,
    views: Vec<ViewType>,
}

impl Primitive {
    pub fn new(
        config: &Configuration,
        primitive: &gltf::Primitive,
        mesh: &gltf::Mesh,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        buffer_info: &BufferInfo,
    ) -> Self {
        let attributes = primitive.attributes();

        let layouts: Vec<_> = attributes
            .map(|(attr, _)| match attr {
                gltf::Semantic::Positions => wgpu::VertexBufferLayout {
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }],
                    array_stride: 12,
                },
                gltf::Semantic::Normals => wgpu::VertexBufferLayout {
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 1,
                    }],
                    array_stride: 12,
                },
                gltf::Semantic::TexCoords(0) => wgpu::VertexBufferLayout {
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 2,
                    }],
                    array_stride: 8,
                },
                _ => panic!("Vertex attribute {:?} not implemented!", attr),
            })
            .collect();

        let label = &format!(
            "pipeline for mesh '{}' with material '{}'",
            mesh.name().unwrap_or("unnamed mesh"),
            primitive.material().name().unwrap_or("unnamed material")
        );

        let pipeline = pipeline::create(
            &config,
            &layouts,
            bind_group_layouts,
            wgpu::include_wgsl!("test.wgsl"),
            Some(&label),
        );

        let mut views: Vec<_> = primitive
            .attributes()
            .map(|(_, attr)| buffer_info.views()[attr.index()])
            .collect();

        views.push(
            primitive
                .indices()
                .map(|indices| buffer_info.views()[indices.index()])
                .expect("No indices accessor!"),
        );

        Self { pipeline, views }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        buffer_info: &'a BufferInfo,
    ) {
        render_pass.set_pipeline(&self.pipeline);

        for (i, view) in self.views.iter().enumerate() {
            match view {
                ViewType::Vertex(info) => {
                    render_pass.set_vertex_buffer(
                        i as u32,
                        buffer_info
                            .vertex_buffer()
                            .slice(info.offset..info.offset + info.lenght),
                    );
                }
                ViewType::Index(info) => {
                    render_pass.set_index_buffer(
                        buffer_info
                            .index_buffer()
                            .slice(info.offset..info.offset + info.lenght),
                        wgpu::IndexFormat::Uint16,
                    );

                    // Requiring that the index view is the last view (all vertex buffers have been set)
                    render_pass.draw_indexed(0..info.lenght as u32 / 2, 0, 0..1);
                }
            }
        }
    }
}

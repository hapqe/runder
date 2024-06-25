use crate::renderer::Configuration;

pub struct BindGroupEntryInfo<'a> {
    /// where the bindgroup should will be used
    pub visibility: wgpu::ShaderStages,
    /// what data the bindgroup will be using
    pub resource: wgpu::BindingResource<'a>,
}

pub struct BindGroupInfo {
    pub layout: wgpu::BindGroupLayout,
    pub group: wgpu::BindGroup,
}

impl<'a> BindGroupEntryInfo<'a> {
    pub fn new(visibility: wgpu::ShaderStages, resource: wgpu::BindingResource<'a>) -> Self {
        Self {
            visibility,
            resource,
        }
    }
}

pub fn create_bindgroup(
    config: &Configuration,
    entries: &[BindGroupEntryInfo],
    label: Option<&str>,
) -> BindGroupInfo {
    let layout = config
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &entries
                .iter()
                .enumerate()
                .map(|(i, entry)| wgpu::BindGroupLayoutEntry {
                    binding: i as u32,
                    visibility: entry.visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                })
                .collect::<Vec<_>>(),
            label: label
                .map(|l| format!("bindgroup layout for '{}'", l))
                .as_deref(),
        });

    let bind_group = config.device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        entries: &entries
            .iter()
            .enumerate()
            .map(|(i, entry)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: entry.resource.clone(),
            })
            .collect::<Vec<_>>(),
        label: label.map(|l| format!("bindgroup for '{}'", l)).as_deref(),
    });

    BindGroupInfo {
        layout,
        group: bind_group,
    }
}

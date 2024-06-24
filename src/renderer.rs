use winit::window::Window;

use log::info;

use crate::graph::Graph;

pub struct Configuration<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    // there are unsafe references so a reference here ensures a drop
    // according to https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#first-some-housekeeping-state
    pub window: &'a Window,
}

/// A renderer, assuming that there is a surface to draw on
pub struct RendererState<'a> {
    pub config: Configuration<'a>,

    pub graph: Graph,
}

impl<'a> RendererState<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        // creating the render texture
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let config = Configuration {
            window,
            surface,
            device,
            queue,
            surface_config: config,
            size,
        };

        let graph = Graph::create(&config);

        Self { config, graph }
    }

    pub fn window(&self) -> &Window {
        &self.config.window
    }

    // copy of https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#resize
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let config = &mut self.config;

        info!(
            "resize to width: {} height: {}!",
            new_size.width, new_size.height
        );

        if new_size.width > 0 && new_size.height > 0 {
            config.size = new_size;
            config.surface_config.width = new_size.width;
            config.surface_config.height = new_size.height;
            config
                .surface
                .configure(&config.device, &config.surface_config);
        }
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let config = &mut self.config;

        let output = config.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = config
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Color Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.8,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None, // Depth for later
            ..Default::default()
        });

        for primitive in self.graph.primitives() {
            primitive.render(&mut render_pass, &self.graph.buffer_info());
        }

        drop(render_pass);

        // Has to be an iterator, hence once
        config.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

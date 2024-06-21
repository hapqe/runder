use log::{error, warn};
use winit::{
    event::{self, Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::WindowBuilder,
};

use crate::renderer::Renderer;

pub async fn init() {
    let event_loop = EventLoopBuilder::new().build().unwrap();

    let window = WindowBuilder::new()
        .with_title("snagg")
        .build(&event_loop)
        .unwrap();

    let mut renderer = Renderer::new(&window).await;

    event_loop.run(move |event, target| match event {
        // Close window, uppon requesting
        Event::WindowEvent { window_id, event } if window_id == renderer.window.id() => match event
        {
            WindowEvent::CloseRequested => target.exit(),
            WindowEvent::Resized(physical_size) => {
                renderer.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                renderer.window().request_redraw();

                // TODO: Render Update should happen here!

                // as implemented in https://github.com/sotrh/learn-wgpu/tree/master/code/beginner/tutorial2-surface/
                match renderer.draw() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        renderer.resize(renderer.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("OutOfMemory");
                        target.exit();
                    }

                    // This happens when the a frame takes too long to present
                    Err(wgpu::SurfaceError::Timeout) => {
                        warn!("Surface timeout")
                    }
                }
            }
            _ => (),
        },
        _ => (),
    });
}

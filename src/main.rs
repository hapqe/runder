mod app;
mod gltf_helper;
mod graph;
mod pipeline;
mod renderer;

use renderer::RendererState;

fn main() {
    // create a window
    pollster::block_on(app::init());
}

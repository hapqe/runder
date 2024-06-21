mod app;
mod pipeline;
mod renderer;

use renderer::Renderer;

fn main() {
    // more beautiful logging
    env_logger::init();

    // create a window
    pollster::block_on(app::init());
}

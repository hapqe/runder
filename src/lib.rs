mod app;
mod bindgroup;
mod camera;
mod graph;
mod math;
mod mesh;
mod model_buffer_info;
mod pipeline;
mod primitive;
mod renderer;
mod uniform_buffer;
mod view;

pub fn run() {
    // create a window
    pollster::block_on(app::init());
}

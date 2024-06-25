mod app;
mod bindgroup;
mod buffer_info;
mod graph;
mod math;
mod mesh;
mod pipeline;
mod primitive;
mod renderer;
mod view;

pub fn run() {
    // create a window
    pollster::block_on(app::init());
}

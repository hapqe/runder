mod app;
mod buffer_info;
mod graph;
mod pipeline;
mod primitive;
mod renderer;
mod view;

fn main() {
    // create a window
    pollster::block_on(app::init());
}

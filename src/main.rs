mod app;
mod camera;
mod model;
mod renderer;
mod ui;

fn main() {
    println!("Mini CAD Viewer");
    renderer::setup::init();

    // Main loop
    loop {}
}

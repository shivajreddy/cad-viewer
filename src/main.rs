mod app;
mod camera;
mod model;
mod renderer;
mod ui;

use app::App;

use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mini CAD Viewer");

    // 1. Create the event loop
    let event_loop = EventLoop::new()?;

    // 2. Instanstiate the app
    let mut app = App::new();

    // 3. Run app
    event_loop.run_app(&mut app)?;

    Ok(())
}

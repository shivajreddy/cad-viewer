/*
 * top-level application state
*/

use wgpu::InstanceDescriptor;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::{Window, WindowAttributes};

const WINDOW_TITLE: &str = "CAD Viewer";

#[derive(Default)]
pub struct App {
    window: Option<Window>,
}

impl App {
    pub fn new() -> Self {
        App { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default().with_title(WINDOW_TITLE))
            .unwrap();

        // 1
        let inst = wgpu::Instance::new(InstanceDescriptor::new_without_display_handle());

        // 2
        inst.create_surface(&window);

        // 3
        let adapter = inst.request_adapter(options);

        // 4
        adapter.request_device();

        // 5
        surface.configure()

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

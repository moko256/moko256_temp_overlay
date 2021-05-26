mod app;
mod app_renderer;
mod render_primitives;

use app::App;
use app_renderer::AppRenderer;

use raw_window_handle::HasRawWindowHandle;
use raw_window_handle::RawWindowHandle;
//use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const TEXTURE_WIDTH: u32 = 800;
const TEXTURE_HEIGHT: u32 = 600;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("temp_overlay")
        .with_inner_size(PhysicalSize::new(TEXTURE_WIDTH, TEXTURE_HEIGHT))
        .with_transparent(true)
        // .with_position(PhysicalPosition::new(0.0, 0.0))
        // .with_decorations(false)
        // .with_always_on_top(true)
        .build(&event_loop)
        .unwrap();
    let hwnd = match window.raw_window_handle() {
        RawWindowHandle::Windows(hwnd) => Some(hwnd.hwnd),
        _ => None,
    }
    .unwrap();

    let mut app = App::new(TEXTURE_WIDTH as f32, TEXTURE_HEIGHT as f32);
    let mut app_renderer = AppRenderer::new(hwnd, TEXTURE_WIDTH, TEXTURE_HEIGHT);

    app.resize(TEXTURE_WIDTH as f32, TEXTURE_HEIGHT as f32);
    app_renderer.resize(TEXTURE_WIDTH, TEXTURE_HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                app.resize(size.width as f32, size.height as f32);
                app_renderer.resize(size.width, size.height);
            }
            Event::MainEventsCleared => {
                if app.need_redraw_and_consume() {
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                app_renderer.draw(&app.primitives);
            }
            _ => (),
        }
    });
}

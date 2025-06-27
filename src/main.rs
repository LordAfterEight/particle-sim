use winit::event_loop::EventLoop;
use winit::window::Window;
use pixels;

fn main() {
    println!("Hello, world!");
    let event_loop = EventLoop::new().unwrap();
    let window_attributes = Window::default_attributes()
        .with_title("Rusty Pixels")
        .with_resizable(false)
        .with_blur(true)
        .with_transparent(true);
    let window = event_loop.create_window(window_attributes).unwrap();
}

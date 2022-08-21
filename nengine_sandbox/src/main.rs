use nengine::platform::CrossPlatformWindow;
use nengine::platform::Window;
use nengine::Event;

fn main() {
    let mut window = Window::new(800, 600, "Nengine Sandbox", false);
    window.set_event_callback(|event| {
        if let Event::MouseScroll { x, y } = event {
            println!("[INFO]: Scroll event: {} {}", x, y);
        }
    });

    window.show();

    while window.is_open() {
        window.poll_events();
    }
}

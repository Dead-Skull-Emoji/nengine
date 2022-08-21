use nengine::platform::CrossPlatformWindow;
use nengine::platform::Window;
use nengine::Event;
use nengine::MouseButton;

fn main() {
    let mut window = Window::new(800, 600, "Nengine Sandbox", false);
    window.set_event_callback(|event| {
        if let Event::MouseScroll { x, y } = event {
            println!("[INFO]: Scroll event: {} {}", x, y);
        } else if let Event::MouseButton { button, is_press } = event {
            if let MouseButton::Right = button {
                if !is_press {
                    println!("[INFO]: Right click button released!");
                } else {
                    println!("[INFO]: Right click button pressed!");
                }
            }
        }
    });

    window.show();

    while window.is_open() {
        window.poll_events();
    }
}

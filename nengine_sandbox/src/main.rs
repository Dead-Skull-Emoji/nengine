use nengine::platform::CrossPlatformWindow;
use nengine::platform::Window;
use nengine::Event;
use nengine::MouseButton;

fn main() {
    let mut window = Window::new(800, 600, "Nengine Sandbox", false);
    window.set_event_callback(|event| {
        if let Event::MouseButton { button, is_press } = event {
            if is_press {
                if let MouseButton::Left = button {
                    println!("Left mouse button!");
                } else if let MouseButton::Right = button {
                    println!("Right mouse button!");
                }
            }
        }
    });

    window.show();

    while window.is_open() {
        window.poll_events();
    }
}

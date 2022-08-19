use nengine::platform::Window;
use nengine::platform::CrossPlatformWindow;

fn main() {
    let mut window = Window::new(800, 600, "Nengine Sandbox", true);
    
    window.show();
    
    while window.is_open() {
        window.poll_events();
    }
}

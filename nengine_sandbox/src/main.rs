use nengine::platform::Window;

fn main() {
    let window = Window::new(800, 600, "Nengine Sandbox", false);
    
    window.show();
    
    while window.is_open() {
        window.poll_events();
    }
}

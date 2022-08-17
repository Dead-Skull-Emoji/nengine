use nengine::core::Window;

fn main() {
    let mut window = Window::new(800, 600, "bozo", false);
    
    window.show();
    
    while window.is_open() {
        window.poll_events();
    }
}

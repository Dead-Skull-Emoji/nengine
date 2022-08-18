use nengine::platform::Window;

fn main() {
    let window = Window::new(800, 600, "bozo\0", false).unwrap();
    
    window.show();
    
    while window.is_open() {
        window.poll_events();
    }
}

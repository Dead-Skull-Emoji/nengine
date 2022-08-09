use nengine::core::Nengine;

fn main() {
    let mut nengine = Nengine::new();
    
    while nengine.is_running() {
        nengine.update();
    }
}

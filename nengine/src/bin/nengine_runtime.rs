use nengine::core::Nengine;

fn main() {
    let mut nengine = Nengine::new();
    nengine.init();
    
    while nengine.is_running() {
        nengine.update();
    }
}

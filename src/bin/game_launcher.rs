use nengine::engine_core;

fn main() {
    engine_core::init();
    
    while engine_core::update() {}
    
    engine_core::stop();
}
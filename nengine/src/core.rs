pub struct Nengine {}

impl Nengine {
    pub fn new() -> Nengine {
        println!("[INFO]: Initialized the Nengine.");

        return Nengine {};
    }
    
    pub fn is_running(&self) -> bool {
        return true;
    }
    
    pub fn update(&mut self) {
        println!("[INFO]: Updating the Nengine.");
    }
}

impl Drop for Nengine {
    fn drop(&mut self) {
        println!("[INFO]: Shutting down the Nengine.");
    }
}

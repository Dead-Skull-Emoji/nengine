use nengine_vulkan::backend::Instance;

fn main() {
    let _instance = Instance::new("Nengine Sandbox", 0, true).unwrap();
    
    Instance::enumerate_instance_extension_names().iter().for_each(|extension| println!("[INFO]: Found Vulkan extension {}", extension));
}

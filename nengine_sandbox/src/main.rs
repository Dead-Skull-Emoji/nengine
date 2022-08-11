use nengine_vulkan::backend::Instance;

fn main() {
    Instance::enumerate_instance_extension_names()
        .iter()
        .for_each(|extension| println!("[INFO]: Found Vulkan extension {}", extension));
    let instance = Instance::new("Nengine Sandbox", 0, true).unwrap();
    instance
        .enumerate_physical_devices()
        .iter()
        .for_each(|device| {
            println!(
                "[INFO]: Found Vulkan physical device {}",
                device.get_properties().device_name
            )
        });
}

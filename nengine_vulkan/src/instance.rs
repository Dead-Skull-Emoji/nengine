use crate::raw::*;

fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (((variant)) << 29) | (((major)) << 22) | (((minor)) << 12) | ((patch))
}

pub struct Instance {
    raw_handle: VkInstance
}

impl Instance {
    pub fn new(app_name: &str, app_version: u32, enable_validation: bool) -> Result<Instance, String> {
        unsafe {
            let app_name = app_name.to_owned() + "\0";
            
            let app_info = VkApplicationInfo {
                sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: std::ptr::null(),
                pApplicationName: app_name.as_ptr() as *const i8,
                applicationVersion: app_version,
                pEngineName: b"Nengine\0".as_ptr() as *const i8,
                engineVersion: VK_VERSION_1_0,
                apiVersion: make_api_version(0, 1, 3, 0)
            };
            
            let validation_layers = vec![
                b"VK_LAYER_KHRONOS_validation".as_ptr() as *const i8
            ];
            
            let create_info = VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                pApplicationInfo: &app_info,
                enabledLayerCount: 0,
                ppEnabledLayerNames: if enable_validation { validation_layers.as_ptr() } else { std::ptr::null() },
                enabledExtensionCount: if enable_validation { validation_layers.len().try_into().unwrap() } else { 0 },
                ppEnabledExtensionNames: std::ptr::null()
            };
            
            let mut instance = std::ptr::null_mut();
            let result = vkCreateInstance(&create_info, std::ptr::null(), &mut instance);
            
            if result == VK_SUCCESS {
                Ok(Instance { raw_handle: instance })
            } else {
                Err("Failed to create Instance!".to_string())
            }
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self.raw_handle, std::ptr::null());
        }
    }
}
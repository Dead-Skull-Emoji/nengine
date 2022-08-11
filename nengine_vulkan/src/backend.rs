pub mod raw;

use raw::*;
use std::ffi::c_void;
use std::ffi::CStr;

fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    ((variant) << 29) | ((major) << 22) | ((minor) << 12) | (patch)
}

unsafe extern "C" fn debug_callback(
    _message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    _message_type: VkDebugUtilsMessageTypeFlagsEXT,
    callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> VkBool32 {
    println!(
        "[VULKAN]: {}",
        CStr::from_ptr((*callback_data).pMessage).to_str().unwrap()
    );

    return VK_FALSE;
}

fn get_debug_messenger_info() -> VkDebugUtilsMessengerCreateInfoEXT {
    VkDebugUtilsMessengerCreateInfoEXT {
        sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        pNext: std::ptr::null(),
        flags: 0,
        messageSeverity: VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT
            | VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT
            | VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
        messageType: VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
            | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
            | VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
        pfnUserCallback: Some(debug_callback),
        pUserData: std::ptr::null_mut(),
    }
}

pub struct Instance {
    raw_handle: VkInstance,
}

impl Instance {
    pub fn new(
        app_name: &str,
        app_version: u32,
        enable_validation: bool,
    ) -> Result<Instance, String> {
        unsafe {
            let app_name = app_name.to_owned() + "\0";

            let app_info = VkApplicationInfo {
                sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: std::ptr::null(),
                pApplicationName: app_name.as_ptr() as *const i8,
                applicationVersion: app_version,
                pEngineName: b"Nengine\0".as_ptr() as *const i8,
                engineVersion: VK_VERSION_1_0,
                apiVersion: make_api_version(0, 1, 0, 0),
            };

            let validation_layers = vec![b"VK_LAYER_KHRONOS_validation\0".as_ptr() as *const i8];

            let mut extensions = Vec::new();

            if enable_validation {
                extensions.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME.as_ptr() as *const i8);
            }

            let debug_messenger_info = get_debug_messenger_info();

            let create_info = VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: if enable_validation {
                    (&debug_messenger_info as *const VkDebugUtilsMessengerCreateInfoEXT)
                        as *const c_void
                } else {
                    std::ptr::null()
                },
                flags: 0,
                pApplicationInfo: &app_info,
                enabledLayerCount: if enable_validation {
                    validation_layers.len().try_into().unwrap()
                } else {
                    0
                },
                ppEnabledLayerNames: if enable_validation {
                    validation_layers.as_ptr()
                } else {
                    std::ptr::null()
                },
                enabledExtensionCount: extensions.len().try_into().unwrap(),
                ppEnabledExtensionNames: if !extensions.is_empty() {
                    extensions.as_ptr()
                } else {
                    std::ptr::null()
                },
            };

            let mut instance = std::ptr::null_mut();
            let result = vkCreateInstance(&create_info, std::ptr::null(), &mut instance);

            if result == VK_SUCCESS {
                Ok(Instance {
                    raw_handle: instance,
                })
            } else {
                Err("Failed to create Instance!".to_string())
            }
        }
    }

    pub fn enumerate_instance_extension_names() -> Vec<String> {
        unsafe {
            let mut extension_count = 0;
            vkEnumerateInstanceExtensionProperties(
                std::ptr::null(),
                &mut extension_count,
                std::ptr::null_mut(),
            );

            let mut extensions = Vec::with_capacity(extension_count.try_into().unwrap());
            vkEnumerateInstanceExtensionProperties(
                std::ptr::null(),
                &mut extension_count,
                extensions.as_mut_ptr(),
            );
            extensions.set_len(extension_count.try_into().unwrap());

            extensions
                .iter()
                .map(|extension| {
                    String::from_utf8(
                        extension
                            .extensionName
                            .iter()
                            .map(|c| c.clone() as u8)
                            .collect(),
                    )
                    .unwrap()
                })
                .collect()
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

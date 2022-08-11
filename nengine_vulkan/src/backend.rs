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
                apiVersion: make_api_version(0, 1, 1, 0),
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

    pub fn enumerate_physical_devices(&self) -> Vec<PhysicalDevice> {
        unsafe {
            let mut device_count = 0;
            vkEnumeratePhysicalDevices(self.raw_handle, &mut device_count, std::ptr::null_mut());

            let mut devices = Vec::with_capacity(device_count.try_into().unwrap());
            vkEnumeratePhysicalDevices(self.raw_handle, &mut device_count, devices.as_mut_ptr());
            devices.set_len(device_count.try_into().unwrap());

            devices
                .iter()
                .map(|device| PhysicalDevice {
                    raw_handle: device.clone(),
                })
                .collect()
        }
    }

    pub fn create_debug_utils_messenger(&self) -> Result<DebugUtilsMessengerEXT, ()> {
        unsafe {
            let create_info = get_debug_messenger_info();

            if let Some(func) = vkGetInstanceProcAddr(
                self.raw_handle,
                b"vkCreateDebugUtilsMessengerEXT\0".as_ptr() as *const i8,
            ) {
                let func = std::mem::transmute::<
                    unsafe extern "C" fn(),
                    unsafe extern "C" fn(
                        VkInstance,
                        *const VkDebugUtilsMessengerCreateInfoEXT,
                        *const VkAllocationCallbacks,
                        *mut VkDebugUtilsMessengerEXT,
                    ) -> VkResult,
                >(func);

                let mut debug_messenger = std::ptr::null_mut();
                func(
                    self.raw_handle,
                    &create_info,
                    std::ptr::null(),
                    &mut debug_messenger,
                );

                Ok(DebugUtilsMessengerEXT {
                    raw_handle: debug_messenger,
                    instance: &self,
                })
            } else {
                Err(())
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

pub struct PhysicalDeviceProperties {
    pub device_name: String,
}

pub struct PhysicalDevice {
    raw_handle: VkPhysicalDevice,
}

impl PhysicalDevice {
    pub fn get_properties(&self) -> PhysicalDeviceProperties {
        unsafe {
            let mut device_properties = VkPhysicalDeviceProperties {
                apiVersion: 0,
                driverVersion: 0,
                vendorID: 0,
                deviceID: 0,
                deviceType: 0,
                deviceName: [0; 256],
                pipelineCacheUUID: [0; 16],
                limits: VkPhysicalDeviceLimits {
                    maxImageDimension1D: 0,
                    maxImageDimension2D: 0,
                    maxImageDimension3D: 0,
                    maxImageDimensionCube: 0,
                    maxImageArrayLayers: 0,
                    maxTexelBufferElements: 0,
                    maxUniformBufferRange: 0,
                    maxStorageBufferRange: 0,
                    maxPushConstantsSize: 0,
                    maxMemoryAllocationCount: 0,
                    maxSamplerAllocationCount: 0,
                    bufferImageGranularity: 0,
                    sparseAddressSpaceSize: 0,
                    maxBoundDescriptorSets: 0,
                    maxPerStageDescriptorSamplers: 0,
                    maxPerStageDescriptorUniformBuffers: 0,
                    maxPerStageDescriptorStorageBuffers: 0,
                    maxPerStageDescriptorSampledImages: 0,
                    maxPerStageDescriptorStorageImages: 0,
                    maxPerStageDescriptorInputAttachments: 0,
                    maxPerStageResources: 0,
                    maxDescriptorSetSamplers: 0,
                    maxDescriptorSetUniformBuffers: 0,
                    maxDescriptorSetUniformBuffersDynamic: 0,
                    maxDescriptorSetStorageBuffers: 0,
                    maxDescriptorSetStorageBuffersDynamic: 0,
                    maxDescriptorSetSampledImages: 0,
                    maxDescriptorSetStorageImages: 0,
                    maxDescriptorSetInputAttachments: 0,
                    maxVertexInputAttributes: 0,
                    maxVertexInputBindings: 0,
                    maxVertexInputAttributeOffset: 0,
                    maxVertexInputBindingStride: 0,
                    maxVertexOutputComponents: 0,
                    maxTessellationGenerationLevel: 0,
                    maxTessellationPatchSize: 0,
                    maxTessellationControlPerVertexInputComponents: 0,
                    maxTessellationControlPerVertexOutputComponents: 0,
                    maxTessellationControlPerPatchOutputComponents: 0,
                    maxTessellationControlTotalOutputComponents: 0,
                    maxTessellationEvaluationInputComponents: 0,
                    maxTessellationEvaluationOutputComponents: 0,
                    maxGeometryShaderInvocations: 0,
                    maxGeometryInputComponents: 0,
                    maxGeometryOutputComponents: 0,
                    maxGeometryOutputVertices: 0,
                    maxGeometryTotalOutputComponents: 0,
                    maxFragmentInputComponents: 0,
                    maxFragmentOutputAttachments: 0,
                    maxFragmentDualSrcAttachments: 0,
                    maxFragmentCombinedOutputResources: 0,
                    maxComputeSharedMemorySize: 0,
                    maxComputeWorkGroupCount: [0; 3],
                    maxComputeWorkGroupInvocations: 0,
                    maxComputeWorkGroupSize: [0; 3],
                    subPixelPrecisionBits: 0,
                    subTexelPrecisionBits: 0,
                    mipmapPrecisionBits: 0,
                    maxDrawIndexedIndexValue: 0,
                    maxDrawIndirectCount: 0,
                    maxSamplerLodBias: 0.0,
                    maxSamplerAnisotropy: 0.0,
                    maxViewports: 0,
                    maxViewportDimensions: [0; 2],
                    viewportBoundsRange: [0.0; 2],
                    viewportSubPixelBits: 0,
                    minMemoryMapAlignment: 0,
                    minTexelBufferOffsetAlignment: 0,
                    minUniformBufferOffsetAlignment: 0,
                    minStorageBufferOffsetAlignment: 0,
                    minTexelOffset: 0,
                    maxTexelOffset: 0,
                    minTexelGatherOffset: 0,
                    maxTexelGatherOffset: 0,
                    minInterpolationOffset: 0.0,
                    maxInterpolationOffset: 0.0,
                    subPixelInterpolationOffsetBits: 0,
                    maxFramebufferWidth: 0,
                    maxFramebufferHeight: 0,
                    maxFramebufferLayers: 0,
                    framebufferColorSampleCounts: 0,
                    framebufferDepthSampleCounts: 0,
                    framebufferStencilSampleCounts: 0,
                    framebufferNoAttachmentsSampleCounts: 0,
                    maxColorAttachments: 0,
                    sampledImageColorSampleCounts: 0,
                    sampledImageIntegerSampleCounts: 0,
                    sampledImageDepthSampleCounts: 0,
                    sampledImageStencilSampleCounts: 0,
                    storageImageSampleCounts: 0,
                    maxSampleMaskWords: 0,
                    timestampComputeAndGraphics: 0,
                    timestampPeriod: 0.0,
                    maxClipDistances: 0,
                    maxCullDistances: 0,
                    maxCombinedClipAndCullDistances: 0,
                    discreteQueuePriorities: 0,
                    pointSizeRange: [0.0; 2],
                    lineWidthRange: [0.0; 2],
                    pointSizeGranularity: 0.0,
                    lineWidthGranularity: 0.0,
                    strictLines: 0,
                    standardSampleLocations: 0,
                    optimalBufferCopyOffsetAlignment: 0,
                    optimalBufferCopyRowPitchAlignment: 0,
                    nonCoherentAtomSize: 0,
                },
                sparseProperties: VkPhysicalDeviceSparseProperties {
                    residencyStandard2DBlockShape: 0,
                    residencyStandard2DMultisampleBlockShape: 0,
                    residencyStandard3DBlockShape: 0,
                    residencyAlignedMipSize: 0,
                    residencyNonResidentStrict: 0,
                },
            };
            vkGetPhysicalDeviceProperties(self.raw_handle, &mut device_properties);

            PhysicalDeviceProperties {
                device_name: String::from_utf8(
                    device_properties
                        .deviceName
                        .iter()
                        .map(|x| x.clone() as u8)
                        .collect(),
                )
                .unwrap(),
            }
        }
    }
}

pub struct DebugUtilsMessengerEXT<'a> {
    raw_handle: VkDebugUtilsMessengerEXT,
    instance: &'a Instance,
}

impl<'a> Drop for DebugUtilsMessengerEXT<'a> {
    fn drop(&mut self) {
        unsafe {
            if let Some(func) = vkGetInstanceProcAddr(
                self.instance.raw_handle,
                b"vkDestroyDebugUtilsMessengerEXT\0".as_ptr() as *const i8,
            ) {
                let func = std::mem::transmute::<
                    unsafe extern "C" fn(),
                    unsafe extern "C" fn(
                        VkInstance,
                        VkDebugUtilsMessengerEXT,
                        *const VkAllocationCallbacks,
                    ),
                >(func);

                func(self.instance.raw_handle, self.raw_handle, std::ptr::null());
            }
        }
    }
}

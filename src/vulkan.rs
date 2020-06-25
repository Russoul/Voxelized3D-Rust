
use glad_vulkan::*;
use std::ptr::null;
use std::ptr::null_mut;
use std::mem::MaybeUninit;

pub unsafe fn create_instance() -> Option<vk::VkDevice>{

    let mut instance_info = MaybeUninit::<vk::VkInstanceCreateInfo>::uninit();
    let mut instance= MaybeUninit::<vk::VkInstance>::uninit();

    let res = vk::CreateInstance(instance_info.as_mut_ptr(), std::ptr::null(), instance.as_mut_ptr());
    if res != vk::VkResult::VK_SUCCESS{
        eprintln!("error initializing vulkan: {:?}", res);
        return None
    }

    let mut device_count = 0u32;

    vk::EnumeratePhysicalDevices(instance.assume_init(), &mut device_count, null_mut());

    let mut physical_devices = Vec::<vk::VkPhysicalDevice>::new();
    physical_devices.reserve(device_count as usize);
    vk::EnumeratePhysicalDevices(instance.assume_init(), &mut device_count, physical_devices.as_mut_ptr());

    if device_count == 0{
        None
    }else{


        for i in 0..device_count as usize {
            let mut prop_count = 0u32;
            vk::GetPhysicalDeviceQueueFamilyProperties(physical_devices[i], &mut prop_count, null_mut());
            let mut props = Vec::<vk::VkQueueFamilyProperties>::new();
            props.reserve(prop_count as usize);
            vk::GetPhysicalDeviceQueueFamilyProperties(physical_devices[i], &mut prop_count, props.as_mut_ptr());

            //TODO do something here
        }

        let prior = [0.0f32];
        let mut device_q_info = vk::VkDeviceQueueCreateInfo{
            sType:vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext:null(),
            queueFamilyIndex:0, // ?
            queueCount:1,
            pQueuePriorities:prior.as_ptr(),
            flags:0
        };

        let mut device_create_info = vk::VkDeviceCreateInfo{
            sType:vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext:null(),
            queueCreateInfoCount:1,
            pQueueCreateInfos:&device_q_info,
            enabledExtensionCount:0,
            ppEnabledExtensionNames:null(),
            enabledLayerCount:0,
            ppEnabledLayerNames:null(),
            pEnabledFeatures:null(),
            flags:0
        };

        let mut device = MaybeUninit::<vk::VkDevice>::uninit();
        let device_created = vk::CreateDevice(physical_devices[0], &device_create_info, null(), device.as_mut_ptr());

        if device_created != vk::VkResult::VK_SUCCESS{
            eprintln!("error initializing vulkan: {:?}", device_created);
            return None
        }

        Some(device.assume_init())
    }
}
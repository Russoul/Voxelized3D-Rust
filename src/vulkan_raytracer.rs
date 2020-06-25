use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::image::{StorageImage, ImageAccess};
use vulkano::image::Dimensions;
use vulkano::format::Format;
use vulkano::descriptor::descriptor_set::{PersistentDescriptorSet, DescriptorSetDesc};
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::buffer::BufferUsage;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::pipeline::ComputePipeline;
use vulkano::command_buffer::CommandBuffer;
use vulkano::sync::GpuFuture;


use image::{ImageBuffer, Rgba};

use std::sync::Arc;
use std::fs;
use std::io::Read;

use matrix::*;
use math::*;
use renderer::Cam;
use std::vec::Vec as Vector;
use time::precise_time_ns;

mod cs{
    vulkano_shaders::shader!{
        ty: "compute",
        path: "./assets/shaders/vulkan/raytrace.cs"
    }
}


#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Vec3f{
    x : f32,
    y : f32,
    z : f32,
    w : f32
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Triangle3f{
    p1 : Vec3f,
    p2 : Vec3f,
    p3 : Vec3f
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Rayf{
    start : Vec3f,
    dir : Vec3f
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Camf{
    pos : Vec3f,
    look : Vec3f,
    up : Vec3f
}

#[derive(Clone, Debug)]
#[repr(C)]
struct Data{
    tr: Box<Triangle3<f32>>
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Input{
    cam : Camf,
    rays : [Rayf;4],
    num_triangles : u32,
    num_terrain : u32,
    num_shapes : u32,
    num_ops : u32
}

impl From<Vec3<f32>> for Vec3f{
    fn from(v: Vec3<f32>) -> Self {
        Vec3f{x : v.x, y : v.y, z : v.z, w : 0.0}
    }
}

impl From<Cam> for Camf{
    fn from(cam: Cam) -> Self {
        Camf{pos : From::from(cam.pos), look : From::from(cam.look), up : From::from(cam.up)}
    }
}

impl From<Ray<f32>> for Rayf{
    fn from(ray: Ray<f32>) -> Self {
        Rayf{start : From::from(ray.start), dir : From::from(ray.dir)}
    }
}


impl Input{
    fn new(cam : Cam, rays : [Ray<f32>; 4], num_triangles : u32, num_terrain : u32, num_shapes : u32, num_ops : u32) -> Input{
        Input{cam:From::from(cam), rays: [From::from(rays[0]), From::from(rays[1]), From::from(rays[2]), From::from(rays[3])], num_triangles, num_shapes, num_terrain, num_ops}
    }
}

/*impl Input{
    pub fn empty() -> Input{
        let ray = Ray{start:Vec3::empty(), dir:Vec3::empty()};
        Input{cam:Cam{pos:Vec3::empty(), look:Vec::empty(), up:Vec3::empty()},
              rays:[ray, ray, ray, ray]}
    }
}*/



pub fn setup(width : u32, height : u32, cam : &Cam, triangles_rt : &Vector<Triangle3<f32>>) -> Vector<u8>{
    let aspect = width as f32 / height as f32;
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("[Vulkan] Failed to create Vulkan instance");

    let physical = PhysicalDevice::enumerate(&instance).next().expect("[Vulkan] No devices available");

    for family in physical.queue_families() {
        println!("[Vulkan] Found a queue family with {:?} queue(s)", family.queues_count());
    }

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("[Vulkan] Couldn't find a graphical queue family");


    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device") };
    let queue = queues.next().unwrap();

    let shader = cs::Shader::load(device.clone())
        .expect("[Vulkan] Failed to create shader module");

    let compute_pipeline = Arc::new(ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
        .expect("[Vulkan] Failed to create compute pipeline"));

    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width, height },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    //let test_tri = vec![Triangle3f{p1:Vec3f{x:-0.2, y:0.0, z:-1.0, w : 0.0}, p2:Vec3f{x:0.2, y:0.0, z:-1.0, w : 0.0}, p3:Vec3f{x:0.0, y:0.3, z:-1.0, w : 0.0}}];
    //let test_tri1 = vec![Triangle3::<f32>{p1 : vec3![-0.2, 0.0, -1.0], p2 : vec3![0.2, 0.0, -1.0], p3 : vec3![0.0, 0.3, -1.0]}];
    let rays = bounding_rays(cam, 90.0, aspect, 0.1);
    let triangles = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                                   triangles_rt.clone().into_iter()).unwrap(); //TODO copying (change buffer ty ?)


    let input = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
                                                  Input::new(cam.clone(), rays, triangles_rt.len() as u32, 8, 2, 2)).unwrap();

    let terrain = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
        [0.0f32, 0.0, -2.0, 1.0,   0.8, 0.0, -2.0, 0.5]).unwrap();

    let shapes = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
                                                 [0u32, 0]).unwrap();

    let ops = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
                                                [0, 2u32]).unwrap();

    let set = Arc::new(PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
        .add_image(image.clone()).unwrap()
        //.add_buffer(triangles.clone()).unwrap()
        .add_buffer(input.clone()).unwrap()
        .add_buffer(triangles.clone()).unwrap()
        .add_buffer(terrain.clone()).unwrap()
        .add_buffer(shapes.clone()).unwrap()
        .add_buffer(ops.clone()).unwrap()
        .build().unwrap()
    );
    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                             (0 .. width * height * 4).map(|_| 0u8))
        .expect("[Vulkan] Failed to create buffer");


    let t1 = time::precise_time_ns();
    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .dispatch([width / 8 + 1, height / 8 + 1 /*TODO better dims*/, 1], compute_pipeline.clone(), set.clone(), ()).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();
    let t2 = time::precise_time_ns();

    println!("frame render time {}, triangles : {}", (t2 - t1)/1000000, triangles_rt.len());

    let buffer_content = buf.read().unwrap();
    //let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, &buffer_content[..]).unwrap();

    //image.save("image.png").unwrap();

    Vector::from(&buffer_content[..]) //TODO copying here

}

pub fn setup_raw(){
    
}
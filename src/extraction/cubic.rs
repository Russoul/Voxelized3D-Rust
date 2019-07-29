
use libc::*;
use std::ffi::CString;
use std::ffi::CStr;
use na::{Vector3,Real};
use ptr;
use renderer::*;
use math::*;
use noise::{NoiseFn, Perlin};

#[derive(Debug)]
#[repr(C)]
pub struct Node{
    parent : *mut Node,
    children : [*mut Node;8],
    density : f32,
}


extern{
    pub fn alloc_grid(size : usize, grid : *mut *mut *mut Node);
    pub fn gen_dense_grid(size : usize, center : Vector3<f32>, extent : f32, dense_grid : *mut *mut Node);
    pub fn gen_dense_grid_custom(size : usize, center : Vector3<f32>, extent : f32, dense_grid : *mut *mut Node, f : extern fn(&(&Perlin,Cube<f32>), Vector3<f32>) -> f32, perlin : &(&Perlin,Cube<f32>));
    pub fn simplify_grid_recursively(size : usize, center : Vector3<f32>, extent : f32, dense_grid : *mut *mut Node) -> *mut *mut Node;
    pub fn init_noise();
    pub fn print_octree(node : *mut Node, lev : usize);
}


pub fn centers() -> [Vector3<f32>;8]{
    [Vector3::new(-0.5, -0.5, -0.5),
     Vector3::new(0.5, -0.5, -0.5),
     Vector3::new(0.5, -0.5, 0.5),
     Vector3::new(-0.5, -0.5, 0.5),

     Vector3::new(-0.5, 0.5, -0.5),
     Vector3::new(0.5, 0.5, -0.5),
     Vector3::new(0.5, 0.5, 0.5),
     Vector3::new(-0.5, 0.5, 0.5)]
}

pub unsafe fn for_each_leaf<A>(root : *mut Node, f : &mut Box<A>, center : Vector3<f32>, extent : f32, level : usize) where A : FnMut(*mut Node, Vector3<f32>, f32, usize){
    if(root == ptr::null_mut()) {return;}

    let mut leaf = true;

    for i in 0..8{
        if((*root).children[i] != ptr::null_mut()){
            leaf = false;
            break;
        }
    }

    if(leaf){
        f(root, center, extent, level);
    }else{
        for i in 0..8{
            let new_center = centers()[i] * extent + center;
            for_each_leaf((*root).children[i], f, new_center, extent/2.0, level + 1)
        }
    }
}


//0 to 1.0
fn octave_perlin3(perlin : &Perlin, x : f32, y : f32, z : f32, octaves : usize, persistence : f32) -> f32{
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude : f32 = 1.0;
    let mut max_value = 0.0;

    let k = 2.0.powi((octaves - 1) as i32);

    for i in 0..octaves{
        total += (perlin.get([(x * frequency / k) as f64, (y * frequency / k) as f64, (z * frequency / k) as f64]) + 1.0)/2.0 * amplitude as f64;
        max_value += amplitude;
        amplitude *= persistence as f32;
        frequency *= 2.0;
    }

    total as f32 / max_value
}


pub unsafe fn test_cubic_octree(render : &mut RendererVertFragDef){
    init_noise();
    let mut dense_grid : *mut *mut Node = ptr::null_mut();
    let size = 128;
    let center = Vector3::new(0.0, 0.0, 0.0);
    let extent = 8.0;

    let perlin = Perlin::new();


    extern fn f(d : &(&Perlin,Cube<f32>), x : Vector3<f32>) -> f32{
        let perlin = d.0;
        let cube = d.1;
        //octave_perlin3(&perlin, v.x, v.y, v.z, 8, 0.7)
        if point3_inside_square3_inclusive(&x, &cube){
            let den = -octave_perlin3(perlin, x.x - (cube.center.x - cube.extent), x.y - (cube.center.y - cube.extent), x.z - (cube.center.z - cube.extent), 4, 0.56) * 2.0 * cube.extent;
            let dy = (x.y - (cube.center.y - cube.extent) ); //cube.extent / 2.0 ; // 0 - 1
            //println!("{} {} {}", den, dy, x.y);
            den + dy
        }else{
            0.01
        }
    }

    alloc_grid(size, &mut dense_grid);
    gen_dense_grid_custom(size, center, extent, dense_grid, f, &(&perlin,Cube{center,extent}));
    let simplified = simplify_grid_recursively(size, center, extent, dense_grid);
    //print_octree(*simplified, 0);


    for_each_leaf(*simplified, &mut Box::new( |node : *mut Node, center, ext, lev|{
            if (*node).density.abs() <= 0.5{
                //add_square3_bounds_color(render, Cube{center, extent : ext}, Vector3::new(1.0,1.0,1.0));
                add_cube_color_normal(render, Cube{center, extent : ext}, Vector3::new(0.6, 0.4, 0.4));
            }

        }), center, extent, 0);
}


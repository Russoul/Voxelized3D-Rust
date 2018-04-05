use std;
use na::*;
use math::*;
use renderer::*;
use alga::general::*;
use std::rc::Rc;
use std::cell::RefCell;
use num::PrimInt;
use std::collections::HashMap;
use qef_bindings::*;



// static corner_points : Vec<Vector3<f32>> = {
//     vec![
//         Vector3::new(0.0,0.0,0.0),
//         Vector3::new(1.0,0.0,0.0),
//         Vector3::new(1.0,0.0,1.0),  //clockwise starting from zero y min
//         Vector3::new(0.0,0.0,1.0),

//         Vector3::new(0.0,1.0,0.0),
//         Vector3::new(1.0,1.0,0.0), //y max
//         Vector3::new(1.0,1.0,1.0),
//         Vector3::new(0.0,1.0,1.0)

//     ]
// };


// static edge_pairs :  Vec<Vector2<usize>> = {
//     vec![
//         Vector2::new(0,1),
//         Vector2::new(1,2),
//         Vector2::new(3,2),
//         Vector2::new(0,3),

//         Vector2::new(4,5), 
//         Vector2::new(5,6), //5
//         Vector2::new(7,6), //6
//         Vector2::new(4,7), 

//         Vector2::new(4,0),
//         Vector2::new(1,5), 
//         Vector2::new(2,6), //10
//         Vector2::new(3,7)  
//     ]
// };

// #[derive(Clone, Debug)]
// pub struct HermiteData<T : Real>{
//     pub a : T,//length of one edge of a cubic cell
//     pub cell_count : usize, //number of cells along each axis, must be a power of 2
//     pub pos : Vector3<T>, //min corner of the AABB
//     pub grid : Vec<T>,
//     pub hermite : Vec<Vec<Plane<T>>>, //for now(not efficient) this Vec contains hermite data for each cell(duplications)
//     //after filling the grid each cell must be initialized (set to Some)
// }

// pub struct Octree<T : Real>{
//     pub level : usize,//starting from 0 for roots
//     pub children : Option<Vec<*mut Octree<T>>>,
//     pub pos : Vec<u8>,

//     //oct indexing of this node relative to the root
//     //to be replaced by bitfield:
//     //root node has pos equal to 0
//     //next node can have position: (0-7) + 1
//     //next: (0-7 0-7) + 1
//     //...
//     //(<>) is a number in base 8 each digit is separated by space
// }

// impl<T : Real> Octree<T>{
    
// }

// //in local coordinates [0,0,0] - [1,1,1]
// unsafe fn get_bounding_cube(node : &(*mut Octree<f32>)) -> Square3<f32>{
//     let mut res = Square3{center : Vector3::new(0.5, 0.5, 0.5), extent : 0.5};

//     for i in &(**node).pos{
//         let ext = res.extent / 2.0;
//         match i.clone(){
//             0 => {
//                 res = Square3{center : res.center * 0.5, extent : ext};
//             },
//             1 => {
//                 res = Square3{center : Vector3::new(res.center.x * 1.5, res.center.y * 0.5, res.center.z * 0.5), extent : ext};
//             },
//             2 => {
//                 res = Square3{center : Vector3::new(res.center.x * 1.5, res.center.y * 0.5, res.center.z * 1.5), extent : ext};
//             },
//             3 => {
//                 res = Square3{center : Vector3::new(res.center.x * 0.5, res.center.y * 0.5, res.center.z * 1.5), extent : ext};
//             },



//             4 => {
//                 res = Square3{center : Vector3::new(res.center.x * 0.5, res.center.y * 1.5, res.center.z * 0.5), extent : ext};
//             },
//             5 => {
//                 res = Square3{center : Vector3::new(res.center.x * 1.5, res.center.y * 1.5, res.center.z * 0.5), extent : ext};
//             },
//             6 => {
//                 res = Square3{center : Vector3::new(res.center.x * 1.5, res.center.y * 1.5, res.center.z * 1.5), extent : ext};
//             },
//             7 => {
//                 res = Square3{center : Vector3::new(res.center.x * 0.5, res.center.y * 1.5, res.center.z * 1.5), extent : ext};
//             },
//             _ => (),
//         }
//     }

//     res
// }

// fn scala_local_cube_bounds(bounds : &Square3<f32>, data : &HermiteData<f32>) -> Square3<f32>{
//     let e = data.a * (data.cell_count/2) as f32;
//     Square3{center : bounds.center * e, extent : bounds.extent * e}
// }


// struct SampleRange{ //corners are included
//     pub xmin : usize,
//     pub ymin : usize,
//     pub zmin : usize,
//     pub xmax : usize,
//     pub ymax : usize,
//     pub zmax : usize,
// }

// fn sample_range_for_scaled_local_bounds(bounds : &Square3<f32>, data : &HermiteData<f32>) -> SampleRange{
//     let xmin = bounds.center.x - bounds.extent;
//     let ymin = bounds.center.y - bounds.extent;
//     let zmin = bounds.center.z - bounds.extent;

//     let xmax = bounds.center.x + bounds.extent;
//     let ymax = bounds.center.y + bounds.extent;
//     let zmax = bounds.center.z + bounds.extent;

//     SampleRange{
//         xmin : (xmin / data.a) as usize,
//         ymin : (ymin / data.a) as usize,
//         zmin : (zmin / data.a) as usize,
//         xmax : (xmax / data.a) as usize,
//         ymax : (ymax / data.a) as usize,
//         zmax : (zmax / data.a) as usize,
//     }


// }

// // unsafe fn get_corner_samples(node : &mut Octree<f32>, data : &HermiteData<f32>) -> [f32;8]{

// // }


// //cur starts from 0
// //pos starts from empty Vec
// pub unsafe fn initialize_base_grid(res : *mut *mut Octree<f32>, n0 : usize, cur : usize, pos : Vec<u8>){


//     if n0 != cur{
//         let mut children = Vec::with_capacity(8);
//         for i in 0..8{
//             let mut new_pos = pos.clone();
//             new_pos.push(i as u8);
//             initialize_base_grid(&mut children[i], n0, cur + 1, new_pos);
//         }
//         *res = &mut Octree{level : cur, children : Some(children), pos}
//     }else{
//         *res = &mut Octree{level : cur, children : None, pos}
//     }
// }

// pub unsafe fn subdivide_cell(hermite_data : &HermiteData<f32>, cell : *mut *mut Octree<f32>){

// }

// //hermite data should be generated alongside isosurface extraction (on GPU) to increase performance as hermite data is almost always generated real-time from noise function
// //input: hermite_data : pregenerated hermite data, n0 : size (spanning one dimension) of the coarse uniform grid
// pub fn cubical_marching_squares(hermite_data : &HermiteData<f32>, n0 : usize){
//     unsafe{
//         let mut base_grid = std::ptr::null_mut();
//         initialize_base_grid(&mut base_grid, n0, 0, Vec::with_capacity(0));
//     }
// }
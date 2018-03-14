use std;
use na::*;
use math::*;
use renderer::*;
use alga::general::*;
use num::PrimInt;


#[derive(Clone, Debug)]
pub struct VoxelData<T : Copy + PartialEq + std::fmt::Debug> where T : 'static{
    pub densities : [T;8], //do we need it ?
    pub config : usize,
    pub planes : Vec<Plane<T>>,
}

impl<T : Real + Identity<Additive>> VoxelData<T>{
    pub fn is_heterogeneous(&self) -> bool{
        let sign = self.densities[0];
        for i in 1..8{
            if self.densities[i] * sign <= T::zero() {return true}
        }
        false
    }
}

pub struct Octree<T>{
    pub parent : *mut Octree<T>,
    pub children : *mut (*mut Octree<T>),
    pub data : VoxelData<T>,
}

pub fn corner_points() -> Vec< Vector3<f32> >{
    vec![
        Vector3::new(0.0,0.0,0.0),
        Vector3::new(1.0,0.0,0.0),
        Vector3::new(1.0,0.0,1.0),  //clockwise starting from zero y min
        Vector3::new(0.0,0.0,1.0),

        Vector3::new(0.0,1.0,0.0),
        Vector3::new(1.0,1.0,0.0), //y max
        Vector3::new(1.0,1.0,1.0),
        Vector3::new(0.0,1.0,1.0)

    ]
}

pub fn sample_voxeldata(bounds : Square3<f32>, f : DenFn3<f32>, corner_points : Vec<Vector3<f32>>) -> VoxelData<Tf32>{
    
    let mut corner_samples = [f32;8];
    let mut config = 0;
    
    for i in 0..8{
        corner_samples[i] = f(bounds.center + bounds.extent * corner_points[i]);
        if densities[i] < 0.0{
            config |= 1 << i;
        }
    }


}

fn is_const_sign(a : f32, b : f32) -> bool {
    if a > 0.0 { b > 0.0} else {b <= 0.0}
}

pub fn construct_octree(f : DenFn3<f32>, offset : Vector3<f32>, a : f32, size : usize) -> *mut Octree<f32>{
    for i in 0..(size/2){
        for j in 0..(size/2){
            for k in 0..(size/2){

            }
        }
    }
}
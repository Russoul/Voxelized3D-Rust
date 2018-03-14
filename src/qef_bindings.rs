use libc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;
use std::str;
use std;
use na;
use na::*;

#[derive(Debug ,Clone, Copy)]
#[repr(C)]
pub struct vec3{
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

#[derive(Debug ,Clone, Copy)]
#[repr(C)]
struct vec4{
    x : f32,
    y : f32,
    z : f32,
    w : f32
}

#[derive(Debug ,Clone, Copy)]
#[repr(C)]
struct mat3_sym{
    a11 : f32,
    a12 : f32,
    a13 : f32,
    a22 : f32,
    a23 : f32,
    a33 : f32,
}

extern{
    fn qef_solve(ATA : mat3_sym, Atb : vec3, pointAccum : vec4, out : *mut vec3) -> f32;
}

pub fn qef_solve_r(ATA : Matrix3<f32>, ATb : Vector3<f32>, point_accum : Vector4<f32>, out : &mut Vector3<f32>) -> f32{
    unsafe{
        let ATb_n = vec3{x : ATb.x, y : ATb.y, z : ATb.z};
        
        let ATA_n = mat3_sym{a11 : ATA[(0,0)], a12 : ATA[(0,1)], a13 : ATA[(0,2)], a22 : ATA[(1,1)], a23 : ATA[(1,2)], a33 : ATA[(2,2)]};
        let mut pa = vec4{x : point_accum.x, y : point_accum.y, z : point_accum.z, w : point_accum.w};
        let mut out_n = vec3{x : out.x, y : out.y, z : out.z};

        let res = qef_solve(ATA_n, ATb_n, pa, &mut out_n);

        //println!("out_n {:?}", &out_n);

        *out = Vector3::new(out_n.x, out_n.y, out_n.z);

        //println!("out {}", &(*out));
        
        res

    }
}

pub fn qef_add_r(n : Vector3<f32>, p : Vector3<f32>, ATA : &mut Matrix3<f32>, ATb : &mut Vector3<f32>, point_accum : &mut Vector4<f32>){
    unsafe{
        

        ATA[(0,0)] += n.x * n.x;
        ATA[(0,1)] += n.x * n.y;
        ATA[(0,2)] += n.x * n.z;

        ATA[(1,1)] += n.y * n.y;
        ATA[(1,2)] += n.y * n.z;
        ATA[(2,2)] += n.z * n.z;

        let b = dot(&p, &n);
        *ATb = ATb.clone() + n * b;
        
        *point_accum = (*point_accum).clone() + Vector4::new(p.x, p.y, p.z, 1.0);


    }


}
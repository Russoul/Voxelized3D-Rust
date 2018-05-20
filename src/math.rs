use na::*;
use na::storage::Storage;
use na::storage::ContiguousStorage;
use generic_array::ArrayLength;
use typenum::{Prod};
use alga::linear::FiniteDimInnerSpace;
use alga::general::SupersetOf;
use std::fmt::Debug;
use std;
use typenum;
use generic_array;
use rand::Rng;
use noise::{NoiseFn, Perlin};

#[derive(Clone, Copy, Debug)]
pub struct Triangle2<T : Scalar + Copy>{
    pub p1: Vector2<T>,
    pub p2: Vector2<T>,
    pub p3: Vector2<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle3<T : Scalar + Copy>{
    pub p1: Vector3<T>,
    pub p2: Vector3<T>,
    pub p3: Vector3<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Line2<T : Scalar + Copy> {
    pub start : Vector2<T>,
    pub end : Vector2<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Line3<T : Scalar + Copy> {
    pub start : Vector3<T>,
    pub end : Vector3<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Plane<T : Scalar + Copy> {
    pub point : Vector3<T>,
    pub normal : Vector3<T>,
}

//axis aligned
#[derive(Clone, Copy, Debug)]
pub struct Square2<T : Scalar>{
    pub center : Vector2<T>,
    pub extent : T,
}

//axis aligned
#[derive(Clone, Copy, Debug)]
pub struct Square3<T : Real>{
    pub center : Vector3<T>,
    pub extent : T,
}

impl<T : Real> Square3<T>{
    pub fn min(&self) -> Vector3<T>{
        self.center - Vector3::new(self.extent,self.extent,self.extent)
    }

    pub fn max(&self) -> Vector3<T>{
        self.center + Vector3::new(self.extent,self.extent,self.extent)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere<T : Scalar>{
    pub center : Vector3<T>,
    pub rad : T,
}






pub type DenFn2<'a, T> = Box<Fn(Vector2<T>) -> T + 'a>;
pub type DenFn3<T> = Box<Fn(Vector3<T>) -> T>;


pub fn intersection2<'a, 'b, T : Real>(a : &'a DenFn2<'a, T>, b : &'a DenFn2<'a, T>) -> DenFn2<'b, T> where 'a : 'b{
    Box::new(move |x|{Real::max(a(x), b(x))})
}

pub fn intersection2_move<'a, 'b, T : Real>(a : DenFn2<'a, T>, b : DenFn2<'a, T>) -> DenFn2<'b, T> where 'a : 'b{
    Box::new(move |x|{Real::max((*a)(x), (*b)(x))})
}

pub fn union2<'a,'b, T : Real>(a : &'a DenFn2<T>, b : &'a DenFn2<T>) -> DenFn2<'b, T> where 'a : 'b{
    Box::new(move |x| {Real::min(a(x), b(x))})
}


pub fn difference2<'a,'b, T : Real>(a : &'a DenFn2<T>, b : &'a DenFn2<T>) -> DenFn2<'b, T> where 'a : 'b{
    Box::new(move |x| {Real::max(a(x), -b(x))})
}

pub fn intersection3<T : Real>(a : DenFn3<T>, b : DenFn3<T>) -> DenFn3<T>{
    Box::new(move |x|{Real::max(a(x), b(x))})
}

pub fn union3<T : Real>(a : DenFn3<T>, b : DenFn3<T>) -> DenFn3<T>{
    Box::new(move |x| {Real::min(a(x), b(x))})
}

pub fn difference3<T : Real>(a : DenFn3<T>, b : DenFn3<T>) -> DenFn3<T>{
    Box::new(move |x| {Real::max(a(x), -b(x))})
}

//0 to 1.0
pub fn octave_perlin2(perlin : &Perlin, x : f32, z : f32, octaves : usize, persistence : f32) -> f32{
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;

    let k = 2.0.powi((octaves - 1) as i32);

    for i in 0..octaves{
        total += (perlin.get([(x * frequency / k) as f64, (z * frequency / k) as f64]) + 1.0)/2.0 * amplitude as f64;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }

    total as f32 / max_value
}

pub fn noise_f32(perlin : Perlin, cube : Square3<f32>) -> DenFn3<f32>{
    Box::new( move |x| {
        if point3_inside_square3_inclusive(&x, &cube){
            let den = -octave_perlin2(&perlin, x.x - (cube.center.x - cube.extent), x.z - (cube.center.z - cube.extent), 4, 0.56) * 2.0 * cube.extent;
            let dy = (x.y - (cube.center.y - cube.extent) ); //cube.extent / 2.0 ; // 0 - 1
            //println!("{} {} {}", den, dy, x.y);
            den + dy
        }else{
            0.01
        }
        
    })
}


pub fn mk_circle2<'a, T : Real + Copy>(center : Vector2<T>, rad : T) -> DenFn2<'a, T>{
    Box::new(move |x|{
        let dist = x - center;
        dist.dot(&dist) - rad * rad
    })
}

pub fn mk_half_plane2_left<'a, T : Real + Copy>(x : T) -> DenFn2<'a, T>{
    Box::new(move |p|{p.x - x})
}

pub fn mk_half_plane2_right<'a, T : Real + Copy>(x : T) -> DenFn2<'a, T>{
    Box::new(move |p|{x - p.x})
}


pub fn mk_half_plane2_lower<'a, T : Real + Copy>(y : T) -> DenFn2<'a, T>{
    Box::new(move |p|{p.y - y})
}

pub fn mk_half_plane2_upper<'a, T : Real + Copy>(y : T) -> DenFn2<'a, T>{
    Box::new(move |p|{y - p.y})
}



pub fn union3_mat<T : Real>(a : DenFn3<T>, b : DenFn3<T>) -> DenFn3<T>{
    Box::new(move |x| {Real::min(a(x), b(x))})
}

pub fn difference3_mat<T : Real>(a : DenFn3<T>, b : DenFn3<T>) -> DenFn3<T>{
    Box::new(move |x| {Real::max(a(x), -b(x))})
}

pub fn mk_half_space_x_neg<T : Real + Copy>(x : T) -> DenFn3<T>{
    Box::new(move |p|{p.x - x})
}

pub fn mk_half_space_x_pos<T : Real + Copy>(x : T) -> DenFn3<T>{
    Box::new(move |p|{x - p.x})
}

pub fn mk_half_space_y_neg<T : Real + Copy>(y : T) -> DenFn3<T>{
    Box::new(move |p|{p.y - y})
}

pub fn mk_half_space_y_pos<T : Real + Copy>(y : T) -> DenFn3<T>{
    Box::new(move |p|{y - p.y})
}

pub fn mk_half_space_z_neg<T : Real + Copy>(z : T) -> DenFn3<T>{
    Box::new(move |p|{p.z - z})
}

pub fn mk_half_space_z_pos<T : Real + Copy>(z : T) -> DenFn3<T>{
    Box::new(move |p|{z - p.z})
}

pub fn mk_rectangle2<'a, T : Real + Copy>(center : Vector2<T>, extent : Vector2<T>) -> DenFn2<'a, T> {
    let right = mk_half_plane2_right(center.x - extent.x);
    let left = mk_half_plane2_left(center.x + extent.x);

    let lower = mk_half_plane2_lower(center.y + extent.y);
    let upper = mk_half_plane2_upper(center.y - extent.y);

    let i1 = intersection2_move(left, right);
    let i2 = intersection2_move(upper, lower);

    intersection2_move(i1, i2)
}



pub fn mk_aabb<T : Real + Copy>(center : Vector3<T>, extent : Vector3<T>) -> DenFn3<T> {
    let x_neg = mk_half_space_x_neg(center.x + extent.x);
    let x_pos = mk_half_space_x_pos(center.x - extent.x);

    let y_neg = mk_half_space_y_neg(center.y + extent.y);
    let y_pos = mk_half_space_y_pos(center.y - extent.y);

    let z_neg = mk_half_space_z_neg(center.z + extent.z);
    let z_pos = mk_half_space_z_pos(center.z - extent.z);

    let ix = intersection3(x_neg, x_pos);
    let iy = intersection3(y_neg, y_pos);
    let iz = intersection3(z_neg, z_pos);

    let ixy = intersection3(ix, iy);

    intersection3(ixy, iz)
}

pub fn mk_half_space_pos<T : Real + Copy>(plane : Plane<T>) -> DenFn3<T>{
     Box::new(move |p|{
        let d = p - plane.point;
        let dist = d.dot(&plane.normal);
        -dist 
     })
}

pub fn mk_half_space_neg<T : Real + Copy>(plane : Plane<T>) -> DenFn3<T>{
     Box::new(move |p|{
        let d = p - plane.point;
        let dist = d.dot(&plane.normal);
        dist 
     })
}

pub fn mk_obb<T : Real + Copy>(center : Vector3<T>, right : Vector3<T>, up : Vector3<T>, extent : Vector3<T>) -> DenFn3<T> {
    let r_neg = mk_half_space_neg(Plane{point : center + right * extent.x, normal : right});
    let r_pos = mk_half_space_pos(Plane{point : center - right * extent.x, normal : right});

    let u_neg = mk_half_space_neg(Plane{point : center + up * extent.y, normal : up});
    let u_pos = mk_half_space_pos(Plane{point : center - up * extent.y, normal : up});

    let look = right.cross(&up);

    let l_neg = mk_half_space_neg(Plane{point : center + look * extent.z, normal : look});
    let l_pos = mk_half_space_pos(Plane{point : center - look * extent.z, normal : look});

    let ix = intersection3(r_neg, r_pos);
    let iy = intersection3(u_neg, u_pos);
    let iz = intersection3(l_neg, l_pos);

    let ixy = intersection3(ix, iy);

    intersection3(ixy, iz)
}

pub fn mk_sphere<T : Real + Copy>(sphere : Sphere<T>) -> DenFn3<T>{
    Box::new(move |x|{
        let dist = x - sphere.center;
        dist.dot(&dist) - sphere.rad * sphere.rad
    })
}

pub fn mk_torus_z<T : Real>(R : T, r : T, offset : Vector3<T>) -> DenFn3<T>{
    Box::new(move |p|{
        let x = p - offset;
        let a = (x.x * x.x + x.y * x.y).sqrt() - R;
        a * a + x.z * x.z - r * r
    })
}

pub fn mk_torus_y<T : Real>(R : T, r : T, offset : Vector3<T>) -> DenFn3<T>{
    Box::new(move |p|{
        let x = p - offset;
        let a = (x.x * x.x + x.z * x.z).sqrt() - R;
        a * a + x.y * x.y - r * r
    })
}


pub fn mk_sphere_displacement<'f, T : Real + Copy>(sphere : Sphere<T>, f : Box<Fn(Vector3<T>) -> T>) -> DenFn3<T>{
    Box::new(move |x|{
        let dist = x - sphere.center;
        dist.dot(&dist) - sphere.rad * sphere.rad * f(dist.normalize())
    })
}

pub fn distance_point2_line2<T : Real>(point2 : &Vector2<T>, line2 : &Line2<T>) -> T{
    let d = line2.start - line2.end;
    let norm = d.normalize();
    let n = Vector2::new(-norm.y, norm.x);
    let vec = point2 - line2.start;
    Real::abs(n.dot(&vec))
}

pub fn distance_point3_plane<T : Real>(point3 : &Vector3<T>, plane : &Plane<T>) -> T{
    let vec = point3 - plane.point;
    Real::abs(plane.normal.dot(&vec))
}

pub fn point3_inside_square3_inclusive<T : Real>(point3 : &Vector3<T>, square3 : &Square3<T>) -> bool{
    point3.x <= square3.center.x + square3.extent &&
    point3.x >= square3.center.x - square3.extent &&

    point3.y <= square3.center.y + square3.extent &&
    point3.y >= square3.center.y - square3.extent &&

    point3.z <= square3.center.z + square3.extent &&
    point3.z >= square3.center.z - square3.extent
}

pub fn point3_inside_sphere_inclusive<T : Real>(point3 : &Vector3<T>, sphere : Sphere<T>) -> bool {
    let d = point3 - sphere.center;
    d.dot(&d) <= sphere.rad * sphere.rad
}

pub fn vec3f_vec3d(a : Vector3<f64>) -> Vector3<f32>{
    Vector3::new(a.x as f32, a.y as f32, a.z as f32)
}

//column-major
pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32;16]{
    [2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
     0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom),
     0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
     0.0, 0.0, 0.0, 1.0
    ]
}

//column-major
pub fn perspective(fovy : f32, aspect : f32, near : f32, far : f32) -> Matrix4<f32>{
    let top = near * (std::f32::consts::PI / 180.0 * fovy / 2.0).tan();
    let bottom = -top;
    let right = top * aspect;
    let left = -right;
    Matrix4::new(2.0 * near / (right - left), 0.0, (right + left) / (right - left), 0.0,
                 0.0, 2.0 * near / (top - bottom), (top + bottom) / (top - bottom), 0.0,
                 0.0, 0.0, -(far + near) / (far - near), -2.0 * far * near / (far - near),
                 0.0, 0.0, -1.0, 0.0)
}

//column-major
pub fn view_dir(pos : Vector3<f32>, look : Vector3<f32>, up : Vector3<f32>) -> Matrix4<f32>{
    let za = -look;
    let xa = up.cross(&za);
    let ya = za.cross(&xa);

    Matrix4::new(xa.x, ya.x, za.x, 0.0,
                 xa.y, ya.y, za.y, 0.0,
                 xa.z, ya.z, za.z, 0.0,
                 -dot(&xa,&pos), -dot(&ya,&pos), -dot(&za,&pos), 1.0).transpose()
}
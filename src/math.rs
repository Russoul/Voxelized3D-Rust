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
pub struct Square3<T : Scalar>{
    pub center : Vector3<T>,
    pub extent : T,
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere<T : Scalar>{
    pub center : Vector3<T>,
    pub rad : T,
}


pub struct VoxelGrid2<T : Real + Copy>{
    pub a : T,
    pub size_x : usize,
    pub size_y : usize,
    pub grid : Vec<T>,
}

impl<T : Real + SupersetOf<f32>> VoxelGrid2<T>{

    pub fn vertices_x(&self) -> usize {self.size_x + 1}
    pub fn vertices_y(&self) -> usize {self.size_y + 1}

    pub fn new(a : T, size_x : usize, size_y : usize) -> VoxelGrid2<T>{
        let grid = vec![convert(0.0);(size_x + 1) * (size_y + 1)];

        VoxelGrid2{a,size_x, size_y, grid}
    }

    pub fn get(&self, x : usize, y : usize) -> T{
        self.grid[y * self.vertices_x() + x]
    }

    pub fn set(&mut self, x : usize, y : usize, value : T){
        let vx = self.vertices_x();
        self.grid[y * vx + x] = value;
    }

    pub fn get_point(&self, x : usize, y : usize) -> Vector2<T>{
        Vector2::new(self.a * convert::<f32, T>(x as f32), self.a * convert::<f32, T>(y as f32))
    }

    pub fn square2(&self, x : usize, y : usize) -> Square2<T>{
        Square2{center : Vector2::new(convert::<f32,T>(x as f32 + 0.5) * self.a, convert::<f32,T>(y as f32 + 0.5) * self.a), extent: self.a / convert(2.0)}
    }
}

pub struct VoxelGrid3<T : Real + Copy>{
    pub a : T,
    pub size_x : usize,
    pub size_y : usize,
    pub size_z : usize,
    pub grid : Vec<T>,
}

impl<T : Real + SupersetOf<f32>> VoxelGrid3<T>{

    pub fn vertices_x(&self) -> usize {self.size_x + 1}
    pub fn vertices_y(&self) -> usize {self.size_y + 1}
    pub fn vertices_z(&self) -> usize {self.size_z + 1}

    pub fn new(a : T, size_x : usize, size_y : usize, size_z : usize) -> VoxelGrid3<T>{
        let grid = vec![convert(0.0);(size_x + 1) * (size_y + 1) * (size_z + 1)];

        VoxelGrid3{a,size_x, size_y, size_z, grid}
    }

    pub fn get(&self, x : usize, y : usize, z : usize) -> T{
        self.grid[z * self.vertices_y() * self.vertices_x() + y * self.vertices_x() + x]
    }

    pub fn set(&mut self, x : usize, y : usize, z : usize, value : T){
        let vx = self.vertices_x();
        let vy = self.vertices_y();
        self.grid[z * vy * vx + y * vx + x] = value;
    }


    pub fn get_point(&self, x : usize, y : usize, z : usize) -> Vector3<T>{
        Vector3::new(self.a * convert::<f32, T>(x as f32), self.a * convert::<f32, T>(y as f32), self.a * convert::<f32, T>(z as f32))
    }

    //bounding box of the cube
    pub fn square3(&self, x : usize, y : usize, z : usize) -> Square3<T>{
        Square3{center : Vector3::new(convert::<f32,T>(x as f32 + 0.5) * self.a, convert::<f32,T>(y as f32 + 0.5) * self.a, convert::<f32,T>(z as f32 + 0.5) * self.a), extent: self.a / convert(2.0)}
    }
}

pub type DenFn2<T> = Box<Fn(Vector2<T>) -> T>;
pub type DenFn3<T> = Box<Fn(Vector3<T>) -> T>;

pub fn intersection<T : Real>(a : DenFn2<T>, b : DenFn2<T>) -> DenFn2<T>{
    Box::new(move |x|{Real::max(a(x), b(x))})
}

pub fn union<T : Real>(a : DenFn2<T>, b : DenFn2<T>) -> DenFn2<T>{
    Box::new(move |x| {Real::min(a(x), b(x))})
}

pub fn difference<T : Real>(a : DenFn2<T>, b : DenFn2<T>) -> DenFn2<T>{
    Box::new(move |x| {Real::max(a(x), -b(x))})
}

pub fn mk_circle2<T : Real + Copy>(center : Vector2<T>, rad : T) -> DenFn2<T>{
    Box::new(move |x|{
        let dist = x - center;
        dist.dot(&dist) - rad * rad
    })
}

pub fn mk_half_plane_left<T : Real + Copy>(x : T) -> DenFn2<T>{
    Box::new(move |p|{p.x - x})
}

pub fn mk_half_plane_right<T : Real + Copy>(x : T) -> DenFn2<T>{
    Box::new(move |p|{x - p.x})
}


pub fn mk_half_plane_lower<T : Real + Copy>(y : T) -> DenFn2<T>{
    Box::new(move |p|{p.y - y})
}

pub fn mk_half_plane_upper<T : Real + Copy>(y : T) -> DenFn2<T>{
    Box::new(move |p|{y - p.y})
}

pub fn mk_rectangle2<T : Real + Copy>(center : Vector2<T>, extent : Vector2<T>) -> DenFn2<T> {
    let right = mk_half_plane_right(center.x - extent.x);
    let left = mk_half_plane_left(center.x + extent.x);

    let lower = mk_half_plane_lower(center.y + extent.y);
    let upper = mk_half_plane_upper(center.y - extent.y);

    let i1 = intersection(left, right);
    let i2 = intersection(upper, lower);

    intersection(i1, i2)
}

pub fn mk_sphere<T : Real + Copy>(sphere : Sphere<T>) -> DenFn3<T>{
    Box::new(move |x|{
        let dist = x - sphere.center;
        dist.dot(&dist) - sphere.rad * sphere.rad
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
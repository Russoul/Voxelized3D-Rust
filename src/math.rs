use na::{convert, U1, U2, U3, Dynamic, MatrixArray, Scalar, MatrixVec, Vector, Vector2, VectorN, Real, DimName};
use na::storage::Storage;
use na::storage::ContiguousStorage;
use generic_array::ArrayLength;
use typenum::{Prod};
use alga::linear::FiniteDimInnerSpace;
use alga::general::SupersetOf;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Triangle<V> where V : FiniteDimInnerSpace + Copy{
    pub p1: V,
    pub p2: V,
    pub p3: V,
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

#[derive(Clone, Copy, Debug)]
pub struct Line<V> where V : FiniteDimInnerSpace + Copy{
    pub start : V,
    pub end : V,
}

#[derive(Clone, Copy, Debug)]
pub struct Square2<T : Scalar>{
    pub center : Vector2<T>,
    pub extent : T,
}

pub type DenFn2<T> = Box<Fn(Vector2<T>) -> T>;

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

pub fn distance_point2_line2<T : Real>(point2 : Vector2<T>, line2 : Line<Vector2<T>>) -> T{
    let d = line2.start - line2.end;
    let norm = d.normalize();
    let n = Vector2::new(-norm.y, norm.x);
    let vec = point2 - line2.start;
    Real::abs(n.dot(&vec))
}


pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32;16]{
    [2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
     0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom),
     0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
     0.0, 0.0, 0.0, 1.0
    ]
}
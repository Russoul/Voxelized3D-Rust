use na::{U1, U2, U3, Dynamic, MatrixArray, MatrixVec, Vector, VectorN, Real, DimName};
use na::storage::Storage;
use na::storage::ContiguousStorage;
use generic_array::ArrayLength;
use typenum::{Prod};
use alga::linear::FiniteDimInnerSpace;


pub struct Triangle<V> where V : FiniteDimInnerSpace + Copy{
    pub p1: V,
    pub p2: V,
    pub p3: V,
}

pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32;16]{
    [2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
     0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom),
     0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
     0.0, 0.0, 0.0, 1.0
    ]
}
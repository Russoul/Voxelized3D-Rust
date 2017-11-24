use vector::*;
use generic_array::*;

pub struct Triangle<T, N : ArrayLength<T>>{
    pub p1: Vector<T,N>,
    pub p2: Vector<T,N>,
    pub p3: Vector<T,N>,
}

pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32;16]{
    [2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
     0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom),
     0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
     0.0, 0.0, 0.0, 1.0
    ]
}
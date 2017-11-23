use vector::*;
use generic_array::*;

pub struct Triangle<T, N : ArrayLength<T>>{
    pub p1: Vector<T,N>,
    pub p2: Vector<T,N>,
    pub p3: Vector<T,N>,
}

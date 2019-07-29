use matrix::*;

use typenum::*;
use std::ops::*;
use generic_array::*;
use alga::general::*;
use typenum::consts::*;
use std::mem;
use std::fmt::Debug;

impl <T : Identity<Additive>, N, M> Mat<T,N,M> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{


    fn new_empty() -> Mat<T,N,M>{
        let ar = GenericArray::generate(|_| T::identity());
        Mat{ar}
    }
}

impl<T : Copy> Vec2<T>{
    fn new(x : T, y : T) -> Vec2<T>{
        Vec::<T, U2>{ar : GenericArray::<T, U2>::clone_from_slice(&[x,y])}
    }

}

impl<T : Copy> Vec3<T>{
    fn new(x : T, y : T, z : T) -> Vec3<T>{
        Vec::<T, U3>{ar : GenericArray::<T, U3>::clone_from_slice(&[x,y,z])}
    }

}


impl<T : Scalar + Mul<Output=T> + Add<Output=T> + Sub<Output=T>> Vec3<T>{

    #[inline]
    fn cross(self, other : Vec3<T>) -> Vec3<T>{
        cross(self, other)
    }
}

impl <T, N, M> Mat<T,N,M> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{


    fn get(&self, i : usize) -> &T{
        &self.ar[i]
    }
}

impl<T,N> Index<usize> for Vec<T,N> where
    N : Mul<U1>,
    Prod<N,U1> : ArrayLength<T>,{

    type Output = T;

    fn index(&self, i : usize) -> &T{
        &self.ar[i]
    }

}


impl<T,N> IndexMut<usize> for Vec<T,N> where
    N : Mul<U1>,
    Prod<N,U1> : ArrayLength<T>,{


    fn index_mut(&mut self, i : usize) -> &mut T{
        &mut self.ar[i]
    }

}

impl<T,N,M> Index<(usize, usize)> for Mat<T,N,M> where
    N : Unsigned,
    M : Unsigned,
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{

    type Output = T;

    fn index(&self, i : (usize, usize)) -> &T{
        &self.ar[i.0 * N::to_usize() + i.1]
    }

}

impl<T,N,M> IndexMut<(usize, usize)> for Mat<T,N,M> where
    N : Unsigned,
    M : Unsigned,
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{


    fn index_mut(&mut self, i : (usize, usize)) -> &mut T{
        &mut self.ar[i.0 * N::to_usize() + i.1]
    }

}

impl<'a,
    'b,
    T : Add<Output=T> + Copy,
    N,
    M>

Add<& 'b Mat<T,N,M>>

for & 'a Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{

    type Output = Mat<T,N,M>;

    fn add(self, other : & 'b Mat<T,N,M>) -> Mat<T,N,M>{
        Mat{ar : GenericArray::<T, Prod<N, M>>::
        generate(&|i| self.get(i).clone() + other.get(i).clone())}
    }

}

impl<'a,
    'b,
    T : Sub<Output=T> + Copy,
    N,
    M>

Sub<& 'b Mat<T,N,M>>

for & 'a Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{

    type Output = Mat<T,N,M>;

    fn sub(self, other : & 'b Mat<T,N,M>) -> Mat<T,N,M>{
        Mat{ar : GenericArray::<T, Prod<N, M>>::
        generate(&|i| self.get(i).clone() - other.get(i).clone())}
    }

}

fn dot<T : Scalar + Identity<Additive> + Mul<Output=T> + Add<Output=T>, N : Unsigned + Mul<U1>>(that : Vec<T,N>, other : Vec<T,N>) -> T where N : Mul<U1>, Prod<N,U1> : ArrayLength<T>,{
    let mut res = T::identity();
    for i in 0..<N as Unsigned>::to_usize(){
        res = res + that.ar[i] * other.ar[i];
    }

    res
}

fn cross<T : Scalar + Mul<Output=T> + Add<Output=T> + Sub<Output=T>>(that : Vec3<T>, other : Vec3<T>) -> Vec3<T>{
    Vec3::new(that.y * other.z - other.y * that.z, other.x * that.z -that.x * other.z, that.x * other.y - other.x * that.y)
}

impl<
    T : Mul<Output=T> + Add<Output=T> + Scalar + AbstractMonoid<Additive>,
    N>

Vec<T,N> where N : Mul<U1> + Unsigned, Prod<N,U1> : ArrayLength<T>{


    #[inline]
    fn dot(self, other : Vec<T,N>) -> T{
        dot(self, other)
    }

}

macro_rules! vec2 {
    ( $x:expr , $y:expr) => {
        {
            Vec2::new($x, $y)
        }
    };
}

macro_rules! vec3 {
    ( $x:expr , $y:expr, $z:expr ) => {
        {
            Vec3::new($x, $y, $z)
        }
    };
}

pub fn test_matrices(){
    let a = Vec::<_, U3>{ar : arr!(i32; 1,0,0)};
    let b = Vec::<_, U3>{ar : arr!(i32; -1,0,0)};
    let c = (&a) + (&b);

    let mut d = vec3!(1,2,3);
    let x = d.x;
    println!("d.x = {}", x);

    let i = vec3!(1,0,0);
    let j = vec3!(0,1,0);

    let j1 = j;

    d[1] = 0;
    d[(0,1)] = 0;

    println!("{:?} + {:?} = {:?}", a, b, c);
    println!("{}", (d).dot(d));
    println!("{}",  dot(d, d));
    println!("{:?}", cross(i, j));

    //TODO macros for creation
}
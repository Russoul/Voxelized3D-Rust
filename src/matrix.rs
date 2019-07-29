use typenum::*;
use std::ops::*;
use generic_array::*;
use alga::general::*;
use typenum::consts::*;
use std::mem;
use std::fmt::Debug;

//Mat, new,

pub trait Scalar: Copy + PartialEq + Debug  {
}
impl<T: Copy + PartialEq + Debug> Scalar for T {}

#[derive(Clone, Debug)]
pub struct Mat<T,N,M> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>{

        pub ar : GenericArray<T, typenum::Prod<N,M>>,
}

impl<T : Scalar, N : Clone, M : Clone> Copy for Mat<T, N, M>where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,
    GenericArray<T, typenum::Prod<N,M>> : Copy{

}


macro_rules! coords_impl(
    ($T: ident; $($comps: ident),*) => {
        /// Data structure used to provide access to matrix and vector coordinates with the dot
        /// notation, e.g., `v.x` is the same as `v[0]` for a vector.
        #[repr(C)]
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $T<N : Scalar> {
            $(pub $comps: N),*
        }
    }
);

coords_impl!(X; x);
coords_impl!(XY; x, y);
coords_impl!(XYZ; x, y, z);
coords_impl!(XYZW; x, y, z, w);


macro_rules! deref_impl(
    ($R: ty, $C: ty; $Target: ident) => {
        impl<N : Scalar> Deref for Mat<N, $R, $C>{
            type Target = $Target<N>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { mem::transmute(&self.ar) }
            }
        }

        impl<N : Scalar> DerefMut for Mat<N, $R, $C>{
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { mem::transmute(&mut self.ar) }
            }
        }
    }
);

deref_impl!(U1, U1; X);
deref_impl!(U2, U1; XY);
deref_impl!(U3, U1; XYZ);
deref_impl!(U4, U1; XYZW);

coords_impl!(M2x2; m11, m21,
                   m12, m22);
coords_impl!(M3x3; m11, m21, m31,
                   m12, m22, m32,
                   m13, m23, m33);

coords_impl!(M4x4; m11, m21, m31, m41,
                   m12, m22, m32, m42,
                   m13, m23, m33, m43,
                   m14, m24, m34, m44);

deref_impl!(U2, U2; M2x2);
deref_impl!(U3, U3; M3x3);
deref_impl!(U4, U4; M4x4);

pub type Vec<T, N> = Mat<T,N,U1>;
pub type Vec2<T> = Vec<T,U2>;
pub type Vec3<T> = Vec<T,U3>;
pub type Vec4<T> = Vec<T,U3>;

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



use typenum::*;
use std::ops::*;
use generic_array::*;
use alga::general::*;
use typenum::consts::*;
use std::mem;
use std::fmt::{Debug, Display, Formatter};
use math::{givens_rot, givens_rot_qr, qr_eigen};
use std::fmt;

//Mat, new,

pub trait Value: Copy + PartialEq + Debug  {
}
impl<T: Copy + PartialEq + Debug> Value for T {}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Mat<T : Value,N : Clone + Unsigned,M : Clone + Unsigned> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>{

        pub ar : GenericArray<T, typenum::Prod<N,M>>,
}

impl<T : Value, N : Clone + Unsigned, M : Clone + Unsigned> Copy for Mat<T, N, M>where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,
    GenericArray<T, typenum::Prod<N,M>> : Copy{

}

impl<T : Value + Display, N : Unsigned + Clone, M : Unsigned + Clone> Display for Mat<T, N, M> where N : Mul<M>, Prod<N, M> : ArrayLength<T>{

    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for i in 0..N::to_usize(){
            write!(f, "[ ");
            for j in 0..M::to_usize(){
                write!(f, "{:10.4} ", self[(i, j)]);
            }
            writeln!(f, "]");
        }
        Ok(())
    }
}

macro_rules! coords_impl(
    ($T: ident; $($comps: ident),*) => {
        /// Data structure used to provide access to matrix and vector coordinates with the dot
        /// notation, e.g., `v.x` is the same as `v[0]` for a vector.
        #[repr(C)]
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $T<N : Value> {
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
        impl<N : Value> Deref for Mat<N, $R, $C>{
            type Target = $Target<N>;


            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { mem::transmute(&self.ar) }
            }
        }

        impl<N : Value> DerefMut for Mat<N, $R, $C>{
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
pub type Mat3<T> = Mat<T, U3, U3>;
pub type Mat4<T> = Mat<T, U4, U4>;

impl <T : Value, N : Clone + Unsigned, M : Clone + Unsigned> Mat<T,N,M> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{


    pub fn as_slice(&self) -> &[T]{
        self.ar.as_slice()
    }

}

impl <T : Value + Identity<Additive>, N : Clone + Unsigned, M : Clone + Unsigned> Mat<T,N,M> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{


    pub fn empty() -> Mat<T,N,M>{
        let ar = GenericArray::generate(|_| T::identity());
        Mat{ar}
    }

}

impl <T : Value + Identity<Additive> + Identity<Multiplicative>, N : Unsigned + Clone> Mat<T,N,N> where
    N : Mul<N>,
    Prod<N,N> : ArrayLength<T>,{


    pub fn identity() -> Mat<T,N,N>{
        let mut m = Mat::<T, N, N>::empty();
        for i in 0..N::to_usize(){
            m[(i,i)] = T::id(Multiplicative)
        }
        m
    }

}

impl<T : Value> Vec2<T>{
    pub fn new(x : T, y : T) -> Vec2<T>{
        Vec::<T, U2>{ar : GenericArray::<T, U2>::clone_from_slice(&[x,y])}
    }

}

impl<T : Value> Vec3<T>{
    pub fn new(x : T, y : T, z : T) -> Vec3<T>{
        Vec::<T, U3>{ar : GenericArray::<T, U3>::clone_from_slice(&[x,y,z])}
    }

}

impl<T : Value> Mat3<T>{
    pub fn new(m11 : T, m12 : T, m13 : T,
               m21 : T, m22 : T, m23 : T,
               m31 : T, m32 : T, m33 : T) -> Mat3<T>{
        Mat::<T, U3, U3>{ar : GenericArray::<T, U9>::clone_from_slice(&[m11, m12, m13,
                                                                         m21, m22, m23,
                                                                         m31, m32, m33])}
    }

}

impl<T : Value> Mat4<T>{
    pub fn new(m11 : T, m12 : T, m13 : T, m14 : T,
               m21 : T, m22 : T, m23 : T, m24 : T,
               m31 : T, m32 : T, m33 : T, m34 : T,
               m41 : T, m42 : T, m43 : T, m44 : T) -> Mat4<T>{
        Mat::<T, U4, U4>{ar : GenericArray::<T, U16>::clone_from_slice(&[m11, m12, m13, m14,
                                                                              m21, m22, m23, m24,
                                                                              m31, m32, m33, m34,
                                                                              m41, m42, m43, m44])}
    }

}


impl<T : Value + Mul<Output=T> + Add<Output=T> + Sub<Output=T>> Vec3<T>{

        #[inline]
        pub fn cross(self, other : Vec3<T>) -> Vec3<T>{
                cross(self, other)
        }
}

impl <T : Value, N : Unsigned + Clone, M : Unsigned + Clone> Mat<T,N,M> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{


        pub fn get(&self, i : usize) -> T{
                self.ar[i]
        }
}

impl<T : Value,N : Clone + Unsigned> Index<usize> for Vec<T,N> where
    N : Mul<U1>,
    Prod<N,U1> : ArrayLength<T>,{

        type Output = T;

        fn index(&self, i : usize) -> &T{
                &self.ar[i]
        }

}


impl<T : Value,N : Clone + Unsigned> IndexMut<usize> for Vec<T,N> where
    N : Mul<U1>,
    Prod<N,U1> : ArrayLength<T>,{


        fn index_mut(&mut self, i : usize) -> &mut T{
                &mut self.ar[i]
        }

}

impl<T : Value,N,M> Index<(usize, usize)> for Mat<T,N,M> where
    N : Unsigned + Clone,
    M : Unsigned + Clone,
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{

        type Output = T;

        fn index(&self, i : (usize, usize)) -> &T{
                &self.ar[i.0 * M::to_usize() + i.1]
        }

}

impl<T : Value,N,M> IndexMut<(usize, usize)> for Mat<T,N,M> where
    N : Unsigned + Clone,
    M : Unsigned + Clone,
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{


        fn index_mut(&mut self, i : (usize, usize)) -> &mut T{
                &mut self.ar[i.0 * M::to_usize() + i.1]
        }

}

impl<   T : Add<Output=T> + Value,
        N : Clone + Unsigned,
        M : Clone + Unsigned>

Add<Mat<T,N,M>>

for Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{

    type Output = Mat<T,N,M>;

    fn add(self, other : Mat<T,N,M>) -> Mat<T,N,M>{
            Mat{ar : GenericArray::<T, Prod<N, M>>::
            generate(&|i| self.get(i) + other.get(i))}
    }

}

impl<   T : Add<Output=T> + Value,
    N : Clone + Unsigned,
    M : Clone + Unsigned>

AddAssign<Mat<T,N,M>>

for Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{


    fn add_assign(&mut self, other : Mat<T,N,M>){
        *self = Mat{ar : GenericArray::<T, Prod<N, M>>::
        generate(&|i| self.get(i) + other.get(i))}
    }

}

impl<   T : Mul<Output=T> + Value,
    N : Clone + Unsigned,
    M : Clone + Unsigned>

MulAssign<T>

for Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{


    fn mul_assign(&mut self, k : T){
        *self = Mat{ar : GenericArray::<T, Prod<N, M>>::
        generate(&|i| self.get(i) * k)}
    }

}

impl<   T : Sub<Output=T> + Value,
    N : Clone + Unsigned,
    M : Clone + Unsigned>

SubAssign<Mat<T,N,M>>

for Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{


    fn sub_assign(&mut self, other : Mat<T,N,M>){
        *self = Mat{ar : GenericArray::<T, Prod<N, M>>::
        generate(&|i| self.get(i) - other.get(i))}
    }

}

impl<   T : Neg<Output=T> + Value,
    N : Clone + Unsigned,
    M : Clone + Unsigned>

Neg

for Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{

    type Output = Mat<T,N,M>;

    fn neg(self) -> Mat<T,N,M>{
        Mat{ar : GenericArray::<T, Prod<N, M>>::
        generate(&|i| -self.get(i))}
    }

}

impl<   T : Mul<Output=T> + Value,
    N : Clone + Unsigned,
    M : Clone + Unsigned>

Mul<T>

for Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{

    type Output = Mat<T,N,M>;

    fn mul(self, k : T) -> Mat<T,N,M>{
        Mat{ar : GenericArray::<T, Prod<N, M>>::
        generate(&|i| self.get(i) * k)}
    }

}

impl<
        T : Add<Output=T> + Value + Mul<Output=T> + AdditiveMonoid,
        N : Unsigned + Clone,
        M : Unsigned + Clone,
        L : Unsigned + Clone>

Mul<Mat<T,M,L>>

for Mat<T,N,M> where N : Mul<M>, N : Mul<L>, M : Mul<L>, Prod<N,M> : ArrayLength<T>, Prod<M, L> : ArrayLength<T>, Prod<N, L> : ArrayLength<T>{

    type Output = Mat<T,N,L>;

    fn mul(self, other : Mat<T,M,L>) -> Mat<T,N,L>{

        let mut c = Mat::<T, N, L>::empty();

        for i in 0..N::to_usize(){
            for j in 0..L::to_usize(){
                c[(i,j)] = T::identity();
                for k in 0..M::to_usize(){
                    c[(i,j)] += self[(i,k)] * other[(k,j)];
                }
            }
        }

        c
    }

}

impl<A : Value + Identity<Additive>, N : Unsigned + Clone, M : Unsigned + Clone> Mat<A, N, M> where N : Mul<M>, Prod<N, M> : ArrayLength<A>, M : Mul<N>, Prod<M, N> : ArrayLength<A>{
    pub fn transpose(&self) -> Mat<A, M, N>{
        let mut r = Mat::<A, M, N>::empty();
        for i in 0..N::to_usize(){
            for j in 0..M::to_usize(){
                r[(j, i)] =  self[(i, j)];
            }
        }

        r
    }
}

impl<   T : Sub<Output=T> + Value,
        N : Clone + Unsigned,
        M : Clone + Unsigned>

Sub<Mat<T,N,M>>
for Mat<T,N,M> where N : Mul<M>, Prod<N,M> : ArrayLength<T>{
        type Output = Mat<T,N,M>;

        fn sub(self, other : Mat<T,N,M>) -> Mat<T,N,M>{
                Mat{ar : GenericArray::<T, Prod<N, M>>::
                generate(&|i| self.get(i) - other.get(i))}
        }

}

pub fn dot<T : Value + Identity<Additive> + Mul<Output=T> + AddAssign, N : Unsigned + Clone + Mul<U1>>(that : Vec<T,N>, other : Vec<T,N>) -> T where N : Mul<U1>, Prod<N,U1> : ArrayLength<T>,{
        let mut res = T::identity();
        for i in 0..<N as Unsigned>::to_usize(){
                res += that.ar[i] * other.ar[i];
        }

        res
}

pub fn cross<T : Value + Mul<Output=T> + Add<Output=T> + Sub<Output=T>>(that : Vec3<T>, other : Vec3<T>) -> Vec3<T>{
        Vec3::new(that.y * other.z - other.y * that.z, other.x * that.z - that.x * other.z, that.x * other.y - other.x * that.y)
}

impl<
    T : Mul<Output=T> + AddAssign + AbstractMonoid<Additive> + Value,
    N : Clone + Unsigned>

Vec<T,N> where N : Mul<U1> + Unsigned, Prod<N,U1> : ArrayLength<T>{


    #[inline]
    pub fn dot(self, other : Vec<T,N>) -> T{
        dot(self, other)
    }


}

impl<
    T : Real,
    N : Clone + Unsigned>

Vec<T,N> where N : Mul<U1> + Unsigned, Prod<N,U1> : ArrayLength<T>, GenericArray<T, Prod<N, U1>> : Copy{

    #[inline]
    pub fn norm(self) -> T{
        T::sqrt(dot(self, self))
    }

    #[inline]
    pub fn normalize(self) -> Vec<T, N>{
        self * (T::one() / self.norm())
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


#[cfg(test)]
pub fn test_matrices(){
    let a = Vec::<_, U3>{ar : arr!(i32; 1,0,0)};
    let b = Vec::<_, U3>{ar : arr!(i32; -1,0,0)};
    let c = a + b;

    let mut d = vec3!(1,2,3);
    let x = d.x;
    println!("d.x = {}", x);

    let i = vec3!(1,0,0);
    let j = vec3!(0,1,0);

    let j1 = j;

    d[1] = 0;
    d[(0,1)] = 0;

    println!("{:?} + {:?} = {:?}", a, b, c);
    println!("{}", d.dot(d));
    println!("{}",  dot(d, d));
    println!("{:?}", cross(i, j));

    let m1 = Mat3::new(6.0, 5.0, 0.0,
                       5.0, 1.0, 4.0,
                       0.0, 4.0, 3.0);

    println!("m1");
    println!("{}", m1);

    let upper = givens_rot(m1, 0.001);

    let m2 = Mat4::new(
        52.0, 30.0, 49.0, 28.0,
        30.0, 50.0, 8.0, 44.0,
        49.0, 8.0, 46.0, 16.0,
        28.0, 44.0, 16.0, 22.0
    );

    println!("{}", upper);
    println!("{}", givens_rot_qr(m2, 0.00001).1);
    println!("{}", qr_eigen(m2, 0.00001, 0.0001).0);

    //TODO macros for creation
}



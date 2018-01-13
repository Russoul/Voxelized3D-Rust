use typenum::*;
use std::ops::*;
use generic_array::*;
use alga::general::*;
use typenum::consts::*;

#[derive(Clone, Debug)]
pub struct Mat<T,N,M> where
    N : Mul<M>,
    Prod<N,M> : ArrayLength<T>,{

        pub ar : GenericArray<T, typenum::Prod<N,M>>,
}

type Vec<T, N> = Mat<T,N,U1>;
type Vec2<T> = Vec<T,U2>;
type Vec3<T> = Vec<T,U3>;
type Vec4<T> = Vec<T,U3>;


impl <T : Identity<Additive>, N, M> Mat<T,N,M> where
    N : Mul<M>,
    typenum::Prod<N,M> : ArrayLength<T>,{
    

    fn new_empty() -> Mat<T,N,M>{
        let ar = GenericArray::generate(|_| T::identity());
        Mat{ar}
    }
}

impl<T : Copy> Vec2<T>{
    fn new(x : T, y : T) -> Vec2<T>{
        Vec::<T, U2>{ar : GenericArray::<T, U2>::clone_from_slice(&[x,y])}
    }

    fn x(&self) -> T{
        self.ar[0]
    }

    fn y(&self) -> T{
        self.ar[1]
    }
}

impl<T : Copy> Vec3<T>{
    fn new(x : T, y : T, z : T) -> Vec3<T>{
        Vec::<T, U3>{ar : GenericArray::<T, U3>::clone_from_slice(&[x,y,z])}
    }

    fn x(&self) -> T{
        self.ar[0]
    }

    fn y(&self) -> T{
        self.ar[1]
    }

    fn z(&self) -> T{
        self.ar[2]
    }
}


impl<T : Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T>> Vec3<T>{

    fn cross(&self, other : &Vec3<T>) -> Vec3<T>{
        Vec3::new(self.y() * other.z() - other.y() * self.z(), other.x() * self.z() -self.x() * other.z(), self.x() * other.y() - other.x() * self.y())
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

impl<
     T : Mul<Output=T> + Add<Output=T> + Copy + AbstractMonoid<Additive>,
     N>
     
Vec<T,N> where N : Mul<U1> + Unsigned, Prod<N,U1> : ArrayLength<T>{
    

    fn dot(&self, other : &Vec<T,N>) -> T{
        let mut res = T::identity();
        for i in 0..<N as Unsigned>::to_usize(){
            res = res + self.ar[i] * other.ar[i];
        }

        res
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

    let i = vec3!(1,0,0);
    let j = vec3!(0,1,0);
    
    d[1] = 0;
    d[(0,1)] = 0;

    println!("{:?} + {:?} = {:?}", a, b, c);
    println!("{}", (&d).dot(&d));
    println!("{:?}", (&i).cross(&j));

    //TODO macros for creation
}
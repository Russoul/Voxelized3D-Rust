use alga;
use std;
use noise::{NoiseFn, Perlin};
use alga::general::Real;
use num;
use std::ops::{Mul, Add, Neg};
use typenum::{Prod, Unsigned, Minimum, Min, U1, U2, U3, U4};
use matrix::*;
use generic_array::ArrayLength;
use alga::general::{MultiplicativeMonoid, AdditiveMonoid};
use std::fmt::Debug;
use std::convert::identity;
use generic_array::GenericArray;

#[derive(Clone, Copy, Debug)]
pub struct Triangle2<T : Value>{
    pub p1: Vec2<T>,
    pub p2: Vec2<T>,
    pub p3: Vec2<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle3<T : Value>{
    pub p1: Vec3<T>,
    pub p2: Vec3<T>,
    pub p3: Vec3<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Line2<T : Value> {
    pub start : Vec2<T>,
    pub end : Vec2<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Line3<T : Value> {
    pub start : Vec3<T>,
    pub end : Vec3<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Plane<T : Value> {
    pub point : Vec3<T>,
    pub normal : Vec3<T>,
}

//axis aligned
#[derive(Clone, Copy, Debug)]
pub struct Square2<T : Value>{
    pub center : Vec2<T>,
    pub extent : T,
}

//axis aligned
#[derive(Clone, Copy, Debug)]
pub struct Cube<T : Value>{
    pub center : Vec3<T>,
    pub extent : T,
}

impl<T : Real> Cube<T>{
    pub fn min(self) -> Vec3<T>{
        self.center - Vec3::new(self.extent,self.extent,self.extent)
    }

    pub fn max(self) -> Vec3<T>{
        self.center + Vec3::new(self.extent,self.extent,self.extent)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere<T : Value>{
    pub center : Vec3<T>,
    pub rad : T,
}



pub type DenFn2<'a, T> = Box<Fn(Vec2<T>) -> T + 'a>;
pub type DenFn3<T> = Box<Fn(Vec3<T>) -> T>;


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

pub fn noise_f32(perlin : Perlin, cube : Cube<f32>) -> DenFn3<f32>{
    Box::new( move |x| {
        if point3_inside_cube_inclusive(x, cube){
            let den = -octave_perlin2(&perlin, x.x - (cube.center.x - cube.extent), x.z - (cube.center.z - cube.extent), 4, 0.56) * 2.0 * cube.extent;
            let dy = (x.y - (cube.center.y - cube.extent) ); //cube.extent / 2.0 ; // 0 - 1
            //println!("{} {} {}", den, dy, x.y);
            den + dy
        }else{
            0.01
        }
        
    })
}


pub fn mk_circle2<'a, T : Real + Copy>(center : Vec2<T>, rad : T) -> DenFn2<'a, T>{
    Box::new(move |x|{
        let dist = x - center;
        dot(dist,dist) - rad * rad
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

pub fn mk_half_space_x_neg<T : Real>(x : T) -> DenFn3<T>{
    Box::new(move |p|{p.x - x})
}

pub fn mk_half_space_x_pos<T : Real>(x : T) -> DenFn3<T>{
    Box::new(move |p|{x - p.x})
}

pub fn mk_half_space_y_neg<T : Real>(y : T) -> DenFn3<T>{
    Box::new(move |p|{p.y - y})
}

pub fn mk_half_space_y_pos<T : Real>(y : T) -> DenFn3<T>{
    Box::new(move |p|{y - p.y})
}

pub fn mk_half_space_z_neg<T : Real>(z : T) -> DenFn3<T>{
    Box::new(move |p|{p.z - z})
}

pub fn mk_half_space_z_pos<T : Real>(z : T) -> DenFn3<T>{
    Box::new(move |p|{z - p.z})
}

pub fn mk_rectangle2<'a, T : Real>(center : Vec2<T>, extent : Vec2<T>) -> DenFn2<'a, T> {
    let right = mk_half_plane2_right(center.x - extent.x);
    let left = mk_half_plane2_left(center.x + extent.x);

    let lower = mk_half_plane2_lower(center.y + extent.y);
    let upper = mk_half_plane2_upper(center.y - extent.y);

    let i1 = intersection2_move(left, right);
    let i2 = intersection2_move(upper, lower);

    intersection2_move(i1, i2)
}



pub fn mk_aabb<T : Real + Copy>(center : Vec3<T>, extent : Vec3<T>) -> DenFn3<T> {
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

pub fn mk_half_space_pos<T : Real>(plane : Plane<T>) -> DenFn3<T>{
     Box::new(move |p|{
        let d = p - plane.point;
        let dist = dot(d,plane.normal);
        -dist 
     })
}

pub fn mk_half_space_neg<T : Real>(plane : Plane<T>) -> DenFn3<T>{
     Box::new(move |p|{
        let d = p - plane.point;
        let dist = dot(d, plane.normal);
        dist 
     })
}

pub fn mk_obb<T : Real>(center : Vec3<T>, right : Vec3<T>, up : Vec3<T>, extent : Vec3<T>) -> DenFn3<T> {
    let r_neg = mk_half_space_neg(Plane{point : center + right * extent.x, normal : right});
    let r_pos = mk_half_space_pos(Plane{point : center - right * extent.x, normal : right});

    let u_neg = mk_half_space_neg(Plane{point : center + up * extent.y, normal : up});
    let u_pos = mk_half_space_pos(Plane{point : center - up * extent.y, normal : up});

    let look = cross(right,up);

    let l_neg = mk_half_space_neg(Plane{point : center + look * extent.z, normal : look});
    let l_pos = mk_half_space_pos(Plane{point : center - look * extent.z, normal : look});

    let ix = intersection3(r_neg, r_pos);
    let iy = intersection3(u_neg, u_pos);
    let iz = intersection3(l_neg, l_pos);

    let ixy = intersection3(ix, iy);

    intersection3(ixy, iz)
}

pub fn mk_sphere<T : Real>(sphere : Sphere<T>) -> DenFn3<T>{
    Box::new(move |x|{
        let dist = x - sphere.center;
        dot(dist,dist) - sphere.rad * sphere.rad
    })
}

pub fn mk_torus_z<T : Real>(r_big : T, r : T, offset : Vec3<T>) -> DenFn3<T>{
    Box::new(move |p|{
        let x = p - offset;
        let a = (x.x * x.x + x.y * x.y).sqrt() - r_big;
        a * a + x.z * x.z - r * r
    })
}

pub fn mk_torus_y<T : Real>(r_big : T, r : T, offset : Vec3<T>) -> DenFn3<T>{
    Box::new(move |p|{
        let x = p - offset;
        let a = (x.x * x.x + x.z * x.z).sqrt() - r_big;
        a * a + x.y * x.y - r * r
    })
}


pub fn mk_sphere_displacement<T : Real>(sphere : Sphere<T>, f : Box<Fn(Vec3<T>) -> T>) -> DenFn3<T>{
    Box::new(move |x|{
        let dist = x - sphere.center;
        dot(dist,dist) - sphere.rad * sphere.rad * f(dist.normalize())
    })
}

pub fn distance_point2_line2<T : Real>(point2 : Vec2<T>, line2 : Line2<T>) -> T{
    let d = line2.start - line2.end;
    let norm = d.normalize();
    let n = Vec2::new(-norm.y, norm.x);
    let vec = point2 - line2.start;
    Real::abs(n.dot(vec))
}

pub fn distance_point3_plane<T : Real>(point3 : Vec3<T>, plane : Plane<T>) -> T{
    let vec = point3 - plane.point;
    Real::abs(dot(plane.normal,vec))
}

pub fn point3_inside_cube_inclusive<T : Real>(point3 : Vec3<T>, square3 : Cube<T>) -> bool{
    point3.x <= square3.center.x + square3.extent &&
    point3.x >= square3.center.x - square3.extent &&

    point3.y <= square3.center.y + square3.extent &&
    point3.y >= square3.center.y - square3.extent &&

    point3.z <= square3.center.z + square3.extent &&
    point3.z >= square3.center.z - square3.extent
}

pub fn point3_inside_sphere_inclusive<T : Real>(point3 : Vec3<T>, sphere : Sphere<T>) -> bool {
    let d = point3 - sphere.center;
    dot(d, d) <= sphere.rad * sphere.rad
}

pub fn rot_mat3<T : Real>(u : Vec3<T>, rad : T) -> Mat3<T>{
    let c = T::cos(rad);
    let s = T::sin(rad);
    let one = T::one();
    Mat3::new(
        c + u.x*u.x*(one - c), u.x*u.y*(one - c) - u.z*s, u.x*u.z*(one - c) + u.y*s,
        u.y*u.x*(one - c) + u.z*s, c + u.y*u.y*(one - c), u.y*u.z*(one - c) - u.x*s,
        u.z*u.x*(one - c) - u.y*s - u.y*s, u.z*u.y*(one - c) + u.x*s, c + u.z*u.z*(one - c)
    )
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
pub fn perspective(fovy : f32, aspect : f32, near : f32, far : f32) -> Mat4<f32>{
    let top = near * (std::f32::consts::PI / 180.0 * fovy / 2.0).tan();
    let bottom = -top;
    let right = top * aspect;
    let left = -right;
    Mat4::new(2.0 * near / (right - left), 0.0, (right + left) / (right - left), 0.0,
                 0.0, 2.0 * near / (top - bottom), (top + bottom) / (top - bottom), 0.0,
                 0.0, 0.0, -(far + near) / (far - near), -2.0 * far * near / (far - near),
                 0.0, 0.0, -1.0, 0.0)
}

//column-major
pub fn view_dir(pos : Vec3<f32>, look : Vec3<f32>, up : Vec3<f32>) -> Mat4<f32>{
    let za = -look;
    let xa = cross(up, za);
    let ya = cross(za, xa);

    Mat4::new(xa.x, ya.x, za.x, 0.0,
                 xa.y, ya.y, za.y, 0.0,
                 xa.z, ya.z, za.z, 0.0,
                 -dot(xa,pos), -dot(ya,pos), -dot(za,pos), 1.0).transpose()
}



pub fn factorial<A : num::Unsigned + Copy>(e : A) -> A{
    if e == num::one::<A>() || e == num::zero::<A>(){
        num::one::<A>()
    }else{
        e * factorial(e - num::one::<A>())
    }
}

pub fn givens_mat<A : Value + Mul + Add + MultiplicativeMonoid + AdditiveMonoid + Neg<Output=A>, N : Unsigned + Debug + Clone>(l : usize, m : usize, c : A, s : A) -> Mat<A, N, N> where N : Mul<N>, Prod<N, N> : ArrayLength<A>{
    let mut mat = Mat::identity();
    mat[(l, l)] = c;
    mat[(l - 1, l - 1)] = c;
    mat[(l - 1, l)] = -s;
    mat[(l, l - 1)] = s;

    mat
}

//returns only R
pub fn givens_rot<A : Value + Mul + Add + MultiplicativeMonoid + AdditiveMonoid + Neg<Output=A> + Real, N : Unsigned + Mul<N> + Debug + Clone, M : Unsigned + Debug + Clone>(m : Mat<A, N, M>, eps : A) -> Mat<A, N, M> where
    Prod<N,N> : ArrayLength<A>, Prod<N,M> : ArrayLength<A>, N : Mul<M>,  GenericArray<A, typenum::Prod<N,M>> : Copy
{
    let mut c = m;
    for j in 0..M::to_usize(){
        for i in (j+1..N::to_usize()).rev(){
            if <A as Real>::abs(c[(i, j)]) > eps{
                let a = c[(i - 1, j)];
                let b = c[(i, j)];
                let r = A::sqrt(a * a + b * b);
                let cos = a/r;
                let sin = b/r;
                println!("givens mat i = {}, j = {}, c = {}, s = {}\ngivens\n{}\nres\n{}", i + 1, j + 1, cos, sin, givens_mat::<A, N>(i, j, cos, sin), givens_mat::<A, N>(i, j, cos, sin).transpose() * c);
                c = givens_mat::<A, N>(i, j, cos, sin).transpose() * c;
            }
        }
    }

    c
}

pub fn givens_rot_qr<A : Value + Mul + Add + MultiplicativeMonoid + AdditiveMonoid + Neg<Output=A> + Real, N : Unsigned + Mul<N> + Debug + Clone, M : Unsigned + Debug + Clone>(m : Mat<A, N, M>, eps : A) -> (Mat<A, N, N>, Mat<A, N, M>) where
    Prod<N,N> : ArrayLength<A>, Prod<N,M> : ArrayLength<A>, N : Mul<M>, GenericArray<A, typenum::Prod<N,N>> : Copy
{
    let mut c = m;
    let mut q = Mat::identity();
    for j in 0..M::to_usize(){
        for i in (j+1..N::to_usize()).rev(){
            if <A as Real>::abs(c[(i, j)]) > eps{
                let a = c[(i - 1, j)];
                let b = c[(i, j)];
                let r = A::sqrt(a * a + b * b);
                let cos = a/r;
                let sin = b/r;
                let q_ij = givens_mat::<A, N>(i, j, cos, sin);
                c = q_ij.transpose() * c;
                q = q * q_ij;
            }
        }
    }

    (q, c)
}

//computes SDV of symmetric matrix
pub fn qr_eigen<A : Value + Mul + Add + MultiplicativeMonoid + AdditiveMonoid + Neg<Output=A> + Real, N : Unsigned + Mul<N> + Debug + Clone>(m : Mat<A, N, N>, eps_0 : A, eps_stop : A) -> (Vec<A, N>, Mat<A, N, N>) where
    Prod<N,N> : ArrayLength<A>, GenericArray<A, typenum::Prod<N,N>> : Copy, N : Min<N>, N : Mul<U1>, Prod<N, U1> : ArrayLength<A>{
    let mut x = m;
    let mut q = Mat::identity();

    let mut con = true;
    let mut num_iter = 0usize;
    while con || num_iter > 30{

        let (q_n, r) = givens_rot_qr(x, eps_0);
        //println!("here {:?}", r);
        x = r * q_n;
        q = q * q_n;

        let mut sum = A::zero();
        let mut count = 0usize;

        for i in 0..N::to_usize(){
            for j in 0..N::to_usize(){
                if i != j{
                    sum += <A as Real>::abs(x[(i,j)]); //TODO diagonal
                    count += 1;
                }
            }
        }

        if sum < eps_stop * A::from_usize(count).unwrap(){
            con = false;
        }
    }

    let mut e = Vec::empty();

    for i in 0..N::to_usize(){
        e[i] = x[(i,i)];
    }

    (e, q)
}
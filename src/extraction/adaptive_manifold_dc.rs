use std;
use math::*;
use std::vec::{Vec as Vector};
use renderer::*;
use alga::general::*;
use alga::general::SupersetOf;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use matrix::*;
use typenum::{U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12};
use std::process::exit;
use lapacke::{sgeqrf, sgesvd};
use std::ptr::{null, null_mut};
use downcast_rs::Downcast;
use util::{ar_mut, val_const, ar_const, val_mut};
use generic_array::*;
use num::Zero;
use vulkano::buffer::TypedBufferAccess;

#[derive(Clone, Debug)]
struct Qef<T> where T : Value{
    mat_a : Mat3<T> , //TODO store only none-zero elements (6)
    vec_b : Vec3<T>,
    r : T,
    centroid : Vec3<T>,
    vertex : Option<Vec3<T>>
}

impl<T : Value + Identity<Additive>> Qef<T>{
    pub fn new() -> Qef<T>{
        Qef{
            mat_a : Mat3::empty(),
            vec_b : Vec3::empty(),
            r : T::identity(),
            centroid : Vec3::empty(),
            vertex : None
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq, Copy, Debug)]
#[repr(C)]
enum NodeType{
    Homo,
    Hetero,
    Interior
}

trait TypeOfNode{
    fn tpe() -> NodeType;
}

#[derive(Clone)]
#[repr(C)]
struct Node{
    pub tpe : NodeType,
    pub depth : u8,
    pub corner_signs : u8,
    pub parent : *mut Node
}



//leaf node
#[derive(Clone)]
#[repr(C)]
struct HomogeneousNode
{
    pub tpe : NodeType,
    pub depth : u8,
    pub corner_signs : u8, //can be only 0(pos) or 255(neg)
    pub parent : *mut Node

}

impl HomogeneousNode{
    pub fn is_positive(&self) -> bool{
        self.corner_signs == 0
    }
}

//leaf node
#[derive(Clone)]
#[repr(C)]
struct HeterogeneousNode<T : Value>{
    pub tpe : NodeType,
    pub depth : u8,
    pub corner_signs : u8,
    pub parent : *mut Node,
    pub qefs : Vector<Qef<T>>,
}


fn downcast<Ty : TypeOfNode>(node : *mut Node) -> Option<*mut Ty>{
    unsafe{
        if (*node).tpe == Ty::tpe(){
            Some(node as *mut Ty)
        }else{
            None
        }
    }
}

fn upcast<Ty : TypeOfNode>(node : *mut Ty) -> *mut Node{
    unsafe { node as *mut Node }
}

impl<T : Value> TypeOfNode for HeterogeneousNode<T>{
    fn tpe() -> NodeType {
        NodeType::Hetero
    }
}
impl<T : Value> TypeOfNode for InteriorNode<T>{
    fn tpe() -> NodeType {
        NodeType::Interior
    }
}
impl TypeOfNode for HomogeneousNode{
    fn tpe() -> NodeType {
        NodeType::Homo
    }
}

#[derive(Clone)]
#[repr(C)]
struct InteriorNode<T : Value>{
    pub tpe : NodeType,
    pub depth : u8,
    pub corner_signs : u8,
    pub parent : *mut Node,
    pub children : Vec<*mut Node, U8>,
    pub collapsible : bool, //whether its children can be collapsed into one vertex or not
    pub collapsed : bool,
    pub qefs : Vector<(Qef<T>, Vector<u32>, HashSet<u8>)> //0b...xxx; where xxx - binary index of the child, ... - binary index of the qef in that child
}

//256 x 16
pub fn edge_table() -> Vector< Vector< isize > >{
    vec![
        vec![-2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 8, 3, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 8, 3, -1, 1, 2, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![9, 0, 2, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![10, 2, 3, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 8, 11, 2, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 9, 0, -1, 2, 3, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![8, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 7, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 8, 4, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 7, 9, -2, -1, -1, 1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 8, 4, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 0, 3, 7, 4, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 10, 9, -1, 8, 7, 4, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 7, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![8, 4, 7, -1, 3, 11, 2, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 4, 7, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![9, 0, 1, -1, 8, 4, 7, -1, 2, 3, 11, -2, -1, -1, -1, -1],
        vec![1, 2, 4, 7, 9, 11, -2, -1, -1, -1, -1, -1-1, -1, -1, -1, -1],
        vec![3, 11, 10, 1, -1, 8, 7, 4, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 7, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 7, 8, -1, 0, 3, 11, 10, 9, -2, -1, -1, -1, -1, -1, -1],
        vec![4, 7, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![9, 5, 4, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![9, 5, 4, -1, 0, 8, 3, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 5, 4, 1, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 5, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 9, 5, 4, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 0, 8, -1, 1, 2, 10, -1, 4, 9, 5, -2, -1, -1, -1, -1],
        vec![0, 2, 4, 5, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 5, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![9, 5, 4, -1, 2, 3, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![9, 4, 5, -1, 0, 2, 11, 8, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -1, 0, 4, 5, 1, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 5, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 10, 1, -1, 9, 4, 5, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 9, 5, -1, 0, 1, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 5, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 4, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 7, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 5, 7, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 5, 7, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 5, 3, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 8, 7, 5, 9, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 0, 3, 7, 5, 9, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 5, 7, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 5, 7, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 11, -1, 5, 7, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 5, 7, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 11, -1, 0, 1, 5, 7, 8, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 5, 7, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 10, 1, -1, 8, 7, 5, 9, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 5, 7, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 5, 7, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 7, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 6, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 8, -1, 10, 5, 6, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 10, 5, 6, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![10, 5, 6, -1, 3, 8, 9, 1, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 5, 6, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 8, -1, 1, 2, 6, 5, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 5, 6, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 5, 6, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -1, 10, 6, 5, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![10, 5, 6, -1, 0, 8, 2, 11, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -1, 0, 1, 9, -1, 10, 5, 6, -2, -1, -1, -1, -1],
        vec![10, 5, 6, -1, 11, 2, 8, 9, 1, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 5, 6, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 5, 6, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 5, 6, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 6, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![8, 7, 4, -1, 5, 6, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 6, 10, -1, 0, 3, 4, 7, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![10, 5, 6, -1, 1, 9, 0, -1, 8, 7, 4, -2, -1, -1, -1, -1],
        vec![10, 5, 6, -1, 7, 4, 9, 1, 3, -2, -1, -1, -1, -1, -1, -1],
        vec![8, 7, 4, -1, 1, 2, 6, 5, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 7, 4, -1, 1, 2, 6, 5, -2, -1, -1, -1, -1, -1, -1],
        vec![8, 7, 4, -1, 0, 9, 2, 5, 6, -2, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 5, 6, 7, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -1, 5, 6, 10, -1, 8, 7, 4, -2, -1, -1, -1, -1],
        vec![10, 5, 6, -1, 0, 2, 11, 7, 4, -2, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -1, 0, 1, 9, -1, 10, 5, 6, -1, 8, 7, 4, -2],
        vec![10, 5, 6, -1, 7, 4, 11, 2, 1, 9, -2, -1, -1, -1, -1, -1],
        vec![8, 7, 4, -1, 3, 11, 6, 5, 1, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 5, 6, 7, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![8, 7, 4, -1, 6, 5, 9, 0, 11, 3, -2, -1, -1, -1, -1, -1],
        vec![4, 5, 6, 7, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 6, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 8, -1, 9, 10, 6, 4, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 6, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 6, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 6, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 8, -1, 1, 2, 4, 6, 9, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 4, 6, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 6, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 2, 3, -1, 9, 4, 10, 6, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 11, 8, -1, 9, 4, 6, 10, -2, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 11, -1, 0, 1, 4, 6, 10, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 6, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 6, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 6, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 6, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 6, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![6, 7, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 6, 7, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 6, 7, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 6, 7, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 6, 7, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 2, 3, 6, 7, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 6, 7, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 6, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -1, 10, 6, 9, 7, 8, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 6, 7, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![3, 11, 2, -1, 8, 7, 0, 1, 10, 6, -2, -1, -1, -1, -1, -1],
        vec![1, 2, 6, 7, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 6, 7, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 6, 7, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 6, 7, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![6, 7, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 0, 3, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 0, 9, 1, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 1, 3, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 1, 2, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 1, 2, 10, -1, 0, 3, 8, -2, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 0, 9, 10, 2, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 2, 3, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 6, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 6, 7, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 3, 2, 6, 7, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 6, 7, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 6, 7, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 6, 7, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 6, 7, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![6, 7, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 6, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 6, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 8, 4, 11, 6, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 6, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 8, 4, 6, 11, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 0, 3, 11, 6, 4, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 9, 10, 2, -1, 8, 4, 11, 6, -2, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 6, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 6, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 4, 6, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 2, 3, 8, 4, 6, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 6, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 6, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 6, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 6, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 6, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![6, 7, 11, -1, 4, 5, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![6, 7, 11, -1, 4, 5, 9, -1, 0, 3, 8, -2, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 1, 0, 5, 4, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 8, 3, 1, 5, 4, -2, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 4, 5, 9, -1, 1, 2, 10, -2, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 4, 5, 9, -1, 1, 2, 10, -1, 0, 3, 8, -2],
        vec![11, 7, 6, -1, 0, 2, 10, 5, 4, -2, -1, -1, -1, -1, -1, -1],
        vec![11, 7, 6, -1, 8, 3, 2, 10, 5, 4, -2, -1, -1, -1, -1, -1],
        vec![4, 5, 9, -1, 3, 2, 6, 7, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 5, 9, -1, 2, 0, 8, 7, 6, -2, -1, -1, -1, -1, -1, -1],
        vec![3, 2, 6, 7, -1, 0, 1, 5, 4, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 5, 6, 7, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![9, 4, 5, -1, 1, 10, 6, 7, 3, -2, -1, -1, -1, -1, -1, -1],
        vec![9, 4, 5, -1, 6, 10, 1, 0, 8, 7, -2, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 5, 6, 7, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 5, 6, 7, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 6, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 5, 6, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 5, 6, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 5, 6, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 9, 5, 6, 11, 8, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -1, 9, 0, 3, 11, 6, 5, -2, -1, -1, -1, -1, -1],
        vec![0, 2, 5, 6, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 5, 6, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 5, 6, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 5, 6, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 2, 3, 5, 6, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 5, 6, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 5, 6, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 5, 6, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 5, 6, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 6, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 7, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 7, 10, 11, -1, 0, 3, 8, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 7, 10, 11, -1, 0, 1, 9, -2, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 7, 10, 11, -1, 3, 8, 9, 1, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 5, 7, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 5, 7, 11, -1, 0, 3, 8, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 5, 7, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 5, 7, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 5, 7, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 5, 7, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 7, 3, 2, 10, 5, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 5, 7, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 5, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 5, 7, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 5, 7, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![5, 7, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 5, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 5, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 8, 11, 10, 4, 5, -2, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 5, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 5, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 2, 3, 4, 5, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 4, 5, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 5, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 5, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 4, 5, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -1, 8, 3, 2, 10, 5, 4, -2, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 5, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 5, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 5, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 5, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 5, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 7, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 8, -1, 10, 9, 4, 7, 11, -2, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 7, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 7, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 7, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 7, 9, 11, -1, 0, 3, 8, -2, -1, -1, -1, -1, -1],
        vec![0, 2, 4, 7, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 7, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 4, 7, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 4, 7, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 2, 3, 4, 7, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 4, 7, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 4, 7, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 4, 7, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 4, 7, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![4, 7, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![8, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 9, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 8, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 10, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 8, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 2, 3, 9, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 8, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 11, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![2, 3, 8, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 2, 9, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 2, 3, 8, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 2, 10, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![1, 3, 8, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 1, 9, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![0, 3, 8, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        vec![-2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]

    ]
}

pub fn vertex_num_table() -> Vector<usize>{
    vec![
        0, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
        1, 1, 2, 1, 2, 2, 2, 1, 2, 1, 3, 1, 2, 1, 2, 1,
        1, 2, 1, 1, 2, 3, 1, 1, 2, 2, 2, 1, 2, 2, 1, 1,
        1, 1, 1, 1, 2, 2, 1, 1, 2, 1, 2, 1, 2, 1, 1, 1,
        1, 2, 2, 2, 1, 2, 1, 1, 2, 2, 3, 2, 1, 1, 1, 1,
        2, 2, 3, 2, 2, 2, 2, 1, 3, 2, 4, 2, 2, 1, 2, 1,
        1, 2, 1, 1, 1, 2, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1,
        1, 2, 2, 2, 2, 3, 2, 2, 1, 1, 2, 1, 1, 1, 1, 1,
        1, 1, 2, 1, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1,
        2, 3, 2, 2, 3, 4, 2, 2, 2, 2, 2, 1, 2, 2, 1, 1,
        1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
        1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
        1, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0
    ]
}

pub fn corner_points() -> Vector< Vec3<f32> >{
    vec![
        Vec3::new(0.0,0.0,0.0),
        Vec3::new(1.0,0.0,0.0),
        Vec3::new(1.0,0.0,1.0),  //clockwise starting from zero y min
        Vec3::new(0.0,0.0,1.0),

        Vec3::new(0.0,1.0,0.0),
        Vec3::new(1.0,1.0,0.0), //y max
        Vec3::new(1.0,1.0,1.0),
        Vec3::new(0.0,1.0,1.0)

    ]
}


pub fn edge_pairs() -> Vector< Vec2<usize> >{
    vec![
        Vec2::new(0,1),
        Vec2::new(1,2),
        Vec2::new(3,2),
        Vec2::new(0,3),

        Vec2::new(4,5),
        Vec2::new(5,6), //5
        Vec2::new(7,6), //6
        Vec2::new(4,7),

        Vec2::new(4,0),
        Vec2::new(1,5),
        Vec2::new(2,6), //10
        Vec2::new(3,7)
    ]
}

//for each child of an octree tells which edges are outer
pub fn outer_edge_table() -> Vector<Vector<u8>>{
    vec![
        vec![0, 3, 8],
        vec![0, 1, 9],
        vec![1, 2, 10],
        vec![2, 3, 11],

        vec![4, 7, 8],
        vec![4, 5, 9],
        vec![5, 6, 10],
        vec![6, 7, 11]
    ]
}

//for each child of an octree tells which edges are inner
pub fn inner_edge_table() -> Vector<Vector<usize>>{
    vec![
        vec![1, 2, 4, 5, 6, 7, 9, 10, 11],
        vec![2, 3, 4, 5, 6, 7, 8, 10, 11],
        vec![0, 3, 4, 5, 6, 7, 8, 9, 11],
        vec![0, 1, 4, 5, 6, 7, 8, 9, 10],

        vec![0, 1, 2, 3, 5, 6, 9, 10, 11],
        vec![0, 1, 2, 3, 6, 7, 8, 10, 11],
        vec![0, 1, 2, 3, 4, 7, 8, 9, 11],
        vec![0, 1, 2, 3, 4, 5, 8, 9, 10]
    ]
}

pub fn cells_for_inner_edge_table() -> Vector<Vector<Vector<usize>>>{
    vec![
        vec![vec![], vec![1], vec![3], vec![],    vec![4], vec![1, 4, 5], vec![3, 4, 7], vec![4],    vec![], vec![1], vec![1, 2, 3], vec![3]],
        vec![vec![], vec![], vec![2], vec![0],    vec![5], vec![5], vec![2, 5, 6], vec![0, 4, 5],    vec![0], vec![], vec![2], vec![0, 2, 3]],
        vec![vec![1], vec![], vec![], vec![3],    vec![1, 5, 6], vec![6], vec![6], vec![3, 6, 7],    vec![0, 1, 3], vec![2], vec![], vec![3]],
        vec![vec![0], vec![2], vec![], vec![],    vec![0, 4, 7], vec![2, 6, 7], vec![7], vec![7],    vec![0], vec![0, 1, 2], vec![2], vec![]],

        vec![vec![0], vec![0, 1, 5], vec![0, 3, 7], vec![0],    vec![], vec![5], vec![7], vec![],    vec![], vec![5], vec![5, 6, 7], vec![7]],
        vec![vec![1], vec![1], vec![1, 2, 6], vec![0, 1, 4],    vec![], vec![], vec![6], vec![4],    vec![4], vec![], vec![6], vec![4, 6, 7]],
        vec![vec![1, 2, 5], vec![2], vec![2], vec![2, 3, 7],    vec![5], vec![], vec![], vec![7],    vec![4, 5, 7], vec![5], vec![], vec![7]],
        vec![vec![0, 3, 4], vec![2, 3, 6], vec![3], vec![3],    vec![4], vec![6], vec![], vec![],    vec![4], vec![4, 5, 6], vec![6], vec![]],
    ]
}

#[derive(Clone, Debug)]
pub struct Cell<T : Real>{
    pub densities : [T;8], //sampled densities at each corner, `i` element corresponds to `i` corner vector of `corner_points` fn.
    pub hermite_data : HashMap<usize, Plane<T>>, //binding between index of a particular edge of the cell and intersection point and normal for that edge
    pub config : usize, //identifier of this particular cell configuration, used in dual marching cubes table
}

#[derive(Clone, Debug)]
pub struct HermiteGrid<T : Real>{
    pub a : T,//length of one edge of a cubic cell
    pub size : usize, //number of cells along each axis
    pub cells : Vector<Option<Cell<T>>>, // length is size^3
    //after filling the grid each cell must be initialized (set to Some)
}


impl<T : Real + SupersetOf<f32>> HermiteGrid<T>{


    pub fn new(a : T, size : usize) -> HermiteGrid<T>{
        let cells = vec![None;(size + 1) * (size + 1) * (size + 1)];

        HermiteGrid{a,size,cells}
    }

    //cell is assumed to be initialized
    pub fn get(&self, x : usize, y : usize, z : usize) -> &Cell<T>{
        self.cells[z * self.size * self.size + y * self.size + x].as_ref().unwrap()
    }

    pub fn set(&mut self, x : usize, y : usize, z : usize, value : Cell<T>){
        let v = self.size;
        self.cells[z * v * v + y * v + x] = Some(value);
    }


    pub fn get_point(&self, x : usize, y : usize, z : usize) -> Vec3<T>{
        Vec3::new(self.a * T::from_subset(&(x as f32)), self.a * T::from_subset(&(y as f32)), self.a * T::from_subset(&(z as f32)))
    }

    //bounding box of the cell
    pub fn cube(&self, x : usize, y : usize, z : usize, offset : Vec3<T>) -> Cube<T>{
        Cube {center : offset + Vec3::new(T::from_subset(&(x as f32 + 0.5)) * self.a, T::from_subset(&(y as f32 + 0.5)) * self.a, T::from_subset(&(z as f32 + 0.5)) * self.a), extent: self.a / T::from_subset(&2.0)}
    }
}


//it is assumed that surface is smooth in the area along the line and density at the ends of the line have different signs
fn sample_surface_intersection(line : Line3<f32>, n : usize, f : impl DenFn3<f32>) -> Vec3<f32>{
    let ext = line.end - line.start;
    let norm = ext.norm();
    let dir = ext * (1.0 / norm);

    //let mut best_abs = std::f32::MAX;
    //let mut best_point : Option<Vec3<f32>> = None;

    let mut center = line.start + ext * 0.5;
    let mut cur_ext = norm * 0.25;

    for _ in 0..n {
        let point1 = center - dir * cur_ext;
        let point2 = center + dir * cur_ext;
        let den1 = f(point1).abs();
        let den2 = f(point2).abs();

        if den1 <= den2 {
            center = point1;
        }else{
            center = point2;
        }
        cur_ext *= 0.5;
    }

    center
}

pub fn sample_normal(point : Vec3<f32>, eps : f32, f : impl DenFn3<f32>) -> Vec3<f32>{
    Vec3::new( f(Vec3::new(point.x + eps, point.y, point.z)) - f(Vec3::new(point.x, point.y, point.z)),
               f(Vec3::new(point.x, point.y + eps, point.z)) - f(Vec3::new(point.x, point.y, point.z)),
               f(Vec3::new(point.x, point.y, point.z + eps)) - f(Vec3::new(point.x, point.y, point.z)) ).normalize()
}

fn is_const_sign(a : f32, b : f32) -> bool {
    if a > 0.0 { b > 0.0} else {b <= 0.0}
}


//outer list corresponds to each vertex to be placed inside the cell
//inner list binds edges according to the EMCT to that vertex
pub fn which_edges_are_signed(table : &Vector< Vector<isize> >, config : u8) -> Vector<Vector<usize>>{
    let entry = &table[config as usize];
    if entry[0] == -2 {return Vector::with_capacity(0)}
    let mut result = Vector::new();
    let mut cur_vertex = Vector::new();
    for i in 0..entry.len(){ //entry.len() is always 16
        let k = entry[i];
        if k >= 0 {cur_vertex.push(k as usize)}
        else if k == -2 {result.push(cur_vertex);return result}
        else { //k == -1
            result.push(cur_vertex);
            cur_vertex = Vector::new();
        }
    }

    result

}


fn construct_qef(planes : &Vector<Plane<f32>>, mass_point : Vec3<f32>) -> Qef<f32>{
    let mut mat : Mat<f32, U6, U4> = Mat::empty();
    //println!("planes count = {}", planes.len()); //6 planes is possible
    for i in 0..usize::min(planes.len(), 6){
        mat[(i, 0)] = planes[i].normal.x;
        mat[(i, 1)] = planes[i].normal.y;
        mat[(i, 2)] = planes[i].normal.z;
        mat[(i, 3)] = planes[i].normal.dot(planes[i].point - mass_point);
    }


    let mut tau = [0f32; 4];
    unsafe{
        sgeqrf(lapacke::Layout::RowMajor, planes.len() as i32, 4,  mat.as_mut_slice(), 4, &mut tau);
    }

    let mat_a = Mat3::new(
        mat[(0, 0)], mat[(0, 1)], mat[(0, 2)],
        0.0, mat[(1, 1)], mat[(1, 2)],
        0.0, 0.0, mat[(2, 2)]
    );


    let b = vec3![mat[(0, 3)], mat[(1, 3)], mat[(2, 3)]];
    let mut residual = mat[(3, 3)] * mat[(3, 3)];
    if planes.len() < 4{
        residual = 0.0;
    }

    Qef{
        mat_a,
        vec_b: b,
        r: residual,
        centroid : mass_point,
        vertex : None
    }
}

fn merge_qefs(qefs : &[Qef<f32>], threshold : f32) -> Option<Qef<f32>>{
    let mut mat = vec![0.0f32; 16 * qefs.len()];
    for i in 0..qefs.len(){
        mat[16*i + 0] = qefs[i].mat_a.m11;
        mat[16*i + 1] = qefs[i].mat_a.m12;
        mat[16*i + 2] = qefs[i].mat_a.m13;
        mat[16*i + 3] = qefs[i].vec_b.x;
        mat[16*i + 4] = 0.0;
        mat[16*i + 5] = qefs[i].mat_a.m22;
        mat[16*i + 6] = qefs[i].mat_a.m23;
        mat[16*i + 7] = qefs[i].vec_b.y;
        mat[16*i + 8] = 0.0;
        mat[16*i + 9] = 0.0;
        mat[16*i + 10] = qefs[i].mat_a.m33;
        mat[16*i + 11] = qefs[i].vec_b.z;
        mat[16*i + 12] = 0.0;
        mat[16*i + 13] = 0.0;
        mat[16*i + 14] = 0.0;
        mat[16*i + 15] = qefs[i].r;
    }


    let mut tau = [0f32; 4];
    unsafe{
        sgeqrf(lapacke::Layout::RowMajor, 4 * qefs.len() as i32, 4,  mat.as_mut_slice(), 4, &mut tau);
    }

    let mat_a = Mat3::new(
        mat[0], mat[1], mat[2],
        0.0, mat[5], mat[6],
        0.0, 0.0, mat[10]
    );


    let b = vec3![mat[3], mat[7], mat[11]];
    let mut residual = mat[15] * mat[15];

    if residual > threshold{
        None
    }else{

        let mut mass = Vec3::<f32>::empty();
        for q in qefs{
            mass += q.centroid;
        }

        mass *= 1.0 / (qefs.len() as f32);

        Some(Qef{
            mat_a,
            vec_b: b,
            r: residual,
            centroid : mass,
            vertex : None
        })
    }



}

fn solve_qef(qef : &mut Qef<f32>){

    if qef.vertex.is_some(){
        return;
    }

    let mut mat_a = qef.mat_a;
    let b = qef.vec_b;

    let mut eigenval = Vec3::empty();
    let mut u = Mat3::empty();
    let mut ut = Mat3::empty();
    let mut cache = [0f32; 2];
    unsafe{
        sgesvd(lapacke::Layout::RowMajor, b'A', b'A', 3, 3, mat_a.as_mut_slice(), 3, eigenval.as_mut_slice(), u.as_mut_slice(), 3, ut.as_mut_slice(), 3, &mut cache);
    }



    let truncate_eps = 0.1;
    let eigenval_mapped = Mat::<f32, U3, U1>{ar : eigenval.ar.map(|v| if v.abs() > truncate_eps {1.0/v} else {0.0})};
    let mut mat_diag : Mat3<f32> = Mat::empty();
    mat_diag[(0, 0)] = eigenval_mapped[0];
    mat_diag[(1, 1)] = eigenval_mapped[1];
    mat_diag[(2, 2)] = eigenval_mapped[2];

    let mat_inverse = ut.transpose() * mat_diag * u.transpose();

    let minimizer = mat_inverse * b + qef.centroid;

    qef.vertex = Some(minimizer);
}

fn find_minimizer_lapacke(planes : &Vector<Plane<f32>>, mass_point : Vec3<f32>) -> Vec3<f32> {
    let mut mat : Mat<f32, U6, U4> = Mat::empty();
    //println!("planes count = {}", planes.len()); //6 planes is possible
    for i in 0..usize::min(planes.len(), 6){
        mat[(i, 0)] = planes[i].normal.x;
        mat[(i, 1)] = planes[i].normal.y;
        mat[(i, 2)] = planes[i].normal.z;
        mat[(i, 3)] = planes[i].normal.dot(planes[i].point - mass_point);
    }


    let mut tau = [0f32; 4];
    unsafe{
        sgeqrf(lapacke::Layout::RowMajor, planes.len() as i32, 4,  mat.as_mut_slice(), 4, &mut tau);
    }

    let mut mat_a = Mat3::new(
        mat[(0, 0)], mat[(0, 1)], mat[(0, 2)],
        0.0, mat[(1, 1)], mat[(1, 2)],
        0.0, 0.0, mat[(2, 2)]
    );


    let b = vec3![mat[(0, 3)], mat[(1, 3)], mat[(2, 3)]];
    let mut residual = mat[(3, 3)] * mat[(3, 3)];
    if planes.len() < 4{
        residual = 0.0;
    }

    let mut eigenval = Vec3::empty();
    let mut u = Mat3::empty();
    let mut ut = Mat3::empty();
    let mut cache = [0f32; 2];
    unsafe{
        sgesvd(lapacke::Layout::RowMajor, b'A', b'A', 3, 3, mat_a.as_mut_slice(), 3, eigenval.as_mut_slice(), u.as_mut_slice(), 3, ut.as_mut_slice(), 3, &mut cache);
    }



    let truncate_eps = 0.1;
    let eigenval_mapped = Mat::<f32, U3, U1>{ar : eigenval.ar.map(|v| if v.abs() > truncate_eps {1.0/v} else {0.0})};
    let mut mat_diag : Mat3<f32> = Mat::empty();
    mat_diag[(0, 0)] = eigenval_mapped[0];
    mat_diag[(1, 1)] = eigenval_mapped[1];
    mat_diag[(2, 2)] = eigenval_mapped[2];

    let mat_inverse = ut.transpose() * mat_diag * u.transpose();

    let minimizer = mat_inverse * b + mass_point;

    minimizer
}

pub unsafe fn sample_grid(f : impl DenFn3<f32>, offset : Vec3<f32>, a : f32, size : usize, accuracy : usize, threshold : f32, render_debug_lines : &mut RendererVertFragDef<()>){

    let max_depth = (size as f32).log2() as u8;

    let edge_table = ::edge_table();

    let octree_cube = Cube{center: offset + Vec3::new(a * size as f32 / 2.0, a * size as f32 / 2.0, a * size as f32 / 2.0), extent : a * size as f32 / 2.0};

    //for densities at grid corners
    let mut signed_grid = vec![false; (size + 1) * (size + 1) * (size + 1)];

    //for grid cells (grid cubes)
    let mut grid= ar_mut::<*mut Node>(size * size * size);

    let index_density = |x : usize, y : usize, z : usize| -> usize {
        z * (size + 1) * (size + 1) + y * (size + 1) + x
    };

    fn index_cell(x : usize, y : usize, z : usize, size : usize) -> usize {
        z * size * size + y * size + x
    }

    //cube bounds of a cell in a grid
    let cube = |x : usize, y : usize, z : usize, a : f32| -> Cube<f32>{
        Cube{center: offset + vec3!((x as f32 + 0.5) * a, (y as f32 + 0.5) * a, (z as f32 + 0.5) * a), extent : a / 2.0}
    };

    let sample_grid_at = |signed_grid : &mut Vector<bool>, x : usize, y : usize, z : usize|{
        signed_grid[index_density(x, y, z)] = if f(offset + vec3!(x as f32 * a, y as f32 * a, z as f32 * a)) < 0.0 {true} else {false}
    };


    let load_cell = |signed_grid : &Vector<bool>, grid : *mut *mut Node, x : usize, y : usize, z : usize|{
        let cell_min = offset + vec3!(x as f32 * a, y as f32* a, z as f32 * a);

        let mut config : u8 = 0;

        if signed_grid[index_density(x, y, z)] {config |= 1;}
        if signed_grid[index_density(x + 1, y, z)] {config |= 2;}
        if signed_grid[index_density(x + 1, y, z + 1)] {config |= 4;}
        if signed_grid[index_density(x, y, z + 1)] {config |= 8;}
        if signed_grid[index_density(x, y + 1, z)] {config |= 16;}
        if signed_grid[index_density(x + 1, y + 1, z)] {config |= 32;}
        if signed_grid[index_density(x + 1, y + 1, z + 1)] {config |= 64;}
        if signed_grid[index_density(x, y + 1, z + 1)] {config |= 128;}

        if config == 0 { //fully outside
            let mut hom = HomogeneousNode{
                tpe: NodeType::Homo,
                depth: max_depth,
                corner_signs: 0,
                parent: null_mut()
            };
            *grid.offset(index_cell(x, y, z, size) as isize) = upcast(val_mut(hom));
        }else if config == 255{ // fully inside
            let mut hom = HomogeneousNode{
                tpe: NodeType::Homo,
                depth: max_depth,
                corner_signs: 255,
                parent: null_mut()
            };
            *grid.offset(index_cell(x, y, z, size) as isize) = upcast(val_mut(hom));
        }else{//hetero
            let vertices = which_edges_are_signed(&edge_table, config);
            let mut qefs = Vector::with_capacity(vertices.len());

            for edges in &vertices{
                let mut planes = Vector::with_capacity(edges.len());
                let mut centroid = Vec3::empty();
                for entry in edges{
                    let corners = edge_pairs()[entry.clone()];
                    let edge = Line3{start: cell_min + corner_points()[corners.x] * a, end : cell_min + corner_points()[corners.y] * a};
                    let intersection = sample_surface_intersection(edge, (accuracy as f32).log2().ceil() as usize, f);
                    let normal = sample_normal(intersection, a/1024.0, f); //TODO division by 1024 is improper for very high sizes
                    centroid += intersection;
                    planes.push(Plane{point:intersection, normal});

                }
                centroid *= 1.0 / (planes.len() as f32);
                let qef = construct_qef(&planes, centroid);
                qefs.push(qef);
            }

            let mut het = HeterogeneousNode{
                tpe: NodeType::Hetero,
                depth: max_depth,
                corner_signs: config,
                qefs,
                parent: null_mut()
            };

            *grid.offset(index_cell(x, y, z, size) as isize) = upcast(val_mut(het));
        }

    };

    fn find_child_index(child : *mut Node, parent : *mut InteriorNode<f32>) -> usize{
        unsafe{
            for i in 0..8{
                if (*parent).children[i] == child{
                    return i;
                }
            }

            0
        }
    }

    fn cluster (node : *mut Node, edge_table : &Vector<Vector<isize>>, threshold : f32){

        unsafe{
            cell_proc(node, &mut |nodes_, indices_|{

                let mut nodes = vec![nodes_[0], nodes_[1], nodes_[2], nodes_[3]];
                nodes.dedup();

                let mut indices = vec![indices_[0], indices_[1], indices_[2], indices_[3]];
                indices.dedup();

                for i in 1..4{ //parent must be the same
                    if (*nodes_[i]).parent != (*nodes_[0]).parent{
                        return;
                    }
                }


                /*if !nodes.foldl(true, |ac, node| ac && {
                    if let Some(int) = downcast::<InteriorNode<f32>>(node){
                        ac && (*int).collapsible
                    }else{
                        ac
                    }
                }){
                    if (*nodes[0]).parent != null_mut(){
                        (*downcast::<InteriorNode<f32>>((*nodes[0]).parent).unwrap()).collapsible = false;
                    }
                }*/

                let parent = downcast::<InteriorNode<f32>>((*nodes[0]).parent).unwrap();


                let mut to_be_merged : Vector<Qef<f32>> = Vector::new();
                let mut relations : Vector<u32> = Vector::new();
                let mut edges : HashSet<u8> = HashSet::new();
                let mut connected = true;
                for i in 0..nodes.len(){
                    let mut found = false;
                    if let Some(node) = downcast::<HeterogeneousNode<f32>>(nodes[i]){
                        let vertices = which_edges_are_signed(edge_table, (*node).corner_signs);
                        for k in 0..vertices.len(){
                            if vertices[k].contains(&indices[i]){
                                let qef = (*node).qefs[k].clone();
                                let child_index = find_child_index(nodes[i], parent);
                                let outer = &outer_edge_table()[child_index];
                                let rel = child_index as u32 | ((k as u32) << 3);
                                to_be_merged.push(qef);
                                relations.push(rel);
                                for o in &vertices[k]{
                                    if outer.contains(&(o.clone() as u8)){
                                        edges.insert(o.clone() as u8);
                                    }
                                }

                                found = true;
                                break;
                            }
                        }


                    }else{
                        let node = downcast::<InteriorNode<f32>>(nodes[i]).unwrap();
                        for k in 0..(*node).qefs.len(){
                            if (*node).qefs[k].2.contains(&(indices[i].clone() as u8)){
                                let qef = (*node).qefs[k].clone();
                                let child_index = find_child_index(nodes[i], parent);
                                let outer = &outer_edge_table()[child_index];
                                let rel = child_index as u32 | ((k as u32) << 3);
                                to_be_merged.push(qef.0);
                                relations.push(rel);
                                for o in &(*node).qefs[k].2{
                                    if outer.contains(o){
                                        edges.insert(o.clone());
                                    }
                                }
                                found = true;
                                break;
                            }
                        }
                    }

                    if !found{
                        connected = false;
                        break;
                    }


                }

                if connected{
                    if let Some(qef) = merge_qefs(to_be_merged.as_slice(), threshold){
                        (*parent).qefs.push((qef, relations, edges));
                    }else{
                        (*(parent as *mut InteriorNode<f32>)).collapsible = false;
                        (*(node as *mut InteriorNode<f32>)).collapsible = false;
                        let mut par = parent; //TODO find better way
                        while par != null_mut(){
                            (*par).collapsible = false;
                            par = (*par).parent as *mut InteriorNode<f32>;
                        }
                    }
                }/*else{ //TODO should not be happening
                    (*(parent as *mut InteriorNode<f32>)).collapsible = false;
                    (*(node as *mut InteriorNode<f32>)).collapsible = false;
                    let mut par = parent;
                    while par != null_mut(){
                        (*par).collapsible = false;
                        par = (*par).parent as *mut InteriorNode<f32>;
                    }
                }*/
            });

            let int = node as *mut InteriorNode<f32>;
            let mut collapsed = (*int).collapsible && !(*int).qefs.is_empty(); //TODO what ??
            for child in (*int).children.ar{
                if let Some(cint) = downcast::<InteriorNode<f32>>(child){
                    if !(*cint).collapsed{
                        collapsed = false;
                        break;
                    }
                }
            }

            (*int).collapsed = collapsed;
        }



    };


    fn cluster2 (node : *mut Node, edge_table : &Vector<Vector<isize>>, threshold : f32){

        unsafe{

            let parent = node as *mut InteriorNode<f32>;

            let edge_proc = &edge_proc_table();

            for entries in edge_proc.ar{
                let nodes = vec![(*parent).children[entries[0]], (*parent).children[entries[1]], (*parent).children[entries[2]], (*parent).children[entries[3]]];
                let indices = vec![entries[4], entries[5], entries[6], entries[7]];



                let mut to_be_merged : Vector<Qef<f32>> = Vector::new();
                let mut relations : Vector<u32> = Vector::new();
                let mut edges : HashSet<u8> = HashSet::new();
                let mut connected = true;
                for i in 0..nodes.len(){
                    let mut found = false;
                    if let Some(node) = downcast::<HeterogeneousNode<f32>>(nodes[i]){
                        let vertices = which_edges_are_signed(edge_table, (*node).corner_signs);
                        for k in 0..vertices.len(){
                            if vertices[k].contains(&indices[i]){
                                let qef = (*node).qefs[k].clone();
                                let child_index = find_child_index(nodes[i], parent);
                                let outer = &outer_edge_table()[child_index];
                                let rel = child_index as u32 | ((k as u32) << 3);
                                relations.push(rel);
                                to_be_merged.push(qef);
                                for o in &vertices[k]{
                                    if outer.contains(&(o.clone() as u8)){
                                        edges.insert(o.clone() as u8);
                                    }
                                }

                                found = true;
                                break;
                            }
                        }


                    }else if let Some(node) = downcast::<InteriorNode<f32>>(nodes[i]){
                        for k in 0..(*node).qefs.len(){
                            if (*node).qefs[k].2.contains(&(indices[i].clone() as u8)){
                                let qef = (*node).qefs[k].clone();
                                let child_index = find_child_index(nodes[i], parent);
                                let outer = &outer_edge_table()[child_index];
                                let rel = child_index as u32 | ((k as u32) << 3);
                                relations.push(rel);
                                to_be_merged.push(qef.0);
                                for o in &(*node).qefs[k].2{
                                    if outer.contains(o){
                                        edges.insert(o.clone());
                                    }
                                }
                                found = true;
                                break;
                            }
                        }
                    }

                    if !found{
                        connected = false;
                        break;
                    }


                }

                if connected{
                    if let Some(qef) = merge_qefs(to_be_merged.as_slice(), threshold){
                        (*parent).qefs.push((qef, relations, edges));
                    }else{
                        (*parent).collapsible = false;
                    }
                }
            }


            let mut collapsed = (*parent).collapsible && !(*parent).qefs.is_empty(); //TODO what ??
            for child in (*parent).children.ar{
                if let Some(cint) = downcast::<InteriorNode<f32>>(child){
                    if !(*cint).collapsed{
                        collapsed = false;
                        break;
                    }
                }
            }
            (*parent).collapsed = collapsed;
        }



    };

    fn cluster3 (node : *mut Node, edge_table : &Vector<Vector<isize>>, threshold : f32){

        unsafe{

            let parent = node as *mut InteriorNode<f32>;

            let edge_proc = &edge_proc_table();
            let inner_edge = &cells_for_inner_edge_table();

            let mut relations : Vector<Vector<(*mut Qef<f32>, Vector<(usize, Vector<usize>)>)>> = vec![Vector::with_capacity(0); 8];

            for i in 0..8{
                let child = (*parent).children[i];
                if let Some(het_child) = downcast::<HeterogeneousNode<f32>>(child){
                    let vertices = which_edges_are_signed(edge_table, (*het_child).corner_signs);
                    for k in 0..vertices.len(){
                        relations[i] = vec![(&mut (*het_child).qefs[k], Vector::new()), vertices.len()];
                        for j in 0..vertices[k].len(){
                            if !inner_edge[i][j].is_empty(){
                                relations[i][k].push((j, inner_edge[i][j].clone()));
                            }
                        }
                    }
                }
            }

            let mut qefs = Vector::new();

            for i in 0..8{
                for j in 0..relations[i].len(){
                    for k in 0..relations[i][k].1.len(){
                        let mut to_be_merged = vec![relations[i][k].1.clone()];
                        if relations[i][k].1.iter().find(|x| x < &&i).is_none(){

                        }
                    }
                }
            }

            for entries in edge_proc.ar{
                let nodes = vec![(*parent).children[entries[0]], (*parent).children[entries[1]], (*parent).children[entries[2]], (*parent).children[entries[3]]];
                let indices = vec![entries[4], entries[5], entries[6], entries[7]];



                let mut to_be_merged : Vector<Qef<f32>> = Vector::new();
                let mut relations : Vector<u32> = Vector::new();
                let mut edges : HashSet<u8> = HashSet::new();
                let mut connected = true;
                for i in 0..nodes.len(){
                    let mut found = false;
                    if let Some(node) = downcast::<HeterogeneousNode<f32>>(nodes[i]){
                        let vertices = which_edges_are_signed(edge_table, (*node).corner_signs);
                        for k in 0..vertices.len(){
                            if vertices[k].contains(&indices[i]){
                                let qef = (*node).qefs[k].clone();
                                let child_index = find_child_index(nodes[i], parent);
                                let outer = &outer_edge_table()[child_index];
                                let rel = child_index as u32 | ((k as u32) << 3);
                                relations.push(rel);
                                to_be_merged.push(qef);
                                for o in &vertices[k]{
                                    if outer.contains(&(o.clone() as u8)){
                                        edges.insert(o.clone() as u8);
                                    }
                                }

                                found = true;
                                break;
                            }
                        }


                    }else if let Some(node) = downcast::<InteriorNode<f32>>(nodes[i]){
                        for k in 0..(*node).qefs.len(){
                            if (*node).qefs[k].2.contains(&(indices[i].clone() as u8)){
                                let qef = (*node).qefs[k].clone();
                                let child_index = find_child_index(nodes[i], parent);
                                let outer = &outer_edge_table()[child_index];
                                let rel = child_index as u32 | ((k as u32) << 3);
                                relations.push(rel);
                                to_be_merged.push(qef.0);
                                for o in &(*node).qefs[k].2{
                                    if outer.contains(o){
                                        edges.insert(o.clone());
                                    }
                                }
                                found = true;
                                break;
                            }
                        }
                    }

                    if !found{
                        connected = false;
                        break;
                    }


                }

                if connected{
                    if let Some(qef) = merge_qefs(to_be_merged.as_slice(), threshold){
                        (*parent).qefs.push((qef, relations, edges));
                    }else{
                        (*parent).collapsible = false;
                    }
                }
            }


            let mut collapsed = (*parent).collapsible && !(*parent).qefs.is_empty(); //TODO what ??
            for child in (*parent).children.ar{
                if let Some(cint) = downcast::<InteriorNode<f32>>(child){
                    if !(*cint).collapsed{
                        collapsed = false;
                        break;
                    }
                }
            }
            (*parent).collapsed = collapsed;
        }



    };

    unsafe fn mk_tree (dense_grid : *mut *mut Node, size : usize, depth : usize, edge_table : &Vector<Vector<isize>>, threshold : f32) -> *mut *mut Node{
        if size == 1{
            (*(*dense_grid as *mut InteriorNode<f32>)).collapsible = false;
            return dense_grid;
        }

        let mut sparse_grid= ar_mut::<*mut Node>(size * size * size / 8);

        for i in 0..size/2{
            for j in 0..size/2{
                for k in 0..size/2{
                    let node_indices : Vec<usize, U8> = Vec::from_slice(
                        &[index_cell(2*i, 2*j, 2*k, size),
                        index_cell(2*i+1, 2*j, 2*k, size),
                        index_cell(2*i+1, 2*j, 2*k+1, size),
                        index_cell(2*i, 2*j, 2*k+1, size),
                        index_cell(2*i, 2*j+1, 2*k, size),
                        index_cell(2*i+1, 2*j+1, 2*k, size),
                        index_cell(2*i+1, 2*j+1, 2*k+1, size),
                        index_cell(2*i, 2*j+1, 2*k+1, size)]);

                    let mut ch = node_indices.map(|i| *dense_grid.offset(i as isize));

                    if ch.foldl(true, |acc, node| acc && (*node).tpe == NodeType::Homo){
                        //all homo
                        //TODO delete all child homo nodes ! currently they are leaked
                        let hom = upcast(val_mut(HomogeneousNode{
                            tpe: NodeType::Homo,
                            depth: (depth - 1) as u8,
                            corner_signs: (*ch[0]).corner_signs,
                            parent: null_mut()
                        }));
                        *sparse_grid.offset(index_cell(i, j, k, size/2) as isize) = hom;
                    }else{

                        let mut corner_signs = 0u8;
                        for i in 0..8usize{
                            corner_signs |= (*ch[i]).corner_signs & (1 << i)
                        }


                        let int = upcast(val_mut(InteriorNode::<f32>{
                            tpe: NodeType::Interior,
                            depth: (depth - 1) as u8,
                            children: ch,
                            qefs : Vector::with_capacity(4), //TODO find out real max (instead of 4)
                            collapsible:true,
                            collapsed:false,
                            parent: null_mut(),
                            corner_signs
                        }));
                        for i in 0..8usize{
                            (*ch[i]).parent = int;
                        }
                        cluster2(int, &edge_table, threshold);

                        //(*downcast::<InteriorNode<f32>>(int).unwrap()).collapsed = (*downcast::<InteriorNode<f32>>(int).unwrap()).collapsible;

                        *sparse_grid.offset(index_cell(i, j, k, size/2) as isize) = int;
                    }
                }
            }
        }

        mk_tree(sparse_grid, size/2, depth - 1, edge_table, threshold)
    };

    fn render_triangles (node : *mut Node, edge_table : &Vector<Vector<isize>>, renderer : &mut RendererVertFragDef<()>){

        let color = vec3![0.0, 1.0, 1.0];

        fn find_best_minimizer(node : *mut Node, k : usize) -> Option<Vec3<f32>>{
            unsafe{
                if (*node).parent == null_mut() || !(*((*node).parent as *mut InteriorNode<f32>)).collapsed{
                    if let Some(het) = downcast::<HeterogeneousNode<f32>>(node){
                        solve_qef(&mut (*het).qefs[k]);
                        Some((*het).qefs[k].vertex.unwrap())
                    }else{
                        let int = downcast::<InteriorNode<f32>>(node).unwrap();
                        solve_qef(&mut (*int).qefs[k].0);
                        Some((*int).qefs[k].0.vertex.unwrap())
                    }
                }else{
                    let parent = downcast::<InteriorNode<f32>>((*node).parent).unwrap();
                    let mut index: isize = -1;
                    let mut count = 0;
                    for i in 0..(*parent).qefs.len(){
                        let (qef, rel, edges) = (*parent).qefs[i].clone();
                        for j in 0..rel.len(){
                            if (*parent).children[(rel[j] & 0b111) as usize] == node && k == ((rel[j] >> 3) as usize){
                                index = i as isize;
                                count += 1;
                            }
                        }
                    }
                    if index > 1{
                        println!(">1 !!!!")
                    }
                    if index >= 0{
                       find_best_minimizer(upcast(parent), index as usize)
                    }else{
                        None
                    }
                }
            }
        }

        unsafe{
            cell_proc(node, &mut |nodes, indices|{
                if nodes[0] != nodes[1]{
                    if nodes[2] != nodes[3]{
                        let mut qefs = Vector::with_capacity(4);
                        for i in 0..4{
                            if let Some(node) = downcast::<HeterogeneousNode<f32>>(nodes[i]){
                                let vertices = which_edges_are_signed(edge_table, (*node).corner_signs);
                                for k in 0..vertices.len(){
                                    if vertices[k].contains(&indices[i]){
                                        let min = find_best_minimizer(nodes[i], k);
                                        if min.is_some(){
                                            qefs.push(min.unwrap());
                                        }
                                        /*solve_qef(&mut (*node).qefs[k]);
                                        qefs.push((*node).qefs[k].vertex.clone().unwrap());*/
                                        break;
                                    }
                                }

                            }/*else{
                                let node = downcast::<InteriorNode<f32>>(nodes[i]).unwrap();
                                for k in 0..(*node).qefs.len(){
                                    if (*node).qefs[k].2.contains(&(indices[i].clone() as u8)){
                                        solve_qef(&mut (*node).qefs[k].0);
                                        qefs.push((*node).qefs[k].0.vertex.clone().unwrap());
                                        break;
                                    }
                                    let mut count = 0;
                                    for k in 0..(*node).qefs.len(){
                                        if (*node).qefs[k].2.contains(&(indices[i].clone() as u8)){
                                            count += 1;
                                        }
                                    }
                                    if count > 1{
                                        println!("> 1 !!!")
                                    }
                                }
                            }*/
                        }
                        if qefs.len() != 4{
                            return;
                        }
                        add_triangle3_bounds_pos_color(renderer, Triangle3{
                            p1: qefs[0],
                            p2: qefs[1],
                            p3: qefs[2]
                        }, color);
                        add_triangle3_bounds_pos_color(renderer, Triangle3{
                            p1: qefs[0],
                            p2: qefs[2],
                            p3: qefs[3]
                        }, color);
                    }else{
                        let mut qefs = Vector::with_capacity(3);
                        for i in 0..3{
                            if let Some(node) = downcast::<HeterogeneousNode<f32>>(nodes[i]){
                                let vertices = which_edges_are_signed(edge_table, (*node).corner_signs);
                                for k in 0..vertices.len(){
                                    if vertices[k].contains(&indices[i]){
                                        let min = find_best_minimizer(nodes[i], k);
                                        if min.is_some(){
                                            qefs.push(min.unwrap());
                                        }
                                        break;
                                    }
                                }
                            }/*else{
                                let node = downcast::<InteriorNode<f32>>(nodes[i]).unwrap();
                                for k in 0..(*node).qefs.len(){
                                    if (*node).qefs[k].2.contains(&(indices[i].clone() as u8)){
                                        solve_qef(&mut (*node).qefs[k].0);
                                        qefs.push((*node).qefs[k].0.vertex.clone().unwrap());
                                        break;
                                    }
                                }
                            }*/
                        }
                        if qefs.len() != 3{
                            return;
                        }
                        add_triangle3_bounds_pos_color(renderer, Triangle3{
                            p1: qefs[0],
                            p2: qefs[1],
                            p3: qefs[2]
                        }, color);

                    }
                }else{
                    let mut qefs = Vector::with_capacity(3);
                    for i in 1..4{
                        if let Some(node) = downcast::<HeterogeneousNode<f32>>(nodes[i]){
                            let vertices = which_edges_are_signed(edge_table, (*node).corner_signs);
                            for k in 0..vertices.len(){
                                if vertices[k].contains(&indices[i]){
                                    let min = find_best_minimizer(nodes[i], k);
                                    if min.is_some(){
                                        qefs.push(min.unwrap());
                                    }
                                    break;
                                }
                            }

                        }/*else{
                            let node = downcast::<InteriorNode<f32>>(nodes[i]).unwrap();
                            for k in 0..(*node).qefs.len(){
                                if (*node).qefs[k].2.contains(&(indices[i].clone() as u8)){
                                    solve_qef(&mut (*node).qefs[k].0);
                                    qefs.push((*node).qefs[k].0.vertex.clone().unwrap());
                                    break;
                                }
                            }
                        }*/
                    }
                    if qefs.len() != 3{
                        return;
                    }

                    add_triangle3_bounds_pos_color(renderer, Triangle3{
                        p1: qefs[0],
                        p2: qefs[1],
                        p3: qefs[2]
                    }, color);

                }
            });
        }

    };


    unsafe fn render_debug(node : *mut Node,
                    cube: Cube<f32>, render_lines : &mut RendererVertFragDef<()>){

        let green = vec3![0.0, 1.0, 0.0];
        let red = vec3![1.0, 0.0, 0.0];
        let yellow = vec3![1.0, 1.0, 0.0];
        let white = vec3![1.0, 1.0, 1.0];

        if let Some(hom) = downcast::<HomogeneousNode>(node){
            //add_cube_bounds_pos_color(render_lines, cube, color_hom);
        }else if let Some(hom) = downcast::<HeterogeneousNode<f32>>(node){
            /*add_cube_bounds_pos_color(render_lines, cube, color_het);
            for qef in &mut (*hom).qefs{
                solve_qef(qef);
                add_cube_bounds_pos_color(render_lines, Cube{center: qef.vertex.unwrap(), extent: 1.0/128.0}, color_int);
            }*/
        }else{
            let int = downcast::<InteriorNode<f32>>(node).unwrap();
            //let col = (*int).children.foldl(true, |ac, c| ac && (*c).tpe != NodeType::Interior);

            if (*int).collapsed{
                if (*int).qefs.is_empty(){
                    add_cube_bounds_pos_color(render_lines, cube, red);
                }else{
                    add_cube_bounds_pos_color(render_lines, cube, green);
                }
                for qef in &mut (*int).qefs{
                    solve_qef(&mut qef.0);
                    add_cube_bounds_pos_color(render_lines, Cube{center: qef.0.vertex.unwrap(), extent: 1.0/256.0}, white);
                }
            }else{
                let new_ext = cube.extent / 2.0;
                for i in 0..8{
                    let new_center = cube.center - vec3![new_ext, new_ext, new_ext] + corner_points()[i] * 2.0 * new_ext; //TODO avoid copying
                    render_debug((*int).children[i], Cube{center : new_center, extent : new_ext}, render_lines)
                }
            }
        }
    }

    //TODO no need for extra triple loop for grid sampling
    for x in 0..(size+1){
        for y in 0..(size+1){
            for z in 0..(size+1){
                sample_grid_at(&mut signed_grid, x, y, z);
            }
        }
    }

    for x in 0..size{
        for y in 0..size{
            for z in 0..size{
                load_cell(&signed_grid, grid, x, y, z)
            }
        }
    }

    let mut tree = mk_tree(grid, size, max_depth as usize, &edge_table, threshold);


    //render_debug(*tree, octree_cube, render_debug_lines);
    render_triangles(*tree, &edge_table, render_debug_lines);
    //render_debug(*tree, octree_cube, render_debug_lines)
}

fn face_proc_table2() -> Vec<usize, U12> {
    Vec::from_slice(&[1, 0, 1, 0,
                      1, 0, 1, 0,
                      2, 2, 2, 2]) //face dir table
}

fn face_proc_table3() -> Vec<Vec<usize, U8>, U3>{
    vec3!(
        Vec::from_slice(&[3,2,6,7,  0,1,5,4]),
        Vec::from_slice(&[1,2,6,5,  0,3,7,4]),
        Vec::from_slice(&[7,6,5,4,  3,2,1,0])
    )
}

fn face_proc_table4() -> Vec<Vec<Vec<usize, U8>, U4>, U3>{
    vec3!(
        vec4!(
            Vec::from_slice(&[6,7,4,5,  11,10,9,8]),
            Vec::from_slice(&[3,7,4,0,  6,2,0,4]),
            Vec::from_slice(&[2,3,0,1,  11,10,9,8]),
            Vec::from_slice(&[2,6,5,1,  6,2,0,4])
        ),
        vec4!(
            Vec::from_slice(&[5,6,7,4,  10,9,8,11]),
            Vec::from_slice(&[6,2,3,7,  1,5,7,3]),
            Vec::from_slice(&[1,2,3,0,  10,9,8,11]),
            Vec::from_slice(&[5,1,0,4,  1,5,7,3])
        ),
        vec4!(
            Vec::from_slice(&[4,5,1,0,  5,7,3,1]),
            Vec::from_slice(&[7,4,0,3,  4,6,2,0]),
            Vec::from_slice(&[7,6,2,3,  5,7,3,1]),
            Vec::from_slice(&[6,5,1,2,  4,6,2,0])
        )
    )
}

fn edge_proc_table() -> Vec<Vec<usize, U8>, U6>{
    Vec{
        ar: GenericArray::clone_from_slice(
            &[
                Vec::from_slice(&[0,1,5,4, 5,7,3,1]),
                Vec::from_slice(&[5,6,2,1, 2,0,4,6]),
                Vec::from_slice(&[6,7,3,2, 3,1,5,7]),
                Vec::from_slice(&[3,0,4,7, 4,6,2,0]),
                Vec::from_slice(&[3,2,1,0, 9,8,11,10]),
                Vec::from_slice(&[7,6,5,4, 9,8,11,10])
            ]
        )
    }
}

fn edge_proc_table2() -> Vec<Vec<usize, U2>, U12>{
    Vec::from_slice(
        &[
            vec2![0, 1],
            vec2![1, 2],
            vec2![3, 2],
            vec2![0, 3],

            vec2![4, 5],
            vec2![5, 6],
            vec2![7, 6],
            vec2![4, 7],

            vec2![0, 4],
            vec2![1, 5],
            vec2![2, 6],
            vec2![3, 7]
        ]
    )
}

fn edge_proc<F : FnMut(Vec<*mut Node, U4>, Vec<usize, U4>)>(nodes : Vec<*mut Node, U4>, indices : Vec<usize, U4>, f : &mut F){
    unsafe{
        //if ((*nodes[0]).tpe == NodeType::Interior && !(*downcast::<InteriorNode<f32>>(nodes[0]).unwrap()).collapsed) || ((*nodes[1]).tpe == NodeType::Interior && !(*downcast::<InteriorNode<f32>>(nodes[1]).unwrap()).collapsed) || ((*nodes[2]).tpe == NodeType::Interior && !(*downcast::<InteriorNode<f32>>(nodes[2]).unwrap()).collapsed) || ((*nodes[3]).tpe == NodeType::Interior  && !(*downcast::<InteriorNode<f32>>(nodes[3]).unwrap()).collapsed){
        if (*nodes[0]).tpe == NodeType::Interior || (*nodes[1]).tpe == NodeType::Interior  || (*nodes[2]).tpe == NodeType::Interior || (*nodes[3]).tpe == NodeType::Interior {
            //subdivide

            let mut sub1 : Vec4<*mut Node> = vec4![null_mut(), null_mut(), null_mut(), null_mut()];
            let mut sub2 : Vec4<*mut Node> = vec4![null_mut(), null_mut(), null_mut(), null_mut()];

            for i in 0..4usize{
                if (*nodes[i]).tpe != NodeType::Interior{
                    sub1[i] = nodes[i];
                    sub2[i] = nodes[i];
                }
                else{
                    let int = downcast::<InteriorNode<f32>>(nodes[i]).unwrap();
                    //if !(*int).collapsed{
                        let p = edge_proc_table2()[indices[i]];
                        sub1[i] = (*int).children[p[0]];
                        sub2[i] = (*int).children[p[1]];
                    /*}else{
                        sub1[i] = nodes[i];
                        sub2[i] = nodes[i];
                    }*/
                }
            }

            edge_proc(Vec4::new(sub1[0], sub1[1], sub1[2], sub1[3]), indices, f);
            edge_proc(Vec4::new(sub2[0], sub2[1], sub2[2], sub2[3]), indices, f);
        }else{
            if (*nodes[0]).tpe == NodeType::Homo || (*nodes[1]).tpe == NodeType::Homo || (*nodes[2]).tpe == NodeType::Homo || (*nodes[3]).tpe == NodeType::Homo{
                return
            }else{
                f(nodes, indices)
            }

        }
    }
}

fn face_proc<F : FnMut(Vec<*mut Node, U4>, Vec<usize, U4>)>(node1: *mut Node, node2: *mut Node, dir : usize, f : &mut F){
    unsafe{
        if (*node1).tpe == NodeType::Homo || (*node2).tpe == NodeType::Homo{
            return
        }

        let n = &face_proc_table4()[dir];
        let t = face_proc_table3()[dir];

        if let Some(aint) = downcast::<InteriorNode<f32>>(node1){
            if let Some(bint) = downcast::<InteriorNode<f32>>(node2){

                /*if (*aint).collapsed{
                    if (*bint).collapsed{
                        return;
                    }else{
                        for i in 0..4usize {
                            face_proc(node1, (*bint).children[t[i + 4]], dir, f);
                            edge_proc(
                                Vec4::new(node1, node1, (*bint).children[n[i][2]], (*bint).children[n[i][3]]),
                                Vec4::new(n[i][4], n[i][5], n[i][6], n[i][7]), f
                            )
                        }
                    }
                }else{
                    if (*bint).collapsed{
                        for i in 0..4usize{
                            face_proc((*aint).children[t[i]], node2, dir, f);
                            edge_proc(
                                Vec4::new((*aint).children[n[i][0]], (*aint).children[n[i][1]], node2, node2),
                                Vec4::new(n[i][4], n[i][5], n[i][6], n[i][7]), f
                            )
                        }
                    }else{
                        for i in 0..4usize{
                            face_proc((*aint).children[t[i]], (*bint).children[t[i + 4]], dir, f);
                            edge_proc(
                                Vec4::new((*aint).children[n[i][0]], (*aint).children[n[i][1]], (*bint).children[n[i][2]], (*bint).children[n[i][3]]),
                                Vec4::new(n[i][4], n[i][5], n[i][6], n[i][7]), f
                            )
                        }
                    }
                }*/

                for i in 0..4usize{
                    face_proc((*aint).children[t[i]], (*bint).children[t[i + 4]], dir, f);
                    edge_proc(
                        Vec4::new((*aint).children[n[i][0]], (*aint).children[n[i][1]], (*bint).children[n[i][2]], (*bint).children[n[i][3]]),
                        Vec4::new(n[i][4], n[i][5], n[i][6], n[i][7]), f
                    )
                }


            }else{
                //if !(*aint).collapsed{
                    for i in 0..4usize{
                        face_proc((*aint).children[t[i]], node2, dir, f);
                        edge_proc(
                            Vec4::new((*aint).children[n[i][0]], (*aint).children[n[i][1]], node2, node2),
                            Vec4::new(n[i][4], n[i][5], n[i][6], n[i][7]), f
                        )
                    }

                //}




            }
        }else{
            if let Some(bint) = downcast::<InteriorNode<f32>>(node2) {
                /*if !(*bint).collapsed{
                    for i in 0..4usize {
                        face_proc(node1, (*bint).children[t[i + 4]], dir, f);
                        edge_proc(
                            Vec4::new(node1, node1, (*bint).children[n[i][2]], (*bint).children[n[i][3]]),
                            Vec4::new(n[i][4], n[i][5], n[i][6], n[i][7]), f
                        )
                    }
                }*/
                for i in 0..4usize {
                    face_proc(node1, (*bint).children[t[i + 4]], dir, f);
                    edge_proc(
                        Vec4::new(node1, node1, (*bint).children[n[i][2]], (*bint).children[n[i][3]]),
                        Vec4::new(n[i][4], n[i][5], n[i][6], n[i][7]), f
                    )
                }
            }

        }
    }
}


fn cell_proc<F : FnMut(Vec<*mut Node, U4>, Vec<usize, U4>)>(node : *mut Node, f : &mut F){

    unsafe{
        if let Some(int) = downcast::<InteriorNode<f32>>(node){
            if !(*int).collapsed{
                for i in 0..8{
                    cell_proc((*int).children[i], f);
                }

                for i in 0..12usize{
                    let pair = edge_proc_table2()[i];
                    let dir = face_proc_table2()[i];
                    face_proc((*int).children[pair[0]], (*int).children[pair[1]], dir, f)
                }

                for i in 0..6usize{
                    let t = edge_proc_table()[i];
                    edge_proc(Vec4::new((*int).children[t[0]], (*int).children[t[1]], (*int).children[t[2]], (*int).children[t[3]]), Vec4::new(t[4], t[5], t[6], t[7]), f)
                }
            }
        }
    }

}
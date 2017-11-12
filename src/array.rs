#![feature(tuple_indexing)]
extern crate num;
extern crate generic_array;

use generic_array::*;
use std::fmt::{Display, Formatter, Result};
use typenum::*;
use std::ops::*;
use std::num::*;
use self::num::Zero;


//newtype for GenericArray, zero runtime cost
pub struct Array<T,N : ArrayLength<T>>(pub GenericArray<T,N>);

impl<T, N : ArrayLength<T>> Array<T,N>{
    pub fn new(a : GenericArray<T,N>) -> Array<T,N>{
        Array(a)
    }

    pub fn get(&self) -> &GenericArray<T,N>{
        &self.0
    }
}


impl<T : Clone,N : ArrayLength<T>> Clone for Array<T,N>{

    fn clone(&self) -> Array<T,N>{
        Array::new((&self.0).clone())
    }
}

impl<T : Display, N : ArrayLength<T>> Display for Array<T,N>{
    fn fmt(&self, f : &mut Formatter) -> Result{
        write!(f, "[");
        for i in 0..N::to_usize(){
            if i != N::to_usize() - 1{
                write!(f, "{}, ", self.get()[i]);
            }
            else{
                write!(f, "{}", self.get()[i]);
            }
        }
        write!(f, "]")
    }
}

impl<T : Display,N> Array<T, N> where N : ArrayLength<T>{
    pub fn print(&self){
      print!("[");
      for i in 0..N::to_usize(){
          if i != N::to_usize() - 1{
              print!("{}, ", self.get()[i]);
          }
          else{
              print!("{}", self.get()[i]);
          }
      }
      println!("]");
   }
}

impl<'a,
     'b,
     T : Add<Output=T> + Copy,
     N : ArrayLength<T>>
    
    Add<& 'b Array<T,N>>
    
for & 'a Array<T,N>{
    
    type Output = Array<T,N>;

    fn add(self, other : & 'b Array<T,N>) -> Array<T,N>{
        Array(GenericArray::<T,N>::
              generate(&|i| self.get()[i] + other.get()[i]))
    }

}

impl<'a,
     'b,
     T : Sub<Output=T> + Copy,
     N : ArrayLength<T>>
    
    Sub<& 'b Array<T,N>>

for & 'a Array<T,N>{
    
    type Output = Array<T,N>;

    fn sub(self, other : & 'b Array<T,N>) -> Array<T,N>{
        Array(GenericArray::<T,N>::
              generate(&|i| self.get()[i] - other.get()[i]))
    } 
}



impl<'a,
     'b,
     T : Mul<Output=T> + Add<Output=T> + Copy + Zero,
     N : ArrayLength<T>>
    
    Mul<& 'b Array<T,N>>
    
for & 'a Array<T,N>{
    
    type Output = T;

    fn mul(self, other : & 'b Array<T,N>) -> T{

        let mut sum : T = <T as Zero>::zero();
        for i in 0.. <N>::to_i32(){
            sum = sum + self.get()[i as usize] * other.get()[i as usize];
        };

        sum
    } 
}






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
//'Vector' is mathematical vector
pub struct Vector<T,N : ArrayLength<T>>(pub GenericArray<T,N>);


//Not possible :( , so the above newtype ^^ is required
/*impl<T, N : ArrayLength<T>> GenericArray<T,N>{
    pub fn test(&self){
        println!("this is test !");
    }
}*/

impl<T, N : ArrayLength<T>> Vector<T,N>{
    pub fn new(a : GenericArray<T,N>) -> Vector<T,N>{
        Vector(a)
    }

    pub fn get(&self) -> &GenericArray<T,N>{
        &self.0
    }
}


impl<T : Clone,N : ArrayLength<T>> Clone for Vector<T,N>{

    fn clone(&self) -> Vector<T,N>{
        Vector::new((&self.0).clone())
    }
}

impl<T : Display, N : ArrayLength<T>> Display for Vector<T,N>{
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

impl<T : Display,N> Vector<T, N> where N : ArrayLength<T>{
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
    
    Add<& 'b Vector<T,N>>
    
for & 'a Vector<T,N>{
    
    type Output = Vector<T,N>;

    fn add(self, other : & 'b Vector<T,N>) -> Vector<T,N>{
        Vector(GenericArray::<T,N>::
              generate(&|i| self.get()[i] + other.get()[i]))
    }

}

impl<'a,
     'b,
     T : Sub<Output=T> + Copy,
     N : ArrayLength<T>>
    
    Sub<& 'b Vector<T,N>>

for & 'a Vector<T,N>{
    
    type Output = Vector<T,N>;

    fn sub(self, other : & 'b Vector<T,N>) -> Vector<T,N>{
        Vector(GenericArray::<T,N>::
              generate(&|i| self.get()[i] - other.get()[i]))
    } 
}



impl<'a,
     'b,
     T : Mul<Output=T> + Add<Output=T> + Copy + Zero,
     N : ArrayLength<T>>
    
    Mul<& 'b Vector<T,N>>
    
for & 'a Vector<T,N>{
    
    type Output = T;

    fn mul(self, other : & 'b Vector<T,N>) -> T{

        let mut sum : T = <T as Zero>::zero();
        for i in 0.. <N>::to_i32(){
            sum = sum + self.get()[i as usize] * other.get()[i as usize];
        };

        sum
    } 
}






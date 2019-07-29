use std;
use na::*;
use math::*;
use renderer::*;
use alga::general::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use qef_bindings::*;

//uniform manifold dual contouring is a modification to dual marching cubes (hermite extension to dual marching cubes)

//dual marching cubes (modification, by Nielson, to original marching cubes)
//taken from:
//https://stackoverflow.com/questions/16638711/dual-marching-cubes-table

//original work:
//https://vis.computer.org/vis2004/DVD/vis/papers/nielson2.pdf

//256 x 16
pub fn edge_table() -> Vec< Vec< isize > >{
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

pub fn vertex_num_table() -> Vec<usize>{
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

pub fn corner_points() -> Vec< Vector3<f32> >{
    vec![
        Vector3::new(0.0,0.0,0.0),
        Vector3::new(1.0,0.0,0.0),
        Vector3::new(1.0,0.0,1.0),  //clockwise starting from zero y min
        Vector3::new(0.0,0.0,1.0),

        Vector3::new(0.0,1.0,0.0),
        Vector3::new(1.0,1.0,0.0), //y max
        Vector3::new(1.0,1.0,1.0),
        Vector3::new(0.0,1.0,1.0)

    ]
}


pub fn edge_pairs() -> Vec < Vector2<usize> >{
    vec![
        Vector2::new(0,1),
        Vector2::new(1,2),
        Vector2::new(3,2),
        Vector2::new(0,3),

        Vector2::new(4,5), 
        Vector2::new(5,6), //5
        Vector2::new(7,6), //6
        Vector2::new(4,7), 

        Vector2::new(4,0),
        Vector2::new(1,5), 
        Vector2::new(2,6), //10
        Vector2::new(3,7)  
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
    pub cells : Vec<Option<Cell<T>>>, // length is size^3
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


    pub fn get_point(&self, x : usize, y : usize, z : usize) -> Vector3<T>{
        Vector3::new(self.a * convert::<f32, T>(x as f32), self.a * convert::<f32, T>(y as f32), self.a * convert::<f32, T>(z as f32))
    }

    //bounding box of the cell
    pub fn cube(&self, x : usize, y : usize, z : usize, offset : Vector3<T>) -> Cube<T>{
        Cube {center : offset + Vector3::new(convert::<f32,T>(x as f32 + 0.5) * self.a, convert::<f32,T>(y as f32 + 0.5) * self.a, convert::<f32,T>(z as f32 + 0.5) * self.a), extent: self.a / convert(2.0)}
    }
}


//it is assumed that surface is smooth in the area along the line and density at the ends of the line have different signs
//TODO handle multiple intersections per edge ???
fn sample_surface_intersection(line : &Line3<f32>, n : usize, f : &DenFn3<f32>) -> Vector3<f32>{
    let ext = line.end - line.start;
    let norm = ext.norm();
    let dir = ext / norm;

    //let mut best_abs = std::f32::MAX;
    //let mut best_point : Option<Vector3<f32>> = None;

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

pub fn sample_normal(point : &Vector3<f32>, eps : f32, f : &DenFn3<f32>) -> Vector3<f32>{
    Vector3::new( f(Vector3::new(point.x + eps, point.y, point.z)) - f(Vector3::new(point.x, point.y, point.z)),
                  f(Vector3::new(point.x, point.y + eps, point.z)) - f(Vector3::new(point.x, point.y, point.z)),
                  f(Vector3::new(point.x, point.y, point.z + eps)) - f(Vector3::new(point.x, point.y, point.z)) ).normalize()
}

fn is_const_sign(a : f32, b : f32) -> bool {
    if a > 0.0 { b > 0.0} else {b <= 0.0}
}

//outer list corresponds to each vertex to be placed inside the cell
//inner list binds edges according to the EMCT to that vertex 
pub fn which_edges_are_signed(table : &Vec< Vec<isize> >, config : usize) -> Vec<Vec<usize>>{
    let entry = &table[config];
    if entry[0] == -2 {return Vec::with_capacity(0)}
    let mut result = Vec::new(); 
    let mut cur_vertex = Vec::new();
    for i in 0..entry.len(){ //entry.len() is always 16
        let k = entry[i];
        if k >= 0 {cur_vertex.push(k as usize)}
        else if k == -2 {result.push(cur_vertex);return result}
        else { //k == -1
            result.push(cur_vertex);
            cur_vertex = Vec::new();
        }
    }

    result

}


fn calc_qef(point : &Vector3<f32>, planes : &Vec<Plane<f32>>) -> f32{
    let mut qef : f32 = 0.0;
    for plane in planes{
        let dist_signed = plane.normal.dot(&(point - plane.point));
        qef += dist_signed * dist_signed;
    }

    qef
}



//works bad
//try delta approuch
//start from the center then find a direction in which qef increases most and move a bit along it
fn solve_qef_iterative(square : &Cube<f32>, threshold : f32, planes : &Vec<Plane<f32>>) -> Vector3<f32>{

    let mut vertex = square.center;
    let mut next_iter = vertex;
    

    while threshold < calc_qef(&vertex, planes){
        let mut qef : f32 = 0.0; //TODO
        for plane in planes{
            let dist_signed = plane.normal.dot(&(vertex - plane.point));
            qef += dist_signed * dist_signed;
            next_iter += plane.normal * dist_signed;
        }

        vertex = next_iter / (planes.len() as f32) * 0.7;
        next_iter = vertex;
    }

    vertex
}

//works but the error is too great
fn solve_qef_analically_ATA_ATb(planes : &Vec<Plane<f32>>) -> Option<Vector3<f32>>{
    let normals : Vec<f32> = planes.iter().flat_map(|x| x.normal.as_slice().to_owned()).collect();
    let mut Abs = Vec::with_capacity(normals.len() * 4 / 3);
    //let intersections : Vec<f32> = planes.iter().flat_map(|x| x.point.as_slice().to_owned()).collect();
    let product : Vec<f32> = planes.iter().map(|x| x.normal.dot(&x.point)).collect();
    for i in 0..product.len(){
        Abs.push(normals[3 * i]);
        Abs.push(normals[3 * i + 1]);
        Abs.push(normals[3 * i + 2]);
        Abs.push(product[i]);
    }

    let A = DMatrix::from_row_slice(normals.len() / 3, 3, normals.as_slice());
    let ATA = (&A).transpose() * &A;
    let b = DMatrix::from_row_slice(product.len(), 1, product.as_slice());
    let ATb = (&A).transpose() * &b; 
    let Ab = DMatrix::from_row_slice(planes.len(), 4, Abs.as_slice());

    let bTb = (&b).transpose() * (&b);
    let mag = bTb.norm();

    let qr = ATA.qr();
    let solved = qr.solve(&ATb);
    if solved.is_some(){
        Some(Vector3::new(solved.as_ref().unwrap()[0], solved.as_ref().unwrap()[1], solved.as_ref().unwrap()[2]))
    }else{
        None
    }
}

fn solve_qef_analically_qr(planes : &Vec<Plane<f32>>, bounds : &Cube<f32>) -> Vector3<f32>{
    let mut masspoint = Vector4::zeros();
    let normals : Vec<f32> = planes.iter().flat_map(|x| {
        masspoint += Vector4::new(x.point.x, x.point.y, x.point.z, 1.0);
        x.normal.as_slice().to_owned()
    }).collect();
    let mut Abs = Vec::with_capacity(normals.len() * 4 / 3);
    //let intersections : Vec<f32> = planes.iter().flat_map(|x| x.point.as_slice().to_owned()).collect();
    let product : Vec<f32> = planes.iter().map(|x| x.normal.dot(&x.point)).collect();
    for i in 0..product.len(){
        Abs.push(normals[3 * i]);
        Abs.push(normals[3 * i + 1]);
        Abs.push(normals[3 * i + 2]);
        Abs.push(product[i]);
    }

    let Ab = DMatrix::from_row_slice(planes.len(), 4, Abs.as_slice());


    let qr1 = Ab.qr();
    let R = qr1.r();

    //println!("R : {}", &R);

    let A1 = R.slice((0,0), (3,3));
    let b1 = R.slice((0,3), (3,1));
    let a1 = A1.fixed_slice::<U3,U3>(0,0);

    //println!("A1 : {}", &A1);
    //println!("b1 : {}", &b1);

    let qr2 = A1.qr();

    //println!("{}", a1.determinant().abs());
    //let det = a1.determinant().abs();
    let try = qr2.solve(&Vector3::new(b1[0], b1[1], b1[2]));
    let solution = 
        match try{
            Some(min) => {
                if point3_inside_sphere_inclusive(&min, Sphere{center : bounds.center, rad : 3.0.sqrt() * bounds.extent * 2.0}){
                    min
                }else{
                    let temp = (masspoint / masspoint.w);
                    Vector3::new(temp.x, temp.y, temp.z)
                }
            },
            None => {
                let temp = (masspoint / masspoint.w);
                Vector3::new(temp.x, temp.y, temp.z)
            }
    };

    //let mut r = unsafe{R.get_unchecked(3,3)};
    //let svd = A1.svd(true,true); 
    //let sol = svd.solve(&b1,0.0); //does not want to work

    //Some(Vector3::new(sol[0], sol[1], sol[2]))  

    solution

  
}

//minimizer + error
fn solve_qef_via_bindings(planes : &Vec<Plane<f32>>) -> (Vector3<f32>,f32) {
    let mut ATA = Matrix3::zeros();
    let mut ATb = Vector3::zeros();

    let mut accum = Vector4::zeros();

    for plane in planes{
        qef_add_r(plane.normal, plane.point, &mut ATA, &mut ATb, &mut accum);
    }

    let mut res = Vector3::zeros();

    //println!("{}", &ATb);
    //println!("{}", &ATA);
    //println!("accum {}", &accum);
    let err = qef_solve_r(ATA, ATb, accum, &mut res);
    //println!("result is {}", &res);
    (res, err)
}

fn sample_qef_brute(square : &Cube<f32>, n : usize, planes : &Vec<Plane<f32>>) -> Vector3<f32> {
    let ext = Vector3::new(square.extent, square.extent, square.extent);
    let min = square.center - ext;

    let mut best_qef = std::f32::MAX;
    let mut best_point = min;

    for i in 0..n{
        for j in 0..n{
            for k in 0..n{
                let point = min + Vector3::new(ext.x * (2.0 * (i as f32) + 1.0) / (n as f32),
                                               ext.y * (2.0 * (j as f32) + 1.0) / (n as f32),
                                               ext.z * (2.0 * (k as f32) + 1.0) / (n as f32));
                let qef = calc_qef(&point, &planes);

                if qef < best_qef{
                    best_qef = qef;
                    best_point = point;
                }
            }
        }
    }

    best_point
}


//constructs grid: calculates hermite data and configuration for each cell
//TODO generating triangles right in this function would benefit performance (no extra looping through cells)
pub fn construct_grid<'f>(f : &'f DenFn3<f32>, offset : Vector3<f32>, a : f32, size : usize, accuracy : usize, render_tr_light : &mut RendererVertFragDef, render_debug_lines : &mut RendererVertFragDef) -> HermiteGrid<f32>{
    let corners = corner_points();
    let edge_pairs = edge_pairs();
    let edge_table = edge_table();

    //bindings between edge and vertex for each cell
    let mut cache : Vec< Option< HashMap<usize, Vector3<f32>  > > > = vec![None;size * size * size];

    let mut load_cell = |grid : &mut HermiteGrid<f32>, x : usize, y : usize, z : usize, cache : &mut Vec<Option<HashMap<usize,Vector3<f32>>>>|{
        let cell_min = offset + Vector3::new(x as f32 * a, y as f32 * a, z as f32 * a);
        let bounds = grid.cube(x,y,z,offset);
        let mut densities = [0.0;8];
        let mut config = 0;
        let mut corner_vertex_count = 0;
        for i in 0..8{
            let p = cell_min + corners[i] * a;
            densities[i] = f(p);
            if densities[i] < 0.0{
                config |= 1 << i;
                corner_vertex_count += 1;
            }
        }

        let vertices = which_edges_are_signed(&edge_table, config);

        let mut hermite_data = HashMap::new();

        let mut cached_cell = HashMap::new();

        if vertices.len() >= 1 { //render cells that contain more than 1 vertex
            //add_square3_bounds_color(render_debug_lines, bounds.clone(), Vector3::new(1.0,0.0,0.0));
        }

        for vertex in vertices{

            let mut cur_planes = Vec::with_capacity(vertex.len());

            

            for edge_id in &vertex{
                let pair = edge_pairs[edge_id.clone()];
                let v1 = corners[pair.x];
                let v2 = corners[pair.y];

                let edge = Line3{start : cell_min + v1 * a, end : cell_min + v2 * a};

                let intersection = sample_surface_intersection(&edge, accuracy, f);
                
                let normal = sample_normal(&intersection, 1e-5, f);
                

                let plane = Plane{point : intersection, normal};
                hermite_data.insert(edge_id.clone(), plane);
                cur_planes.push(plane); //for current vertex QEF processing
            }


            let is_valid_qef_estimation = |minimizer : &Vector3<f32>| -> bool{
                point3_inside_sphere_inclusive(minimizer, Sphere{center : bounds.center, rad : 3.0.sqrt() * bounds.extent * 3.1})
            };
           
            //let minimizer_opt = solve_qef_analically_ATA_ATb(&cur_planes);
            //let minimizer = if minimizer_opt.is_some() {minimizer_opt.unwrap()} else {bounds.center};
            //let minimizer = sample_qef_brute(&bounds, 32, &cur_planes);
            // let minimizer = if(corner_vertex_count > 1){
            //     let try = solve_qef_via_bindings(&cur_planes);
            //     let minimizer = 
            //         if !is_valid_qef_estimation(&try.0){
            //             println!("bad minimizer {}", &try.0);
            //             use rand;
            //             use rand::Rng;
            //             use rand::distributions::{Sample, Range};
            //             let mut rng = rand::thread_rng();
            //             let mut between = Range::new(0.0, 1.0);
            //             let r = between.sample(&mut rng);
            //             let g = between.sample(&mut rng);
            //             let b = between.sample(&mut rng);

            //             add_square3_bounds_color(render_debug_lines, bounds.clone(), Vector3::new(r,g,b));
            //             add_square3_bounds_color(render_debug_lines, Square3{center : try.0, extent : 0.075/4.0}, Vector3::new(r,g,b));
            //             add_line3_color(render_debug_lines, Line3{start : bounds.center, end : try.0}, Vector3::new(r,g,b));

            //             for plane in &cur_planes{
            //                 add_square3_bounds_color(render_debug_lines, Square3{center : plane.point, extent : 0.075/4.0}, Vector3::new(r,g,b));
            //                 add_line3_color(render_debug_lines, Line3{start : plane.point, end : plane.point + plane.normal * (0.075)}, Vector3::new(r,g,b));
            //             }

            //             try.0
            //         }else{
            //             try.0
            //         };

            //     minimizer
            // }else{
            //     println!("sampled");
            //     sample_qef_brute(&bounds, 32, &cur_planes)
            // };
            let minimizer = solve_qef_analically_qr(&cur_planes, &bounds);

            // if !is_valid_qef_estimation(&minimizer){
            //     println!("bad minimizer {}, det {}, err {}", &minimizer, try.1, calc_qef(&minimizer, &cur_planes));
            //     use rand;
            //     use rand::Rng;
            //     use rand::distributions::{Sample, Range};
            //     let mut rng = rand::thread_rng();
            //     let mut between = Range::new(0.0, 1.0);
            //     let r = between.sample(&mut rng);
            //     let g = between.sample(&mut rng);
            //     let b = between.sample(&mut rng);

            //     add_square3_bounds_color(render_debug_lines, bounds.clone(), Vector3::new(r,g,b));
            //     add_square3_bounds_color(render_debug_lines, Square3{center : minimizer, extent : 0.075/4.0}, Vector3::new(r,g,b));
            //     add_line3_color(render_debug_lines, Line3{start : bounds.center, end : minimizer}, Vector3::new(r,g,b));

            //     for plane in &cur_planes{
            //         add_square3_bounds_color(render_debug_lines, Square3{center : plane.point, extent : 0.075/4.0}, Vector3::new(r,g,b));
            //         add_line3_color(render_debug_lines, Line3{start : plane.point, end : plane.point + plane.normal * (0.075)}, Vector3::new(r,g,b));
            //     }

            // }

            //add_square3_bounds_color(render_debug_lines, Square3{center : minimizer, extent : 0.075/4.0}, Vector3::new(1.0,1.0,0.0));

            for edge_id in &vertex { 
                cached_cell.insert(edge_id.clone(), minimizer);//duplicates are not possible
            }

        }

        let t = z * size * size + y * size + x;
        cache[t] = Some(cached_cell);

        let cell = Cell{densities, hermite_data, config};

        grid.set(x, y, z, cell);

    };

    let mut load_cell_cached = |grid : &mut HermiteGrid<f32>, x : usize, y : usize, z : usize, cache : &mut Vec<Option<HashMap<usize,Vector3<f32>>>>|{
        let t = z * size * size + y * size + x;

        let mut load = false;
        {
            let cached = cache[t].as_ref();
            match cached{
                None => load = true,
                Some(_) => (),
            }
        }

        let cached = {
            if load{
                load_cell(grid, x, y, z, cache);
            };
            cache[t].as_ref().unwrap()
        };

        cached.clone() //TODO cloning is bad here
        
    };

    let mut grid = HermiteGrid::new(a, size);
    for y in 0..size-1{
        for z in 0..size-1{
            for x in 0..size-1{

                let cell = load_cell_cached(&mut grid, x,y,z, &mut cache);
                for (edge_id, minimizer) in &cell{
                    let t = minimizer.clone();
                    match edge_id.clone(){ //TODO add triangle vertex only once, use indexing + culling (decide direction by normal)
                        5 => {
                            let r = load_cell_cached(&mut grid, x+1,y,z, &mut cache).get(&7).unwrap().clone();
                            let ru = load_cell_cached(&mut grid, x+1,y+1,z, &mut cache).get(&3).unwrap().clone();
                            let u = load_cell_cached(&mut grid, x,y+1,z, &mut cache).get(&1).unwrap().clone();
                            let normal = &grid.get(x,y,z).hermite_data.get(&5).unwrap().normal;
                            add_triangle_color_normal(render_tr_light, &Triangle3{p1 : t, p2 : r, p3 : ru}, &Vector3::new(1.0, 1.0, 0.0), normal);
                            add_triangle_color_normal(render_tr_light, &Triangle3{p1 : t, p2 : ru, p3 : u}, &Vector3::new(1.0, 1.0, 0.0), normal);
                        },
                        6 => {
                            let f = load_cell_cached(&mut grid, x,y,z+1, &mut cache).get(&4).unwrap().clone();
                            let fu_ = load_cell_cached(&mut grid, x,y+1,z+1, &mut cache);
                            // let config = grid.get(x,y+1,z+1).config;
                            // println!("vertex count {:?}, edges: {:?}, map : {:?}", vertex_num_table()[config], edge_table[config], &fu_);
                            let fu = fu_.get(&0).unwrap().clone();
                            let u = load_cell_cached(&mut grid, x,y+1,z, &mut cache).get(&2).unwrap().clone();
                            let normal = &grid.get(x,y,z).hermite_data.get(&6).unwrap().normal;
                            add_triangle_color_normal(render_tr_light, &Triangle3{p1 : t, p2 : f, p3 : fu}, &Vector3::new(1.0, 1.0, 0.0), normal);
                            add_triangle_color_normal(render_tr_light, &Triangle3{p1 : t, p2 : fu, p3 : u}, &Vector3::new(1.0, 1.0, 0.0), normal);
                        },
                        10 => {
                            let r_ = load_cell_cached(&mut grid, x+1,y,z, &mut cache);
                            //let config = grid.get(x+1,y,z).config;
                            //let config_this = grid.get(x,y,z).config;
                            //println!("this {:?}, errored {:?}", &edge_table[config_this], &edge_table[config]);
                            let r = r_.get(&11).unwrap().clone();
                            let rf = load_cell_cached(&mut grid, x+1,y,z+1, &mut cache).get(&8).unwrap().clone();
                            let f = load_cell_cached(&mut grid, x,y,z+1, &mut cache).get(&9).unwrap().clone();

                            let normal = &grid.get(x,y,z).hermite_data.get(&10).unwrap().normal;

                            add_triangle_color_normal(render_tr_light, &Triangle3{p1 : t, p2 : rf, p3 : r}, &Vector3::new(1.0, 1.0, 0.0), normal);
                            add_triangle_color_normal(render_tr_light, &Triangle3{p1 : t, p2 : f, p3 : rf}, &Vector3::new(1.0, 1.0, 0.0), normal);
                        },
                        _ => ()
                    }
                }
                
            }
        }
    }

    grid
}
use std;
use na::*;
use math::*;
use renderer::*;
use alga::general::*;
use std::rc::Rc;
use std::cell::RefCell;
 use num::PrimInt;

pub struct Octree<T> {
    parent : Option<Rc<RefCell<Octree<T>>>>,
    children : Option <  Vec  <  Option< Rc<RefCell<Octree<T>>> >  >  >, //None or always of size 8
    status : u8, //which children are constructed
    //hetero : u8, //which children are heterogeneous leaves

    pub data: Option<T>,
    pub index : usize,
}

pub struct NodeId(usize);



#[derive(Clone, Debug)]
pub struct VoxelData<T : Copy + PartialEq + std::fmt::Debug> where T : 'static{
    pub densities : [T;8],
    //pub planes : Vec<Plane<T>>,
}

impl<T : Real + Identity<Additive>> VoxelData<T>{
    pub fn is_heterogeneous(&self) -> bool{
        let sign = self.densities[0];
        for i in 1..8{
            if self.densities[i] * sign <= T::zero() {return true}
        }
        false
    }
}


//a - size of leaf
//size - num of leaves in each axis direction
//lev cur level from top

//starting from less significant bits
pub fn count_leading_non_zeros_oct(x : usize, acc : usize) -> usize {
    let rem = x % 9;
    if rem == 0 {return acc}
    else{
        count_leading_non_zeros_oct(x / 9, acc + 1)
    }
}

pub fn index_to_point3(offset : Vector3<f32>, index : usize, a : f32, level : usize, max_level : usize) -> Vector3<f32>{
    use std::string::ToString;
    if index == 0 {return offset}
    else{
        let cur = index & 0b1111;  //100010100010001000001
        //println!("max {}, lev {}, index {:b}, cur {:b}", max_level, level, index, cur);
        let t = 2.pow( (max_level - level) as u32 );
        let new_i = index >> 4;
         match cur{
            1 => index_to_point3(offset + Vector3::new(0.0, 0.0, a * t as f32), new_i, a, level - 1, max_level),
            2 => index_to_point3(offset + Vector3::new(a * t as f32, 0.0, a * t as f32), new_i, a, level - 1, max_level),
            3 => index_to_point3(offset + Vector3::new(a * t as f32, 0.0, 0.0), new_i, a, level - 1, max_level),
            4 => index_to_point3(offset, new_i, a, level - 1, max_level),

            5 => index_to_point3(offset + Vector3::new(0.0, a * t as f32, a * t as f32), new_i, a, level - 1, max_level),
            6 => index_to_point3(offset + Vector3::new(a * t as f32, a * t as f32, a * t as f32), new_i, a, level - 1, max_level),
            7 => index_to_point3(offset + Vector3::new(a * t as f32, a * t as f32, 0.0), new_i, a, level - 1, max_level),
            8 => index_to_point3(offset + Vector3::new(0.0, a * t as f32, 0.0) ,new_i, a, level - 1, max_level),
            _ => panic!(String::from("impossible happend: ") + &cur.to_string())
        }
    }
    
}

//index is in range: 1-8 including both
pub fn fill_in_leaves(f : &DenFn3<f32>, offset : Vector3<f32>, a : f32, size : usize, lev : usize, index : usize, parent : Rc<RefCell<Octree<VoxelData<f32>>>>,
    debug_render_lines : &mut RendererVertFragDef){
    
    let max_level = (size as f32).log2() as usize; //TODO test

    if lev < max_level{

        let parent_index = parent.borrow().index;

        let this_index = {
            let c = 4 * (lev - 1);
                //println!("count leading non zeros {} {}, max level : {}", c, parent_index, max_level);
            parent_index | (index << c)
        };

        //println!("this index1 {:b} at level {}", this_index, lev);

        let tree = Rc::new(RefCell::new(Octree{parent : Some(parent.clone()), children : None, status : 0, index : this_index, data : None}));

        let mut all_homo = true;
        for i in 0..8{
            fill_in_leaves(f, offset, a, size, lev + 1, i + 1, tree.clone(), debug_render_lines);
            if all_homo{
                let maybe_children = &tree.borrow().children;
                let children =  &maybe_children.as_ref().unwrap();
                let child_i = children[i].as_ref().unwrap();
                let data_for_child = &child_i.borrow().data; //child `i` is present at this moment
                if data_for_child.is_some() && data_for_child.as_ref().unwrap().is_heterogeneous(){all_homo = false;}
            }
        }

        if all_homo{
            tree.borrow_mut().children = None;
            tree.borrow_mut().status = 0;
            tree.borrow_mut().data = None; //unite 8 children into one tree
        }else{//this is he
            let pos = index_to_point3(offset, this_index, a, lev, max_level);
            let a_for_level = 2.pow( (max_level - lev) as u32 ) as f32 * a;
            add_square3_bounds_color(debug_render_lines, Square3{center : pos + Vector3::new(a_for_level, a_for_level, a_for_level) / 2.0, extent : a_for_level / 2.0}, Vector3::new(1.0,1.0,1.0) );
        }


        if parent.borrow().children.is_none(){
            let mut n = vec![None;8];
            n[index-1] = Some(tree);
            parent.borrow_mut().children = Some(n);
        }else{
            parent.borrow_mut().children.as_mut().unwrap()[index-1] = Some(tree)
        }

        

        let p_stat = parent.borrow().status;
        parent.borrow_mut().status = p_stat | (1 << (index-1));


    }else{
        
        let parent_index = parent.borrow().index;
        let c = 4 * (lev - 1);
        let this_index = parent_index | (index << c);
        //println!("this index2 {:b} at level {}", this_index, lev);
        let pos = index_to_point3(offset, this_index, a, lev, max_level);

        let d00 = f(Vector3::new(0.0, 0.0, a) + pos);
        let d01 = f(Vector3::new(a,0.0,a) + pos);
        let d02 = f(Vector3::new(a,0.0,0.0) + pos);
        let d03 = f(Vector3::new(0.0,0.0,0.0) + pos);

        let d10 = f(Vector3::new(0.0, a, a) + pos);
        let d11 = f(Vector3::new(a,a,a) + pos);
        let d12 = f(Vector3::new(a,a,0.0) + pos);
        let d13 = f(Vector3::new(0.0,a,0.0) + pos);

        let voxel_data = VoxelData{densities : [d00,d01,d02,d03,  d10,d11,d12,d13]};

        let this = Octree{parent : Some(parent.clone()), children : None, status : 0, index : this_index, data : Some(voxel_data)};


        if parent.borrow().children.is_none(){
            let mut n = vec![None;8];
            n[index-1] = Some(Rc::new(RefCell::new(this)));
            parent.borrow_mut().children = Some(n);
        }else{
            parent.borrow_mut().children.as_mut().unwrap()[index-1] = Some(Rc::new(RefCell::new(this)))
        }


        let p_stat = parent.borrow().status;
        parent.borrow_mut().status = p_stat | (1 << (index-1));
        
    }

}

pub fn make_tree(f : &DenFn3<f32>, offset : Vector3<f32>, a : f32, size : usize, debug_render_lines : &mut RendererVertFragDef) -> Rc<RefCell<Octree<VoxelData<f32>>>>{
    let mut tree = Rc::new(RefCell::new( Octree{parent : None, children : None, status : 0, index : 0, data : None} ));

    for i in 0..8{
        fill_in_leaves(f, offset, a, size, 1, i + 1, tree.clone(), debug_render_lines);
    }

    tree
}


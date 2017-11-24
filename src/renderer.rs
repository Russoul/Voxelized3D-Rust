extern crate libc;
extern crate num;

use typenum::*;
use std::vec::*;
use graphics::*;
use self::libc::*;
use vector::*;
use math::*;
use std::fmt::Debug;

//TODO implement dynamic dispatch
pub struct RendererVertFrag<D : RendererVertFragData>{
    pub data            : D,
    render_mode     : fn (&mut D) -> usize,
    set_attrib_ptrs : fn (&mut D),
    pub construct       : fn (&mut D) -> bool,
    pub deconstruct     : fn (&mut D) -> bool,
    pub draw            : fn (&mut D) -> bool,
    pub shader_name     : fn (&mut D) -> String,
}

pub trait RendererVertFragData{}

pub struct RendererVertFragDataDef{
    pub vertex_size: usize,
    pub vertex_pool: Vec<f32>,
    pub index_pool: Vec<u32>,
    pub vertex_count: u32,
    pub VBO: usize,
    pub VAO: usize,
    pub EBO: usize,
    pub constructed: bool,
    pub render_mode: usize,
    pub shader_name: String,
    pub set_attrib_ptrs: fn(&mut RendererVertFragDataDef),
    pub clear_pools: fn(&mut RendererVertFragDataDef),

}


pub fn add_tringle_color(dat: &mut RendererVertFragDataDef, tr: Triangle<f32,U3>, color: Vector<f32,U3>){
    dat.vertex_pool.push(tr.p1.0[0]);
    dat.vertex_pool.push(tr.p1.0[1]);
    dat.vertex_pool.push(tr.p1.0[2]);

    dat.vertex_pool.push(color.0[0]);
    dat.vertex_pool.push(color.0[1]);
    dat.vertex_pool.push(color.0[2]);

    dat.vertex_pool.push(tr.p2.0[0]);
    dat.vertex_pool.push(tr.p2.0[1]);
    dat.vertex_pool.push(tr.p2.0[2]);

    dat.vertex_pool.push(color.0[0]);
    dat.vertex_pool.push(color.0[1]);
    dat.vertex_pool.push(color.0[2]);

    dat.vertex_pool.push(tr.p3.0[0]);
    dat.vertex_pool.push(tr.p3.0[1]);
    dat.vertex_pool.push(tr.p3.0[2]);

    dat.vertex_pool.push(color.0[0]);
    dat.vertex_pool.push(color.0[1]);
    dat.vertex_pool.push(color.0[2]);

    dat.index_pool.push(dat.vertex_count + 0);
    dat.index_pool.push(dat.vertex_count + 1);
    dat.index_pool.push(dat.vertex_count + 2);


    dat.vertex_count += 3;
}

pub const VERTEX_SIZE_COLOR : usize = 6;

pub fn set_attrib_ptrs_color(dat: &mut RendererVertFragDataDef){
    gl_vertex_attrib_pointer(0, 3, GL_FLOAT, false, VERTEX_SIZE_COLOR * 4,
                             0);
    gl_enable_vertex_attrib_array(0);

    gl_vertex_attrib_pointer(1, 3, GL_FLOAT, false, VERTEX_SIZE_COLOR * 4,
                             3 * 4);
    gl_enable_vertex_attrib_array(1);

}

impl RendererVertFragData for RendererVertFragDataDef{}

pub fn render_vert_frag_def(vs: usize,
                            set_attrib_ptrs : fn (&mut RendererVertFragDataDef),
                            render_mode: usize,
                            shader_name: String)
                            -> RendererVertFrag<RendererVertFragDataDef>{

    fn clear_pools(dat: &mut RendererVertFragDataDef){
        dat.vertex_pool.clear();
        dat.index_pool.clear();
        dat.vertex_count = 0;
    };

    let mut dat = RendererVertFragDataDef{
        vertex_size: vs,
        vertex_pool: Vec::new(),
        index_pool: Vec::new(),
        vertex_count: 0,
        VBO: 0,
        VAO: 0,
        EBO: 0,
        constructed:false,
        render_mode,
        shader_name,
        set_attrib_ptrs,
        clear_pools
    };

    fn construct(dat: &mut RendererVertFragDataDef) -> bool {
        if dat.constructed {return false;};

        dat.VAO = gl_gen_vertex_arrays();
        dat.VBO = gl_gen_buffers();
        dat.EBO = gl_gen_buffers();


        gl_bind_vertex_array(dat.VAO);

        gl_bind_buffer(GL_ARRAY_BUFFER, dat.VBO);
        
        gl_buffer_data(GL_ARRAY_BUFFER,
                       dat.vertex_pool.len(),
                       dat.vertex_pool.as_slice(),
                       GL_STATIC_DRAW);

        gl_bind_buffer(GL_ELEMENT_ARRAY_BUFFER, dat.EBO);
        gl_buffer_data(GL_ELEMENT_ARRAY_BUFFER, dat.index_pool.len(),
                       dat.index_pool.as_slice(),
                       GL_STATIC_DRAW
        );

        (dat.set_attrib_ptrs)(dat);

        gl_bind_buffer(GL_ARRAY_BUFFER, 0);
        gl_bind_vertex_array(0);

        dat.constructed = true;
        
        true
    };

    fn deconstruct(dat: &mut RendererVertFragDataDef) -> bool {
        if !dat.constructed {return false;};

        gl_delete_vertex_arrays(dat.VAO);
        gl_delete_buffers(dat.VBO);
        gl_delete_buffers(dat.EBO);

        dat.constructed = false;

        true
    };


    

    fn draw(dat: &mut RendererVertFragDataDef) -> bool{
        if !dat.constructed {return false;};

        gl_bind_vertex_array(dat.VAO);
        gl_draw_elements(dat.render_mode, dat.index_pool.len(), GL_UNSIGNED_INT, 0);
        gl_bind_vertex_array(0);

        true
        
    };

    fn shader_name_fn(dat: &mut RendererVertFragDataDef)-> String{
        dat.shader_name.clone()
    }

    fn render_mode_fn(dat: &mut RendererVertFragDataDef) -> usize{
        dat.render_mode
    }
    
    RendererVertFrag{construct,
                     deconstruct,
                     draw,
                     render_mode: render_mode_fn,
                     shader_name: shader_name_fn,
                     set_attrib_ptrs,
                     data: dat}
}

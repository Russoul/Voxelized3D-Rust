use std::vec::*;
use graphics::*;
use libc::*;
use math::*;
use std::fmt::Debug;

use na::{Vector3, U3};

pub trait RendererVertFrag{
    fn render_mode       (&self) -> usize;
    fn shader_name       (&self) -> String;
    fn set_attrib_ptrs   (&mut self);
    fn construct         (&mut self) -> bool;
    fn deconstruct       (&mut self) -> bool;
    fn draw              (&mut self) -> bool;
    fn reset             (&mut self); //used to clear/reset all data stored in 'self'
}


pub struct RendererVertFragDef{
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
    pub set_attrib_ptrs: fn(&mut RendererVertFragDef),

}


pub const VERTEX_SIZE_COLOR : usize = 6;

pub fn set_attrib_ptrs_color(_:&mut RendererVertFragDef){
    gl_vertex_attrib_pointer(0, 3, GL_FLOAT, false, VERTEX_SIZE_COLOR * 4,
                             0);
    gl_enable_vertex_attrib_array(0);

    gl_vertex_attrib_pointer(1, 3, GL_FLOAT, false, VERTEX_SIZE_COLOR * 4,
                             3 * 4);
    gl_enable_vertex_attrib_array(1);

}

impl RendererVertFrag for RendererVertFragDef{
    fn render_mode(&self) -> usize {
        self.render_mode
    }

    fn shader_name(&self) -> String {
        self.shader_name.clone()
    }

    fn set_attrib_ptrs(&mut self) {
        (self.set_attrib_ptrs)(self)
    }

    fn construct(&mut self) -> bool {
        if self.constructed {return false;};

        self.VAO = gl_gen_vertex_arrays();
        self.VBO = gl_gen_buffers();
        self.EBO = gl_gen_buffers();


        gl_bind_vertex_array(self.VAO);

        gl_bind_buffer(GL_ARRAY_BUFFER, self.VBO);

        gl_buffer_data(GL_ARRAY_BUFFER,
                       self.vertex_pool.len(),
                       self.vertex_pool.as_slice(),
                       GL_STATIC_DRAW);

        gl_bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.EBO);
        gl_buffer_data(GL_ELEMENT_ARRAY_BUFFER, self.index_pool.len(),
                       self.index_pool.as_slice(),
                       GL_STATIC_DRAW
        );

        self.set_attrib_ptrs();

        gl_bind_buffer(GL_ARRAY_BUFFER, 0);
        gl_bind_vertex_array(0);

        self.constructed = true;

        true
    }

    fn deconstruct(&mut self) -> bool {
        if !self.constructed {return false;};

        gl_delete_vertex_arrays(self.VAO);
        gl_delete_buffers(self.VBO);
        gl_delete_buffers(self.EBO);

        self.constructed = false;

        true
    }

    fn draw(&mut self) -> bool {
        if !self.constructed {return false;};

        gl_bind_vertex_array(self.VAO);
        gl_draw_elements(self.render_mode, self.index_pool.len(), GL_UNSIGNED_INT, 0);
        gl_bind_vertex_array(0);

        true
    }

    fn reset(&mut self) {
        self.vertex_pool.clear();
        self.index_pool.clear();
        self.vertex_count = 0;
    }
}

impl RendererVertFragDef{
    pub fn make(vs: usize,
            set_attrib_ptrs : fn (&mut RendererVertFragDef),
            render_mode: usize,
            shader_name: String) -> RendererVertFragDef{
        RendererVertFragDef{
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
            set_attrib_ptrs
        }
    }
}

pub fn add_tringle_color(dat: &mut RendererVertFragDef, tr: Triangle<Vector3<f32>>, color: Vector3<f32>){
    dat.vertex_pool.push(tr.p1[0]);
    dat.vertex_pool.push(tr.p1[1]);
    dat.vertex_pool.push(tr.p1[2]);

    dat.vertex_pool.push(color[0]);
    dat.vertex_pool.push(color[1]);
    dat.vertex_pool.push(color[2]);

    dat.vertex_pool.push(tr.p2[0]);
    dat.vertex_pool.push(tr.p2[1]);
    dat.vertex_pool.push(tr.p2[2]);

    dat.vertex_pool.push(color[0]);
    dat.vertex_pool.push(color[1]);
    dat.vertex_pool.push(color[2]);

    dat.vertex_pool.push(tr.p3[0]);
    dat.vertex_pool.push(tr.p3[1]);
    dat.vertex_pool.push(tr.p3[2]);

    dat.vertex_pool.push(color[0]);
    dat.vertex_pool.push(color[1]);
    dat.vertex_pool.push(color[2]);

    dat.index_pool.push(dat.vertex_count + 0);
    dat.index_pool.push(dat.vertex_count + 1);
    dat.index_pool.push(dat.vertex_count + 2);


    dat.vertex_count += 3;
}


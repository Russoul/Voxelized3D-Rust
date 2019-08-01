
use std::vec::{Vec as Vector};
use graphics::*;
use math::*;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use std::fs;
use graphics_util::create_program_vf;
use std::io::Read;

use matrix::*;

use glfw::{Action, Context, Key, Glfw, WindowHint};
use time::precise_time_ns;
use std::fmt::Display;

fn load_shaders_vf() -> HashMap<String, Program>{
    let dir : &str = "./assets/shaders/";
    let paths = fs::read_dir(dir).unwrap();
    let mut map : HashMap<String, Program> = HashMap::new();

    for entry in paths{
        let name : String = String::from(entry
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap());

        if !map.contains_key(&name){
            let mut file_vert = fs::File::open(
                dir.to_string() + &name + ".vert").unwrap();
            let mut source_vert = String::new();
            file_vert.read_to_string(&mut source_vert).unwrap();

            let mut file_frag = fs::File::open(
                dir.to_string() + &name + ".frag").unwrap();
            let mut source_frag = String::new();
            file_frag.read_to_string(&mut source_frag).unwrap();

            let prog = create_program_vf(
                &source_vert,
                &source_frag);


            map.insert(name, Program{id: prog});
        }
    }

    map
}

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
    pub vertex_pool: Vector<f32>,
    pub index_pool: Vector<u32>,
    pub vertex_count: u32,
    pub vbo: usize,
    pub vao: usize,
    pub ebo: usize,
    pub constructed: bool,
    pub render_mode: usize,
    pub shader_name: String,
    pub set_attrib_ptrs: fn(&mut RendererVertFragDef),

}

#[derive(Debug, Clone, Copy)]
pub struct Cam{
    pub pos : Vec3<f32>,
    pub look : Vec3<f32>,
    pub up : Vec3<f32>,
}


impl Display for Cam{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Cam{{");
        writeln!(f, "pos = {}, ", self.pos);
        writeln!(f, "look = {}, ", self.look);
        writeln!(f, "up = {}, ", self.up);
        writeln!(f, "}}");
        Ok(())
    }
}

pub struct Renderer{
    pub render_triangles_pos_color : RendererVertFragDef,
    pub render_lines_pos_color : RendererVertFragDef,
    pub render_triangles_lighting_pos_color_normal : RendererVertFragDef,
    pub glfw : Option<glfw::Glfw>,
    pub window : Option<glfw::Window>,
    pub events : Option<Receiver<(f64, glfw::WindowEvent)>>,
    pub frame_buffer_size_callback : Option<Box<Fn(i32, i32)>>,
    pub shaders : HashMap<String, Program>,
    pub camera : Cam,
    last_frame_nt : u64
}

impl Renderer{

    pub fn new(camera : Cam) -> Renderer{
        Renderer{render_triangles_pos_color : RendererVertFragDef::make(VERTEX_SIZE_POS_COLOR, set_attrib_ptrs_pos_color, GL_TRIANGLES, "color"),
                 render_lines_pos_color : RendererVertFragDef::make(VERTEX_SIZE_POS_COLOR, set_attrib_ptrs_pos_color, GL_LINES, "color"),
                 render_triangles_lighting_pos_color_normal : RendererVertFragDef::make(VERTEX_SIZE_POS_COLOR_NORMAL, set_attrib_ptrs_pos_color_normal, GL_TRIANGLES, "lighting"),
                 glfw : None, window : None, events : None, frame_buffer_size_callback : None,
                 shaders : HashMap::new(), camera, last_frame_nt : 0}
    }

    pub fn get_window(&mut self) -> &mut glfw::Window{
        self.window.as_mut().unwrap()
    }
    pub fn get_glfw(&mut self) -> &mut glfw::Glfw{
        self.glfw.as_mut().unwrap()
    }
    pub fn get_events(&self) -> &Receiver<(f64, glfw::WindowEvent)>{
        self.events.as_ref().unwrap()
    }
    pub fn get_camera(&mut self) -> &mut Cam{
        &mut self.camera
    }
    pub fn get_shaders(&mut self) -> &mut HashMap<String, Program>{
        &mut self.shaders
    }

    pub fn init(&mut self, start_width : u32, start_height : u32, title : &str){
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        if cfg!(target_os = "macos") {
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        }
        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw.create_window(start_width, start_height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");


        // Make the window's context current
        window.make_current();
        glad_load_gl_loader();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        self.window = Some(window);
        self.glfw = Some(glfw);
        self.events = Some(events);

        self.shaders = load_shaders_vf();
    }

    pub fn set_framebuffer_size_callback<F : Fn(i32, i32) >(&mut self, callback : F) where F : 'static{
        self.frame_buffer_size_callback = Some(Box::new(callback));
    }

    pub fn run<F : Fn(&mut Renderer, u64)>(&mut self, pre_render : F) where F : 'static{
        let red = Vec3::new(1.0, 0.0, 0.0);
        let green = Vec3::new(0.0, 1.0, 0.0);
        let blue = Vec3::new(0.0, 0.0, 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);

        self.last_frame_nt = precise_time_ns();

        // Loop until the user closes the window
        while !self.get_window().should_close() {
            let t_ns = precise_time_ns();
            let dt_ns = t_ns - self.last_frame_nt;

            pre_render(self, dt_ns);

            let(win_w, win_h) = self.get_window().get_framebuffer_size();
            let aspect =win_w as f32 / win_h as f32;

            let persp = perspective(90.0, aspect, 0.1, 16.0);
            let view = view_dir(self.camera.pos, self.camera.look, self.camera.up);

            let shaderColor = self.get_shaders().get("color").unwrap();
            shaderColor.enable();

            shaderColor.set_float4x4("P", true, persp.as_slice());
            shaderColor.set_float4x4("V", true, view.as_slice());

            if !self.render_lines_pos_color.constructed{
                self.render_lines_pos_color.construct();
            }

            self.render_lines_pos_color.draw();

            if !self.render_triangles_pos_color.constructed{
                self.render_triangles_pos_color.construct();
            }

            self.render_triangles_pos_color.draw();

            let shaderLighting = self.get_shaders().get("lighting").unwrap();
            shaderLighting.enable();


            shaderLighting.set_float4x4("P", true, persp.as_slice());
            shaderLighting.set_float4x4("V", true, view.as_slice());

            shaderLighting.set_vec3f("pointLight.pos" ,Vec3::new(0.0, 8.0,0.0));
            shaderLighting.set_vec3f("pointLight.color" ,(red + green + blue) * 15.0);

            if !self.render_triangles_lighting_pos_color_normal.constructed{
                self.render_triangles_lighting_pos_color_normal.construct();
            }

            self.render_triangles_lighting_pos_color_normal.draw();


            // Swap front and back buffers
            self.get_window().swap_buffers();

            // Poll for and process events
            self.get_glfw().poll_events();
            let window = self.window.as_mut().unwrap();
            for (_, event) in glfw::flush_messages(self.events.as_ref().unwrap()) {
                //println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    },
                    glfw::WindowEvent::FramebufferSize(w, h) => {
                        self.frame_buffer_size_callback.iter().for_each(|f| f(w, h));
                    }
                    _ => {},
                }
            }

            self.last_frame_nt = t_ns;
        }
    }

}


pub const VERTEX_SIZE_POS_COLOR: usize = 6;
pub const VERTEX_SIZE_POS_COLOR_NORMAL: usize = 9;

pub fn set_attrib_ptrs_pos_color(_:&mut RendererVertFragDef){
    gl_vertex_attrib_pointer(0, 3, GL_FLOAT, false, VERTEX_SIZE_POS_COLOR * 4,
                             0);
    gl_enable_vertex_attrib_array(0);

    gl_vertex_attrib_pointer(1, 3, GL_FLOAT, false, VERTEX_SIZE_POS_COLOR * 4,
                             3 * 4);
    gl_enable_vertex_attrib_array(1);

}

pub fn set_attrib_ptrs_pos_color_normal(_:&mut RendererVertFragDef){
    gl_vertex_attrib_pointer(0, 3, GL_FLOAT, false, VERTEX_SIZE_POS_COLOR_NORMAL * 4,
                             0);
    gl_enable_vertex_attrib_array(0);

    gl_vertex_attrib_pointer(1, 3, GL_FLOAT, false, VERTEX_SIZE_POS_COLOR_NORMAL * 4,
                             3 * 4);
    gl_enable_vertex_attrib_array(1);

    gl_vertex_attrib_pointer(2, 3, GL_FLOAT, false, VERTEX_SIZE_POS_COLOR_NORMAL * 4,
                             6 * 4);
    gl_enable_vertex_attrib_array(2);

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

        self.vao = gl_gen_vertex_arrays();
        self.vbo = gl_gen_buffers();
        self.ebo = gl_gen_buffers();


        gl_bind_vertex_array(self.vao);

        gl_bind_buffer(GL_ARRAY_BUFFER, self.vbo);

        gl_buffer_data(GL_ARRAY_BUFFER,
                       self.vertex_pool.len(),
                       self.vertex_pool.as_slice(),
                       GL_STATIC_DRAW);

        gl_bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.ebo);
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

        gl_delete_vertex_arrays(self.vao);
        gl_delete_buffers(self.vbo);
        gl_delete_buffers(self.ebo);

        self.constructed = false;

        true
    }

    fn draw(&mut self) -> bool {
        if !self.constructed {return false;};

        gl_bind_vertex_array(self.vao);
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
            shader_name: &str) -> RendererVertFragDef{
        RendererVertFragDef{
            vertex_size: vs,
            vertex_pool: Vector::new(),
            index_pool: Vector::new(),
            vertex_count: 0,
            vbo: 0,
            vao: 0,
            ebo: 0,
            constructed:false,
            render_mode,
            shader_name : String::from(shader_name.clone()),
            set_attrib_ptrs
        }
    }
}

pub fn add_triangle_color(dat: &mut RendererVertFragDef, tr: Triangle3<f32>, color: Vec3<f32>){
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


pub fn add_triangle_pos_color_normal(dat: &mut RendererVertFragDef, tr: Triangle3<f32>, color: Vec3<f32>, normal : Vec3<f32>){
    dat.vertex_pool.push(tr.p1[0]);
    dat.vertex_pool.push(tr.p1[1]);
    dat.vertex_pool.push(tr.p1[2]);

    dat.vertex_pool.push(color[0]);
    dat.vertex_pool.push(color[1]);
    dat.vertex_pool.push(color[2]);

    dat.vertex_pool.push(normal[0]);
    dat.vertex_pool.push(normal[1]);
    dat.vertex_pool.push(normal[2]);

    dat.vertex_pool.push(tr.p2[0]);
    dat.vertex_pool.push(tr.p2[1]);
    dat.vertex_pool.push(tr.p2[2]);

    dat.vertex_pool.push(color[0]);
    dat.vertex_pool.push(color[1]);
    dat.vertex_pool.push(color[2]);

    dat.vertex_pool.push(normal[0]);
    dat.vertex_pool.push(normal[1]);
    dat.vertex_pool.push(normal[2]);

    dat.vertex_pool.push(tr.p3[0]);
    dat.vertex_pool.push(tr.p3[1]);
    dat.vertex_pool.push(tr.p3[2]);

    dat.vertex_pool.push(color[0]);
    dat.vertex_pool.push(color[1]);
    dat.vertex_pool.push(color[2]);

    dat.vertex_pool.push(normal[0]);
    dat.vertex_pool.push(normal[1]);
    dat.vertex_pool.push(normal[2]);

    dat.index_pool.push(dat.vertex_count + 0);
    dat.index_pool.push(dat.vertex_count + 1);
    dat.index_pool.push(dat.vertex_count + 2);


    dat.vertex_count += 3;
}

fn add_vector_to_pool(dat : &mut RendererVertFragDef, vec : Vec3<f32>){
    dat.vertex_pool.push(vec.x);
    dat.vertex_pool.push(vec.y);
    dat.vertex_pool.push(vec.z);
}

pub fn add_line3_color(dat : &mut RendererVertFragDef, line : Line3<f32>, color : Vec3<f32>){
    add_vector_to_pool(dat, line.start);
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, line.end);
    add_vector_to_pool(dat, color);

    dat.index_pool.push(0 + dat.vertex_count);
    dat.index_pool.push(1 + dat.vertex_count);

    dat.vertex_count += 2;
}

pub fn add_cube_bounds_pos_color(dat : &mut RendererVertFragDef, cube : Cube<f32>, color : Vec3<f32>){
    add_vector_to_pool(dat, Vec3::new(cube.center.x - cube.extent, cube.center.y - cube.extent, cube.center.z - cube.extent));
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, Vec3::new(cube.center.x + cube.extent, cube.center.y - cube.extent, cube.center.z - cube.extent));
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, Vec3::new(cube.center.x + cube.extent, cube.center.y + cube.extent, cube.center.z - cube.extent));
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, Vec3::new(cube.center.x - cube.extent, cube.center.y + cube.extent, cube.center.z - cube.extent));
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, Vec3::new(cube.center.x - cube.extent, cube.center.y - cube.extent, cube.center.z + cube.extent));
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, Vec3::new(cube.center.x + cube.extent, cube.center.y - cube.extent, cube.center.z + cube.extent));
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, Vec3::new(cube.center.x + cube.extent, cube.center.y + cube.extent, cube.center.z + cube.extent));
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, Vec3::new(cube.center.x - cube.extent, cube.center.y + cube.extent, cube.center.z + cube.extent));
    add_vector_to_pool(dat, color);

    let indices : [u32;24] = [0,1,1,2,2,3,3,0, 4,5,5,6,6,7,7,4, 0,4, 1,5, 2,6, 3,7];
    for i in indices.iter() {dat.index_pool.push(i.clone() + dat.vertex_count);}
    dat.vertex_count += 8;
}


//for cubes
fn centers() -> [Vec3<f32>;8]{
    [Vec3::new(-0.5, -0.5, -0.5),
     Vec3::new(0.5, -0.5, -0.5),
     Vec3::new(0.5, -0.5, 0.5),
     Vec3::new(-0.5, -0.5, 0.5),

     Vec3::new(-0.5, 0.5, -0.5),
     Vec3::new(0.5, 0.5, -0.5),
     Vec3::new(0.5, 0.5, 0.5),
     Vec3::new(-0.5, 0.5, 0.5)]
}

pub fn add_cube_color_normal(dat : &mut RendererVertFragDef, cube : Cube<f32>, color : Vec3<f32>){
    let mut corners = [Vec3::empty();8];

    for i in 0..8{
        corners[i] = centers()[i] * 2.0 * cube.extent + cube.center;
    }

    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[7], p2 : corners[0], p3 : corners[3]}, color, Vec3::new(-1.0, 0.0, 0.0));
    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[0], p2 : corners[7], p3 : corners[4]}, color, Vec3::new(-1.0, 0.0, 0.0));

    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[1], p2 : corners[6], p3 : corners[2]}, color, Vec3::new(1.0, 0.0, 0.0));
    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[1], p2 : corners[5], p3 : corners[6]}, color, Vec3::new(1.0, 0.0, 0.0));

    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[0], p2 : corners[4], p3 : corners[1]}, color, Vec3::new(0.0, 0.0, -1.0));
    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[1], p2 : corners[4], p3 : corners[5]}, color, Vec3::new(0.0, 0.0, -1.0));

    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[2], p2 : corners[7], p3 : corners[3]}, color, Vec3::new(0.0, 0.0, 1.0));
    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[2], p2 : corners[6], p3 : corners[7]}, color, Vec3::new(0.0, 0.0, 1.0));

    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[0], p2 : corners[2], p3 : corners[3]}, color, Vec3::new(0.0, -1.0, 0.0));
    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[2], p2 : corners[0], p3 : corners[1]}, color, Vec3::new(0.0, -1.0, 0.0));

    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[6], p2 : corners[4], p3 : corners[7]}, color, Vec3::new(0.0, 1.0, 0.0));
    add_triangle_pos_color_normal(dat, Triangle3{p1 : corners[6], p2 : corners[5], p3 : corners[4]}, color, Vec3::new(0.0, 1.0, 0.0));
}

pub fn add_sphere_color(dat : &mut RendererVertFragDef, sphere : Sphere<f32>, n : usize, m : usize, color : Vec3<f32>){
    use std;
    let pi = std::f32::consts::PI;
    let dphi = 2.0 * pi / n as f32;
    let dpsi = pi / m as f32;

    for i in 0..n{
        let phi = i as f32 * dphi;
        let phi_next = (i + 1) as f32 * dphi;
        for j in 0..m{
            let psi = j as f32 * dpsi;
            let psi_next = (j + 1) as f32 * dpsi;

            let x0 = phi.cos() * psi.sin() * sphere.rad;
            let z0 = -phi.sin() * psi.sin() * sphere.rad;
            let y0 = -psi.cos() * sphere.rad;

            let x1 = phi_next.cos() * psi.sin() * sphere.rad;
            let z1 = -phi_next.sin() * psi.sin() * sphere.rad;
            let y1 = -psi.cos() * sphere.rad;

            let x2 = phi.cos() * psi_next.sin() * sphere.rad;
            let z2 = -phi.sin() * psi_next.sin() * sphere.rad;
            let y2 = -psi_next.cos() * sphere.rad;

            let x3 = phi_next.cos() * psi_next.sin() * sphere.rad;
            let z3 = -phi_next.sin() * psi_next.sin() * sphere.rad;
            let y3 = -psi_next.cos() * sphere.rad;

            let v0 = Vec3::new(x0, y0, z0);
            let v1 = Vec3::new(x1, y1, z1);
            let v2 = Vec3::new(x2, y2, z2);
            let v3 = Vec3::new(x3, y3, z3);

            let normal = (v1 - v0).cross(v2 - v0).normalize();

             add_vector_to_pool(dat, sphere.center + v0);
             add_vector_to_pool(dat, color);
             add_vector_to_pool(dat, normal);

             add_vector_to_pool(dat, sphere.center + v1);
             add_vector_to_pool(dat, color);
             add_vector_to_pool(dat, normal);

             add_vector_to_pool(dat, sphere.center + v2);
             add_vector_to_pool(dat, color);
             add_vector_to_pool(dat, normal);

             add_vector_to_pool(dat, sphere.center + v3);
             add_vector_to_pool(dat, color);
             add_vector_to_pool(dat, normal);
            
        }
    }

    for i in 0..n*m{
        dat.index_pool.push(dat.vertex_count + 4*i as u32);
        dat.index_pool.push(dat.vertex_count + 4*i as u32 + 1);
        dat.index_pool.push(dat.vertex_count + 4*i as u32 + 2);

        dat.index_pool.push(dat.vertex_count + 4*i as u32 + 1);
        dat.index_pool.push(dat.vertex_count + 4*i as u32 + 3);
        dat.index_pool.push(dat.vertex_count + 4*i as u32 + 2);
    }

    dat.vertex_count += n as u32 * m as u32 * 4;

}

pub fn add_grid3_pos_color(dat : &mut RendererVertFragDef, center : Vec3<f32>, tangent : Vec3<f32>, normal : Vec3<f32>, extent : f32, subdiv_num : u32, color : Vec3<f32>){
    let right = tangent.cross(normal) * extent;
    let along = tangent * extent;
    add_vector_to_pool(dat, center - right - along);
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, center + right - along);
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, center + right + along);
    add_vector_to_pool(dat, color);
    add_vector_to_pool(dat, center - right + along);
    add_vector_to_pool(dat, color);

    let a = extent / subdiv_num as f32;
    //TODO inefficient loops(could be done in one)
    for i in 1 .. 2 * subdiv_num{
        add_vector_to_pool(dat, center - right * (extent - i as f32 * a) - along);
        add_vector_to_pool(dat, color);
    }

    for i in 1 .. 2 * subdiv_num{
        add_vector_to_pool(dat, center + right - along * (extent - i as f32 * a) );
        add_vector_to_pool(dat, color);
    }

    for i in 1 .. 2 * subdiv_num{
        add_vector_to_pool(dat, center + right * (extent - i as f32 * a) + along);
        add_vector_to_pool(dat, color);
    }

    for i in 1 .. 2 * subdiv_num{
        add_vector_to_pool(dat, center - right + along * (extent - i as f32 * a) );
        add_vector_to_pool(dat, color);
    }

    dat.index_pool.push(0 + dat.vertex_count);
    dat.index_pool.push(1 + dat.vertex_count);
    dat.index_pool.push(1 + dat.vertex_count);
    dat.index_pool.push(2 + dat.vertex_count);
    dat.index_pool.push(2 + dat.vertex_count);
    dat.index_pool.push(3 + dat.vertex_count);
    dat.index_pool.push(3 + dat.vertex_count);
    dat.index_pool.push(0 + dat.vertex_count);

    let off0 : u32 = 4;
    let off1 : u32 = subdiv_num * 2 - 1;

    for i in 0..off1{
        dat.index_pool.push(off0 + off1 + i + dat.vertex_count);
        dat.index_pool.push(off0 + 4*off1 - i - 1 + dat.vertex_count);
    }

    for i in 0..off1{
        dat.index_pool.push(off0 + i + dat.vertex_count);
        dat.index_pool.push(off0 + 3*off1 - i - 1 + dat.vertex_count);
    }

    dat.vertex_count += 4 + 4 * (2 * subdiv_num - 1)
}


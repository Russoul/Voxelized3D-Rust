

extern crate generic_array;

mod graphics;
mod vector;
mod graphics_util;
mod renderer;
mod math;
mod voxel_renderer;

use graphics::*;
use std::ptr;
use generic_array::*;
use vector::*;
use std::fs;
use std::fs::File;
use std::vec::*;
use std::collections::HashMap;
use graphics_util::*;
use std::io::Read;
use renderer::*;
use math::*;
use voxel_renderer::*;

//TODO mutable global data is unsafe, let us try to avoid it
/*static mut win_width: usize = 0;
static mut win_height: usize = -1;

fn update_win_dim_info(new_width: usize, new_height: usize){
    unsafe{
        win_width = new_width;
        win_height = new_height;
    }
}*/

extern fn framebuf_sz_cb(win : *mut GlfwWindow, w : isize, h : isize){
    gl_viewport(0,0,w,h);
}

extern fn error_cb(n : isize, er : &str){
    println!("{}", er);
}

fn check_for_gl_errors(){
    let mut er: usize = gl_get_error();

    while er != GL_NO_ERROR{
        eprintln!("GL error: {}", er);
        er = gl_get_error();
    }
}

fn update_win_dim_info(info: &mut WindowInfo){
    let mut w: usize = 0;
    let mut h: usize = 0;

    glfw_get_window_size(info.handle, &mut w, &mut h);
    info.width = w;
    info.height = h;
}

fn process_input(win : *mut GlfwWindow){
    if glfw_get_key(win, GLFW_KEY_ESCAPE) == GLFW_PRESS{
        glfw_set_window_should_close(win, true);
    }
    else if glfw_get_key(win, GLFW_KEY_TAB) == GLFW_PRESS{
        //debug

        let mut w : usize = 0;
        let mut h : usize = 0;

        glfw_get_window_size(win, &mut w, &mut h);

        println!("({}, {})", w, h);

        let mon = glfw_get_primary_monitor();
        let vid_mode = glfw_get_video_mode(mon);
        unsafe{
            println!("{:?}", *vid_mode)
        }
    }
}


fn test_closures(){
    fn clo_test<Clo : Fn() -> i32>(clo: Clo) -> i32{
        clo()
    }

    let clo = ||{2};
    clo_test(clo);
}

fn test_vectors(){
    let ar1 = Vector::from(arr![usize;1,2,3,4]);
    let ar2 = ar1.clone();
    ar1.print();

    let ar3
        = &ar1 + &ar1;
    ar3.print();
    let ar4 = &ar1 * &ar1;
    println!("{}", ar4);
    println!("{}", ar3);
    let mapped = Vector::from(ar1.get().map(|x| x + 1));
    println!("{}", mapped);
}

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
            let mut file_vert = File::open(
                dir.to_string() + &name + ".vert").unwrap();
            let mut source_vert = String::new();
            file_vert.read_to_string(&mut source_vert).unwrap();

            let mut file_frag = File::open(
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

fn main() {
    let def_width: usize = 800;
    let def_height: usize = 600;
    test_vectors();
   
    //TODO check if it works
    glfw_set_error_callback(error_cb);
    glfw_init();
    glfw_window_hint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfw_window_hint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfw_window_hint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    let win = glfw_create_window(def_width, def_height, "Voxelized2D");

    if win == ptr::null_mut(){
        glfw_terminate();
        panic!("Failed to create GLFW window !");
    }

    glfw_make_context_current(win);
    glad_load_gl_loader();

    println!("Using GL version: {}", gl_get_string(GL_VERSION));
    
    glfw_set_framebuffer_size_callback(win, framebuf_sz_cb);


    let shaders = load_shaders_vf();
    let mut voxel_renderer = VoxelRenderer::new(&shaders);
    let mut win_info = WindowInfo{width: def_width, height: def_height, handle: win}; //will be updated each frame

    let test_tr = Triangle{p1: Vector(arr![f32;-1.0,-1.0, 0.0]),
                           p2: Vector(arr![f32;1.0, -1.0, 0.0]),
                           p3: Vector(arr![f32;0.0, 1.0, 0.0])};

    let mut renderer = RendererVertFragDef::make(VERTEX_SIZE_COLOR, set_attrib_ptrs_color, GL_TRIANGLES, String::from("color"));

    add_tringle_color(&mut renderer, test_tr, Vector(arr!(f32;1.0,0.0,0.0)));
    //let shader = shaders.get(&String::from("color")).unwrap();
    /*shader.enable();
    let id_mat = [
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0];
    shader.set_float4x4("P", false, &id_mat);
    shader.set_float4x4("V", false, &id_mat);*/

    fn shader_data(shader: &Program, win: &WindowInfo){
        let id_mat = [
            1.0,0.0,0.0,0.0,
            0.0,1.0,0.0,0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0];
        shader.set_float4x4("P", false, &id_mat);
        shader.set_float4x4("V", false, &id_mat);
    }

    let provider = RenderDataProvider{pre_render_state: None, post_render_state: None, shader_data: Some(Box::new(shader_data))};



    let mut render_info = RenderInfo{renderer: Box::new(renderer), provider};

    (&mut render_info.renderer as &mut Box<RendererVertFrag>).construct();

    let id = voxel_renderer.push(RenderLifetime::Manual, RenderTransform::None, render_info).unwrap();

    while !glfw_window_should_close(win){
        update_win_dim_info(&mut win_info);
        process_input(win);

        gl_clear_color(0.2, 0.3, 0.3, 1.0);
        gl_clear(GL_COLOR_BUFFER_BIT);

        voxel_renderer.draw(&win_info);


        glfw_swap_buffers(win);
        glfw_poll_events();

        check_for_gl_errors();
    }

    (&mut voxel_renderer.lifetime_manual_renderers.get_mut(&id).unwrap().renderer as &mut Box<RendererVertFrag>).deconstruct();
    (&mut voxel_renderer.lifetime_manual_renderers.get_mut(&id).unwrap().renderer as &mut Box<RendererVertFrag>).reset();

    glfw_terminate();
}

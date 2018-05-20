//#![feature(box_syntax, box_patterns, clone_closures, copy_closures)]

extern crate generic_array;
extern crate nalgebra as na;
extern crate typenum;
extern crate alga;
extern crate libc;
extern crate ansi_term;
extern crate time;
extern crate rand;
extern crate noise;
extern crate num;

use na::*;
use na::core::Unit;

mod math_tests;
mod qef_bindings;
mod graphics;
mod graphics_util;
mod renderer;
#[macro_use]
mod math;
mod voxel_renderer;
mod dc;
mod dcm;
mod matrix;
mod uniform_manifold_dc;
mod cubic;

use noise::{Perlin};
use graphics::*;
use std::ptr;
use std::fs;
use std::fs::File;
use std::collections::HashMap;
use graphics_util::*;
use std::io::Read;
use renderer::*;
use math::*;
use voxel_renderer::*;
use std::ops::*;
use rand::distributions::{Sample, Range};
use typenum::*;
use core::storage::*;
use generic_array::*;
use uniform_manifold_dc::*;

use time::precise_time_ns;

fn timed<T>(str_fn: &(Fn(u64) -> String), f : &mut (FnMut() -> T)) -> T{
    let t1 = precise_time_ns();
    let ret = f();
    let t2 = precise_time_ns();

    let dt = t2 - t1;

    println!("{}", str_fn(dt));

    ret
}

//F3 : FnMut(A) -> C
fn compose<'l, A, B, C, F1, F2>(f1 : & 'l Box<F1>, f2 : &'l Box<F2>) -> Box<Fn(A) -> C + 'l>
    where F1 : 'l + Fn(A) -> B,
          F2 : 'l + Fn(B) -> C,
          {
    Box::new(move |a : A| {(*f2)((*f1)(a))})
}



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

fn process_input(win : *mut GlfwWindow, dt_ns : u64, camera : &mut Camera){
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


    let dt_s : f32 = dt_ns as f32 / 1000000000.0;

    if glfw_get_key(win, GLFW_KEY_W) == GLFW_PRESS{
        camera.pos += camera.look * dt_s as f32;

    }

    if glfw_get_key(win, GLFW_KEY_S) == GLFW_PRESS{
        camera.pos -= camera.look * dt_s as f32;
    }

    if glfw_get_key(win, GLFW_KEY_A) == GLFW_PRESS{
        let right = camera.look.cross(&camera.up);

        camera.pos -= right * dt_s as f32;
    }

    if glfw_get_key(win, GLFW_KEY_D) == GLFW_PRESS{
        let right = camera.look.cross(&camera.up);

        camera.pos += right * dt_s as f32;
    }

    if glfw_get_key(win, GLFW_KEY_SPACE) == GLFW_PRESS{

        camera.pos += camera.up * dt_s as f32;
    }

    if glfw_get_key(win, GLFW_KEY_LEFT_SHIFT) == GLFW_PRESS{

        camera.pos -= camera.up * dt_s as f32;
    }

    if glfw_get_key(win, GLFW_KEY_LEFT) == GLFW_PRESS{

        let mat = na::Rotation3::from_axis_angle(&Unit::new_unchecked(camera.up), std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
    }
    if glfw_get_key(win, GLFW_KEY_RIGHT) == GLFW_PRESS{

        let mat = na::Rotation3::from_axis_angle(&Unit::new_unchecked(camera.up), -std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
    }
    if glfw_get_key(win, GLFW_KEY_KP_0) == GLFW_PRESS{

        let mat = na::Rotation3::from_axis_angle(&Unit::new_unchecked(camera.look), std::f32::consts::PI * dt_s / 2.0);
        camera.up = (mat * camera.up).normalize();
    }
    if glfw_get_key(win, GLFW_KEY_KP_DECIMAL) == GLFW_PRESS{

        let mat = na::Rotation3::from_axis_angle(&Unit::new_unchecked(camera.look), -std::f32::consts::PI * dt_s / 2.0);
        camera.up = (mat * camera.up).normalize();
    }
    if glfw_get_key(win, GLFW_KEY_RIGHT) == GLFW_PRESS{

        let mat = na::Rotation3::from_axis_angle(&Unit::new_unchecked(camera.up), -std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
    }
    if glfw_get_key(win, GLFW_KEY_UP) == GLFW_PRESS{
        let right = camera.look.cross(&camera.up);
        let mat = na::Rotation3::from_axis_angle(&Unit::new_unchecked(right), std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
        camera.up = (mat * camera.up).normalize();
    }
    if glfw_get_key(win, GLFW_KEY_DOWN) == GLFW_PRESS{
        let right = camera.look.cross(&camera.up);
        let mat = na::Rotation3::from_axis_angle(&Unit::new_unchecked(right), -std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
        camera.up = (mat * camera.up).normalize();
    }

   /* println!("{}", camera.pos);
    println!("{}", camera.look.norm());
    println!("{}", camera.up.norm());*/
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

unsafe fn compose2<'a, A,B,C, F, G>(f : *const F, g : *const G) -> Box<Fn(A) -> C + 'a> where
    F :  Fn(A) -> B + 'a,
    G :  Fn(B) -> C + 'a{
    unsafe{
        Box::new( move |a| (*g)( (*f)(a) ) )
    }
}


fn main(){
   
    let id = |x : usize| x;
    unsafe{
       let f1 = compose2(&id, &id);
       f1(1);
       id(1);

       let rec = mk_rectangle2(Vector2::new(0.0, 0.0), Vector2::new(1.0, 2.0));
       println!("{}", rec(Vector2::new(1.0,0.0)));
    }


    
    //matrix::test_matrices();
    run_voxelized();
}




fn run_voxelized() {
    let def_width: usize = 800;
    let def_height: usize = 600;

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

    glfw_set_input_mode(win, GLFW_STICKY_KEYS, 1);

    let shaders = load_shaders_vf();

    let mut camera = Camera{pos : Vector3::new(0.0,0.0,0.0), look : Vector3::new(0.0,0.0,-1.0), up : Vector3::new(0.0, 1.0, 0.0)};


    let mut voxel_renderer = VoxelRenderer::new(&shaders);
    let mut win_info = WindowInfo{width: def_width, height: def_height, handle: win}; //will be updated each frame




    let mut renderer_tr_light = RendererVertFragDef::make(
        VERTEX_SIZE_COLOR_NORMAL,
        set_attrib_ptrs_color_normal,
        GL_TRIANGLES,
        String::from("lighting"));

    let mut renderer_lines = RendererVertFragDef::make(
        VERTEX_SIZE_COLOR,
        set_attrib_ptrs_color,
        GL_LINES,
        String::from("color")
    );

    let zero = Vector3::new(0.0, 0.0, 0.0);
    let offset = Vector3::new(0.1, 0.1, 0.1);
    let red = Vector3::new(1.0, 0.0, 0.0);
    let green = Vector3::new(0.0, 1.0, 0.0);
    let blue = Vector3::new(0.0, 0.0, 1.0);
    let white = Vector3::new(1.0, 1.0, 1.0);
    


    unsafe{
        cubic::test_cubic_octree(&mut renderer_tr_light);
    }

    add_grid3_color(&mut renderer_lines, zero, Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 1.0, 0.0), 1.0, 8, white);

    add_line3_color(&mut renderer_lines, Line3{start : zero, end : zero + red}, red);
    add_line3_color(&mut renderer_lines, Line3{start : zero, end : zero + green}, green);
    add_line3_color(&mut renderer_lines, Line3{start : zero, end : zero + blue}, blue);

    //add_triangle_color_normal(&mut renderer_tr_light, &Triangle3{p1 : Vector3::new(-0.3, 0.0, -1.0), p2 : Vector3::new(0.3, 0.0, -1.0), p3 : Vector3::new(0.0, 0.5, -1.0)}, &red, &Vector3::new(0.0, 0.0, 1.0));

    //add_square3_bounds_color(&mut renderer_lines, Square3{center : Vector3::new(-0.5, 0.5, -0.5), extent : 0.125 / 2.0}, red + green);




    //====================================
    let BLOCK_SIZE : f32 = 0.125;//2.0;
    let CHUNK_SIZE : usize = 128;//*2;

    let mut grid = dcm::VoxelMaterialGrid3::new(BLOCK_SIZE, CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE);




   

    let sphere1_ = Sphere{center : Vector3::new(4.1 as f32,4.2, 4.3), rad : 2.2};
    let sphere2_ = Sphere{center : Vector3::new(3.0 ,3.0, 3.0), rad : 1.0};
    let sphere1 = dcm::mk_sphere_mat(sphere1_.clone(), 1);
    let sphere11 = dcm::mk_sphere_mat(sphere1_.clone(), 1);
    let sphere2 = dcm::mk_sphere_mat(sphere2_.clone(), 2);
    //let aabb = mk_aabb(Vector3::new(4.0, 5.0, 4.0), Vector3::new(1.0, 1.0, 1.0));

    /* let perlin = Perlin::new();
    let val = perlin.get([42.4, 37.7, 2.8]);
    
    let sphere_disp = mk_sphere_displacement(sphere1_.clone(), box move |x| {
         perlin.get([x.x/2.0, x.y/2.0,x.z/2.0]) + 1.0
         
    }); */

    let den1 = dcm::union3_mat(sphere11, sphere2);
    let den = dcm::intersection3_mat_a(den1, sphere1);
    //let den = union3(den1, aabb);

    //dc::test_sample_normal();


    //ADAPTIVE---------
    // let sp_num = mk_sphere(Sphere{center : Vector3::new(-4.0, -4.0, -4.0), rad : 1.0});
    // let tree = timed(&|dt| format!("make tree took {} ms", dt / 1000000), &mut ||{
    //    make_tree(&sp_num, Vector3::new(-5.0, -5.0, -5.0), BLOCK_SIZE, CHUNK_SIZE, &mut renderer_lines)
    // });

    //-----------------

    //UNIFORM MANIFOLD DC
    let sp_num1 = mk_sphere(Sphere{center : Vector3::new(2.0, 2.0, -1.0), rad : 1.0});
    let sp_num2 = mk_sphere(Sphere{center : Vector3::new(2.0, 2.0, 1.001), rad : 1.0});
    let rec1 = mk_aabb(Vector3::new(2.0,2.0,0.0), Vector3::new(0.2,0.2,0.2));
    let den1 = union3(sp_num1, sp_num2);
    let den = union3(rec1, den1);
    let torusz = mk_torus_z(2.0, 0.8,Vector3::new(0.0,0.0,-4.0));
    let torusy = mk_torus_y(1.6, 0.67,Vector3::new(2.0,0.0,-4.0));
    let perlin = Perlin::new();
    let noise = noise_f32(perlin, Square3{center : Vector3::new(1.0,-1.0,1.0), extent : 3.5} );//perlin.get([p.x,p.y,p.z])  ;
    let two_torus = union3(torusz, torusy);
    let den2 = difference3(two_torus, mk_aabb(Vector3::new(0.0, 3.0, -4.0), Vector3::new(1.5,1.5,1.5)));
    let den3 = union3(den2, mk_sphere(Sphere{center : Vector3::new(0.0, 2.0, -4.0), rad : 1.0}));
    let den4 = union3(den3, mk_obb(Vector3::new(1.0, 1.0, 0.0), Vector3::new(1.0, -1.0, 0.0).normalize(), Vector3::new(1.0, 1.0, 0.5).normalize(), Vector3::new(1.0, 0.5, 0.2)));
    //let den4 = union3(den3, mk_half_space_pos(Plane{point : Vector3::new(0.0, 2.0, -4.0), normal : Vector3::new(1.0, 1.0, 0.0).normalize()}));
    //let den = f;
    //TODO implement DenFn differently, like noise library
    //construct_grid(&den4, Vector3::new(-3.0, -3.0, -8.0), BLOCK_SIZE, CHUNK_SIZE, 8, &mut renderer_tr_light, &mut renderer_lines);

    let test_sphere = Sphere{center : Vector3::new(2.7, 1.0, 0.0), rad : 2.4};
    let test_sphere2 = Sphere{center : Vector3::new(2.7, 3.0, 0.0), rad : 2.4};
    let test_sphere3 = Sphere{center : Vector3::new(2.7, 1.0, 2.7), rad : 1.4};
    let ts1 = mk_sphere(test_sphere);
    let ts2 = mk_sphere(test_sphere2);
    let ts22 = mk_sphere(test_sphere3);
    let ts3 = difference3(ts1, ts2);
    let ts4 = difference3(ts3, ts22);
    //add_sphere_color(&mut renderer_tr_light, &test_sphere, 100, 100, Vector3::new(1.0, 1.0, 1.0));
    //construct_grid(&ts4, Vector3::new(-0.5, -2.5, -2.5), 1.0/8.0, 2*8*8, 32, &mut renderer_tr_light, &mut renderer_lines);
    ///------------------

    // let contour_data = timed(&|dt| format!("op took {} ms", dt / 1000000), &mut ||{
    //     dcm::fill_in_grid(&mut grid, &den, Vector3::new(0.0, 0.0, 0.0));
    //     dcm::make_contour(&grid, &den, 16, &mut renderer_lines) //accurary depends on grid resolution
    // });


    shaders.get("lighting").unwrap().enable();
    shaders.get("lighting").unwrap().set_vec3f("pointLight.pos" ,Vector3::new(0.0, 8.0,0.0));
    shaders.get("lighting").unwrap().set_vec3f("pointLight.color" ,(red + green + blue) * 15.0);

    // println!("generated {} triangles", contour_data.triangles.len());

    // for i in 0..contour_data.triangles.len(){
    //     //add_triangle_color_normal(&mut renderer_tr_light, &contour_data.triangles[i], &contour_data.triangle_colors[i / 2], &contour_data.triangle_normals[i / 2]);
    // }
    //===================================


    fn shader_data(shader: &Program, win: &WindowInfo, camera : &Camera) -> bool{
        let aspect = win.width as f32 / win.height as f32;
       /* let height = 16.0;
        let width = height;*/
        let id_mat = [
            1.0,0.0,0.0,0.0,
            0.0,1.0,0.0,0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0];




        let persp = perspective(90.0, aspect, 0.1, 16.0);
        let view = view_dir(camera.pos, camera.look, camera.up);

        println!("{}", persp);


        shader.set_float4x4("P", false, persp.as_slice());
        shader.set_float4x4("V", false, view.as_slice());

        true

    };

    fn shader_data_lines(shader: &Program, win: &WindowInfo, camera : &Camera) -> bool{
        let aspect = win.width as f32 / win.height as f32;
       /* let height = 16.0;
        let width = height;*/
        let id_mat = [
            1.0,0.0,0.0,0.0,
            0.0,1.0,0.0,0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0];




        let persp = perspective(90.0, aspect, 0.1, 16.0);
        let view = view_dir(camera.pos, camera.look, camera.up);


        shader.set_float4x4("P", false, persp.as_slice());
        shader.set_float4x4("V", false, view.as_slice());
   
        glfw_get_key(win.handle, GLFW_KEY_TAB) != GLFW_PRESS
    };

    let provider = RenderDataProvider{pre_render_state: None, post_render_state: None, shader_data: Some(Box::new(shader_data))};
    let provider_lines = RenderDataProvider{pre_render_state: None, post_render_state: None, shader_data: Some(Box::new(shader_data_lines))};


    let mut render_info = RenderInfo{renderer: Box::new(renderer_tr_light), provider};//moved
    let mut render_info_lines = RenderInfo{renderer: Box::new(renderer_lines), provider: provider_lines}; //moved


    let id_trs = voxel_renderer.push(RenderLifetime::Manual, RenderTransform::None, render_info).unwrap();
    let id_lns = voxel_renderer.push(RenderLifetime::Manual, RenderTransform::None, render_info_lines).unwrap();


    voxel_renderer.manual_mut(&id_trs).construct();
    voxel_renderer.manual_mut(&id_lns).construct();

    let mut last_frame_time = precise_time_ns();
    let mut cur_frame_time = last_frame_time;


   gl_enable(GL_DEPTH_TEST);
    while !glfw_window_should_close(win){

        gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT); 

        last_frame_time = cur_frame_time;
        cur_frame_time = precise_time_ns();

        let dt_ns = cur_frame_time - last_frame_time;


        update_win_dim_info(&mut win_info);
        process_input(win, dt_ns, &mut camera);

        gl_clear_color(0.2, 0.3, 0.3, 1.0);

        voxel_renderer.draw(&win_info, &camera);


        glfw_swap_buffers(win);
        glfw_poll_events();

        check_for_gl_errors();
    }

    voxel_renderer.manual_mut(&id_trs).deconstruct();
    voxel_renderer.manual_mut(&id_trs).reset();

    voxel_renderer.manual_mut(&id_lns).deconstruct();
    voxel_renderer.manual_mut(&id_lns).reset();

    glfw_terminate();
}




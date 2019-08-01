//#![feature(box_syntax, box_patterns, clone_closures, copy_closures)]

extern crate generic_array;
extern crate typenum;
extern crate alga;
extern crate libc;
extern crate ansi_term;
extern crate time;
extern crate rand;
extern crate noise;
extern crate num;
extern crate glfw;


//mod qef_bindings;

use libc::*;
use std::ffi::*;

mod test;
mod graphics;
mod graphics_util;
mod renderer;
mod util;
#[macro_use]
mod math;
#[macro_use]
mod matrix;


mod extraction;
/*use extraction::dcm;
use extraction::cubic;
use extraction::dc;
*/

use extraction::uniform_manifold_dc;
use extraction::uniform_manifold_dc::*;

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
use std::ops::*;
use rand::distributions::{Sample, Range};
use typenum::{Cube as _, Prod, Unsigned};
use generic_array::*;
use matrix::*;


fn handle_input(glfw : &mut glfw::Glfw, win : &mut glfw::Window, dt_ns : u64, camera : &mut Cam){
    if win.get_key(glfw::Key::Escape) == glfw::Action::Press{
        win.set_should_close(true);
    }else{
        if win.get_key(glfw::Key::Tab) == glfw::Action::Press{
            //debug

            let (w, h) = win.get_size();

            println!("[debug] window size: ({}, {})", w, h);

        }
    }
    let dt_s : f32 = dt_ns as f32 / 1000000000.0;

    if win.get_key(glfw::Key::W) == glfw::Action::Press{
        camera.pos += camera.look * dt_s as f32;

    }

    if win.get_key(glfw::Key::S) == glfw::Action::Press{
        camera.pos -= camera.look * dt_s as f32;
    }

    if win.get_key(glfw::Key::A) == glfw::Action::Press{
        let right = camera.look.cross(camera.up);

        camera.pos -= right * dt_s as f32;
    }

    if win.get_key(glfw::Key::D) == glfw::Action::Press{
        let right = camera.look.cross(camera.up);

        camera.pos += right * dt_s as f32;
    }

    if win.get_key(glfw::Key::Space) == glfw::Action::Press{

        camera.pos += camera.up * dt_s as f32;
    }

    if win.get_key(glfw::Key::LeftShift) == glfw::Action::Press{

        camera.pos -= camera.up * dt_s as f32;
    }

    if win.get_key(glfw::Key::Left) == glfw::Action::Press{

        let mat = rot_mat3(camera.up, std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
    }
    if win.get_key(glfw::Key::Right) == glfw::Action::Press{

        let mat = rot_mat3(camera.up, -std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
    }
    if win.get_key(glfw::Key::Kp0) == glfw::Action::Press{

        let mat = rot_mat3(camera.look, std::f32::consts::PI * dt_s / 2.0);
        camera.up = (mat * camera.up).normalize();
    }
    if win.get_key(glfw::Key::KpDecimal) == glfw::Action::Press{

        let mat = rot_mat3(camera.look, -std::f32::consts::PI * dt_s / 2.0);
        camera.up = (mat * camera.up).normalize();
    }
    if win.get_key(glfw::Key::Up) == glfw::Action::Press{
        let right = camera.look.cross(camera.up);
        let mat = rot_mat3(right, std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
        camera.up = (mat * camera.up).normalize();
    }
    if win.get_key(glfw::Key::Down) == glfw::Action::Press{
        let right = camera.look.cross(camera.up);
        let mat = rot_mat3(right, -std::f32::consts::PI * dt_s / 2.0);
        camera.look = (mat * camera.look).normalize();
        camera.up = (mat * camera.up).normalize();
    }

}


fn run(){
    let zero = Vec3::new(0.0, 0.0, 0.0);
    let offset = Vec3::new(0.1, 0.1, 0.1);
    let red = Vec3::new(1.0, 0.0, 0.0);
    let green = Vec3::new(0.0, 1.0, 0.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let def_width: u32 = 800;
    let def_height: u32 = 600;
    let title = "Voxelized 3D";
    let cam = renderer::Cam{look : Vec3::new(0.0, 0.0, -1.0), up : Vec3::new(0.0, 1.0, 0.0), pos : Vec3::new(0.0, 0.0, 0.0)};
    let mut renderer = Renderer::new(cam);
    renderer.init(def_width, def_height, title);
    renderer.set_framebuffer_size_callback(|w, h| println!("new dims {} {}", w, h));

    let BLOCK_SIZE : f32 = 0.125;//2.0;
    let CHUNK_SIZE : usize = 128;//*2;



    //UNIFORM MANIFOLD DC
    let sp_num1 = mk_sphere(Sphere{center : Vec3::new(2.0, 2.0, -1.0), rad : 1.0});
    let sp_num2 = mk_sphere(Sphere{center : Vec3::new(2.0, 2.0, 1.001), rad : 1.0});
    let rec1 = mk_aabb(Vec3::new(2.0,2.0,0.0), Vec3::new(0.2,0.2,0.2));
    let den1 = union3(sp_num1, sp_num2);
    let den = union3(rec1, den1);
    let torusz = mk_torus_z(2.0, 0.8,Vec3::new(0.0,0.0,-4.0));
    let torusy = mk_torus_y(1.6, 0.67,Vec3::new(2.0,0.0,-4.0));
    let perlin = Perlin::new();
    let noise = noise_f32(perlin, Cube{center : Vec3::new(1.0,-1.0,1.0), extent : 3.5} );//perlin.get([p.x,p.y,p.z])  ;
    let two_torus = union3(torusz, torusy);
    let den2 = difference3(two_torus, mk_aabb(Vec3::new(0.0, 3.0, -4.0), Vec3::new(1.5,1.5,1.5)));
    let den3 = union3(den2, mk_sphere(Sphere{center : Vec3::new(0.0, 2.0, -4.0), rad : 1.0}));
    let den4 = union3(den3, mk_obb(Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, -1.0, 0.0).normalize(), Vec3::new(1.0, 1.0, 0.5).normalize(), Vec3::new(1.0, 0.5, 0.2)));
    //let den4 = union3(den3, mk_half_space_pos(Plane{point : Vec3::new(0.0, 2.0, -4.0), normal : Vec3::new(1.0, 1.0, 0.0).normalize()}));
    //let den = f;
    //TODO implement DenFn differently, like noise library
    //construct_grid(&den4, Vec3::new(-3.0, -3.0, -8.0), BLOCK_SIZE, CHUNK_SIZE, 8, &mut renderer_tr_light, &mut renderer_lines);

    let test_sphere = Sphere{center : Vec3::new(2.7, 1.0, 0.0), rad : 2.4};
    let test_sphere2 = Sphere{center : Vec3::new(2.7, 3.0, 0.0), rad : 2.4};
    let test_sphere3 = Sphere{center : Vec3::new(2.7, 1.0, 2.7), rad : 1.4};
    let ts1 = mk_sphere(test_sphere);
    let ts2 = mk_sphere(test_sphere2);
    let ts22 = mk_sphere(test_sphere3);
    let ts3 = difference3(ts1, ts2);
    let ts4 = difference3(ts3, ts22);
    //add_sphere_color(&mut renderer_tr_light, &test_sphere, 100, 100, Vec3::new(1.0, 1.0, 1.0));
    //construct_grid(&ts4, Vec3::new(-0.5, -2.5, -2.5), 1.0/8.0, 2*8*8, 32, &mut renderer.render_triangles_lighting_pos_color_normal, &mut renderer.render_lines_pos_color);
    construct_grid(&den4, Vec3::new(-4.0, -2.5, -4.5), 1.0/8.0, 2*8*8, 32, &mut renderer.render_triangles_lighting_pos_color_normal, &mut renderer.render_lines_pos_color);


    add_triangle_color(&mut renderer.render_triangles_pos_color, Triangle3{p1 : vec3![-0.2, 0.0, -1.0], p2 : vec3![0.2, 0.0, -1.0], p3 : vec3![0.0, 0.3, -1.0]}, red);

    add_grid3_pos_color(&mut renderer.render_lines_pos_color, Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.0, 8, white);


    renderer.run(|renderer, dt_ns| {

        handle_input(renderer.glfw.as_mut().unwrap(), renderer.window.as_mut().unwrap(), dt_ns, &mut renderer.camera);

        gl_enable(GL_DEPTH_TEST);
        gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        gl_clear_color(0.3, 0.2, 0.6, 1.0);
    });
}

fn main(){

    run();
}


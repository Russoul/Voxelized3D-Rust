
extern crate generic_array;

mod graphics;
mod vector;

use graphics::*;
use std::ptr;
use generic_array::*;
use vector::*;

extern fn framebuf_sz_cb(win : *mut GlfwWindow, w : isize, h : isize){
    gl_viewport(0,0,w,h);
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


fn test_vectors(){
    let ar1 = Vector::new(arr![usize;1,2,3,4]);
    let ar2 = ar1.clone();
    ar1.print();

    let ar3
        = &ar1 + &ar1;
    ar3.print();
    let ar4 = &ar1 * &ar1;
    println!("{}", ar4);
    println!("{}", ar3);
    let mapped = Vector::new(ar1.get().map(|x| x + 1));
    println!("{}", mapped);
}


fn main() {
    test_vectors();
   

    glfw_init();
    glfw_window_hint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfw_window_hint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfw_window_hint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    let win = glfw_create_window(800, 600, "Voxelized2D");

    if win == ptr::null_mut(){
        glfw_terminate();
        panic!("Failed to create GLFW window !");
    }

    glfw_make_context_current(win);
    glad_load_gl_loader();

    println!("Using GL version: {}", gl_get_string(GL_VERSION));
    
    glfw_set_framebuffer_size_callback(win, framebuf_sz_cb);

    while !glfw_window_should_close(win){

        process_input(win);

        gl_clear_color(0.2, 0.3, 0.3, 1.0);
        gl_clear(GL_COLOR_BUFFER_BIT);
        
        glfw_swap_buffers(win);
        glfw_poll_events();
    }

    glfw_terminate();
}

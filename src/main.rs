mod graphics;
use graphics::*;
use std::ptr;

extern fn framebuf_sz_cb(win : *mut Glfw_window, w : isize, h : isize){
    gl_viewport(0,0,w,h);
}

fn process_input(win : *mut Glfw_window){
    if glfw_get_key(win, GLFW_KEY_ESCAPE) == GLFW_PRESS{
        glfw_set_window_should_close(win, true);
    }
}

fn main() {
    println!("Hello, world!"); //TODO

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

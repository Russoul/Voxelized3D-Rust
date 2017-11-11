extern crate libc;
use self::libc::*;
use std::ffi::CString;
use std::ptr;


pub static GLFW_CONTEXT_VERSION_MAJOR : usize = 0x00022002;
pub static GLFW_CONTEXT_VERSION_MINOR : usize = 0x00022003;
pub static GLFW_OPENGL_PROFILE : usize = 0x00022008;
pub static GLFW_OPENGL_CORE_PROFILE : usize = 0x00032001;

pub static GLFW_KEY_ESCAPE : usize = 256;

pub static GLFW_PRESS : usize = 1;
pub static GLFW_RELEASE : usize = 0;
pub static GLFW_REPEAT : usize = 2;

pub static GL_COLOR_BUFFER_BIT : usize = 0x00004000;

pub enum Glfw_window{}

#[link(name = "GL")]
#[link(name = "glfw")]
#[link(name = "rsutil")]
extern {
    fn glfwInit();
    fn glfwWindowHint(_enum : c_uint, _val : c_uint);
    fn glfwCreateWindow(width : c_int,
                        height : c_int,
                        title : *const c_char,
                        ptr1 : *mut c_void,
                        ptr2 : *mut c_void) -> *mut Glfw_window;
    fn glfwTerminate();
    fn glfwMakeContextCurrent(window : *mut Glfw_window);
    fn _gladLoadGLLoader();
    fn glViewport(x : c_uint, y : c_uint, w : c_uint, h : c_uint);
    fn glfwSetFramebufferSizeCallback(
        win : *mut Glfw_window,
        callback : extern fn(* mut Glfw_window, isize, isize));
    fn glfwWindowShouldClose(win : *mut Glfw_window) -> bool;
    fn glfwSwapBuffers(win : *mut Glfw_window);
    fn glfwPollEvents();
    fn glfwGetKey(win : *mut Glfw_window, key : usize) -> usize;
    fn glfwSetWindowShouldClose(win : *mut Glfw_window, val : bool);

    fn glClearColor(r : f32, g : f32, b : f32, a : f32);
    fn glClear(val : usize);
}



pub fn glad_load_gl_loader(){
    unsafe{
        _gladLoadGLLoader();
    }
}

pub fn glfw_init(){
    unsafe{
        glfwInit();
    }
}

pub fn glfw_terminate(){
    unsafe{
        glfwTerminate();
    }
}

pub fn glfw_window_hint(c_enum : usize, val : usize){
    unsafe{
        glfwWindowHint(c_enum as c_uint, val as c_uint);
    }
}

pub fn glfw_create_window(w : isize,
                          h : isize,
                          title : &str) -> *mut Glfw_window{
    let p1 : *mut c_void = ptr::null_mut();
    let p2 : *mut c_void = ptr::null_mut();
    unsafe{
        glfwCreateWindow(w as c_int, h as c_int, CString::new(title)
                         .unwrap().as_ptr(),
                         p1,
                         p2)
    }
}

pub fn glfw_make_context_current(win : *mut Glfw_window){
    unsafe{
        glfwMakeContextCurrent(win);
    }
}

pub fn gl_viewport(x : isize, y : isize, w : isize, h : isize){
    unsafe{
        glViewport(x as c_uint, y as c_uint, w as c_uint, h as c_uint);
    }
}

pub fn glfw_set_framebuffer_size_callback(win : *mut Glfw_window, cb:
                                          extern fn(*mut Glfw_window, isize, isize)){
    unsafe {
        glfwSetFramebufferSizeCallback(win, cb);
    }
}

pub fn glfw_window_should_close(win : *mut Glfw_window) -> bool {
    unsafe{
        glfwWindowShouldClose(win)
    }
}

pub fn glfw_swap_buffers(win : *mut Glfw_window){
    unsafe{
        glfwSwapBuffers(win);
    }
}

pub fn glfw_poll_events(){
    unsafe{
        glfwPollEvents();
    }
}

pub fn glfw_get_key(win : *mut Glfw_window, key : usize) -> usize{
    unsafe{
        glfwGetKey(win, key)
    }
}

pub fn glfw_set_window_should_close(win : *mut Glfw_window, val : bool){
    unsafe{
        glfwSetWindowShouldClose(win, val);
    }
}

pub fn gl_clear_color(r : f32, g : f32, b : f32, a : f32){
    unsafe{
        glClearColor(r,g,b,a);
    }
}

pub fn gl_clear(val : usize){
    unsafe{
        glClear(val);
    }
}

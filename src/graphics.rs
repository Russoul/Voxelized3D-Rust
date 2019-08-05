#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use libc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;
use std::str;
use std;
use matrix::{Vec3};

pub struct WindowInfo{
    pub width: usize,
    pub height: usize,
    pub handle: *mut GlfwWindow, //TODO many GL functions take mutable ptr to GlfwWindow, but it is unsafe to leave it as mut in this struct
}

pub enum GlfwWindow{}
pub enum GlfwMonitor{}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Program{
    pub id: usize,
    
}

impl Program{
    pub fn get_uniform(&self, name: &str) -> isize{
        gl_get_uniform_location(self.id, name)
    }

    pub fn is_in_use(&self) -> bool {
        let mut cur_id = 0;
        gl_get_integerv(GL_CURRENT_PROGRAM, &mut cur_id);
        self.id == cur_id as usize
    }

    pub fn enable(&self){
        if !self.is_in_use(){
            gl_use_program(self.id);
        }
    }

    pub fn disable(&self){
        if self.is_in_use(){
            gl_use_program(0);
        }
    }

    pub fn set_bool(&self, name: &str, val: bool){
        self.enable();
        gl_uniform1i(self.get_uniform(name), if val {1} else {0});
    }


    pub fn set_int(&self, name: &str, val: isize){
        self.enable();
        gl_uniform1i(self.get_uniform(name), val);
    }

    pub fn set_float(&self, name: &str, val: f32){
        self.enable();
        gl_uniform1f(self.get_uniform(name), val);
    }

    pub fn set_float2(&self, name: &str, val1: f32, val2: f32){
        self.enable();
        gl_uniform2f(self.get_uniform(name), val1, val2);
    }

    pub fn set_float3(&self, name: &str, val1: f32, val2: f32, val3: f32){
        self.enable();
        gl_uniform3f(self.get_uniform(name), val1, val2, val3);
    }

    pub fn set_vec3f(&self, name: &str, vec : Vec3<f32>){
        self.enable();
        gl_uniform3f(self.get_uniform(name), vec.x, vec.y, vec.z);
    }

    pub fn set_float4(&self, name: &str, val1: f32, val2: f32, val3: f32, val4: f32){
        self.enable();
        gl_uniform4f(self.get_uniform(name), val1, val2, val3, val4);
    }


    //mat is assumed to be in column major order
    pub fn set_float4x4(&self, name: &str, transpose: bool, mat: &[f32]){
        self.enable();
        gl_uniform_matrix4fv(self.get_uniform(name), transpose, mat)
    }
    
}

#[derive(Debug)]
#[repr(C)]
pub struct GlfwVidMode{
    width : c_int,
    height : c_int,
    red_bits : c_int,
    green_bits : c_int,
    blue_bits : c_int,
    refresh_rate : c_int,
}

#[cfg_attr(windows, link(name = "opengl32"))]
#[cfg_attr(linux, link(name = "GL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenGL", kind = "framework"))]
#[cfg_attr(target_os = "macos", link(name = "Cocoa", kind = "framework"))]
#[cfg_attr(target_os = "macos", link(name = "IOKit", kind = "framework"))]
#[cfg_attr(target_os = "macos", link(name = "CoreVideo", kind = "framework"))]

//-framework Cocoa -framework OpenGL -framework IOKit -framework CoreVideo
//#[link(name = "OpenGL", kind = "framework")]
#[link(name = "glfw3")]
#[link(name = "rsutil")]
extern {
    fn glfwInit();
    fn glfwWindowHint(_enum : c_uint, _val : c_uint);
    fn glfwCreateWindow(width : c_int,
                        height : c_int,
                        title : *const c_char,
                        ptr1 : *mut c_void,
                        ptr2 : *mut c_void) -> *mut GlfwWindow;
    fn glfwTerminate();
    fn glfwMakeContextCurrent(window : *mut GlfwWindow);
    fn _gladLoadGLLoader();
    fn glViewport(x : c_uint, y : c_uint, w : c_uint, h : c_uint);
    fn glfwSetFramebufferSizeCallback(
        win : *mut GlfwWindow,
        callback : extern fn(* mut GlfwWindow, isize, isize));
    fn glfwSetKeyCallback(win : *mut GlfwWindow,
                          callback : extern fn (*mut GlfwWindow, isize, isize, isize, isize));
    fn glfwSetMouseButtonCallback(win : *mut GlfwWindow, cb :
                                  extern fn(*mut GlfwWindow, isize, isize, isize));
    fn glfwSetErrorCallback(cb : extern fn(c_int, *const c_char));//TODO works with &str?
    fn glfwWindowShouldClose(win : *mut GlfwWindow) -> bool;
    fn glfwSwapBuffers(win : *mut GlfwWindow);
    fn glfwPollEvents();
    fn glfwGetKey(win : *mut GlfwWindow, key : usize) -> usize;
    fn glfwSetWindowShouldClose(win : *mut GlfwWindow, val : bool);
    fn glfwSetInputMode(win : *mut GlfwWindow, mode : usize, value : isize);

    fn glfwGetWindowSize(win : *mut GlfwWindow, w : *mut usize, h : *mut usize);
    fn glfwGetVideoMode(mon : *mut GlfwMonitor) -> *mut GlfwVidMode;
    fn glfwGetPrimaryMonitor() -> *mut GlfwMonitor;
    fn glfwSetWindowPos(win : *mut GlfwWindow, x : usize, y : usize);
    fn glfwSwapInterval(mode : isize);
    
    fn glClearColor(r : f32, g : f32, b : f32, a : f32);
    fn glClear(val : usize);
    fn glGetString(val : usize) -> *mut c_char;
    fn glEnable(val : usize);
    fn glDisable(val : usize);

    fn glCreateProgram()->usize;
    fn glCreateShader(typee: usize)->usize;
    fn glShaderSource(shader: usize, count: usize,
                      sources: *const *const c_char,
                      lens: *const usize);
    fn glCompileShader(id: usize);
    fn glGetShaderiv(id: usize, param: usize, res: *mut usize);
    fn glGetShaderInfoLog(shader:usize, max_len: usize, len: *mut usize,
                          info: *mut u8);
    fn glAttachShader(program: usize, shader: usize);
    fn glLinkProgram(program: usize);
    fn glValidateProgram(program: usize);
    fn glDeleteShader(shader: usize);


    fn glGenVertexArrays(size: usize, arrs: *mut usize);
    fn glGenBuffers(size: usize, bufs: *mut usize);
    fn glDeleteVertexArrays(size: usize, arrs: *const usize);
    fn glDeleteBuffers(size: usize, bufs: *const usize);

    fn glBindVertexArray(ar: usize);
    fn glBindBuffer(typee: usize, buf: usize);
    fn glBufferData(target: usize, size: usize, data: *const c_void, usage: usize);
    fn glDrawElements(mode: usize, count: usize, typee: usize, indices: usize);

    fn glVertexAttribPointer(index: usize, size: usize, typee: usize, normalized: bool, stride: usize, offset: usize);
    fn glEnableVertexAttribArray(index: usize);


    fn glGetUniformLocation(program: usize, name: *const c_char)->isize;
    fn glUniform1i(loc: isize, val: isize);
    fn glUniform1f(loc: isize, val: f32);
    fn glUniform2f(loc: isize, val1: f32, val2: f32);
    fn glUniform3f(loc: isize, val1: f32, val2: f32, val3: f32);
    fn glUniform4f(loc: isize, val1: f32, val2: f32, val3: f32, val4: f32);
    fn glUniformMatrix4fv(loc: isize, count: usize, transpose: usize, matrix_col_major: *const f32);
    fn glUseProgram(id: usize);
    fn glGetIntegerv(param: usize, out: *mut isize);
    fn glGetError() -> usize;
    fn glActiveTexture(tex : usize);
    fn glGenTextures(count : u32, textures : *mut u32);
    fn glBindTexture(ty : usize, tex : u32);
    fn glTexParameteri(ty : usize, par : usize, val : usize);
    fn glTexImage2D(target : usize, level : i32, internal_format : usize, w : u32, h : u32, border : i32, format : usize, ty : usize, data : *const c_void);
    fn glGenerateMipmap(target : usize);
    fn glDebugMessageCallback(source : usize, ty : usize, id : u32, severity : usize, size : u32, msg : *const c_char, par : *const c_void);
}


pub fn gl_generate_mipmap(target : usize){
    unsafe{
        glGenerateMipmap(target);
    }
}

pub fn gl_tex_image_2d<T>(target : usize, level : i32, internal_format : usize, w : u32, h : u32, border : i32, format : usize, ty : usize, data : &[T]){
    unsafe{
        glTexImage2D(target, level, internal_format, w, h, border, format, ty, std::mem::transmute(data.as_ptr()));
    }
}

pub fn gl_tex_parameteri(ty : usize, par : usize, val : usize){
    unsafe{
        glTexParameteri(ty, par, val);
    }
}

pub fn gl_gen_textures(count : u32, textures : &mut [u32]){
    unsafe{
        glGenTextures(count, std::mem::transmute(textures.as_mut_ptr()));
    }
}

pub fn gl_bind_texture(ty : usize, tex : u32){
    unsafe{
        glBindTexture(ty, tex);
    }
}

pub fn gl_active_texture(tex : usize){
    unsafe{glActiveTexture(tex)};
}

pub fn gl_enable(val : usize){
    unsafe{glEnable(val);}
}

pub fn gl_disable(val : usize){
    unsafe{glDisable(val);}
}

pub fn glfw_set_input_mode(win : *mut GlfwWindow, mode : usize, value : isize){
    unsafe{
        glfwSetInputMode(win, mode, value);
    }
}

pub fn gl_delete_shader(shader: usize){
    unsafe{glDeleteShader(shader)}
}

pub fn gl_get_error() -> usize{
    unsafe{glGetError()}
}

pub fn gl_uniform_matrix4fv(loc: isize, transpose: bool, mat: &[f32]){
    unsafe{
        glUniformMatrix4fv(loc, 1, if transpose {GL_TRUE}else{GL_FALSE}, mat.as_ptr())
    }
}

pub fn gl_get_integerv(param: usize, out: *mut isize){
    unsafe{glGetIntegerv(param, out)}
}

pub fn gl_use_program(id: usize){
    unsafe{
        glUseProgram(id);
    }
}

pub fn gl_uniform1i(loc: isize, val: isize){
    unsafe{
        glUniform1i(loc, val)
    }
}

pub fn gl_uniform1f(loc: isize, val: f32){
    unsafe{
        glUniform1f(loc, val)
    }
}

pub fn gl_uniform2f(loc: isize, val1: f32, val2: f32){
    unsafe{
        glUniform2f(loc, val1, val2)
    }
}

pub fn gl_uniform3f(loc: isize, val1: f32, val2: f32, val3: f32){
    unsafe{
        glUniform3f(loc, val1, val2, val3)
    }
}

pub fn gl_uniform4f(loc: isize, val1: f32, val2: f32, val3: f32, val4: f32){
    unsafe{
        glUniform4f(loc, val1, val2, val3, val4)
    }
}


pub fn gl_draw_elements(mode: usize, count: usize, typee: usize, indices: usize){
    unsafe{glDrawElements(mode, count, typee, indices)};
}

pub fn gl_gen_vertex_arrays() -> usize{
    unsafe{
        let mut ar: usize = 0;
        glGenVertexArrays(1, &mut ar);

        ar
    }
}

pub fn gl_gen_buffers() -> usize{
    unsafe{
        let mut buf: usize = 0;
        glGenBuffers(1, &mut buf);

        buf
    }
}

pub fn gl_delete_vertex_arrays(ar: usize){
    unsafe{
        glDeleteVertexArrays(1, &ar)
    }
}

pub fn gl_delete_buffers(buf : usize){
    unsafe{
        glDeleteBuffers(1, &buf)
    }
}

pub fn gl_bind_vertex_array(ar: usize){
    unsafe{
        glBindVertexArray(ar);
    }
}

pub fn gl_bind_buffer(typee:usize, buf: usize){
    unsafe{
        glBindBuffer(typee, buf);
    }
}

//be precise about 'T' type argument
pub fn gl_buffer_data<T>(target: usize, num : usize, data: &[T], usage: usize){
    unsafe{
        glBufferData(target, std::mem::size_of::<T>() * num, std::mem::transmute::<*const T,*const c_void>(data.as_ptr()), usage);
    }
}

pub fn gl_vertex_attrib_pointer(index: usize, size: usize, typee: usize, norm: bool, stride: usize, offset: usize){
    unsafe{glVertexAttribPointer(index, size, typee, norm, stride, offset)}
}

pub fn gl_enable_vertex_attrib_array(index: usize){
    unsafe{glEnableVertexAttribArray(index)}
}

pub fn gl_get_uniform_location(program: usize, name : &str)->isize{
    unsafe{glGetUniformLocation(program, CString::new(name).unwrap().as_ptr())}
}

pub fn gl_attach_shader(prog: usize, shader: usize){
    unsafe{glAttachShader(prog, shader)}
}

pub fn gl_link_program(prog: usize){
    unsafe{glLinkProgram(prog)}
}

pub fn gl_validate_program(prog: usize){
    unsafe{glValidateProgram(prog)}
}

pub fn gl_get_shader_info_log(shader: usize) -> String{
    unsafe{
        const len: usize = 4096;//TODO placeholder
        let mut info : Vec<u8> = Vec::with_capacity(len);
        let mut len_ret : usize = 0;
        glGetShaderInfoLog(shader, len, &mut len_ret, info.as_mut_ptr());
        for i in info.iter(){
            println!("{}", i)
        }
        info.set_len(len_ret);
        let string = String::from_utf8(info).unwrap();

        string
        
    }
}

pub fn gl_get_shaderiv(id: usize, param: usize, res: *mut usize){
    unsafe{glGetShaderiv(id, param, res)}
}

pub fn gl_compile_shader(id:usize){
    unsafe{glCompileShader(id)}
}

pub fn gl_shader_source(shader: usize, source: &str){
    unsafe{
        let one: usize = 1;
        glShaderSource(shader, one, &(source.as_ptr() as *const i8), &source.len());
    }
}

pub fn gl_create_shader(typee: usize)->usize{
    unsafe{glCreateShader(typee)}
}

pub fn gl_create_program()->usize{
    unsafe{glCreateProgram()}
}

pub fn glfw_set_error_callback(cb : extern fn(c_int, *const c_char)){
    unsafe{
        glfwSetErrorCallback(cb);
    }
}

pub fn glfw_swap_interval(mode : isize){
    unsafe{
        glfwSwapInterval(mode);
    }
}

pub fn glfw_set_window_pos(win : *mut GlfwWindow, x : usize, y : usize){
    unsafe{
        glfwSetWindowPos(win, x, y);
    }
}

pub fn glfw_get_window_size(win : *mut GlfwWindow, w : *mut usize, h : *mut usize){
    unsafe{
        glfwGetWindowSize(win, w, h);
    }
}

pub fn glfw_get_video_mode(mon : *mut GlfwMonitor) -> *mut GlfwVidMode{
    unsafe{
        glfwGetVideoMode(mon)
    }
}

pub fn glfw_get_primary_monitor() -> *mut GlfwMonitor{
    unsafe{
        glfwGetPrimaryMonitor()
    }
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

pub fn glfw_create_window(w : usize,
                          h : usize,
                          title : &str) -> *mut GlfwWindow{
    let p1 : *mut c_void = ptr::null_mut();
    let p2 : *mut c_void = ptr::null_mut();
    unsafe{
        glfwCreateWindow(w as c_int, h as c_int, CString::new(title)
                         .unwrap().as_ptr(),
                         p1,
                         p2)
    }
}

pub fn glfw_make_context_current(win : *mut GlfwWindow){
    unsafe{
        glfwMakeContextCurrent(win);
    }
}

pub fn gl_viewport(x : isize, y : isize, w : isize, h : isize){
    unsafe{
        glViewport(x as c_uint, y as c_uint, w as c_uint, h as c_uint);
    }
}

pub fn glfw_set_framebuffer_size_callback(win : *mut GlfwWindow, cb:
                                          extern fn(*mut GlfwWindow, isize, isize)){
    unsafe {
        glfwSetFramebufferSizeCallback(win, cb);
    }
}

pub fn glfw_set_key_callback(win : *mut GlfwWindow, cb:
                             extern fn (*mut GlfwWindow, isize, isize, isize, isize)){
    unsafe{
        glfwSetKeyCallback(win, cb);
    }
}

pub fn glfw_set_mouse_button_callback(win : *mut GlfwWindow, cb:
                                      extern fn (*mut GlfwWindow, isize, isize, isize)){
    unsafe{
        glfwSetMouseButtonCallback(win, cb);
    }
}

pub fn glfw_window_should_close(win : *mut GlfwWindow) -> bool {
    unsafe{
        glfwWindowShouldClose(win)
    }
}

pub fn glfw_swap_buffers(win : *mut GlfwWindow){
    unsafe{
        glfwSwapBuffers(win);
    }
}

pub fn glfw_poll_events(){
    unsafe{
        glfwPollEvents();
    }
}

pub fn glfw_get_key(win : *mut GlfwWindow, key : usize) -> usize{
    unsafe{
        glfwGetKey(win, key)
    }
}

pub fn glfw_set_window_should_close(win : *mut GlfwWindow, val : bool){
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

pub fn gl_get_string<'a>(val : usize) -> & 'a str{
    unsafe{
        CStr::from_ptr(glGetString(val)).to_str().unwrap()
    }
}


pub const  GL_DEPTH_BUFFER_BIT : usize = 0x00000100;
pub const  GL_STENCIL_BUFFER_BIT : usize = 0x00000400;
pub const  GL_COLOR_BUFFER_BIT : usize = 0x00004000;
pub const  GL_FALSE : usize = 0;
pub const  GL_TRUE : usize = 1;
pub const  GL_POINTS : usize = 0x0000;
pub const  GL_LINES : usize = 0x0001;
pub const  GL_LINE_LOOP : usize = 0x0002;
pub const  GL_LINE_STRIP : usize = 0x0003;
pub const  GL_TRIANGLES : usize = 0x0004;
pub const  GL_TRIANGLE_STRIP : usize = 0x0005;
pub const  GL_TRIANGLE_FAN : usize = 0x0006;
pub const  GL_NEVER : usize = 0x0200;
pub const  GL_LESS : usize = 0x0201;
pub const  GL_EQUAL : usize = 0x0202;
pub const  GL_LEQUAL : usize = 0x0203;
pub const  GL_GREATER : usize = 0x0204;
pub const  GL_NOTEQUAL : usize = 0x0205;
pub const  GL_GEQUAL : usize = 0x0206;
pub const  GL_ALWAYS : usize = 0x0207;
pub const  GL_ZERO : usize = 0;
pub const  GL_ONE : usize = 1;
pub const  GL_SRC_COLOR : usize = 0x0300;
pub const  GL_ONE_MINUS_SRC_COLOR : usize = 0x0301;
pub const  GL_SRC_ALPHA : usize = 0x0302;
pub const  GL_ONE_MINUS_SRC_ALPHA : usize = 0x0303;
pub const  GL_DST_ALPHA : usize = 0x0304;
pub const  GL_ONE_MINUS_DST_ALPHA : usize = 0x0305;
pub const  GL_DST_COLOR : usize = 0x0306;
pub const  GL_ONE_MINUS_DST_COLOR : usize = 0x0307;
pub const  GL_SRC_ALPHA_SATURATE : usize = 0x0308;
pub const  GL_NONE : usize = 0;
pub const  GL_FRONT_LEFT : usize = 0x0400;
pub const  GL_FRONT_RIGHT : usize = 0x0401;
pub const  GL_BACK_LEFT : usize = 0x0402;
pub const  GL_BACK_RIGHT : usize = 0x0403;
pub const  GL_FRONT : usize = 0x0404;
pub const  GL_BACK : usize = 0x0405;
pub const  GL_LEFT : usize = 0x0406;
pub const  GL_RIGHT : usize = 0x0407;
pub const  GL_FRONT_AND_BACK : usize = 0x0408;
pub const  GL_NO_ERROR : usize = 0;
pub const  GL_INVALID_ENUM : usize = 0x0500;
pub const  GL_INVALID_VALUE : usize = 0x0501;
pub const  GL_INVALID_OPERATION : usize = 0x0502;
pub const  GL_OUT_OF_MEMORY : usize = 0x0505;
pub const  GL_CW : usize = 0x0900;
pub const  GL_CCW : usize = 0x0901;
pub const  GL_POINT_SIZE : usize = 0x0B11;
pub const  GL_POINT_SIZE_RANGE : usize = 0x0B12;
pub const  GL_POINT_SIZE_GRANULARITY : usize = 0x0B13;
pub const  GL_LINE_SMOOTH : usize = 0x0B20;
pub const  GL_LINE_WIDTH : usize = 0x0B21;
pub const  GL_LINE_WIDTH_RANGE : usize = 0x0B22;
pub const  GL_LINE_WIDTH_GRANULARITY : usize = 0x0B23;
pub const  GL_POLYGON_MODE : usize = 0x0B40;
pub const  GL_POLYGON_SMOOTH : usize = 0x0B41;
pub const  GL_CULL_FACE : usize = 0x0B44;
pub const  GL_CULL_FACE_MODE : usize = 0x0B45;
pub const  GL_FRONT_FACE : usize = 0x0B46;
pub const  GL_DEPTH_RANGE : usize = 0x0B70;
pub const  GL_DEPTH_TEST : usize = 0x0B71;
pub const  GL_DEPTH_WRITEMASK : usize = 0x0B72;
pub const  GL_DEPTH_CLEAR_VALUE : usize = 0x0B73;
pub const  GL_DEPTH_FUNC : usize = 0x0B74;
pub const  GL_STENCIL_TEST : usize = 0x0B90;
pub const  GL_STENCIL_CLEAR_VALUE : usize = 0x0B91;
pub const  GL_STENCIL_FUNC : usize = 0x0B92;
pub const  GL_STENCIL_VALUE_MASK : usize = 0x0B93;
pub const  GL_STENCIL_FAIL : usize = 0x0B94;
pub const  GL_STENCIL_PASS_DEPTH_FAIL : usize = 0x0B95;
pub const  GL_STENCIL_PASS_DEPTH_PASS : usize = 0x0B96;
pub const  GL_STENCIL_REF : usize = 0x0B97;
pub const  GL_STENCIL_WRITEMASK : usize = 0x0B98;
pub const  GL_VIEWPORT : usize = 0x0BA2;
pub const  GL_DITHER : usize = 0x0BD0;
pub const  GL_BLEND_DST : usize = 0x0BE0;
pub const  GL_BLEND_SRC : usize = 0x0BE1;
pub const  GL_BLEND : usize = 0x0BE2;
pub const  GL_LOGIC_OP_MODE : usize = 0x0BF0;
pub const  GL_DRAW_BUFFER : usize = 0x0C01;
pub const  GL_READ_BUFFER : usize = 0x0C02;
pub const  GL_SCISSOR_BOX : usize = 0x0C10;
pub const  GL_SCISSOR_TEST : usize = 0x0C11;
pub const  GL_COLOR_CLEAR_VALUE : usize = 0x0C22;
pub const  GL_COLOR_WRITEMASK : usize = 0x0C23;
pub const  GL_DOUBLEBUFFER : usize = 0x0C32;
pub const  GL_STEREO : usize = 0x0C33;
pub const  GL_LINE_SMOOTH_HINT : usize = 0x0C52;
pub const  GL_POLYGON_SMOOTH_HINT : usize = 0x0C53;
pub const  GL_UNPACK_SWAP_BYTES : usize = 0x0CF0;
pub const  GL_UNPACK_LSB_FIRST : usize = 0x0CF1;
pub const  GL_UNPACK_ROW_LENGTH : usize = 0x0CF2;
pub const  GL_UNPACK_SKIP_ROWS : usize = 0x0CF3;
pub const  GL_UNPACK_SKIP_PIXELS : usize = 0x0CF4;
pub const  GL_UNPACK_ALIGNMENT : usize = 0x0CF5;
pub const  GL_PACK_SWAP_BYTES : usize = 0x0D00;
pub const  GL_PACK_LSB_FIRST : usize = 0x0D01;
pub const  GL_PACK_ROW_LENGTH : usize = 0x0D02;
pub const  GL_PACK_SKIP_ROWS : usize = 0x0D03;
pub const  GL_PACK_SKIP_PIXELS : usize = 0x0D04;
pub const  GL_PACK_ALIGNMENT : usize = 0x0D05;
pub const  GL_MAX_TEXTURE_SIZE : usize = 0x0D33;
pub const  GL_MAX_VIEWPORT_DIMS : usize = 0x0D3A;
pub const  GL_SUBPIXEL_BITS : usize = 0x0D50;
pub const  GL_TEXTURE_1D : usize = 0x0DE0;
pub const  GL_TEXTURE_2D : usize = 0x0DE1;
pub const  GL_TEXTURE_WIDTH : usize = 0x1000;
pub const  GL_TEXTURE_HEIGHT : usize = 0x1001;
pub const  GL_TEXTURE_BORDER_COLOR : usize = 0x1004;
pub const  GL_DONT_CARE : usize = 0x1100;
pub const  GL_FASTEST : usize = 0x1101;
pub const  GL_NICEST : usize = 0x1102;
pub const  GL_BYTE : usize = 0x1400;
pub const  GL_UNSIGNED_BYTE : usize = 0x1401;
pub const  GL_SHORT : usize = 0x1402;
pub const  GL_UNSIGNED_SHORT : usize = 0x1403;
pub const  GL_INT : usize = 0x1404;
pub const  GL_UNSIGNED_INT : usize = 0x1405;
pub const  GL_FLOAT : usize = 0x1406;
pub const  GL_CLEAR : usize = 0x1500;
pub const  GL_AND : usize = 0x1501;
pub const  GL_AND_REVERSE : usize = 0x1502;
pub const  GL_COPY : usize = 0x1503;
pub const  GL_AND_INVERTED : usize = 0x1504;
pub const  GL_NOOP : usize = 0x1505;
pub const  GL_XOR : usize = 0x1506;
pub const  GL_OR : usize = 0x1507;
pub const  GL_NOR : usize = 0x1508;
pub const  GL_EQUIV : usize = 0x1509;
pub const  GL_INVERT : usize = 0x150A;
pub const  GL_OR_REVERSE : usize = 0x150B;
pub const  GL_COPY_INVERTED : usize = 0x150C;
pub const  GL_OR_INVERTED : usize = 0x150D;
pub const  GL_NAND : usize = 0x150E;
pub const  GL_SET : usize = 0x150F;
pub const  GL_TEXTURE : usize = 0x1702;
pub const  GL_COLOR : usize = 0x1800;
pub const  GL_DEPTH : usize = 0x1801;
pub const  GL_STENCIL : usize = 0x1802;
pub const  GL_STENCIL_INDEX : usize = 0x1901;
pub const  GL_DEPTH_COMPONENT : usize = 0x1902;
pub const  GL_RED : usize = 0x1903;
pub const  GL_GREEN : usize = 0x1904;
pub const  GL_BLUE : usize = 0x1905;
pub const  GL_ALPHA : usize = 0x1906;
pub const  GL_RGB : usize = 0x1907;
pub const  GL_RGBA : usize = 0x1908;
pub const  GL_POINT : usize = 0x1B00;
pub const  GL_LINE : usize = 0x1B01;
pub const  GL_FILL : usize = 0x1B02;
pub const  GL_KEEP : usize = 0x1E00;
pub const  GL_REPLACE : usize = 0x1E01;
pub const  GL_INCR : usize = 0x1E02;
pub const  GL_DECR : usize = 0x1E03;
pub const  GL_VENDOR : usize = 0x1F00;
pub const  GL_RENDERER : usize = 0x1F01;
pub const  GL_VERSION : usize = 0x1F02;
pub const  GL_EXTENSIONS : usize = 0x1F03;
pub const  GL_NEAREST : usize = 0x2600;
pub const  GL_LINEAR : usize = 0x2601;
pub const  GL_NEAREST_MIPMAP_NEAREST : usize = 0x2700;
pub const  GL_LINEAR_MIPMAP_NEAREST : usize = 0x2701;
pub const  GL_NEAREST_MIPMAP_LINEAR : usize = 0x2702;
pub const  GL_LINEAR_MIPMAP_LINEAR : usize = 0x2703;
pub const  GL_TEXTURE_MAG_FILTER : usize = 0x2800;
pub const  GL_TEXTURE_MIN_FILTER : usize = 0x2801;
pub const  GL_TEXTURE_WRAP_S : usize = 0x2802;
pub const  GL_TEXTURE_WRAP_T : usize = 0x2803;
pub const  GL_REPEAT : usize = 0x2901;
pub const  GL_COLOR_LOGIC_OP : usize = 0x0BF2;
pub const  GL_POLYGON_OFFSET_UNITS : usize = 0x2A00;
pub const  GL_POLYGON_OFFSET_POINT : usize = 0x2A01;
pub const  GL_POLYGON_OFFSET_LINE : usize = 0x2A02;
pub const  GL_POLYGON_OFFSET_FILL : usize = 0x8037;
pub const  GL_POLYGON_OFFSET_FACTOR : usize = 0x8038;
pub const  GL_TEXTURE_BINDING_1D : usize = 0x8068;
pub const  GL_TEXTURE_BINDING_2D : usize = 0x8069;
pub const  GL_TEXTURE_INTERNAL_FORMAT : usize = 0x1003;
pub const  GL_TEXTURE_RED_SIZE : usize = 0x805C;
pub const  GL_TEXTURE_GREEN_SIZE : usize = 0x805D;
pub const  GL_TEXTURE_BLUE_SIZE : usize = 0x805E;
pub const  GL_TEXTURE_ALPHA_SIZE : usize = 0x805F;
pub const  GL_DOUBLE : usize = 0x140A;
pub const  GL_PROXY_TEXTURE_1D : usize = 0x8063;
pub const  GL_PROXY_TEXTURE_2D : usize = 0x8064;
pub const  GL_R3_G3_B2 : usize = 0x2A10;
pub const  GL_RGB4 : usize = 0x804F;
pub const  GL_RGB5 : usize = 0x8050;
pub const  GL_RGB8 : usize = 0x8051;
pub const  GL_RGB10 : usize = 0x8052;
pub const  GL_RGB12 : usize = 0x8053;
pub const  GL_RGB16 : usize = 0x8054;
pub const  GL_RGBA2 : usize = 0x8055;
pub const  GL_RGBA4 : usize = 0x8056;
pub const  GL_RGB5_A1 : usize = 0x8057;
pub const  GL_RGBA8 : usize = 0x8058;
pub const  GL_RGB10_A2 : usize = 0x8059;
pub const  GL_RGBA12 : usize = 0x805A;
pub const  GL_RGBA16 : usize = 0x805B;
pub const  GL_UNSIGNED_BYTE_3_3_2 : usize = 0x8032;
pub const  GL_UNSIGNED_SHORT_4_4_4_4 : usize = 0x8033;
pub const  GL_UNSIGNED_SHORT_5_5_5_1 : usize = 0x8034;
pub const  GL_UNSIGNED_INT_8_8_8_8 : usize = 0x8035;
pub const  GL_UNSIGNED_INT_10_10_10_2 : usize = 0x8036;
pub const  GL_TEXTURE_BINDING_3D : usize = 0x806A;
pub const  GL_PACK_SKIP_IMAGES : usize = 0x806B;
pub const  GL_PACK_IMAGE_HEIGHT : usize = 0x806C;
pub const  GL_UNPACK_SKIP_IMAGES : usize = 0x806D;
pub const  GL_UNPACK_IMAGE_HEIGHT : usize = 0x806E;
pub const  GL_TEXTURE_3D : usize = 0x806F;
pub const  GL_PROXY_TEXTURE_3D : usize = 0x8070;
pub const  GL_TEXTURE_DEPTH : usize = 0x8071;
pub const  GL_TEXTURE_WRAP_R : usize = 0x8072;
pub const  GL_MAX_3D_TEXTURE_SIZE : usize = 0x8073;
pub const  GL_UNSIGNED_BYTE_2_3_3_REV : usize = 0x8362;
pub const  GL_UNSIGNED_SHORT_5_6_5 : usize = 0x8363;
pub const  GL_UNSIGNED_SHORT_5_6_5_REV : usize = 0x8364;
pub const  GL_UNSIGNED_SHORT_4_4_4_4_REV : usize = 0x8365;
pub const  GL_UNSIGNED_SHORT_1_5_5_5_REV : usize = 0x8366;
pub const  GL_UNSIGNED_INT_8_8_8_8_REV : usize = 0x8367;
pub const  GL_UNSIGNED_INT_2_10_10_10_REV : usize = 0x8368;
pub const  GL_BGR : usize = 0x80E0;
pub const  GL_BGRA : usize = 0x80E1;
pub const  GL_MAX_ELEMENTS_VERTICES : usize = 0x80E8;
pub const  GL_MAX_ELEMENTS_INDICES : usize = 0x80E9;
pub const  GL_CLAMP_TO_EDGE : usize = 0x812F;
pub const  GL_TEXTURE_MIN_LOD : usize = 0x813A;
pub const  GL_TEXTURE_MAX_LOD : usize = 0x813B;
pub const  GL_TEXTURE_BASE_LEVEL : usize = 0x813C;
pub const  GL_TEXTURE_MAX_LEVEL : usize = 0x813D;
pub const  GL_SMOOTH_POINT_SIZE_RANGE : usize = 0x0B12;
pub const  GL_SMOOTH_POINT_SIZE_GRANULARITY : usize = 0x0B13;
pub const  GL_SMOOTH_LINE_WIDTH_RANGE : usize = 0x0B22;
pub const  GL_SMOOTH_LINE_WIDTH_GRANULARITY : usize = 0x0B23;
pub const  GL_ALIASED_LINE_WIDTH_RANGE : usize = 0x846E;
pub const  GL_TEXTURE0 : usize = 0x84C0;
pub const  GL_TEXTURE1 : usize = 0x84C1;
pub const  GL_TEXTURE2 : usize = 0x84C2;
pub const  GL_TEXTURE3 : usize = 0x84C3;
pub const  GL_TEXTURE4 : usize = 0x84C4;
pub const  GL_TEXTURE5 : usize = 0x84C5;
pub const  GL_TEXTURE6 : usize = 0x84C6;
pub const  GL_TEXTURE7 : usize = 0x84C7;
pub const  GL_TEXTURE8 : usize = 0x84C8;
pub const  GL_TEXTURE9 : usize = 0x84C9;
pub const  GL_TEXTURE10 : usize = 0x84CA;
pub const  GL_TEXTURE11 : usize = 0x84CB;
pub const  GL_TEXTURE12 : usize = 0x84CC;
pub const  GL_TEXTURE13 : usize = 0x84CD;
pub const  GL_TEXTURE14 : usize = 0x84CE;
pub const  GL_TEXTURE15 : usize = 0x84CF;
pub const  GL_TEXTURE16 : usize = 0x84D0;
pub const  GL_TEXTURE17 : usize = 0x84D1;
pub const  GL_TEXTURE18 : usize = 0x84D2;
pub const  GL_TEXTURE19 : usize = 0x84D3;
pub const  GL_TEXTURE20 : usize = 0x84D4;
pub const  GL_TEXTURE21 : usize = 0x84D5;
pub const  GL_TEXTURE22 : usize = 0x84D6;
pub const  GL_TEXTURE23 : usize = 0x84D7;
pub const  GL_TEXTURE24 : usize = 0x84D8;
pub const  GL_TEXTURE25 : usize = 0x84D9;
pub const  GL_TEXTURE26 : usize = 0x84DA;
pub const  GL_TEXTURE27 : usize = 0x84DB;
pub const  GL_TEXTURE28 : usize = 0x84DC;
pub const  GL_TEXTURE29 : usize = 0x84DD;
pub const  GL_TEXTURE30 : usize = 0x84DE;
pub const  GL_TEXTURE31 : usize = 0x84DF;
pub const  GL_ACTIVE_TEXTURE : usize = 0x84E0;
pub const  GL_MULTISAMPLE : usize = 0x809D;
pub const  GL_SAMPLE_ALPHA_TO_COVERAGE : usize = 0x809E;
pub const  GL_SAMPLE_ALPHA_TO_ONE : usize = 0x809F;
pub const  GL_SAMPLE_COVERAGE : usize = 0x80A0;
pub const  GL_SAMPLE_BUFFERS : usize = 0x80A8;
pub const  GL_SAMPLES : usize = 0x80A9;
pub const  GL_SAMPLE_COVERAGE_VALUE : usize = 0x80AA;
pub const  GL_SAMPLE_COVERAGE_INVERT : usize = 0x80AB;
pub const  GL_TEXTURE_CUBE_MAP : usize = 0x8513;
pub const  GL_TEXTURE_BINDING_CUBE_MAP : usize = 0x8514;
pub const  GL_TEXTURE_CUBE_MAP_POSITIVE_X : usize = 0x8515;
pub const  GL_TEXTURE_CUBE_MAP_NEGATIVE_X : usize = 0x8516;
pub const  GL_TEXTURE_CUBE_MAP_POSITIVE_Y : usize = 0x8517;
pub const  GL_TEXTURE_CUBE_MAP_NEGATIVE_Y : usize = 0x8518;
pub const  GL_TEXTURE_CUBE_MAP_POSITIVE_Z : usize = 0x8519;
pub const  GL_TEXTURE_CUBE_MAP_NEGATIVE_Z : usize = 0x851A;
pub const  GL_PROXY_TEXTURE_CUBE_MAP : usize = 0x851B;
pub const  GL_MAX_CUBE_MAP_TEXTURE_SIZE : usize = 0x851C;
pub const  GL_COMPRESSED_RGB : usize = 0x84ED;
pub const  GL_COMPRESSED_RGBA : usize = 0x84EE;
pub const  GL_TEXTURE_COMPRESSION_HINT : usize = 0x84EF;
pub const  GL_TEXTURE_COMPRESSED_IMAGE_SIZE : usize = 0x86A0;
pub const  GL_TEXTURE_COMPRESSED : usize = 0x86A1;
pub const  GL_NUM_COMPRESSED_TEXTURE_FORMATS : usize = 0x86A2;
pub const  GL_COMPRESSED_TEXTURE_FORMATS : usize = 0x86A3;
pub const  GL_CLAMP_TO_BORDER : usize = 0x812D;
pub const  GL_BLEND_DST_RGB : usize = 0x80C8;
pub const  GL_BLEND_SRC_RGB : usize = 0x80C9;
pub const  GL_BLEND_DST_ALPHA : usize = 0x80CA;
pub const  GL_BLEND_SRC_ALPHA : usize = 0x80CB;
pub const  GL_POINT_FADE_THRESHOLD_SIZE : usize = 0x8128;
pub const  GL_DEPTH_COMPONENT16 : usize = 0x81A5;
pub const  GL_DEPTH_COMPONENT24 : usize = 0x81A6;
pub const  GL_DEPTH_COMPONENT32 : usize = 0x81A7;
pub const  GL_MIRRORED_REPEAT : usize = 0x8370;
pub const  GL_MAX_TEXTURE_LOD_BIAS : usize = 0x84FD;
pub const  GL_TEXTURE_LOD_BIAS : usize = 0x8501;
pub const  GL_INCR_WRAP : usize = 0x8507;
pub const  GL_DECR_WRAP : usize = 0x8508;
pub const  GL_TEXTURE_DEPTH_SIZE : usize = 0x884A;
pub const  GL_TEXTURE_COMPARE_MODE : usize = 0x884C;
pub const  GL_TEXTURE_COMPARE_FUNC : usize = 0x884D;
pub const  GL_FUNC_ADD : usize = 0x8006;
pub const  GL_FUNC_SUBTRACT : usize = 0x800A;
pub const  GL_FUNC_REVERSE_SUBTRACT : usize = 0x800B;
pub const  GL_MIN : usize = 0x8007;
pub const  GL_MAX : usize = 0x8008;
pub const  GL_CONSTANT_COLOR : usize = 0x8001;
pub const  GL_ONE_MINUS_CONSTANT_COLOR : usize = 0x8002;
pub const  GL_CONSTANT_ALPHA : usize = 0x8003;
pub const  GL_ONE_MINUS_CONSTANT_ALPHA : usize = 0x8004;
pub const  GL_BUFFER_SIZE : usize = 0x8764;
pub const  GL_BUFFER_USAGE : usize = 0x8765;
pub const  GL_QUERY_COUNTER_BITS : usize = 0x8864;
pub const  GL_CURRENT_QUERY : usize = 0x8865;
pub const  GL_QUERY_RESULT : usize = 0x8866;
pub const  GL_QUERY_RESULT_AVAILABLE : usize = 0x8867;
pub const  GL_ARRAY_BUFFER : usize = 0x8892;
pub const  GL_ELEMENT_ARRAY_BUFFER : usize = 0x8893;
pub const  GL_ARRAY_BUFFER_BINDING : usize = 0x8894;
pub const  GL_ELEMENT_ARRAY_BUFFER_BINDING : usize = 0x8895;
pub const  GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING : usize = 0x889F;
pub const  GL_READ_ONLY : usize = 0x88B8;
pub const  GL_WRITE_ONLY : usize = 0x88B9;
pub const  GL_READ_WRITE : usize = 0x88BA;
pub const  GL_BUFFER_ACCESS : usize = 0x88BB;
pub const  GL_BUFFER_MAPPED : usize = 0x88BC;
pub const  GL_BUFFER_MAP_POINTER : usize = 0x88BD;
pub const  GL_STREAM_DRAW : usize = 0x88E0;
pub const  GL_STREAM_READ : usize = 0x88E1;
pub const  GL_STREAM_COPY : usize = 0x88E2;
pub const  GL_CONST_DRAW : usize = 0x88E4;
pub const  GL_CONST_READ : usize = 0x88E5;
pub const  GL_CONST_COPY : usize = 0x88E6;
pub const  GL_DYNAMIC_DRAW : usize = 0x88E8;
pub const  GL_DYNAMIC_READ : usize = 0x88E9;
pub const  GL_DYNAMIC_COPY : usize = 0x88EA;
pub const  GL_SAMPLES_PASSED : usize = 0x8914;
pub const  GL_SRC1_ALPHA : usize = 0x8589;
pub const  GL_BLEND_EQUATION_RGB : usize = 0x8009;
pub const  GL_VERTEX_ATTRIB_ARRAY_ENABLED : usize = 0x8622;
pub const  GL_VERTEX_ATTRIB_ARRAY_SIZE : usize = 0x8623;
pub const  GL_VERTEX_ATTRIB_ARRAY_STRIDE : usize = 0x8624;
pub const  GL_VERTEX_ATTRIB_ARRAY_TYPE : usize = 0x8625;
pub const  GL_CURRENT_VERTEX_ATTRIB : usize = 0x8626;
pub const  GL_VERTEX_PROGRAM_POINT_SIZE : usize = 0x8642;
pub const  GL_VERTEX_ATTRIB_ARRAY_POINTER : usize = 0x8645;
pub const  GL_STENCIL_BACK_FUNC : usize = 0x8800;
pub const  GL_STENCIL_BACK_FAIL : usize = 0x8801;
pub const  GL_STENCIL_BACK_PASS_DEPTH_FAIL : usize = 0x8802;
pub const  GL_STENCIL_BACK_PASS_DEPTH_PASS : usize = 0x8803;
pub const  GL_MAX_DRAW_BUFFERS : usize = 0x8824;
pub const  GL_DRAW_BUFFER0 : usize = 0x8825;
pub const  GL_DRAW_BUFFER1 : usize = 0x8826;
pub const  GL_DRAW_BUFFER2 : usize = 0x8827;
pub const  GL_DRAW_BUFFER3 : usize = 0x8828;
pub const  GL_DRAW_BUFFER4 : usize = 0x8829;
pub const  GL_DRAW_BUFFER5 : usize = 0x882A;
pub const  GL_DRAW_BUFFER6 : usize = 0x882B;
pub const  GL_DRAW_BUFFER7 : usize = 0x882C;
pub const  GL_DRAW_BUFFER8 : usize = 0x882D;
pub const  GL_DRAW_BUFFER9 : usize = 0x882E;
pub const  GL_DRAW_BUFFER10 : usize = 0x882F;
pub const  GL_DRAW_BUFFER11 : usize = 0x8830;
pub const  GL_DRAW_BUFFER12 : usize = 0x8831;
pub const  GL_DRAW_BUFFER13 : usize = 0x8832;
pub const  GL_DRAW_BUFFER14 : usize = 0x8833;
pub const  GL_DRAW_BUFFER15 : usize = 0x8834;
pub const  GL_BLEND_EQUATION_ALPHA : usize = 0x883D;
pub const  GL_MAX_VERTEX_ATTRIBS : usize = 0x8869;
pub const  GL_VERTEX_ATTRIB_ARRAY_NORMALIZED : usize = 0x886A;
pub const  GL_MAX_TEXTURE_IMAGE_UNITS : usize = 0x8872;
pub const  GL_FRAGMENT_SHADER : usize = 0x8B30;
pub const  GL_VERTEX_SHADER : usize = 0x8B31;
pub const  GL_MAX_FRAGMENT_UNIFORM_COMPONENTS : usize = 0x8B49;
pub const  GL_MAX_VERTEX_UNIFORM_COMPONENTS : usize = 0x8B4A;
pub const  GL_MAX_VARYING_FLOATS : usize = 0x8B4B;
pub const  GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS : usize = 0x8B4C;
pub const  GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS : usize = 0x8B4D;
pub const  GL_SHADER_TYPE : usize = 0x8B4F;
pub const  GL_FLOAT_VEC2 : usize = 0x8B50;
pub const  GL_FLOAT_VEC3 : usize = 0x8B51;
pub const  GL_FLOAT_VEC4 : usize = 0x8B52;
pub const  GL_INT_VEC2 : usize = 0x8B53;
pub const  GL_INT_VEC3 : usize = 0x8B54;
pub const  GL_INT_VEC4 : usize = 0x8B55;
pub const  GL_BOOL : usize = 0x8B56;
pub const  GL_BOOL_VEC2 : usize = 0x8B57;
pub const  GL_BOOL_VEC3 : usize = 0x8B58;
pub const  GL_BOOL_VEC4 : usize = 0x8B59;
pub const  GL_FLOAT_MAT2 : usize = 0x8B5A;
pub const  GL_FLOAT_MAT3 : usize = 0x8B5B;
pub const  GL_FLOAT_MAT4 : usize = 0x8B5C;
pub const  GL_SAMPLER_1D : usize = 0x8B5D;
pub const  GL_SAMPLER_2D : usize = 0x8B5E;
pub const  GL_SAMPLER_3D : usize = 0x8B5F;
pub const  GL_SAMPLER_CUBE : usize = 0x8B60;
pub const  GL_SAMPLER_1D_SHADOW : usize = 0x8B61;
pub const  GL_SAMPLER_2D_SHADOW : usize = 0x8B62;
pub const  GL_DELETE_STATUS : usize = 0x8B80;
pub const  GL_COMPILE_STATUS : usize = 0x8B81;
pub const  GL_LINK_STATUS : usize = 0x8B82;
pub const  GL_VALIDATE_STATUS : usize = 0x8B83;
pub const  GL_INFO_LOG_LENGTH : usize = 0x8B84;
pub const  GL_ATTACHED_SHADERS : usize = 0x8B85;
pub const  GL_ACTIVE_UNIFORMS : usize = 0x8B86;
pub const  GL_ACTIVE_UNIFORM_MAX_LENGTH : usize = 0x8B87;
pub const  GL_SHADER_SOURCE_LENGTH : usize = 0x8B88;
pub const  GL_ACTIVE_ATTRIBUTES : usize = 0x8B89;
pub const  GL_ACTIVE_ATTRIBUTE_MAX_LENGTH : usize = 0x8B8A;
pub const  GL_FRAGMENT_SHADER_DERIVATIVE_HINT : usize = 0x8B8B;
pub const  GL_SHADING_LANGUAGE_VERSION : usize = 0x8B8C;
pub const  GL_CURRENT_PROGRAM : usize = 0x8B8D;
pub const  GL_POINT_SPRITE_COORD_ORIGIN : usize = 0x8CA0;
pub const  GL_LOWER_LEFT : usize = 0x8CA1;
pub const  GL_UPPER_LEFT : usize = 0x8CA2;
pub const  GL_STENCIL_BACK_REF : usize = 0x8CA3;
pub const  GL_STENCIL_BACK_VALUE_MASK : usize = 0x8CA4;
pub const  GL_STENCIL_BACK_WRITEMASK : usize = 0x8CA5;
pub const  GL_PIXEL_PACK_BUFFER : usize = 0x88EB;
pub const  GL_PIXEL_UNPACK_BUFFER : usize = 0x88EC;
pub const  GL_PIXEL_PACK_BUFFER_BINDING : usize = 0x88ED;
pub const  GL_PIXEL_UNPACK_BUFFER_BINDING : usize = 0x88EF;
pub const  GL_FLOAT_MAT2x3 : usize = 0x8B65;
pub const  GL_FLOAT_MAT2x4 : usize = 0x8B66;
pub const  GL_FLOAT_MAT3x2 : usize = 0x8B67;
pub const  GL_FLOAT_MAT3x4 : usize = 0x8B68;
pub const  GL_FLOAT_MAT4x2 : usize = 0x8B69;
pub const  GL_FLOAT_MAT4x3 : usize = 0x8B6A;
pub const  GL_SRGB : usize = 0x8C40;
pub const  GL_SRGB8 : usize = 0x8C41;
pub const  GL_SRGB_ALPHA : usize = 0x8C42;
pub const  GL_SRGB8_ALPHA8 : usize = 0x8C43;
pub const  GL_COMPRESSED_SRGB : usize = 0x8C48;
pub const  GL_COMPRESSED_SRGB_ALPHA : usize = 0x8C49;
pub const  GL_COMPARE_REF_TO_TEXTURE : usize = 0x884E;
pub const  GL_CLIP_DISTANCE0 : usize = 0x3000;
pub const  GL_CLIP_DISTANCE1 : usize = 0x3001;
pub const  GL_CLIP_DISTANCE2 : usize = 0x3002;
pub const  GL_CLIP_DISTANCE3 : usize = 0x3003;
pub const  GL_CLIP_DISTANCE4 : usize = 0x3004;
pub const  GL_CLIP_DISTANCE5 : usize = 0x3005;
pub const  GL_CLIP_DISTANCE6 : usize = 0x3006;
pub const  GL_CLIP_DISTANCE7 : usize = 0x3007;
pub const  GL_MAX_CLIP_DISTANCES : usize = 0x0D32;
pub const  GL_MAJOR_VERSION : usize = 0x821B;
pub const  GL_MINOR_VERSION : usize = 0x821C;
pub const  GL_NUM_EXTENSIONS : usize = 0x821D;
pub const  GL_CONTEXT_FLAGS : usize = 0x821E;
pub const  GL_COMPRESSED_RED : usize = 0x8225;
pub const  GL_COMPRESSED_RG : usize = 0x8226;
pub const  GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT : usize = 0x00000001;
pub const  GL_RGBA32F : usize = 0x8814;
pub const  GL_RGB32F : usize = 0x8815;
pub const  GL_RGBA16F : usize = 0x881A;
pub const  GL_RGB16F : usize = 0x881B;
pub const  GL_VERTEX_ATTRIB_ARRAY_INTEGER : usize = 0x88FD;
pub const  GL_MAX_ARRAY_TEXTURE_LAYERS : usize = 0x88FF;
pub const  GL_MIN_PROGRAM_TEXEL_OFFSET : usize = 0x8904;
pub const  GL_MAX_PROGRAM_TEXEL_OFFSET : usize = 0x8905;
pub const  GL_CLAMP_READ_COLOR : usize = 0x891C;
pub const  GL_FIXED_ONLY : usize = 0x891D;
pub const  GL_MAX_VARYING_COMPONENTS : usize = 0x8B4B;
pub const  GL_TEXTURE_1D_ARRAY : usize = 0x8C18;
pub const  GL_PROXY_TEXTURE_1D_ARRAY : usize = 0x8C19;
pub const  GL_TEXTURE_2D_ARRAY : usize = 0x8C1A;
pub const  GL_PROXY_TEXTURE_2D_ARRAY : usize = 0x8C1B;
pub const  GL_TEXTURE_BINDING_1D_ARRAY : usize = 0x8C1C;
pub const  GL_TEXTURE_BINDING_2D_ARRAY : usize = 0x8C1D;
pub const  GL_R11F_G11F_B10F : usize = 0x8C3A;
pub const  GL_UNSIGNED_INT_10F_11F_11F_REV : usize = 0x8C3B;
pub const  GL_RGB9_E5 : usize = 0x8C3D;
pub const  GL_UNSIGNED_INT_5_9_9_9_REV : usize = 0x8C3E;
pub const  GL_TEXTURE_SHARED_SIZE : usize = 0x8C3F;
pub const  GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH : usize = 0x8C76;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_MODE : usize = 0x8C7F;
pub const  GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS : usize = 0x8C80;
pub const  GL_TRANSFORM_FEEDBACK_VARYINGS : usize = 0x8C83;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_START : usize = 0x8C84;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_SIZE : usize = 0x8C85;
pub const  GL_PRIMITIVES_GENERATED : usize = 0x8C87;
pub const  GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN : usize = 0x8C88;
pub const  GL_RASTERIZER_DISCARD : usize = 0x8C89;
pub const  GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS : usize = 0x8C8A;
pub const  GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS : usize = 0x8C8B;
pub const  GL_INTERLEAVED_ATTRIBS : usize = 0x8C8C;
pub const  GL_SEPARATE_ATTRIBS : usize = 0x8C8D;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER : usize = 0x8C8E;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_BINDING : usize = 0x8C8F;
pub const  GL_RGBA32UI : usize = 0x8D70;
pub const  GL_RGB32UI : usize = 0x8D71;
pub const  GL_RGBA16UI : usize = 0x8D76;
pub const  GL_RGB16UI : usize = 0x8D77;
pub const  GL_RGBA8UI : usize = 0x8D7C;
pub const  GL_RGB8UI : usize = 0x8D7D;
pub const  GL_RGBA32I : usize = 0x8D82;
pub const  GL_RGB32I : usize = 0x8D83;
pub const  GL_RGBA16I : usize = 0x8D88;
pub const  GL_RGB16I : usize = 0x8D89;
pub const  GL_RGBA8I : usize = 0x8D8E;
pub const  GL_RGB8I : usize = 0x8D8F;
pub const  GL_RED_INTEGER : usize = 0x8D94;
pub const  GL_GREEN_INTEGER : usize = 0x8D95;
pub const  GL_BLUE_INTEGER : usize = 0x8D96;
pub const  GL_RGB_INTEGER : usize = 0x8D98;
pub const  GL_RGBA_INTEGER : usize = 0x8D99;
pub const  GL_BGR_INTEGER : usize = 0x8D9A;
pub const  GL_BGRA_INTEGER : usize = 0x8D9B;
pub const  GL_SAMPLER_1D_ARRAY : usize = 0x8DC0;
pub const  GL_SAMPLER_2D_ARRAY : usize = 0x8DC1;
pub const  GL_SAMPLER_1D_ARRAY_SHADOW : usize = 0x8DC3;
pub const  GL_SAMPLER_2D_ARRAY_SHADOW : usize = 0x8DC4;
pub const  GL_SAMPLER_CUBE_SHADOW : usize = 0x8DC5;
pub const  GL_UNSIGNED_INT_VEC2 : usize = 0x8DC6;
pub const  GL_UNSIGNED_INT_VEC3 : usize = 0x8DC7;
pub const  GL_UNSIGNED_INT_VEC4 : usize = 0x8DC8;
pub const  GL_INT_SAMPLER_1D : usize = 0x8DC9;
pub const  GL_INT_SAMPLER_2D : usize = 0x8DCA;
pub const  GL_INT_SAMPLER_3D : usize = 0x8DCB;
pub const  GL_INT_SAMPLER_CUBE : usize = 0x8DCC;
pub const  GL_INT_SAMPLER_1D_ARRAY : usize = 0x8DCE;
pub const  GL_INT_SAMPLER_2D_ARRAY : usize = 0x8DCF;
pub const  GL_UNSIGNED_INT_SAMPLER_1D : usize = 0x8DD1;
pub const  GL_UNSIGNED_INT_SAMPLER_2D : usize = 0x8DD2;
pub const  GL_UNSIGNED_INT_SAMPLER_3D : usize = 0x8DD3;
pub const  GL_UNSIGNED_INT_SAMPLER_CUBE : usize = 0x8DD4;
pub const  GL_UNSIGNED_INT_SAMPLER_1D_ARRAY : usize = 0x8DD6;
pub const  GL_UNSIGNED_INT_SAMPLER_2D_ARRAY : usize = 0x8DD7;
pub const  GL_QUERY_WAIT : usize = 0x8E13;
pub const  GL_QUERY_NO_WAIT : usize = 0x8E14;
pub const  GL_QUERY_BY_REGION_WAIT : usize = 0x8E15;
pub const  GL_QUERY_BY_REGION_NO_WAIT : usize = 0x8E16;
pub const  GL_BUFFER_ACCESS_FLAGS : usize = 0x911F;
pub const  GL_BUFFER_MAP_LENGTH : usize = 0x9120;
pub const  GL_BUFFER_MAP_OFFSET : usize = 0x9121;
pub const  GL_DEPTH_COMPONENT32F : usize = 0x8CAC;
pub const  GL_DEPTH32F_STENCIL8 : usize = 0x8CAD;
pub const  GL_FLOAT_32_UNSIGNED_INT_24_8_REV : usize = 0x8DAD;
pub const  GL_INVALID_FRAMEBUFFER_OPERATION : usize = 0x0506;
pub const  GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING : usize = 0x8210;
pub const  GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE : usize = 0x8211;
pub const  GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE : usize = 0x8212;
pub const  GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE : usize = 0x8213;
pub const  GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE : usize = 0x8214;
pub const  GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE : usize = 0x8215;
pub const  GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE : usize = 0x8216;
pub const  GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE : usize = 0x8217;
pub const  GL_FRAMEBUFFER_DEFAULT : usize = 0x8218;
pub const  GL_FRAMEBUFFER_UNDEFINED : usize = 0x8219;
pub const  GL_DEPTH_STENCIL_ATTACHMENT : usize = 0x821A;
pub const  GL_MAX_RENDERBUFFER_SIZE : usize = 0x84E8;
pub const  GL_DEPTH_STENCIL : usize = 0x84F9;
pub const  GL_UNSIGNED_INT_24_8 : usize = 0x84FA;
pub const  GL_DEPTH24_STENCIL8 : usize = 0x88F0;
pub const  GL_TEXTURE_STENCIL_SIZE : usize = 0x88F1;
pub const  GL_TEXTURE_RED_TYPE : usize = 0x8C10;
pub const  GL_TEXTURE_GREEN_TYPE : usize = 0x8C11;
pub const  GL_TEXTURE_BLUE_TYPE : usize = 0x8C12;
pub const  GL_TEXTURE_ALPHA_TYPE : usize = 0x8C13;
pub const  GL_TEXTURE_DEPTH_TYPE : usize = 0x8C16;
pub const  GL_UNSIGNED_NORMALIZED : usize = 0x8C17;
pub const  GL_FRAMEBUFFER_BINDING : usize = 0x8CA6;
pub const  GL_DRAW_FRAMEBUFFER_BINDING : usize = 0x8CA6;
pub const  GL_RENDERBUFFER_BINDING : usize = 0x8CA7;
pub const  GL_READ_FRAMEBUFFER : usize = 0x8CA8;
pub const  GL_DRAW_FRAMEBUFFER : usize = 0x8CA9;
pub const  GL_READ_FRAMEBUFFER_BINDING : usize = 0x8CAA;
pub const  GL_RENDERBUFFER_SAMPLES : usize = 0x8CAB;
pub const  GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE : usize = 0x8CD0;
pub const  GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME : usize = 0x8CD1;
pub const  GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL : usize = 0x8CD2;
pub const  GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE : usize = 0x8CD3;
pub const  GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER : usize = 0x8CD4;
pub const  GL_FRAMEBUFFER_COMPLETE : usize = 0x8CD5;
pub const  GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT : usize = 0x8CD6;
pub const  GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT : usize = 0x8CD7;
pub const  GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER : usize = 0x8CDB;
pub const  GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER : usize = 0x8CDC;
pub const  GL_FRAMEBUFFER_UNSUPPORTED : usize = 0x8CDD;
pub const  GL_MAX_COLOR_ATTACHMENTS : usize = 0x8CDF;
pub const  GL_COLOR_ATTACHMENT0 : usize = 0x8CE0;
pub const  GL_COLOR_ATTACHMENT1 : usize = 0x8CE1;
pub const  GL_COLOR_ATTACHMENT2 : usize = 0x8CE2;
pub const  GL_COLOR_ATTACHMENT3 : usize = 0x8CE3;
pub const  GL_COLOR_ATTACHMENT4 : usize = 0x8CE4;
pub const  GL_COLOR_ATTACHMENT5 : usize = 0x8CE5;
pub const  GL_COLOR_ATTACHMENT6 : usize = 0x8CE6;
pub const  GL_COLOR_ATTACHMENT7 : usize = 0x8CE7;
pub const  GL_COLOR_ATTACHMENT8 : usize = 0x8CE8;
pub const  GL_COLOR_ATTACHMENT9 : usize = 0x8CE9;
pub const  GL_COLOR_ATTACHMENT10 : usize = 0x8CEA;
pub const  GL_COLOR_ATTACHMENT11 : usize = 0x8CEB;
pub const  GL_COLOR_ATTACHMENT12 : usize = 0x8CEC;
pub const  GL_COLOR_ATTACHMENT13 : usize = 0x8CED;
pub const  GL_COLOR_ATTACHMENT14 : usize = 0x8CEE;
pub const  GL_COLOR_ATTACHMENT15 : usize = 0x8CEF;
pub const  GL_COLOR_ATTACHMENT16 : usize = 0x8CF0;
pub const  GL_COLOR_ATTACHMENT17 : usize = 0x8CF1;
pub const  GL_COLOR_ATTACHMENT18 : usize = 0x8CF2;
pub const  GL_COLOR_ATTACHMENT19 : usize = 0x8CF3;
pub const  GL_COLOR_ATTACHMENT20 : usize = 0x8CF4;
pub const  GL_COLOR_ATTACHMENT21 : usize = 0x8CF5;
pub const  GL_COLOR_ATTACHMENT22 : usize = 0x8CF6;
pub const  GL_COLOR_ATTACHMENT23 : usize = 0x8CF7;
pub const  GL_COLOR_ATTACHMENT24 : usize = 0x8CF8;
pub const  GL_COLOR_ATTACHMENT25 : usize = 0x8CF9;
pub const  GL_COLOR_ATTACHMENT26 : usize = 0x8CFA;
pub const  GL_COLOR_ATTACHMENT27 : usize = 0x8CFB;
pub const  GL_COLOR_ATTACHMENT28 : usize = 0x8CFC;
pub const  GL_COLOR_ATTACHMENT29 : usize = 0x8CFD;
pub const  GL_COLOR_ATTACHMENT30 : usize = 0x8CFE;
pub const  GL_COLOR_ATTACHMENT31 : usize = 0x8CFF;
pub const  GL_DEPTH_ATTACHMENT : usize = 0x8D00;
pub const  GL_STENCIL_ATTACHMENT : usize = 0x8D20;
pub const  GL_FRAMEBUFFER : usize = 0x8D40;
pub const  GL_RENDERBUFFER : usize = 0x8D41;
pub const  GL_RENDERBUFFER_WIDTH : usize = 0x8D42;
pub const  GL_RENDERBUFFER_HEIGHT : usize = 0x8D43;
pub const  GL_RENDERBUFFER_INTERNAL_FORMAT : usize = 0x8D44;
pub const  GL_STENCIL_INDEX1 : usize = 0x8D46;
pub const  GL_STENCIL_INDEX4 : usize = 0x8D47;
pub const  GL_STENCIL_INDEX8 : usize = 0x8D48;
pub const  GL_STENCIL_INDEX16 : usize = 0x8D49;
pub const  GL_RENDERBUFFER_RED_SIZE : usize = 0x8D50;
pub const  GL_RENDERBUFFER_GREEN_SIZE : usize = 0x8D51;
pub const  GL_RENDERBUFFER_BLUE_SIZE : usize = 0x8D52;
pub const  GL_RENDERBUFFER_ALPHA_SIZE : usize = 0x8D53;
pub const  GL_RENDERBUFFER_DEPTH_SIZE : usize = 0x8D54;
pub const  GL_RENDERBUFFER_STENCIL_SIZE : usize = 0x8D55;
pub const  GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE : usize = 0x8D56;
pub const  GL_MAX_SAMPLES : usize = 0x8D57;
pub const  GL_INDEX : usize = 0x8222;
pub const  GL_FRAMEBUFFER_SRGB : usize = 0x8DB9;
pub const  GL_HALF_FLOAT : usize = 0x140B;
pub const  GL_MAP_READ_BIT : usize = 0x0001;
pub const  GL_MAP_WRITE_BIT : usize = 0x0002;
pub const  GL_MAP_INVALIDATE_RANGE_BIT : usize = 0x0004;
pub const  GL_MAP_INVALIDATE_BUFFER_BIT : usize = 0x0008;
pub const  GL_MAP_FLUSH_EXPLICIT_BIT : usize = 0x0010;
pub const  GL_MAP_UNSYNCHRONIZED_BIT : usize = 0x0020;
pub const  GL_COMPRESSED_RED_RGTC1 : usize = 0x8DBB;
pub const  GL_COMPRESSED_SIGNED_RED_RGTC1 : usize = 0x8DBC;
pub const  GL_COMPRESSED_RG_RGTC2 : usize = 0x8DBD;
pub const  GL_COMPRESSED_SIGNED_RG_RGTC2 : usize = 0x8DBE;
pub const  GL_RG : usize = 0x8227;
pub const  GL_RG_INTEGER : usize = 0x8228;
pub const  GL_R8 : usize = 0x8229;
pub const  GL_R16 : usize = 0x822A;
pub const  GL_RG8 : usize = 0x822B;
pub const  GL_RG16 : usize = 0x822C;
pub const  GL_R16F : usize = 0x822D;
pub const  GL_R32F : usize = 0x822E;
pub const  GL_RG16F : usize = 0x822F;
pub const  GL_RG32F : usize = 0x8230;
pub const  GL_R8I : usize = 0x8231;
pub const  GL_R8UI : usize = 0x8232;
pub const  GL_R16I : usize = 0x8233;
pub const  GL_R16UI : usize = 0x8234;
pub const  GL_R32I : usize = 0x8235;
pub const  GL_R32UI : usize = 0x8236;
pub const  GL_RG8I : usize = 0x8237;
pub const  GL_RG8UI : usize = 0x8238;
pub const  GL_RG16I : usize = 0x8239;
pub const  GL_RG16UI : usize = 0x823A;
pub const  GL_RG32I : usize = 0x823B;
pub const  GL_RG32UI : usize = 0x823C;
pub const  GL_VERTEX_ARRAY_BINDING : usize = 0x85B5;
pub const  GL_SAMPLER_2D_RECT : usize = 0x8B63;
pub const  GL_SAMPLER_2D_RECT_SHADOW : usize = 0x8B64;
pub const  GL_SAMPLER_BUFFER : usize = 0x8DC2;
pub const  GL_INT_SAMPLER_2D_RECT : usize = 0x8DCD;
pub const  GL_INT_SAMPLER_BUFFER : usize = 0x8DD0;
pub const  GL_UNSIGNED_INT_SAMPLER_2D_RECT : usize = 0x8DD5;
pub const  GL_UNSIGNED_INT_SAMPLER_BUFFER : usize = 0x8DD8;
pub const  GL_TEXTURE_BUFFER : usize = 0x8C2A;
pub const  GL_MAX_TEXTURE_BUFFER_SIZE : usize = 0x8C2B;
pub const  GL_TEXTURE_BINDING_BUFFER : usize = 0x8C2C;
pub const  GL_TEXTURE_BUFFER_DATA_STORE_BINDING : usize = 0x8C2D;
pub const  GL_TEXTURE_RECTANGLE : usize = 0x84F5;
pub const  GL_TEXTURE_BINDING_RECTANGLE : usize = 0x84F6;
pub const  GL_PROXY_TEXTURE_RECTANGLE : usize = 0x84F7;
pub const  GL_MAX_RECTANGLE_TEXTURE_SIZE : usize = 0x84F8;
pub const  GL_R8_SNORM : usize = 0x8F94;
pub const  GL_RG8_SNORM : usize = 0x8F95;
pub const  GL_RGB8_SNORM : usize = 0x8F96;
pub const  GL_RGBA8_SNORM : usize = 0x8F97;
pub const  GL_R16_SNORM : usize = 0x8F98;
pub const  GL_RG16_SNORM : usize = 0x8F99;
pub const  GL_RGB16_SNORM : usize = 0x8F9A;
pub const  GL_RGBA16_SNORM : usize = 0x8F9B;
pub const  GL_SIGNED_NORMALIZED : usize = 0x8F9C;
pub const  GL_PRIMITIVE_RESTART : usize = 0x8F9D;
pub const  GL_PRIMITIVE_RESTART_INDEX : usize = 0x8F9E;
pub const  GL_COPY_READ_BUFFER : usize = 0x8F36;
pub const  GL_COPY_WRITE_BUFFER : usize = 0x8F37;
pub const  GL_UNIFORM_BUFFER : usize = 0x8A11;
pub const  GL_UNIFORM_BUFFER_BINDING : usize = 0x8A28;
pub const  GL_UNIFORM_BUFFER_START : usize = 0x8A29;
pub const  GL_UNIFORM_BUFFER_SIZE : usize = 0x8A2A;
pub const  GL_MAX_VERTEX_UNIFORM_BLOCKS : usize = 0x8A2B;
pub const  GL_MAX_GEOMETRY_UNIFORM_BLOCKS : usize = 0x8A2C;
pub const  GL_MAX_FRAGMENT_UNIFORM_BLOCKS : usize = 0x8A2D;
pub const  GL_MAX_COMBINED_UNIFORM_BLOCKS : usize = 0x8A2E;
pub const  GL_MAX_UNIFORM_BUFFER_BINDINGS : usize = 0x8A2F;
pub const  GL_MAX_UNIFORM_BLOCK_SIZE : usize = 0x8A30;
pub const  GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS : usize = 0x8A31;
pub const  GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS : usize = 0x8A32;
pub const  GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS : usize = 0x8A33;
pub const  GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT : usize = 0x8A34;
pub const  GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH : usize = 0x8A35;
pub const  GL_ACTIVE_UNIFORM_BLOCKS : usize = 0x8A36;
pub const  GL_UNIFORM_TYPE : usize = 0x8A37;
pub const  GL_UNIFORM_SIZE : usize = 0x8A38;
pub const  GL_UNIFORM_NAME_LENGTH : usize = 0x8A39;
pub const  GL_UNIFORM_BLOCK_INDEX : usize = 0x8A3A;
pub const  GL_UNIFORM_OFFSET : usize = 0x8A3B;
pub const  GL_UNIFORM_ARRAY_STRIDE : usize = 0x8A3C;
pub const  GL_UNIFORM_MATRIX_STRIDE : usize = 0x8A3D;
pub const  GL_UNIFORM_IS_ROW_MAJOR : usize = 0x8A3E;
pub const  GL_UNIFORM_BLOCK_BINDING : usize = 0x8A3F;
pub const  GL_UNIFORM_BLOCK_DATA_SIZE : usize = 0x8A40;
pub const  GL_UNIFORM_BLOCK_NAME_LENGTH : usize = 0x8A41;
pub const  GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS : usize = 0x8A42;
pub const  GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES : usize = 0x8A43;
pub const  GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER : usize = 0x8A44;
pub const  GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER : usize = 0x8A45;
pub const  GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER : usize = 0x8A46;
pub const  GL_INVALID_INDEX : usize = 0xFFFFFFFF;
pub const  GL_CONTEXT_CORE_PROFILE_BIT : usize = 0x00000001;
pub const  GL_CONTEXT_COMPATIBILITY_PROFILE_BIT : usize = 0x00000002;
pub const  GL_LINES_ADJACENCY : usize = 0x000A;
pub const  GL_LINE_STRIP_ADJACENCY : usize = 0x000B;
pub const  GL_TRIANGLES_ADJACENCY : usize = 0x000C;
pub const  GL_TRIANGLE_STRIP_ADJACENCY : usize = 0x000D;
pub const  GL_PROGRAM_POINT_SIZE : usize = 0x8642;
pub const  GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS : usize = 0x8C29;
pub const  GL_FRAMEBUFFER_ATTACHMENT_LAYERED : usize = 0x8DA7;
pub const  GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS : usize = 0x8DA8;
pub const  GL_GEOMETRY_SHADER : usize = 0x8DD9;
pub const  GL_GEOMETRY_VERTICES_OUT : usize = 0x8916;
pub const  GL_GEOMETRY_INPUT_TYPE : usize = 0x8917;
pub const  GL_GEOMETRY_OUTPUT_TYPE : usize = 0x8918;
pub const  GL_MAX_GEOMETRY_UNIFORM_COMPONENTS : usize = 0x8DDF;
pub const  GL_MAX_GEOMETRY_OUTPUT_VERTICES : usize = 0x8DE0;
pub const  GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS : usize = 0x8DE1;
pub const  GL_MAX_VERTEX_OUTPUT_COMPONENTS : usize = 0x9122;
pub const  GL_MAX_GEOMETRY_INPUT_COMPONENTS : usize = 0x9123;
pub const  GL_MAX_GEOMETRY_OUTPUT_COMPONENTS : usize = 0x9124;
pub const  GL_MAX_FRAGMENT_INPUT_COMPONENTS : usize = 0x9125;
pub const  GL_CONTEXT_PROFILE_MASK : usize = 0x9126;
pub const  GL_DEPTH_CLAMP : usize = 0x864F;
pub const  GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION : usize = 0x8E4C;
pub const  GL_FIRST_VERTEX_CONVENTION : usize = 0x8E4D;
pub const  GL_LAST_VERTEX_CONVENTION : usize = 0x8E4E;
pub const  GL_PROVOKING_VERTEX : usize = 0x8E4F;
pub const  GL_TEXTURE_CUBE_MAP_SEAMLESS : usize = 0x884F;
pub const  GL_MAX_SERVER_WAIT_TIMEOUT : usize = 0x9111;
pub const  GL_OBJECT_TYPE : usize = 0x9112;
pub const  GL_SYNC_CONDITION : usize = 0x9113;
pub const  GL_SYNC_STATUS : usize = 0x9114;
pub const  GL_SYNC_FLAGS : usize = 0x9115;
pub const  GL_SYNC_FENCE : usize = 0x9116;
pub const  GL_SYNC_GPU_COMMANDS_COMPLETE : usize = 0x9117;
pub const  GL_UNSIGNALED : usize = 0x9118;
pub const  GL_SIGNALED : usize = 0x9119;
pub const  GL_ALREADY_SIGNALED : usize = 0x911A;
pub const  GL_TIMEOUT_EXPIRED : usize = 0x911B;
pub const  GL_CONDITION_SATISFIED : usize = 0x911C;
pub const  GL_WAIT_FAILED : usize = 0x911D;
pub const  GL_TIMEOUT_IGNORED : usize = 0xFFFFFFFFFFFFFFFF;
pub const  GL_SYNC_FLUSH_COMMANDS_BIT : usize = 0x00000001;
pub const  GL_SAMPLE_POSITION : usize = 0x8E50;
pub const  GL_SAMPLE_MASK : usize = 0x8E51;
pub const  GL_SAMPLE_MASK_VALUE : usize = 0x8E52;
pub const  GL_MAX_SAMPLE_MASK_WORDS : usize = 0x8E59;
pub const  GL_TEXTURE_2D_MULTISAMPLE : usize = 0x9100;
pub const  GL_PROXY_TEXTURE_2D_MULTISAMPLE : usize = 0x9101;
pub const  GL_TEXTURE_2D_MULTISAMPLE_ARRAY : usize = 0x9102;
pub const  GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY : usize = 0x9103;
pub const  GL_TEXTURE_BINDING_2D_MULTISAMPLE : usize = 0x9104;
pub const  GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY : usize = 0x9105;
pub const  GL_TEXTURE_SAMPLES : usize = 0x9106;
pub const  GL_TEXTURE_FIXED_SAMPLE_LOCATIONS : usize = 0x9107;
pub const  GL_SAMPLER_2D_MULTISAMPLE : usize = 0x9108;
pub const  GL_INT_SAMPLER_2D_MULTISAMPLE : usize = 0x9109;
pub const  GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE : usize = 0x910A;
pub const  GL_SAMPLER_2D_MULTISAMPLE_ARRAY : usize = 0x910B;
pub const  GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY : usize = 0x910C;
pub const  GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY : usize = 0x910D;
pub const  GL_MAX_COLOR_TEXTURE_SAMPLES : usize = 0x910E;
pub const  GL_MAX_DEPTH_TEXTURE_SAMPLES : usize = 0x910F;
pub const  GL_MAX_INTEGER_SAMPLES : usize = 0x9110;
pub const  GL_VERTEX_ATTRIB_ARRAY_DIVISOR : usize = 0x88FE;
pub const  GL_SRC1_COLOR : usize = 0x88F9;
pub const  GL_ONE_MINUS_SRC1_COLOR : usize = 0x88FA;
pub const  GL_ONE_MINUS_SRC1_ALPHA : usize = 0x88FB;
pub const  GL_MAX_DUAL_SOURCE_DRAW_BUFFERS : usize = 0x88FC;
pub const  GL_ANY_SAMPLES_PASSED : usize = 0x8C2F;
pub const  GL_SAMPLER_BINDING : usize = 0x8919;
pub const  GL_RGB10_A2UI : usize = 0x906F;
pub const  GL_TEXTURE_SWIZZLE_R : usize = 0x8E42;
pub const  GL_TEXTURE_SWIZZLE_G : usize = 0x8E43;
pub const  GL_TEXTURE_SWIZZLE_B : usize = 0x8E44;
pub const  GL_TEXTURE_SWIZZLE_A : usize = 0x8E45;
pub const  GL_TEXTURE_SWIZZLE_RGBA : usize = 0x8E46;
pub const  GL_TIME_ELAPSED : usize = 0x88BF;
pub const  GL_TIMESTAMP : usize = 0x8E28;
pub const  GL_INT_2_10_10_10_REV : usize = 0x8D9F;
pub const  GL_SAMPLE_SHADING : usize = 0x8C36;
pub const  GL_MIN_SAMPLE_SHADING_VALUE : usize = 0x8C37;
pub const  GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET : usize = 0x8E5E;
pub const  GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET : usize = 0x8E5F;
pub const  GL_TEXTURE_CUBE_MAP_ARRAY : usize = 0x9009;
pub const  GL_TEXTURE_BINDING_CUBE_MAP_ARRAY : usize = 0x900A;
pub const  GL_PROXY_TEXTURE_CUBE_MAP_ARRAY : usize = 0x900B;
pub const  GL_SAMPLER_CUBE_MAP_ARRAY : usize = 0x900C;
pub const  GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW : usize = 0x900D;
pub const  GL_INT_SAMPLER_CUBE_MAP_ARRAY : usize = 0x900E;
pub const  GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY : usize = 0x900F;
pub const  GL_DRAW_INDIRECT_BUFFER : usize = 0x8F3F;
pub const  GL_DRAW_INDIRECT_BUFFER_BINDING : usize = 0x8F43;
pub const  GL_GEOMETRY_SHADER_INVOCATIONS : usize = 0x887F;
pub const  GL_MAX_GEOMETRY_SHADER_INVOCATIONS : usize = 0x8E5A;
pub const  GL_MIN_FRAGMENT_INTERPOLATION_OFFSET : usize = 0x8E5B;
pub const  GL_MAX_FRAGMENT_INTERPOLATION_OFFSET : usize = 0x8E5C;
pub const  GL_FRAGMENT_INTERPOLATION_OFFSET_BITS : usize = 0x8E5D;
pub const  GL_MAX_VERTEX_STREAMS : usize = 0x8E71;
pub const  GL_DOUBLE_VEC2 : usize = 0x8FFC;
pub const  GL_DOUBLE_VEC3 : usize = 0x8FFD;
pub const  GL_DOUBLE_VEC4 : usize = 0x8FFE;
pub const  GL_DOUBLE_MAT2 : usize = 0x8F46;
pub const  GL_DOUBLE_MAT3 : usize = 0x8F47;
pub const  GL_DOUBLE_MAT4 : usize = 0x8F48;
pub const  GL_DOUBLE_MAT2x3 : usize = 0x8F49;
pub const  GL_DOUBLE_MAT2x4 : usize = 0x8F4A;
pub const  GL_DOUBLE_MAT3x2 : usize = 0x8F4B;
pub const  GL_DOUBLE_MAT3x4 : usize = 0x8F4C;
pub const  GL_DOUBLE_MAT4x2 : usize = 0x8F4D;
pub const  GL_DOUBLE_MAT4x3 : usize = 0x8F4E;
pub const  GL_ACTIVE_SUBROUTINES : usize = 0x8DE5;
pub const  GL_ACTIVE_SUBROUTINE_UNIFORMS : usize = 0x8DE6;
pub const  GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS : usize = 0x8E47;
pub const  GL_ACTIVE_SUBROUTINE_MAX_LENGTH : usize = 0x8E48;
pub const  GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH : usize = 0x8E49;
pub const  GL_MAX_SUBROUTINES : usize = 0x8DE7;
pub const  GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS : usize = 0x8DE8;
pub const  GL_NUM_COMPATIBLE_SUBROUTINES : usize = 0x8E4A;
pub const  GL_COMPATIBLE_SUBROUTINES : usize = 0x8E4B;
pub const  GL_PATCHES : usize = 0x000E;
pub const  GL_PATCH_VERTICES : usize = 0x8E72;
pub const  GL_PATCH_DEFAULT_INNER_LEVEL : usize = 0x8E73;
pub const  GL_PATCH_DEFAULT_OUTER_LEVEL : usize = 0x8E74;
pub const  GL_TESS_CONTROL_OUTPUT_VERTICES : usize = 0x8E75;
pub const  GL_TESS_GEN_MODE : usize = 0x8E76;
pub const  GL_TESS_GEN_SPACING : usize = 0x8E77;
pub const  GL_TESS_GEN_VERTEX_ORDER : usize = 0x8E78;
pub const  GL_TESS_GEN_POINT_MODE : usize = 0x8E79;
pub const  GL_ISOLINES : usize = 0x8E7A;
pub const  GL_FRACTIONAL_ODD : usize = 0x8E7B;
pub const  GL_FRACTIONAL_EVEN : usize = 0x8E7C;
pub const  GL_MAX_PATCH_VERTICES : usize = 0x8E7D;
pub const  GL_MAX_TESS_GEN_LEVEL : usize = 0x8E7E;
pub const  GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS : usize = 0x8E7F;
pub const  GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS : usize = 0x8E80;
pub const  GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS : usize = 0x8E81;
pub const  GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS : usize = 0x8E82;
pub const  GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS : usize = 0x8E83;
pub const  GL_MAX_TESS_PATCH_COMPONENTS : usize = 0x8E84;
pub const  GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS : usize = 0x8E85;
pub const  GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS : usize = 0x8E86;
pub const  GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS : usize = 0x8E89;
pub const  GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS : usize = 0x8E8A;
pub const  GL_MAX_TESS_CONTROL_INPUT_COMPONENTS : usize = 0x886C;
pub const  GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS : usize = 0x886D;
pub const  GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS : usize = 0x8E1E;
pub const  GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS : usize = 0x8E1F;
pub const  GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER : usize = 0x84F0;
pub const  GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER : usize = 0x84F1;
pub const  GL_STATIC_DRAW: usize = 0x88E4;
pub const  GL_TESS_EVALUATION_SHADER : usize = 0x8E87;
pub const  GL_TESS_CONTROL_SHADER : usize = 0x8E88;
pub const  GL_TRANSFORM_FEEDBACK : usize = 0x8E22;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED : usize = 0x8E23;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE : usize = 0x8E24;
pub const  GL_TRANSFORM_FEEDBACK_BINDING : usize = 0x8E25;
pub const  GL_MAX_TRANSFORM_FEEDBACK_BUFFERS : usize = 0x8E70;
pub const  GL_FIXED : usize = 0x140C;
pub const  GL_IMPLEMENTATION_COLOR_READ_TYPE : usize = 0x8B9A;
pub const  GL_IMPLEMENTATION_COLOR_READ_FORMAT : usize = 0x8B9B;
pub const  GL_LOW_FLOAT : usize = 0x8DF0;
pub const  GL_MEDIUM_FLOAT : usize = 0x8DF1;
pub const  GL_HIGH_FLOAT : usize = 0x8DF2;
pub const  GL_LOW_INT : usize = 0x8DF3;
pub const  GL_MEDIUM_INT : usize = 0x8DF4;
pub const  GL_HIGH_INT : usize = 0x8DF5;
pub const  GL_SHADER_COMPILER : usize = 0x8DFA;
pub const  GL_SHADER_BINARY_FORMATS : usize = 0x8DF8;
pub const  GL_NUM_SHADER_BINARY_FORMATS : usize = 0x8DF9;
pub const  GL_MAX_VERTEX_UNIFORM_VECTORS : usize = 0x8DFB;
pub const  GL_MAX_VARYING_VECTORS : usize = 0x8DFC;
pub const  GL_MAX_FRAGMENT_UNIFORM_VECTORS : usize = 0x8DFD;
pub const  GL_RGB565 : usize = 0x8D62;
pub const  GL_PROGRAM_BINARY_RETRIEVABLE_HINT : usize = 0x8257;
pub const  GL_PROGRAM_BINARY_LENGTH : usize = 0x8741;
pub const  GL_NUM_PROGRAM_BINARY_FORMATS : usize = 0x87FE;
pub const  GL_PROGRAM_BINARY_FORMATS : usize = 0x87FF;
pub const  GL_VERTEX_SHADER_BIT : usize = 0x00000001;
pub const  GL_FRAGMENT_SHADER_BIT : usize = 0x00000002;
pub const  GL_GEOMETRY_SHADER_BIT : usize = 0x00000004;
pub const  GL_TESS_CONTROL_SHADER_BIT : usize = 0x00000008;
pub const  GL_TESS_EVALUATION_SHADER_BIT : usize = 0x00000010;
pub const  GL_ALL_SHADER_BITS : usize = 0xFFFFFFFF;
pub const  GL_PROGRAM_SEPARABLE : usize = 0x8258;
pub const  GL_ACTIVE_PROGRAM : usize = 0x8259;
pub const  GL_PROGRAM_PIPELINE_BINDING : usize = 0x825A;
pub const  GL_MAX_VIEWPORTS : usize = 0x825B;
pub const  GL_VIEWPORT_SUBPIXEL_BITS : usize = 0x825C;
pub const  GL_VIEWPORT_BOUNDS_RANGE : usize = 0x825D;
pub const  GL_LAYER_PROVOKING_VERTEX : usize = 0x825E;
pub const  GL_VIEWPORT_INDEX_PROVOKING_VERTEX : usize = 0x825F;
pub const  GL_UNDEFINED_VERTEX : usize = 0x8260;
pub const  GL_COPY_READ_BUFFER_BINDING : usize = 0x8F36;
pub const  GL_COPY_WRITE_BUFFER_BINDING : usize = 0x8F37;
pub const  GL_TRANSFORM_FEEDBACK_ACTIVE : usize = 0x8E24;
pub const  GL_TRANSFORM_FEEDBACK_PAUSED : usize = 0x8E23;
pub const  GL_UNPACK_COMPRESSED_BLOCK_WIDTH : usize = 0x9127;
pub const  GL_UNPACK_COMPRESSED_BLOCK_HEIGHT : usize = 0x9128;
pub const  GL_UNPACK_COMPRESSED_BLOCK_DEPTH : usize = 0x9129;
pub const  GL_UNPACK_COMPRESSED_BLOCK_SIZE : usize = 0x912A;
pub const  GL_PACK_COMPRESSED_BLOCK_WIDTH : usize = 0x912B;
pub const  GL_PACK_COMPRESSED_BLOCK_HEIGHT : usize = 0x912C;
pub const  GL_PACK_COMPRESSED_BLOCK_DEPTH : usize = 0x912D;
pub const  GL_PACK_COMPRESSED_BLOCK_SIZE : usize = 0x912E;
pub const  GL_NUM_SAMPLE_COUNTS : usize = 0x9380;
pub const  GL_MIN_MAP_BUFFER_ALIGNMENT : usize = 0x90BC;
pub const  GL_ATOMIC_COUNTER_BUFFER : usize = 0x92C0;
pub const  GL_ATOMIC_COUNTER_BUFFER_BINDING : usize = 0x92C1;
pub const  GL_ATOMIC_COUNTER_BUFFER_START : usize = 0x92C2;
pub const  GL_ATOMIC_COUNTER_BUFFER_SIZE : usize = 0x92C3;
pub const  GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE : usize = 0x92C4;
pub const  GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS : usize = 0x92C5;
pub const  GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES : usize = 0x92C6;
pub const  GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER : usize = 0x92C7;
pub const  GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER : usize = 0x92C8;
pub const  GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER : usize = 0x92C9;
pub const  GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER : usize = 0x92CA;
pub const  GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER : usize = 0x92CB;
pub const  GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS : usize = 0x92CC;
pub const  GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS : usize = 0x92CD;
pub const  GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS : usize = 0x92CE;
pub const  GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS : usize = 0x92CF;
pub const  GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS : usize = 0x92D0;
pub const  GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS : usize = 0x92D1;
pub const  GL_MAX_VERTEX_ATOMIC_COUNTERS : usize = 0x92D2;
pub const  GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS : usize = 0x92D3;
pub const  GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS : usize = 0x92D4;
pub const  GL_MAX_GEOMETRY_ATOMIC_COUNTERS : usize = 0x92D5;
pub const  GL_MAX_FRAGMENT_ATOMIC_COUNTERS : usize = 0x92D6;
pub const  GL_MAX_COMBINED_ATOMIC_COUNTERS : usize = 0x92D7;
pub const  GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE : usize = 0x92D8;
pub const  GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS : usize = 0x92DC;
pub const  GL_ACTIVE_ATOMIC_COUNTER_BUFFERS : usize = 0x92D9;
pub const  GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX : usize = 0x92DA;
pub const  GL_UNSIGNED_INT_ATOMIC_COUNTER : usize = 0x92DB;
pub const  GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT : usize = 0x00000001;
pub const  GL_ELEMENT_ARRAY_BARRIER_BIT : usize = 0x00000002;
pub const  GL_UNIFORM_BARRIER_BIT : usize = 0x00000004;
pub const  GL_TEXTURE_FETCH_BARRIER_BIT : usize = 0x00000008;
pub const  GL_SHADER_IMAGE_ACCESS_BARRIER_BIT : usize = 0x00000020;
pub const  GL_COMMAND_BARRIER_BIT : usize = 0x00000040;
pub const  GL_PIXEL_BUFFER_BARRIER_BIT : usize = 0x00000080;
pub const  GL_TEXTURE_UPDATE_BARRIER_BIT : usize = 0x00000100;
pub const  GL_BUFFER_UPDATE_BARRIER_BIT : usize = 0x00000200;
pub const  GL_FRAMEBUFFER_BARRIER_BIT : usize = 0x00000400;
pub const  GL_TRANSFORM_FEEDBACK_BARRIER_BIT : usize = 0x00000800;
pub const  GL_ATOMIC_COUNTER_BARRIER_BIT : usize = 0x00001000;
pub const  GL_ALL_BARRIER_BITS : usize = 0xFFFFFFFF;
pub const  GL_MAX_IMAGE_UNITS : usize = 0x8F38;
pub const  GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS : usize = 0x8F39;
pub const  GL_IMAGE_BINDING_NAME : usize = 0x8F3A;
pub const  GL_IMAGE_BINDING_LEVEL : usize = 0x8F3B;
pub const  GL_IMAGE_BINDING_LAYERED : usize = 0x8F3C;
pub const  GL_IMAGE_BINDING_LAYER : usize = 0x8F3D;
pub const  GL_IMAGE_BINDING_ACCESS : usize = 0x8F3E;
pub const  GL_IMAGE_1D : usize = 0x904C;
pub const  GL_IMAGE_2D : usize = 0x904D;
pub const  GL_IMAGE_3D : usize = 0x904E;
pub const  GL_IMAGE_2D_RECT : usize = 0x904F;
pub const  GL_IMAGE_CUBE : usize = 0x9050;
pub const  GL_IMAGE_BUFFER : usize = 0x9051;
pub const  GL_IMAGE_1D_ARRAY : usize = 0x9052;
pub const  GL_IMAGE_2D_ARRAY : usize = 0x9053;
pub const  GL_IMAGE_CUBE_MAP_ARRAY : usize = 0x9054;
pub const  GL_IMAGE_2D_MULTISAMPLE : usize = 0x9055;
pub const  GL_IMAGE_2D_MULTISAMPLE_ARRAY : usize = 0x9056;
pub const  GL_INT_IMAGE_1D : usize = 0x9057;
pub const  GL_INT_IMAGE_2D : usize = 0x9058;
pub const  GL_INT_IMAGE_3D : usize = 0x9059;
pub const  GL_INT_IMAGE_2D_RECT : usize = 0x905A;
pub const  GL_INT_IMAGE_CUBE : usize = 0x905B;
pub const  GL_INT_IMAGE_BUFFER : usize = 0x905C;
pub const  GL_INT_IMAGE_1D_ARRAY : usize = 0x905D;
pub const  GL_INT_IMAGE_2D_ARRAY : usize = 0x905E;
pub const  GL_INT_IMAGE_CUBE_MAP_ARRAY : usize = 0x905F;
pub const  GL_INT_IMAGE_2D_MULTISAMPLE : usize = 0x9060;
pub const  GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY : usize = 0x9061;
pub const  GL_UNSIGNED_INT_IMAGE_1D : usize = 0x9062;
pub const  GL_UNSIGNED_INT_IMAGE_2D : usize = 0x9063;
pub const  GL_UNSIGNED_INT_IMAGE_3D : usize = 0x9064;
pub const  GL_UNSIGNED_INT_IMAGE_2D_RECT : usize = 0x9065;
pub const  GL_UNSIGNED_INT_IMAGE_CUBE : usize = 0x9066;
pub const  GL_UNSIGNED_INT_IMAGE_BUFFER : usize = 0x9067;
pub const  GL_UNSIGNED_INT_IMAGE_1D_ARRAY : usize = 0x9068;
pub const  GL_UNSIGNED_INT_IMAGE_2D_ARRAY : usize = 0x9069;
pub const  GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY : usize = 0x906A;
pub const  GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE : usize = 0x906B;
pub const  GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY : usize = 0x906C;
pub const  GL_MAX_IMAGE_SAMPLES : usize = 0x906D;
pub const  GL_IMAGE_BINDING_FORMAT : usize = 0x906E;
pub const  GL_IMAGE_FORMAT_COMPATIBILITY_TYPE : usize = 0x90C7;
pub const  GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE : usize = 0x90C8;
pub const  GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS : usize = 0x90C9;
pub const  GL_MAX_VERTEX_IMAGE_UNIFORMS : usize = 0x90CA;
pub const  GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS : usize = 0x90CB;
pub const  GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS : usize = 0x90CC;
pub const  GL_MAX_GEOMETRY_IMAGE_UNIFORMS : usize = 0x90CD;
pub const  GL_MAX_FRAGMENT_IMAGE_UNIFORMS : usize = 0x90CE;
pub const  GL_MAX_COMBINED_IMAGE_UNIFORMS : usize = 0x90CF;
pub const  GL_COMPRESSED_RGBA_BPTC_UNORM : usize = 0x8E8C;
pub const  GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM : usize = 0x8E8D;
pub const  GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT : usize = 0x8E8E;
pub const  GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT : usize = 0x8E8F;
pub const  GL_TEXTURE_IMMUTABLE_FORMAT : usize = 0x912F;
pub const  GL_NUM_SHADING_LANGUAGE_VERSIONS : usize = 0x82E9;
pub const  GL_VERTEX_ATTRIB_ARRAY_LONG : usize = 0x874E;
pub const  GL_COMPRESSED_RGB8_ETC2 : usize = 0x9274;
pub const  GL_COMPRESSED_SRGB8_ETC2 : usize = 0x9275;
pub const  GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 : usize = 0x9276;
pub const  GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 : usize = 0x9277;
pub const  GL_COMPRESSED_RGBA8_ETC2_EAC : usize = 0x9278;
pub const  GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC : usize = 0x9279;
pub const  GL_COMPRESSED_R11_EAC : usize = 0x9270;
pub const  GL_COMPRESSED_SIGNED_R11_EAC : usize = 0x9271;
pub const  GL_COMPRESSED_RG11_EAC : usize = 0x9272;
pub const  GL_COMPRESSED_SIGNED_RG11_EAC : usize = 0x9273;
pub const  GL_PRIMITIVE_RESTART_FIXED_INDEX : usize = 0x8D69;
pub const  GL_ANY_SAMPLES_PASSED_CONSERVATIVE : usize = 0x8D6A;
pub const  GL_MAX_ELEMENT_INDEX : usize = 0x8D6B;
pub const  GL_COMPUTE_SHADER : usize = 0x91B9;
pub const  GL_MAX_COMPUTE_UNIFORM_BLOCKS : usize = 0x91BB;
pub const  GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS : usize = 0x91BC;
pub const  GL_MAX_COMPUTE_IMAGE_UNIFORMS : usize = 0x91BD;
pub const  GL_MAX_COMPUTE_SHARED_MEMORY_SIZE : usize = 0x8262;
pub const  GL_MAX_COMPUTE_UNIFORM_COMPONENTS : usize = 0x8263;
pub const  GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS : usize = 0x8264;
pub const  GL_MAX_COMPUTE_ATOMIC_COUNTERS : usize = 0x8265;
pub const  GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS : usize = 0x8266;
pub const  GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS : usize = 0x90EB;
pub const  GL_MAX_COMPUTE_WORK_GROUP_COUNT : usize = 0x91BE;
pub const  GL_MAX_COMPUTE_WORK_GROUP_SIZE : usize = 0x91BF;
pub const  GL_COMPUTE_WORK_GROUP_SIZE : usize = 0x8267;
pub const  GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER : usize = 0x90EC;
pub const  GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER : usize = 0x90ED;
pub const  GL_DISPATCH_INDIRECT_BUFFER : usize = 0x90EE;
pub const  GL_DISPATCH_INDIRECT_BUFFER_BINDING : usize = 0x90EF;
pub const  GL_COMPUTE_SHADER_BIT : usize = 0x00000020;
pub const  GL_DEBUG_OUTPUT_SYNCHRONOUS : usize = 0x8242;
pub const  GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH : usize = 0x8243;
pub const  GL_DEBUG_CALLBACK_FUNCTION : usize = 0x8244;
pub const  GL_DEBUG_CALLBACK_USER_PARAM : usize = 0x8245;
pub const  GL_DEBUG_SOURCE_API : usize = 0x8246;
pub const  GL_DEBUG_SOURCE_WINDOW_SYSTEM : usize = 0x8247;
pub const  GL_DEBUG_SOURCE_SHADER_COMPILER : usize = 0x8248;
pub const  GL_DEBUG_SOURCE_THIRD_PARTY : usize = 0x8249;
pub const  GL_DEBUG_SOURCE_APPLICATION : usize = 0x824A;
pub const  GL_DEBUG_SOURCE_OTHER : usize = 0x824B;
pub const  GL_DEBUG_TYPE_ERROR : usize = 0x824C;
pub const  GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR : usize = 0x824D;
pub const  GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR : usize = 0x824E;
pub const  GL_DEBUG_TYPE_PORTABILITY : usize = 0x824F;
pub const  GL_DEBUG_TYPE_PERFORMANCE : usize = 0x8250;
pub const  GL_DEBUG_TYPE_OTHER : usize = 0x8251;
pub const  GL_MAX_DEBUG_MESSAGE_LENGTH : usize = 0x9143;
pub const  GL_MAX_DEBUG_LOGGED_MESSAGES : usize = 0x9144;
pub const  GL_DEBUG_LOGGED_MESSAGES : usize = 0x9145;
pub const  GL_DEBUG_SEVERITY_HIGH : usize = 0x9146;
pub const  GL_DEBUG_SEVERITY_MEDIUM : usize = 0x9147;
pub const  GL_DEBUG_SEVERITY_LOW : usize = 0x9148;
pub const  GL_DEBUG_TYPE_MARKER : usize = 0x8268;
pub const  GL_DEBUG_TYPE_PUSH_GROUP : usize = 0x8269;
pub const  GL_DEBUG_TYPE_POP_GROUP : usize = 0x826A;
pub const  GL_DEBUG_SEVERITY_NOTIFICATION : usize = 0x826B;
pub const  GL_MAX_DEBUG_GROUP_STACK_DEPTH : usize = 0x826C;
pub const  GL_DEBUG_GROUP_STACK_DEPTH : usize = 0x826D;
pub const  GL_BUFFER : usize = 0x82E0;
pub const  GL_SHADER : usize = 0x82E1;
pub const  GL_PROGRAM : usize = 0x82E2;
pub const  GL_QUERY : usize = 0x82E3;
pub const  GL_PROGRAM_PIPELINE : usize = 0x82E4;
pub const  GL_SAMPLER : usize = 0x82E6;
pub const  GL_MAX_LABEL_LENGTH : usize = 0x82E8;
pub const  GL_DEBUG_OUTPUT : usize = 0x92E0;
pub const  GL_CONTEXT_FLAG_DEBUG_BIT : usize = 0x00000002;
pub const  GL_MAX_UNIFORM_LOCATIONS : usize = 0x826E;
pub const  GL_FRAMEBUFFER_DEFAULT_WIDTH : usize = 0x9310;
pub const  GL_FRAMEBUFFER_DEFAULT_HEIGHT : usize = 0x9311;
pub const  GL_FRAMEBUFFER_DEFAULT_LAYERS : usize = 0x9312;
pub const  GL_FRAMEBUFFER_DEFAULT_SAMPLES : usize = 0x9313;
pub const  GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS : usize = 0x9314;
pub const  GL_MAX_FRAMEBUFFER_WIDTH : usize = 0x9315;
pub const  GL_MAX_FRAMEBUFFER_HEIGHT : usize = 0x9316;
pub const  GL_MAX_FRAMEBUFFER_LAYERS : usize = 0x9317;
pub const  GL_MAX_FRAMEBUFFER_SAMPLES : usize = 0x9318;
pub const  GL_INTERNALFORMAT_SUPPORTED : usize = 0x826F;
pub const  GL_INTERNALFORMAT_PREFERRED : usize = 0x8270;
pub const  GL_INTERNALFORMAT_RED_SIZE : usize = 0x8271;
pub const  GL_INTERNALFORMAT_GREEN_SIZE : usize = 0x8272;
pub const  GL_INTERNALFORMAT_BLUE_SIZE : usize = 0x8273;
pub const  GL_INTERNALFORMAT_ALPHA_SIZE : usize = 0x8274;
pub const  GL_INTERNALFORMAT_DEPTH_SIZE : usize = 0x8275;
pub const  GL_INTERNALFORMAT_STENCIL_SIZE : usize = 0x8276;
pub const  GL_INTERNALFORMAT_SHARED_SIZE : usize = 0x8277;
pub const  GL_INTERNALFORMAT_RED_TYPE : usize = 0x8278;
pub const  GL_INTERNALFORMAT_GREEN_TYPE : usize = 0x8279;
pub const  GL_INTERNALFORMAT_BLUE_TYPE : usize = 0x827A;
pub const  GL_INTERNALFORMAT_ALPHA_TYPE : usize = 0x827B;
pub const  GL_INTERNALFORMAT_DEPTH_TYPE : usize = 0x827C;
pub const  GL_INTERNALFORMAT_STENCIL_TYPE : usize = 0x827D;
pub const  GL_MAX_WIDTH : usize = 0x827E;
pub const  GL_MAX_HEIGHT : usize = 0x827F;
pub const  GL_MAX_DEPTH : usize = 0x8280;
pub const  GL_MAX_LAYERS : usize = 0x8281;
pub const  GL_MAX_COMBINED_DIMENSIONS : usize = 0x8282;
pub const  GL_COLOR_COMPONENTS : usize = 0x8283;
pub const  GL_DEPTH_COMPONENTS : usize = 0x8284;
pub const  GL_STENCIL_COMPONENTS : usize = 0x8285;
pub const  GL_COLOR_RENDERABLE : usize = 0x8286;
pub const  GL_DEPTH_RENDERABLE : usize = 0x8287;
pub const  GL_STENCIL_RENDERABLE : usize = 0x8288;
pub const  GL_FRAMEBUFFER_RENDERABLE : usize = 0x8289;
pub const  GL_FRAMEBUFFER_RENDERABLE_LAYERED : usize = 0x828A;
pub const  GL_FRAMEBUFFER_BLEND : usize = 0x828B;
pub const  GL_READ_PIXELS : usize = 0x828C;
pub const  GL_READ_PIXELS_FORMAT : usize = 0x828D;
pub const  GL_READ_PIXELS_TYPE : usize = 0x828E;
pub const  GL_TEXTURE_IMAGE_FORMAT : usize = 0x828F;
pub const  GL_TEXTURE_IMAGE_TYPE : usize = 0x8290;
pub const  GL_GET_TEXTURE_IMAGE_FORMAT : usize = 0x8291;
pub const  GL_GET_TEXTURE_IMAGE_TYPE : usize = 0x8292;
pub const  GL_MIPMAP : usize = 0x8293;
pub const  GL_MANUAL_GENERATE_MIPMAP : usize = 0x8294;
pub const  GL_AUTO_GENERATE_MIPMAP : usize = 0x8295;
pub const  GL_COLOR_ENCODING : usize = 0x8296;
pub const  GL_SRGB_READ : usize = 0x8297;
pub const  GL_SRGB_WRITE : usize = 0x8298;
pub const  GL_FILTER : usize = 0x829A;
pub const  GL_VERTEX_TEXTURE : usize = 0x829B;
pub const  GL_TESS_CONTROL_TEXTURE : usize = 0x829C;
pub const  GL_TESS_EVALUATION_TEXTURE : usize = 0x829D;
pub const  GL_GEOMETRY_TEXTURE : usize = 0x829E;
pub const  GL_FRAGMENT_TEXTURE : usize = 0x829F;
pub const  GL_COMPUTE_TEXTURE : usize = 0x82A0;
pub const  GL_TEXTURE_SHADOW : usize = 0x82A1;
pub const  GL_TEXTURE_GATHER : usize = 0x82A2;
pub const  GL_TEXTURE_GATHER_SHADOW : usize = 0x82A3;
pub const  GL_SHADER_IMAGE_LOAD : usize = 0x82A4;
pub const  GL_SHADER_IMAGE_STORE : usize = 0x82A5;
pub const  GL_SHADER_IMAGE_ATOMIC : usize = 0x82A6;
pub const  GL_IMAGE_TEXEL_SIZE : usize = 0x82A7;
pub const  GL_IMAGE_COMPATIBILITY_CLASS : usize = 0x82A8;
pub const  GL_IMAGE_PIXEL_FORMAT : usize = 0x82A9;
pub const  GL_IMAGE_PIXEL_TYPE : usize = 0x82AA;
pub const  GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST : usize = 0x82AC;
pub const  GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST : usize = 0x82AD;
pub const  GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE : usize = 0x82AE;
pub const  GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE : usize = 0x82AF;
pub const  GL_TEXTURE_COMPRESSED_BLOCK_WIDTH : usize = 0x82B1;
pub const  GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT : usize = 0x82B2;
pub const  GL_TEXTURE_COMPRESSED_BLOCK_SIZE : usize = 0x82B3;
pub const  GL_CLEAR_BUFFER : usize = 0x82B4;
pub const  GL_TEXTURE_VIEW : usize = 0x82B5;
pub const  GL_VIEW_COMPATIBILITY_CLASS : usize = 0x82B6;
pub const  GL_FULL_SUPPORT : usize = 0x82B7;
pub const  GL_CAVEAT_SUPPORT : usize = 0x82B8;
pub const  GL_IMAGE_CLASS_4_X_32 : usize = 0x82B9;
pub const  GL_IMAGE_CLASS_2_X_32 : usize = 0x82BA;
pub const  GL_IMAGE_CLASS_1_X_32 : usize = 0x82BB;
pub const  GL_IMAGE_CLASS_4_X_16 : usize = 0x82BC;
pub const  GL_IMAGE_CLASS_2_X_16 : usize = 0x82BD;
pub const  GL_IMAGE_CLASS_1_X_16 : usize = 0x82BE;
pub const  GL_IMAGE_CLASS_4_X_8 : usize = 0x82BF;
pub const  GL_IMAGE_CLASS_2_X_8 : usize = 0x82C0;
pub const  GL_IMAGE_CLASS_1_X_8 : usize = 0x82C1;
pub const  GL_IMAGE_CLASS_11_11_10 : usize = 0x82C2;
pub const  GL_IMAGE_CLASS_10_10_10_2 : usize = 0x82C3;
pub const  GL_VIEW_CLASS_128_BITS : usize = 0x82C4;
pub const  GL_VIEW_CLASS_96_BITS : usize = 0x82C5;
pub const  GL_VIEW_CLASS_64_BITS : usize = 0x82C6;
pub const  GL_VIEW_CLASS_48_BITS : usize = 0x82C7;
pub const  GL_VIEW_CLASS_32_BITS : usize = 0x82C8;
pub const  GL_VIEW_CLASS_24_BITS : usize = 0x82C9;
pub const  GL_VIEW_CLASS_16_BITS : usize = 0x82CA;
pub const  GL_VIEW_CLASS_8_BITS : usize = 0x82CB;
pub const  GL_VIEW_CLASS_S3TC_DXT1_RGB : usize = 0x82CC;
pub const  GL_VIEW_CLASS_S3TC_DXT1_RGBA : usize = 0x82CD;
pub const  GL_VIEW_CLASS_S3TC_DXT3_RGBA : usize = 0x82CE;
pub const  GL_VIEW_CLASS_S3TC_DXT5_RGBA : usize = 0x82CF;
pub const  GL_VIEW_CLASS_RGTC1_RED : usize = 0x82D0;
pub const  GL_VIEW_CLASS_RGTC2_RG : usize = 0x82D1;
pub const  GL_VIEW_CLASS_BPTC_UNORM : usize = 0x82D2;
pub const  GL_VIEW_CLASS_BPTC_FLOAT : usize = 0x82D3;
pub const  GL_UNIFORM : usize = 0x92E1;
pub const  GL_UNIFORM_BLOCK : usize = 0x92E2;
pub const  GL_PROGRAM_INPUT : usize = 0x92E3;
pub const  GL_PROGRAM_OUTPUT : usize = 0x92E4;
pub const  GL_BUFFER_VARIABLE : usize = 0x92E5;
pub const  GL_SHADER_STORAGE_BLOCK : usize = 0x92E6;
pub const  GL_VERTEX_SUBROUTINE : usize = 0x92E8;
pub const  GL_TESS_CONTROL_SUBROUTINE : usize = 0x92E9;
pub const  GL_TESS_EVALUATION_SUBROUTINE : usize = 0x92EA;
pub const  GL_GEOMETRY_SUBROUTINE : usize = 0x92EB;
pub const  GL_FRAGMENT_SUBROUTINE : usize = 0x92EC;
pub const  GL_COMPUTE_SUBROUTINE : usize = 0x92ED;
pub const  GL_VERTEX_SUBROUTINE_UNIFORM : usize = 0x92EE;
pub const  GL_TESS_CONTROL_SUBROUTINE_UNIFORM : usize = 0x92EF;
pub const  GL_TESS_EVALUATION_SUBROUTINE_UNIFORM : usize = 0x92F0;
pub const  GL_GEOMETRY_SUBROUTINE_UNIFORM : usize = 0x92F1;
pub const  GL_FRAGMENT_SUBROUTINE_UNIFORM : usize = 0x92F2;
pub const  GL_COMPUTE_SUBROUTINE_UNIFORM : usize = 0x92F3;
pub const  GL_TRANSFORM_FEEDBACK_VARYING : usize = 0x92F4;
pub const  GL_ACTIVE_RESOURCES : usize = 0x92F5;
pub const  GL_MAX_NAME_LENGTH : usize = 0x92F6;
pub const  GL_MAX_NUM_ACTIVE_VARIABLES : usize = 0x92F7;
pub const  GL_MAX_NUM_COMPATIBLE_SUBROUTINES : usize = 0x92F8;
pub const  GL_NAME_LENGTH : usize = 0x92F9;
pub const  GL_TYPE : usize = 0x92FA;
pub const  GL_ARRAY_SIZE : usize = 0x92FB;
pub const  GL_OFFSET : usize = 0x92FC;
pub const  GL_BLOCK_INDEX : usize = 0x92FD;
pub const  GL_ARRAY_STRIDE : usize = 0x92FE;
pub const  GL_MATRIX_STRIDE : usize = 0x92FF;
pub const  GL_IS_ROW_MAJOR : usize = 0x9300;
pub const  GL_ATOMIC_COUNTER_BUFFER_INDEX : usize = 0x9301;
pub const  GL_BUFFER_BINDING : usize = 0x9302;
pub const  GL_BUFFER_DATA_SIZE : usize = 0x9303;
pub const  GL_NUM_ACTIVE_VARIABLES : usize = 0x9304;
pub const  GL_ACTIVE_VARIABLES : usize = 0x9305;
pub const  GL_REFERENCED_BY_VERTEX_SHADER : usize = 0x9306;
pub const  GL_REFERENCED_BY_TESS_CONTROL_SHADER : usize = 0x9307;
pub const  GL_REFERENCED_BY_TESS_EVALUATION_SHADER : usize = 0x9308;
pub const  GL_REFERENCED_BY_GEOMETRY_SHADER : usize = 0x9309;
pub const  GL_REFERENCED_BY_FRAGMENT_SHADER : usize = 0x930A;
pub const  GL_REFERENCED_BY_COMPUTE_SHADER : usize = 0x930B;
pub const  GL_TOP_LEVEL_ARRAY_SIZE : usize = 0x930C;
pub const  GL_TOP_LEVEL_ARRAY_STRIDE : usize = 0x930D;
pub const  GL_LOCATION : usize = 0x930E;
pub const  GL_LOCATION_INDEX : usize = 0x930F;
pub const  GL_IS_PER_PATCH : usize = 0x92E7;
pub const  GL_SHADER_STORAGE_BUFFER : usize = 0x90D2;
pub const  GL_SHADER_STORAGE_BUFFER_BINDING : usize = 0x90D3;
pub const  GL_SHADER_STORAGE_BUFFER_START : usize = 0x90D4;
pub const  GL_SHADER_STORAGE_BUFFER_SIZE : usize = 0x90D5;
pub const  GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS : usize = 0x90D6;
pub const  GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS : usize = 0x90D7;
pub const  GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS : usize = 0x90D8;
pub const  GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS : usize = 0x90D9;
pub const  GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS : usize = 0x90DA;
pub const  GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS : usize = 0x90DB;
pub const  GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS : usize = 0x90DC;
pub const  GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS : usize = 0x90DD;
pub const  GL_MAX_SHADER_STORAGE_BLOCK_SIZE : usize = 0x90DE;
pub const  GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT : usize = 0x90DF;
pub const  GL_SHADER_STORAGE_BARRIER_BIT : usize = 0x00002000;
pub const  GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES : usize = 0x8F39;
pub const  GL_DEPTH_STENCIL_TEXTURE_MODE : usize = 0x90EA;
pub const  GL_TEXTURE_BUFFER_OFFSET : usize = 0x919D;
pub const  GL_TEXTURE_BUFFER_SIZE : usize = 0x919E;
pub const  GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT : usize = 0x919F;
pub const  GL_TEXTURE_VIEW_MIN_LEVEL : usize = 0x82DB;
pub const  GL_TEXTURE_VIEW_NUM_LEVELS : usize = 0x82DC;
pub const  GL_TEXTURE_VIEW_MIN_LAYER : usize = 0x82DD;
pub const  GL_TEXTURE_VIEW_NUM_LAYERS : usize = 0x82DE;
pub const  GL_TEXTURE_IMMUTABLE_LEVELS : usize = 0x82DF;
pub const  GL_VERTEX_ATTRIB_BINDING : usize = 0x82D4;
pub const  GL_VERTEX_ATTRIB_RELATIVE_OFFSET : usize = 0x82D5;
pub const  GL_VERTEX_BINDING_DIVISOR : usize = 0x82D6;
pub const  GL_VERTEX_BINDING_OFFSET : usize = 0x82D7;
pub const  GL_VERTEX_BINDING_STRIDE : usize = 0x82D8;
pub const  GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET : usize = 0x82D9;
pub const  GL_MAX_VERTEX_ATTRIB_BINDINGS : usize = 0x82DA;
pub const  GL_VERTEX_BINDING_BUFFER : usize = 0x8F4F;
pub const  GL_DISPLAY_LIST : usize = 0x82E7;
pub const  GL_MAX_VERTEX_ATTRIB_STRIDE : usize = 0x82E5;
pub const  GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED : usize = 0x8221;
pub const  GL_TEXTURE_BUFFER_BINDING : usize = 0x8C2A;
pub const  GL_MAP_PERSISTENT_BIT : usize = 0x0040;
pub const  GL_MAP_COHERENT_BIT : usize = 0x0080;
pub const  GL_DYNAMIC_STORAGE_BIT : usize = 0x0100;
pub const  GL_CLIENT_STORAGE_BIT : usize = 0x0200;
pub const  GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT : usize = 0x00004000;
pub const  GL_BUFFER_IMMUTABLE_STORAGE : usize = 0x821F;
pub const  GL_BUFFER_STORAGE_FLAGS : usize = 0x8220;
pub const  GL_CLEAR_TEXTURE : usize = 0x9365;
pub const  GL_LOCATION_COMPONENT : usize = 0x934A;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_INDEX : usize = 0x934B;
pub const  GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE : usize = 0x934C;
pub const  GL_QUERY_BUFFER : usize = 0x9192;
pub const  GL_QUERY_BUFFER_BARRIER_BIT : usize = 0x00008000;
pub const  GL_QUERY_BUFFER_BINDING : usize = 0x9193;
pub const  GL_QUERY_RESULT_NO_WAIT : usize = 0x9194;
pub const  GL_MIRROR_CLAMP_TO_EDGE : usize = 0x8743;
pub const  GL_CONTEXT_LOST : usize = 0x0507;
pub const  GL_NEGATIVE_ONE_TO_ONE : usize = 0x935E;
pub const  GL_ZERO_TO_ONE : usize = 0x935F;
pub const  GL_CLIP_ORIGIN : usize = 0x935C;
pub const  GL_CLIP_DEPTH_MODE : usize = 0x935D;
pub const  GL_QUERY_WAIT_INVERTED : usize = 0x8E17;
pub const  GL_QUERY_NO_WAIT_INVERTED : usize = 0x8E18;
pub const  GL_QUERY_BY_REGION_WAIT_INVERTED : usize = 0x8E19;
pub const  GL_QUERY_BY_REGION_NO_WAIT_INVERTED : usize = 0x8E1A;
pub const  GL_MAX_CULL_DISTANCES : usize = 0x82F9;
pub const  GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES : usize = 0x82FA;
pub const  GL_TEXTURE_TARGET : usize = 0x1006;
pub const  GL_QUERY_TARGET : usize = 0x82EA;
pub const  GL_GUILTY_CONTEXT_RESET : usize = 0x8253;
pub const  GL_INNOCENT_CONTEXT_RESET : usize = 0x8254;
pub const  GL_UNKNOWN_CONTEXT_RESET : usize = 0x8255;
pub const  GL_RESET_NOTIFICATION_STRATEGY : usize = 0x8256;
pub const  GL_LOSE_CONTEXT_ON_RESET : usize = 0x8252;
pub const  GL_NO_RESET_NOTIFICATION : usize = 0x8261;
pub const  GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT : usize = 0x00000004;
pub const  GL_CONTEXT_RELEASE_BEHAVIOR : usize = 0x82FB;
pub const  GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH : usize = 0x82FC;
pub const  GL_SHADER_BINARY_FORMAT_SPIR_V : usize = 0x9551;
pub const  GL_SPIR_V_BINARY : usize = 0x9552;
pub const  GL_PARAMETER_BUFFER : usize = 0x80EE;
pub const  GL_PARAMETER_BUFFER_BINDING : usize = 0x80EF;
pub const  GL_CONTEXT_FLAG_NO_ERROR_BIT : usize = 0x00000008;
pub const  GL_VERTICES_SUBMITTED : usize = 0x82EE;
pub const  GL_PRIMITIVES_SUBMITTED : usize = 0x82EF;
pub const  GL_VERTEX_SHADER_INVOCATIONS : usize = 0x82F0;
pub const  GL_TESS_CONTROL_SHADER_PATCHES : usize = 0x82F1;
pub const  GL_TESS_EVALUATION_SHADER_INVOCATIONS : usize = 0x82F2;
pub const  GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED : usize = 0x82F3;
pub const  GL_FRAGMENT_SHADER_INVOCATIONS : usize = 0x82F4;
pub const  GL_COMPUTE_SHADER_INVOCATIONS : usize = 0x82F5;
pub const  GL_CLIPPING_INPUT_PRIMITIVES : usize = 0x82F6;
pub const  GL_CLIPPING_OUTPUT_PRIMITIVES : usize = 0x82F7;
pub const  GL_POLYGON_OFFSET_CLAMP : usize = 0x8E1B;
pub const  GL_SPIR_V_EXTENSIONS : usize = 0x9553;
pub const  GL_NUM_SPIR_V_EXTENSIONS : usize = 0x9554;
pub const  GL_TEXTURE_MAX_ANISOTROPY : usize = 0x84FE;
pub const  GL_MAX_TEXTURE_MAX_ANISOTROPY : usize = 0x84FF;
pub const  GL_TRANSFORM_FEEDBACK_OVERFLOW : usize = 0x82EC;
pub const  GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW : usize = 0x82ED;


/*************************************************************************
 * GLFW API tokens
 *************************************************************************/

pub const  GLFW_VERSION_MAJOR : usize = 3;

pub const  GLFW_VERSION_MINOR : usize = 2;

pub const  GLFW_VERSION_REVISION : usize = 1;



pub const  GLFW_TRUE : usize = 1;

pub const  GLFW_FALSE : usize = 0;

pub const  GLFW_RELEASE : usize = 0;

pub const  GLFW_PRESS : usize = 1;

pub const  GLFW_REPEAT : usize = 2;


/* The unknown key */
pub const  GLFW_KEY_UNKNOWN : isize = -1;

/* Printable keys */
pub const  GLFW_KEY_SPACE : usize = 32;
pub const  GLFW_KEY_APOSTROPHE : usize = 39;
  /* ' */
pub const  GLFW_KEY_COMMA : usize = 44;
  /* , */
pub const  GLFW_KEY_MINUS : usize = 45;
  /* - */
pub const  GLFW_KEY_PERIOD : usize = 46;
  /* . */
pub const  GLFW_KEY_SLASH : usize = 47;
  /* / */
pub const  GLFW_KEY_0 : usize = 48;
pub const  GLFW_KEY_1 : usize = 49;
pub const  GLFW_KEY_2 : usize = 50;
pub const  GLFW_KEY_3 : usize = 51;
pub const  GLFW_KEY_4 : usize = 52;
pub const  GLFW_KEY_5 : usize = 53;
pub const  GLFW_KEY_6 : usize = 54;
pub const  GLFW_KEY_7 : usize = 55;
pub const  GLFW_KEY_8 : usize = 56;
pub const  GLFW_KEY_9 : usize = 57;
pub const  GLFW_KEY_SEMICOLON : usize = 59;
  /* ; */
pub const  GLFW_KEY_EQUAL : usize = 61;
  /* = */
pub const  GLFW_KEY_A : usize = 65;
pub const  GLFW_KEY_B : usize = 66;
pub const  GLFW_KEY_C : usize = 67;
pub const  GLFW_KEY_D : usize = 68;
pub const  GLFW_KEY_E : usize = 69;
pub const  GLFW_KEY_F : usize = 70;
pub const  GLFW_KEY_G : usize = 71;
pub const  GLFW_KEY_H : usize = 72;
pub const  GLFW_KEY_I : usize = 73;
pub const  GLFW_KEY_J : usize = 74;
pub const  GLFW_KEY_K : usize = 75;
pub const  GLFW_KEY_L : usize = 76;
pub const  GLFW_KEY_M : usize = 77;
pub const  GLFW_KEY_N : usize = 78;
pub const  GLFW_KEY_O : usize = 79;
pub const  GLFW_KEY_P : usize = 80;
pub const  GLFW_KEY_Q : usize = 81;
pub const  GLFW_KEY_R : usize = 82;
pub const  GLFW_KEY_S : usize = 83;
pub const  GLFW_KEY_T : usize = 84;
pub const  GLFW_KEY_U : usize = 85;
pub const  GLFW_KEY_V : usize = 86;
pub const  GLFW_KEY_W : usize = 87;
pub const  GLFW_KEY_X : usize = 88;
pub const  GLFW_KEY_Y : usize = 89;
pub const  GLFW_KEY_Z : usize = 90;
pub const  GLFW_KEY_LEFT_BRACKET : usize = 91;
  /* [ */
pub const  GLFW_KEY_BACKSLASH : usize = 92;
  /* \ */
pub const  GLFW_KEY_RIGHT_BRACKET : usize = 93;
  /* ] */
pub const  GLFW_KEY_GRAVE_ACCENT : usize = 96;
  /* ` */
pub const  GLFW_KEY_WORLD_1 : usize = 161;
 /* non-US #1 */
pub const  GLFW_KEY_WORLD_2 : usize = 162;
 /* non-US #2 */

/* Function keys */
pub const  GLFW_KEY_ESCAPE : usize = 256;
pub const  GLFW_KEY_ENTER : usize = 257;
pub const  GLFW_KEY_TAB : usize = 258;
pub const  GLFW_KEY_BACKSPACE : usize = 259;
pub const  GLFW_KEY_INSERT : usize = 260;
pub const  GLFW_KEY_DELETE : usize = 261;
pub const  GLFW_KEY_RIGHT : usize = 262;
pub const  GLFW_KEY_LEFT : usize = 263;
pub const  GLFW_KEY_DOWN : usize = 264;
pub const  GLFW_KEY_UP : usize = 265;
pub const  GLFW_KEY_PAGE_UP : usize = 266;
pub const  GLFW_KEY_PAGE_DOWN : usize = 267;
pub const  GLFW_KEY_HOME : usize = 268;
pub const  GLFW_KEY_END : usize = 269;
pub const  GLFW_KEY_CAPS_LOCK : usize = 280;
pub const  GLFW_KEY_SCROLL_LOCK : usize = 281;
pub const  GLFW_KEY_NUM_LOCK : usize = 282;
pub const  GLFW_KEY_PRINT_SCREEN : usize = 283;
pub const  GLFW_KEY_PAUSE : usize = 284;
pub const  GLFW_KEY_F1 : usize = 290;
pub const  GLFW_KEY_F2 : usize = 291;
pub const  GLFW_KEY_F3 : usize = 292;
pub const  GLFW_KEY_F4 : usize = 293;
pub const  GLFW_KEY_F5 : usize = 294;
pub const  GLFW_KEY_F6 : usize = 295;
pub const  GLFW_KEY_F7 : usize = 296;
pub const  GLFW_KEY_F8 : usize = 297;
pub const  GLFW_KEY_F9 : usize = 298;
pub const  GLFW_KEY_F10 : usize = 299;
pub const  GLFW_KEY_F11 : usize = 300;
pub const  GLFW_KEY_F12 : usize = 301;
pub const  GLFW_KEY_F13 : usize = 302;
pub const  GLFW_KEY_F14 : usize = 303;
pub const  GLFW_KEY_F15 : usize = 304;
pub const  GLFW_KEY_F16 : usize = 305;
pub const  GLFW_KEY_F17 : usize = 306;
pub const  GLFW_KEY_F18 : usize = 307;
pub const  GLFW_KEY_F19 : usize = 308;
pub const  GLFW_KEY_F20 : usize = 309;
pub const  GLFW_KEY_F21 : usize = 310;
pub const  GLFW_KEY_F22 : usize = 311;
pub const  GLFW_KEY_F23 : usize = 312;
pub const  GLFW_KEY_F24 : usize = 313;
pub const  GLFW_KEY_F25 : usize = 314;
pub const  GLFW_KEY_KP_0 : usize = 320;
pub const  GLFW_KEY_KP_1 : usize = 321;
pub const  GLFW_KEY_KP_2 : usize = 322;
pub const  GLFW_KEY_KP_3 : usize = 323;
pub const  GLFW_KEY_KP_4 : usize = 324;
pub const  GLFW_KEY_KP_5 : usize = 325;
pub const  GLFW_KEY_KP_6 : usize = 326;
pub const  GLFW_KEY_KP_7 : usize = 327;
pub const  GLFW_KEY_KP_8 : usize = 328;
pub const  GLFW_KEY_KP_9 : usize = 329;
pub const  GLFW_KEY_KP_DECIMAL : usize = 330;
pub const  GLFW_KEY_KP_DIVIDE : usize = 331;
pub const  GLFW_KEY_KP_MULTIPLY : usize = 332;
pub const  GLFW_KEY_KP_SUBTRACT : usize = 333;
pub const  GLFW_KEY_KP_ADD : usize = 334;
pub const  GLFW_KEY_KP_ENTER : usize = 335;
pub const  GLFW_KEY_KP_EQUAL : usize = 336;
pub const  GLFW_KEY_LEFT_SHIFT : usize = 340;
pub const  GLFW_KEY_LEFT_CONTROL : usize = 341;
pub const  GLFW_KEY_LEFT_ALT : usize = 342;
pub const  GLFW_KEY_LEFT_SUPER : usize = 343;
pub const  GLFW_KEY_RIGHT_SHIFT : usize = 344;
pub const  GLFW_KEY_RIGHT_CONTROL : usize = 345;
pub const  GLFW_KEY_RIGHT_ALT : usize = 346;
pub const  GLFW_KEY_RIGHT_SUPER : usize = 347;
pub const  GLFW_KEY_MENU : usize = 348;

pub const  GLFW_KEY_LAST : usize = GLFW_KEY_MENU;




pub const  GLFW_MOD_SHIFT : usize = 0x0001;

pub const  GLFW_MOD_CONTROL : usize = 0x0002;

pub const  GLFW_MOD_ALT : usize = 0x0004;

pub const  GLFW_MOD_SUPER : usize = 0x0008;


pub const  GLFW_MOUSE_BUTTON_1 : usize = 0;
pub const  GLFW_MOUSE_BUTTON_2 : usize = 1;
pub const  GLFW_MOUSE_BUTTON_3 : usize = 2;
pub const  GLFW_MOUSE_BUTTON_4 : usize = 3;
pub const  GLFW_MOUSE_BUTTON_5 : usize = 4;
pub const  GLFW_MOUSE_BUTTON_6 : usize = 5;
pub const  GLFW_MOUSE_BUTTON_7 : usize = 6;
pub const  GLFW_MOUSE_BUTTON_8 : usize = 7;
pub const  GLFW_MOUSE_BUTTON_LAST : usize = GLFW_MOUSE_BUTTON_8;
pub const  GLFW_MOUSE_BUTTON_LEFT : usize = GLFW_MOUSE_BUTTON_1;
pub const  GLFW_MOUSE_BUTTON_RIGHT : usize = GLFW_MOUSE_BUTTON_2;
pub const  GLFW_MOUSE_BUTTON_MIDDLE : usize = GLFW_MOUSE_BUTTON_3;

pub const  GLFW_JOYSTICK_1 : usize = 0;
pub const  GLFW_JOYSTICK_2 : usize = 1;
pub const  GLFW_JOYSTICK_3 : usize = 2;
pub const  GLFW_JOYSTICK_4 : usize = 3;
pub const  GLFW_JOYSTICK_5 : usize = 4;
pub const  GLFW_JOYSTICK_6 : usize = 5;
pub const  GLFW_JOYSTICK_7 : usize = 6;
pub const  GLFW_JOYSTICK_8 : usize = 7;
pub const  GLFW_JOYSTICK_9 : usize = 8;
pub const  GLFW_JOYSTICK_10 : usize = 9;
pub const  GLFW_JOYSTICK_11 : usize = 10;
pub const  GLFW_JOYSTICK_12 : usize = 11;
pub const  GLFW_JOYSTICK_13 : usize = 12;
pub const  GLFW_JOYSTICK_14 : usize = 13;
pub const  GLFW_JOYSTICK_15 : usize = 14;
pub const  GLFW_JOYSTICK_16 : usize = 15;
pub const  GLFW_JOYSTICK_LAST : usize = GLFW_JOYSTICK_16;

pub const  GLFW_NOT_INITIALIZED : usize = 0x00010001;

pub const  GLFW_NO_CURRENT_CONTEXT : usize = 0x00010002;

pub const  GLFW_INVALID_ENUM : usize = 0x00010003;

pub const  GLFW_INVALID_VALUE : usize = 0x00010004;

pub const  GLFW_OUT_OF_MEMORY : usize = 0x00010005;

pub const  GLFW_API_UNAVAILABLE : usize = 0x00010006;

pub const  GLFW_VERSION_UNAVAILABLE : usize = 0x00010007;

pub const  GLFW_PLATFORM_ERROR : usize = 0x00010008;

pub const  GLFW_FORMAT_UNAVAILABLE : usize = 0x00010009;
pub const  GLFW_NO_WINDOW_CONTEXT : usize = 0x0001000A;


pub const  GLFW_FOCUSED : usize = 0x00020001;
pub const  GLFW_ICONIFIED : usize = 0x00020002;
pub const  GLFW_RESIZABLE : usize = 0x00020003;
pub const  GLFW_VISIBLE : usize = 0x00020004;
pub const  GLFW_DECORATED : usize = 0x00020005;
pub const  GLFW_AUTO_ICONIFY : usize = 0x00020006;
pub const  GLFW_FLOATING : usize = 0x00020007;
pub const  GLFW_MAXIMIZED : usize = 0x00020008;

pub const  GLFW_RED_BITS : usize = 0x00021001;
pub const  GLFW_GREEN_BITS : usize = 0x00021002;
pub const  GLFW_BLUE_BITS : usize = 0x00021003;
pub const  GLFW_ALPHA_BITS : usize = 0x00021004;
pub const  GLFW_DEPTH_BITS : usize = 0x00021005;
pub const  GLFW_STENCIL_BITS : usize = 0x00021006;
pub const  GLFW_ACCUM_RED_BITS : usize = 0x00021007;
pub const  GLFW_ACCUM_GREEN_BITS : usize = 0x00021008;
pub const  GLFW_ACCUM_BLUE_BITS : usize = 0x00021009;
pub const  GLFW_ACCUM_ALPHA_BITS : usize = 0x0002100A;
pub const  GLFW_AUX_BUFFERS : usize = 0x0002100B;
pub const  GLFW_STEREO : usize = 0x0002100C;
pub const  GLFW_SAMPLES : usize = 0x0002100D;
pub const  GLFW_SRGB_CAPABLE : usize = 0x0002100E;
pub const  GLFW_REFRESH_RATE : usize = 0x0002100F;
pub const  GLFW_DOUBLEBUFFER : usize = 0x00021010;

pub const  GLFW_CLIENT_API : usize = 0x00022001;
pub const  GLFW_CONTEXT_VERSION_MAJOR : usize = 0x00022002;
pub const  GLFW_CONTEXT_VERSION_MINOR : usize = 0x00022003;
pub const  GLFW_CONTEXT_REVISION : usize = 0x00022004;
pub const  GLFW_CONTEXT_ROBUSTNESS : usize = 0x00022005;
pub const  GLFW_OPENGL_FORWARD_COMPAT : usize = 0x00022006;
pub const  GLFW_OPENGL_DEBUG_CONTEXT : usize = 0x00022007;
pub const  GLFW_OPENGL_PROFILE : usize = 0x00022008;
pub const  GLFW_CONTEXT_RELEASE_BEHAVIOR : usize = 0x00022009;
pub const  GLFW_CONTEXT_NO_ERROR : usize = 0x0002200A;
pub const  GLFW_CONTEXT_CREATION_API : usize = 0x0002200B;

pub const  GLFW_NO_API : usize = 0;
pub const  GLFW_OPENGL_API : usize = 0x00030001;
pub const  GLFW_OPENGL_ES_API : usize = 0x00030002;

pub const  GLFW_NO_ROBUSTNESS : usize = 0;
pub const  GLFW_NO_RESET_NOTIFICATION : usize = 0x00031001;
pub const  GLFW_LOSE_CONTEXT_ON_RESET : usize = 0x00031002;

pub const  GLFW_OPENGL_ANY_PROFILE : usize = 0;
pub const  GLFW_OPENGL_CORE_PROFILE : usize = 0x00032001;
pub const  GLFW_OPENGL_COMPAT_PROFILE : usize = 0x00032002;

pub const  GLFW_CURSOR : usize = 0x00033001;
pub const  GLFW_STICKY_KEYS : usize = 0x00033002;
pub const  GLFW_STICKY_MOUSE_BUTTONS : usize = 0x00033003;

pub const  GLFW_CURSOR_NORMAL : usize = 0x00034001;
pub const  GLFW_CURSOR_HIDDEN : usize = 0x00034002;
pub const  GLFW_CURSOR_DISABLED : usize = 0x00034003;

pub const  GLFW_ANY_RELEASE_BEHAVIOR : usize = 0;
pub const  GLFW_RELEASE_BEHAVIOR_FLUSH : usize = 0x00035001;
pub const  GLFW_RELEASE_BEHAVIOR_NONE : usize = 0x00035002;

pub const  GLFW_NATIVE_CONTEXT_API : usize = 0x00036001;
pub const  GLFW_EGL_CONTEXT_API : usize = 0x00036002;



pub const  GLFW_ARROW_CURSOR : usize = 0x00036001;

pub const  GLFW_IBEAM_CURSOR : usize = 0x00036002;

pub const  GLFW_CROSSHAIR_CURSOR : usize = 0x00036003;

pub const  GLFW_HAND_CURSOR : usize = 0x00036004;

pub const  GLFW_HRESIZE_CURSOR : usize = 0x00036005;

pub const  GLFW_VRESIZE_CURSOR : usize = 0x00036006;


pub const  GLFW_CONNECTED : usize = 0x00040001;
pub const  GLFW_DISCONNECTED : usize = 0x00040002;

pub const  GLFW_DONT_CARE : isize = -1;

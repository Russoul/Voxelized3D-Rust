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
use glad_gl::gl;
use std::os::raw::c_void;
use glad_gl::gl::GLbitfield;

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
    pub id: u32,
    
}

impl Program{
    pub fn get_uniform(&self, name: &str) -> i32{
        gl_get_uniform_location(self.id, name)
    }

    pub fn is_in_use(&self) -> bool {
        let mut cur_id = 0;
        gl_get_integerv(gl::GL_CURRENT_PROGRAM, &mut cur_id);
        self.id == cur_id as u32
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


    pub fn set_int(&self, name: &str, val: i32){
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

pub fn gl_generate_mipmap(target : u32){
    unsafe{
        gl::GenerateMipmap(target);
    }
}

pub fn gl_tex_image_2d<T>(target : u32, level : i32, internal_format : u32, w : u32, h : u32, border : i32, format : u32, ty : u32, data : &[T]){
    unsafe{
        gl::TexImage2D(target, level, internal_format as i32, w as i32, h as i32, border, format, ty, std::mem::transmute(data.as_ptr()));
    }
}

pub fn gl_tex_parameteri(ty : u32, par : u32, val : u32){
    unsafe{
        gl::TexParameteri(ty, par, val as i32);
    }
}

pub fn gl_gen_textures(count : u32, textures : &mut [u32]){
    unsafe{
        gl::GenTextures(count as i32, std::mem::transmute(textures.as_mut_ptr()));
    }
}

pub fn gl_bind_texture(ty : u32, tex : u32){
    unsafe{
        gl::BindTexture(ty, tex);
    }
}

pub fn gl_active_texture(tex : u32){
    unsafe{gl::ActiveTexture(tex)};
}

pub fn gl_enable(val : u32){
    unsafe{gl::Enable(val);}
}

pub fn gl_disable(val : u32){
    unsafe{gl::Disable(val);}
}

pub fn gl_delete_shader(shader: u32){
    unsafe{gl::DeleteShader(shader)}
}

pub fn gl_get_error() -> u32{
    unsafe{gl::GetError()}
}

pub fn gl_uniform_matrix4fv(loc: i32, transpose: bool, mat: &[f32]){
    unsafe{
        gl::UniformMatrix4fv(loc, 1, if transpose {gl::GL_TRUE}else{gl::GL_FALSE}, mat.as_ptr())
    }
}

pub fn gl_get_integerv(param: u32, out: *mut i32){
    unsafe{gl::GetIntegerv(param, out)}
}

pub fn gl_use_program(id: u32){
    unsafe{
        gl::UseProgram(id);
    }
}

pub fn gl_uniform1i(loc: i32, val: i32){
    unsafe{
        gl::Uniform1i(loc, val)
    }
}

pub fn gl_uniform1f(loc: i32, val: f32){
    unsafe{
        gl::Uniform1f(loc, val)
    }
}

pub fn gl_uniform2f(loc: i32, val1: f32, val2: f32){
    unsafe{
        gl::Uniform2f(loc, val1, val2)
    }
}

pub fn gl_uniform3f(loc: i32, val1: f32, val2: f32, val3: f32){
    unsafe{
        gl::Uniform3f(loc, val1, val2, val3)
    }
}

pub fn gl_uniform4f(loc: i32, val1: f32, val2: f32, val3: f32, val4: f32){
    unsafe{
        gl::Uniform4f(loc, val1, val2, val3, val4)
    }
}


pub fn gl_draw_elements(mode: u32, count: usize, typee: u32, indices: *const c_void){
    unsafe{gl::DrawElements(mode, count as i32, typee, indices)};
}


pub fn gl_gen_vertex_arrays() -> u32{
    unsafe{
        let mut ar: u32 = 0;
        gl::GenVertexArrays(1, &mut ar);

        ar
    }
}

pub fn gl_gen_buffers() -> u32{
    unsafe{
        let mut buf: u32 = 0;
        gl::GenBuffers(1, &mut buf);

        buf
    }
}

pub fn gl_delete_vertex_arrays(ar: u32){
    unsafe{
        gl::DeleteVertexArrays(1, &ar as *const u32);
    }
}

pub fn gl_delete_buffers(buf : u32){
    unsafe{
        gl::DeleteBuffers(1, &buf as *const u32)
    }
}

pub fn gl_bind_vertex_array(ar: u32){
    unsafe{
        gl::BindVertexArray(ar);
    }
}

pub fn gl_bind_buffer(typee:u32, buf: u32){
    unsafe{
        gl::BindBuffer(typee, buf);
    }
}

//be precise about 'T' type argument
pub fn gl_buffer_data<T>(target: u32, num : usize, data: &[T], usage: u32){
    unsafe{
        gl::BufferData(target, std::mem::size_of::<T>() as isize * num as isize, std::mem::transmute(data.as_ptr()), usage);
    }
}

pub fn gl_vertex_attrib_pointer(index: u32, size: u32, typee: u32, norm: bool, stride: u32, offset: u32){
    unsafe{gl::VertexAttribPointer(index, size as i32, typee, if norm {gl::GL_TRUE} else {gl::GL_FALSE}, stride as i32, offset as *const c_void)}
}

pub fn gl_enable_vertex_attrib_array(index: u32){
    unsafe{gl::EnableVertexAttribArray(index)}
}

pub fn gl_get_uniform_location(program: u32, name : &str)->i32{
    unsafe{gl::GetUniformLocation(program, CString::new(name).unwrap().as_ptr())}
}

pub fn gl_attach_shader(prog: u32, shader: u32){
    unsafe{gl::AttachShader(prog, shader)}
}

pub fn gl_link_program(prog: u32){
    unsafe{gl::LinkProgram(prog)}
}

pub fn gl_validate_program(prog: u32){
    unsafe{gl::ValidateProgram(prog)}
}

pub fn gl_get_shader_info_log(shader: u32) -> String{
    unsafe{
        let mut len: i32 = 0;
        gl::GetShaderiv(shader, gl::GL_INFO_LOG_LENGTH, &mut len);
        println!("info log len {}", len);
        let mut info : Vec<u8> = Vec::with_capacity(len as usize);
        let mut len_ret : i32 = 0;
        gl::GetShaderInfoLog(shader, len, &mut len_ret, std::mem::transmute(info.as_mut_ptr()));
       /* for i in info.iter(){
            println!("{}", i)
        }*/
        info.set_len(len_ret as usize);
        let string = String::from_utf8(info).unwrap();

        string
        
    }
}

pub fn gl_get_shaderiv(id: u32, param: u32, res: *mut i32){
    unsafe{gl::GetShaderiv(id, param, res)}
}

pub fn gl_compile_shader(id : u32){
    unsafe{gl::CompileShader(id)}
}

pub fn gl_shader_source(shader: u32, source: &str){
    unsafe{
        let s = source.len() as i32;
        gl::ShaderSource(shader, 1, &(source.as_ptr() as *const i8), &s as *const _);
    }
}

pub fn gl_create_shader(typee: u32)->u32{
    unsafe{gl::CreateShader(typee)}
}

pub fn gl_create_program()->u32{
    unsafe{gl::CreateProgram()}
}

pub fn gl_viewport(x : u32, y : u32, w : u32, h : u32){
    unsafe{
        gl::Viewport(x as i32, y as i32, w as i32, h as i32);
    }
}

pub fn gl_clear_color(r : f32, g : f32, b : f32, a : f32){
    unsafe{
        gl::ClearColor(r,g,b,a);
    }
}

pub fn gl_clear(val : u32){
    unsafe{
        gl::Clear(val);
    }
}

pub fn gl_get_string<'a>(val : u32) -> & 'a str{
    unsafe{
        CStr::from_ptr(gl::GetString(val) as *const i8).to_str().unwrap()
    }
}

pub fn gl_dispatch_compute(num_groups_x : u32, num_groups_y : u32, num_groups_z : u32){
    unsafe{
        gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
    }
}

pub fn gl_memory_barrier(bit : GLbitfield){
    unsafe{
        gl::MemoryBarrier(bit);
    }
}

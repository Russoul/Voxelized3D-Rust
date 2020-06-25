use std::ffi::CString;
use ansi_term::Colour::Red;
use ansi_term::Colour::Blue;
use graphics::*;
use glad_gl::gl;

pub fn create_compute_program(compute_source : &str) -> u32{
    let prog = gl_create_program();
    let comp_id = gl_create_shader(gl::GL_COMPUTE_SHADER);

    gl_shader_source(comp_id, compute_source);

    let mut status: i32 = 0;

    gl_compile_shader(comp_id);
    gl_get_shaderiv(comp_id, gl::GL_COMPILE_STATUS, &mut status);
    if status == 0 {
        eprint!("Failed to compile compute shader !
Source:
-------------------------------------
{}
-------------------------------------
Error:
{}
        ", Blue.paint(compute_source), Red.paint(gl_get_shader_info_log(comp_id)));
        panic!();
    }





    gl_attach_shader(prog, comp_id);
    gl_link_program(prog);
    gl_validate_program(prog);

    gl_delete_shader(comp_id);

    prog
}

pub fn create_vert_frag_program(vert_source: &str, frag_source: &str) -> u32{
    let prog = gl_create_program();
    let vert_id = gl_create_shader(gl::GL_VERTEX_SHADER);
    let frag_id = gl_create_shader(gl::GL_FRAGMENT_SHADER);

    gl_shader_source(vert_id, vert_source);
    gl_shader_source(frag_id, frag_source);

    let mut status: i32 = 0;

    gl_compile_shader(vert_id);
    gl_get_shaderiv(vert_id, gl::GL_COMPILE_STATUS, &mut status);
    if status == 0 {
        eprint!("Failed to compile vertex shader !
Source:
-------------------------------------
{}
-------------------------------------
Error:
{}
        ", Blue.paint(vert_source), Red.paint(gl_get_shader_info_log(vert_id)));
        panic!();
    }



    gl_compile_shader(frag_id);
    gl_get_shaderiv(frag_id, gl::GL_COMPILE_STATUS, &mut status);
    if status == 0 {
         eprint!("Failed to compile fragment shader !
Source:
-------------------------------------
{}
-------------------------------------
Error:
{}
        ", Blue.paint(frag_source), Red.paint(gl_get_shader_info_log(frag_id)));
        panic!();
    }

    gl_attach_shader(prog, vert_id);
    gl_attach_shader(prog, frag_id);
    gl_link_program(prog);
    gl_validate_program(prog);

    gl_delete_shader(vert_id);
    gl_delete_shader(frag_id);

    prog
}

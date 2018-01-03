use libc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;
use graphics::*;
use ansi_term::Colour::Red;
use ansi_term::Colour::Blue;

//vf for vertex&fragment
//exits program on any error occured during shader proccessing
pub fn create_program_vf(vert_source: &str, frag_source: &str) -> usize{
    let prog = gl_create_program();
    let vert_id = gl_create_shader(GL_VERTEX_SHADER);
    let frag_id = gl_create_shader(GL_FRAGMENT_SHADER);

    gl_shader_source(vert_id, vert_source);
    gl_shader_source(frag_id, frag_source);

    let mut status: usize = 0;

    gl_compile_shader(vert_id);
    gl_get_shaderiv(vert_id, GL_COMPILE_STATUS, &mut status);
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
    gl_get_shaderiv(frag_id, GL_COMPILE_STATUS, &mut status);
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


pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw;

pub struct FnPtr {
    ptr: *const raw::c_void,
    is_loaded: bool
}

impl FnPtr {
    pub fn empty() -> FnPtr {
        FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false }
    }

    pub fn load<F>(&mut self, loadfn: &mut F, name: &'static str) where F: FnMut(&'static str) -> *const raw::c_void {
        let loaded = loadfn(name);
        if !loaded.is_null() {
            self.ptr = loaded;
            self.is_loaded = true;
        } else {
            self.ptr = FnPtr::not_initialized as *const raw::c_void;
            self.is_loaded = false;
        };
    }

    pub fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            self.ptr = other.ptr;
            self.is_loaded = other.is_loaded;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! { panic!("gl: function not initialized") }
}

pub mod types {
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use std::os::raw;

pub type GLvoid = raw::c_void;

pub type GLbyte = raw::c_char;
pub type GLubyte = raw::c_uchar;
pub type GLchar = raw::c_char;
pub type GLboolean = raw::c_uchar;

pub type GLshort = raw::c_short;
pub type GLushort = raw::c_ushort;

pub type GLint = raw::c_int;
pub type GLuint = raw::c_uint;
pub type GLint64 = i64;
pub type GLuint64 = u64;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub type GLsizei = GLint;
pub type GLclampx = raw::c_int;
pub type GLfixed = GLint;
pub type GLhalf = raw::c_ushort;
pub type GLhalfNV = raw::c_ushort;
pub type GLhalfARB = raw::c_ushort;

pub type GLenum = raw::c_uint;
pub type GLbitfield = raw::c_uint;

pub type GLfloat = raw::c_float;
pub type GLdouble = raw::c_double;
pub type GLclampf = raw::c_float;
pub type GLclampd = raw::c_double;

pub type GLcharARB = raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = raw::c_uint;

pub enum __GLsync {}

pub type GLsync = *const __GLsync;

pub enum _cl_context {}

pub enum _cl_event {}

pub type GLvdpauSurfaceNV = GLintptr;
pub type GLeglClientBufferEXT = *const raw::c_void;
pub type GLeglImageOES = *const raw::c_void;


pub type GLDEBUGPROC = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLDEBUGPROCAMD = extern "system" fn (
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLVULKANPROCNV = extern "system" fn ();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std;
    use super::types::*;

    pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x92D9;
    pub const GL_ACTIVE_ATTRIBUTES: std::os::raw::c_uint = 0x8B89;
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: std::os::raw::c_uint = 0x8B8A;
    pub const GL_ACTIVE_PROGRAM: std::os::raw::c_uint = 0x8259;
    pub const GL_ACTIVE_RESOURCES: std::os::raw::c_uint = 0x92F5;
    pub const GL_ACTIVE_SUBROUTINES: std::os::raw::c_uint = 0x8DE5;
    pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: std::os::raw::c_uint = 0x8E48;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: std::os::raw::c_uint = 0x8DE6;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: std::os::raw::c_uint = 0x8E47;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: std::os::raw::c_uint = 0x8E49;
    pub const GL_ACTIVE_TEXTURE: std::os::raw::c_uint = 0x84E0;
    pub const GL_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8B86;
    pub const GL_ACTIVE_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A36;
    pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: std::os::raw::c_uint = 0x8A35;
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: std::os::raw::c_uint = 0x8B87;
    pub const GL_ACTIVE_VARIABLES: std::os::raw::c_uint = 0x9305;
    pub const GL_ALIASED_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x846E;
    pub const GL_ALL_BARRIER_BITS: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_ALL_SHADER_BITS: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_ALPHA: std::os::raw::c_uint = 0x1906;
    pub const GL_ALREADY_SIGNALED: std::os::raw::c_uint = 0x911A;
    pub const GL_ALWAYS: std::os::raw::c_uint = 0x0207;
    pub const GL_AND: std::os::raw::c_uint = 0x1501;
    pub const GL_AND_INVERTED: std::os::raw::c_uint = 0x1504;
    pub const GL_AND_REVERSE: std::os::raw::c_uint = 0x1502;
    pub const GL_ANY_SAMPLES_PASSED: std::os::raw::c_uint = 0x8C2F;
    pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: std::os::raw::c_uint = 0x8D6A;
    pub const GL_ARRAY_BUFFER: std::os::raw::c_uint = 0x8892;
    pub const GL_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8894;
    pub const GL_ARRAY_SIZE: std::os::raw::c_uint = 0x92FB;
    pub const GL_ARRAY_STRIDE: std::os::raw::c_uint = 0x92FE;
    pub const GL_ATOMIC_COUNTER_BARRIER_BIT: std::os::raw::c_uint = 0x00001000;
    pub const GL_ATOMIC_COUNTER_BUFFER: std::os::raw::c_uint = 0x92C0;
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x92C5;
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: std::os::raw::c_uint = 0x92C6;
    pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: std::os::raw::c_uint = 0x92C1;
    pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: std::os::raw::c_uint = 0x92C4;
    pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: std::os::raw::c_uint = 0x9301;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: std::os::raw::c_uint = 0x90ED;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: std::os::raw::c_uint = 0x92CB;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: std::os::raw::c_uint = 0x92CA;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: std::os::raw::c_uint = 0x92C8;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: std::os::raw::c_uint = 0x92C9;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: std::os::raw::c_uint = 0x92C7;
    pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: std::os::raw::c_uint = 0x92C3;
    pub const GL_ATOMIC_COUNTER_BUFFER_START: std::os::raw::c_uint = 0x92C2;
    pub const GL_ATTACHED_SHADERS: std::os::raw::c_uint = 0x8B85;
    pub const GL_AUTO_GENERATE_MIPMAP: std::os::raw::c_uint = 0x8295;
    pub const GL_BACK: std::os::raw::c_uint = 0x0405;
    pub const GL_BACK_LEFT: std::os::raw::c_uint = 0x0402;
    pub const GL_BACK_RIGHT: std::os::raw::c_uint = 0x0403;
    pub const GL_BGR: std::os::raw::c_uint = 0x80E0;
    pub const GL_BGRA: std::os::raw::c_uint = 0x80E1;
    pub const GL_BGRA_INTEGER: std::os::raw::c_uint = 0x8D9B;
    pub const GL_BGR_INTEGER: std::os::raw::c_uint = 0x8D9A;
    pub const GL_BLEND: std::os::raw::c_uint = 0x0BE2;
    pub const GL_BLEND_COLOR: std::os::raw::c_uint = 0x8005;
    pub const GL_BLEND_DST: std::os::raw::c_uint = 0x0BE0;
    pub const GL_BLEND_DST_ALPHA: std::os::raw::c_uint = 0x80CA;
    pub const GL_BLEND_DST_RGB: std::os::raw::c_uint = 0x80C8;
    pub const GL_BLEND_EQUATION: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_EQUATION_ALPHA: std::os::raw::c_uint = 0x883D;
    pub const GL_BLEND_EQUATION_RGB: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_SRC: std::os::raw::c_uint = 0x0BE1;
    pub const GL_BLEND_SRC_ALPHA: std::os::raw::c_uint = 0x80CB;
    pub const GL_BLEND_SRC_RGB: std::os::raw::c_uint = 0x80C9;
    pub const GL_BLOCK_INDEX: std::os::raw::c_uint = 0x92FD;
    pub const GL_BLUE: std::os::raw::c_uint = 0x1905;
    pub const GL_BLUE_INTEGER: std::os::raw::c_uint = 0x8D96;
    pub const GL_BOOL: std::os::raw::c_uint = 0x8B56;
    pub const GL_BOOL_VEC2: std::os::raw::c_uint = 0x8B57;
    pub const GL_BOOL_VEC3: std::os::raw::c_uint = 0x8B58;
    pub const GL_BOOL_VEC4: std::os::raw::c_uint = 0x8B59;
    pub const GL_BUFFER: std::os::raw::c_uint = 0x82E0;
    pub const GL_BUFFER_ACCESS: std::os::raw::c_uint = 0x88BB;
    pub const GL_BUFFER_ACCESS_FLAGS: std::os::raw::c_uint = 0x911F;
    pub const GL_BUFFER_BINDING: std::os::raw::c_uint = 0x9302;
    pub const GL_BUFFER_DATA_SIZE: std::os::raw::c_uint = 0x9303;
    pub const GL_BUFFER_IMMUTABLE_STORAGE: std::os::raw::c_uint = 0x821F;
    pub const GL_BUFFER_MAPPED: std::os::raw::c_uint = 0x88BC;
    pub const GL_BUFFER_MAP_LENGTH: std::os::raw::c_uint = 0x9120;
    pub const GL_BUFFER_MAP_OFFSET: std::os::raw::c_uint = 0x9121;
    pub const GL_BUFFER_MAP_POINTER: std::os::raw::c_uint = 0x88BD;
    pub const GL_BUFFER_SIZE: std::os::raw::c_uint = 0x8764;
    pub const GL_BUFFER_STORAGE_FLAGS: std::os::raw::c_uint = 0x8220;
    pub const GL_BUFFER_UPDATE_BARRIER_BIT: std::os::raw::c_uint = 0x00000200;
    pub const GL_BUFFER_USAGE: std::os::raw::c_uint = 0x8765;
    pub const GL_BUFFER_VARIABLE: std::os::raw::c_uint = 0x92E5;
    pub const GL_BYTE: std::os::raw::c_uint = 0x1400;
    pub const GL_CAVEAT_SUPPORT: std::os::raw::c_uint = 0x82B8;
    pub const GL_CCW: std::os::raw::c_uint = 0x0901;
    pub const GL_CLAMP_READ_COLOR: std::os::raw::c_uint = 0x891C;
    pub const GL_CLAMP_TO_BORDER: std::os::raw::c_uint = 0x812D;
    pub const GL_CLAMP_TO_EDGE: std::os::raw::c_uint = 0x812F;
    pub const GL_CLEAR: std::os::raw::c_uint = 0x1500;
    pub const GL_CLEAR_BUFFER: std::os::raw::c_uint = 0x82B4;
    pub const GL_CLEAR_TEXTURE: std::os::raw::c_uint = 0x9365;
    pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: std::os::raw::c_uint = 0x00004000;
    pub const GL_CLIENT_STORAGE_BIT: std::os::raw::c_uint = 0x0200;
    pub const GL_CLIP_DEPTH_MODE: std::os::raw::c_uint = 0x935D;
    pub const GL_CLIP_DISTANCE0: std::os::raw::c_uint = 0x3000;
    pub const GL_CLIP_DISTANCE1: std::os::raw::c_uint = 0x3001;
    pub const GL_CLIP_DISTANCE2: std::os::raw::c_uint = 0x3002;
    pub const GL_CLIP_DISTANCE3: std::os::raw::c_uint = 0x3003;
    pub const GL_CLIP_DISTANCE4: std::os::raw::c_uint = 0x3004;
    pub const GL_CLIP_DISTANCE5: std::os::raw::c_uint = 0x3005;
    pub const GL_CLIP_DISTANCE6: std::os::raw::c_uint = 0x3006;
    pub const GL_CLIP_DISTANCE7: std::os::raw::c_uint = 0x3007;
    pub const GL_CLIP_ORIGIN: std::os::raw::c_uint = 0x935C;
    pub const GL_COLOR: std::os::raw::c_uint = 0x1800;
    pub const GL_COLOR_ATTACHMENT0: std::os::raw::c_uint = 0x8CE0;
    pub const GL_COLOR_ATTACHMENT1: std::os::raw::c_uint = 0x8CE1;
    pub const GL_COLOR_ATTACHMENT10: std::os::raw::c_uint = 0x8CEA;
    pub const GL_COLOR_ATTACHMENT11: std::os::raw::c_uint = 0x8CEB;
    pub const GL_COLOR_ATTACHMENT12: std::os::raw::c_uint = 0x8CEC;
    pub const GL_COLOR_ATTACHMENT13: std::os::raw::c_uint = 0x8CED;
    pub const GL_COLOR_ATTACHMENT14: std::os::raw::c_uint = 0x8CEE;
    pub const GL_COLOR_ATTACHMENT15: std::os::raw::c_uint = 0x8CEF;
    pub const GL_COLOR_ATTACHMENT16: std::os::raw::c_uint = 0x8CF0;
    pub const GL_COLOR_ATTACHMENT17: std::os::raw::c_uint = 0x8CF1;
    pub const GL_COLOR_ATTACHMENT18: std::os::raw::c_uint = 0x8CF2;
    pub const GL_COLOR_ATTACHMENT19: std::os::raw::c_uint = 0x8CF3;
    pub const GL_COLOR_ATTACHMENT2: std::os::raw::c_uint = 0x8CE2;
    pub const GL_COLOR_ATTACHMENT20: std::os::raw::c_uint = 0x8CF4;
    pub const GL_COLOR_ATTACHMENT21: std::os::raw::c_uint = 0x8CF5;
    pub const GL_COLOR_ATTACHMENT22: std::os::raw::c_uint = 0x8CF6;
    pub const GL_COLOR_ATTACHMENT23: std::os::raw::c_uint = 0x8CF7;
    pub const GL_COLOR_ATTACHMENT24: std::os::raw::c_uint = 0x8CF8;
    pub const GL_COLOR_ATTACHMENT25: std::os::raw::c_uint = 0x8CF9;
    pub const GL_COLOR_ATTACHMENT26: std::os::raw::c_uint = 0x8CFA;
    pub const GL_COLOR_ATTACHMENT27: std::os::raw::c_uint = 0x8CFB;
    pub const GL_COLOR_ATTACHMENT28: std::os::raw::c_uint = 0x8CFC;
    pub const GL_COLOR_ATTACHMENT29: std::os::raw::c_uint = 0x8CFD;
    pub const GL_COLOR_ATTACHMENT3: std::os::raw::c_uint = 0x8CE3;
    pub const GL_COLOR_ATTACHMENT30: std::os::raw::c_uint = 0x8CFE;
    pub const GL_COLOR_ATTACHMENT31: std::os::raw::c_uint = 0x8CFF;
    pub const GL_COLOR_ATTACHMENT4: std::os::raw::c_uint = 0x8CE4;
    pub const GL_COLOR_ATTACHMENT5: std::os::raw::c_uint = 0x8CE5;
    pub const GL_COLOR_ATTACHMENT6: std::os::raw::c_uint = 0x8CE6;
    pub const GL_COLOR_ATTACHMENT7: std::os::raw::c_uint = 0x8CE7;
    pub const GL_COLOR_ATTACHMENT8: std::os::raw::c_uint = 0x8CE8;
    pub const GL_COLOR_ATTACHMENT9: std::os::raw::c_uint = 0x8CE9;
    pub const GL_COLOR_BUFFER_BIT: std::os::raw::c_uint = 0x00004000;
    pub const GL_COLOR_CLEAR_VALUE: std::os::raw::c_uint = 0x0C22;
    pub const GL_COLOR_COMPONENTS: std::os::raw::c_uint = 0x8283;
    pub const GL_COLOR_ENCODING: std::os::raw::c_uint = 0x8296;
    pub const GL_COLOR_LOGIC_OP: std::os::raw::c_uint = 0x0BF2;
    pub const GL_COLOR_RENDERABLE: std::os::raw::c_uint = 0x8286;
    pub const GL_COLOR_WRITEMASK: std::os::raw::c_uint = 0x0C23;
    pub const GL_COMMAND_BARRIER_BIT: std::os::raw::c_uint = 0x00000040;
    pub const GL_COMPARE_REF_TO_TEXTURE: std::os::raw::c_uint = 0x884E;
    pub const GL_COMPATIBLE_SUBROUTINES: std::os::raw::c_uint = 0x8E4B;
    pub const GL_COMPILE_STATUS: std::os::raw::c_uint = 0x8B81;
    pub const GL_COMPRESSED_R11_EAC: std::os::raw::c_uint = 0x9270;
    pub const GL_COMPRESSED_RED: std::os::raw::c_uint = 0x8225;
    pub const GL_COMPRESSED_RED_RGTC1: std::os::raw::c_uint = 0x8DBB;
    pub const GL_COMPRESSED_RG: std::os::raw::c_uint = 0x8226;
    pub const GL_COMPRESSED_RG11_EAC: std::os::raw::c_uint = 0x9272;
    pub const GL_COMPRESSED_RGB: std::os::raw::c_uint = 0x84ED;
    pub const GL_COMPRESSED_RGB8_ETC2: std::os::raw::c_uint = 0x9274;
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: std::os::raw::c_uint = 0x9276;
    pub const GL_COMPRESSED_RGBA: std::os::raw::c_uint = 0x84EE;
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC: std::os::raw::c_uint = 0x9278;
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM: std::os::raw::c_uint = 0x8E8C;
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: std::os::raw::c_uint = 0x8E8E;
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: std::os::raw::c_uint = 0x8E8F;
    pub const GL_COMPRESSED_RG_RGTC2: std::os::raw::c_uint = 0x8DBD;
    pub const GL_COMPRESSED_SIGNED_R11_EAC: std::os::raw::c_uint = 0x9271;
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1: std::os::raw::c_uint = 0x8DBC;
    pub const GL_COMPRESSED_SIGNED_RG11_EAC: std::os::raw::c_uint = 0x9273;
    pub const GL_COMPRESSED_SIGNED_RG_RGTC2: std::os::raw::c_uint = 0x8DBE;
    pub const GL_COMPRESSED_SRGB: std::os::raw::c_uint = 0x8C48;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: std::os::raw::c_uint = 0x9279;
    pub const GL_COMPRESSED_SRGB8_ETC2: std::os::raw::c_uint = 0x9275;
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: std::os::raw::c_uint = 0x9277;
    pub const GL_COMPRESSED_SRGB_ALPHA: std::os::raw::c_uint = 0x8C49;
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: std::os::raw::c_uint = 0x8E8D;
    pub const GL_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A3;
    pub const GL_COMPUTE_SHADER: std::os::raw::c_uint = 0x91B9;
    pub const GL_COMPUTE_SHADER_BIT: std::os::raw::c_uint = 0x00000020;
    pub const GL_COMPUTE_SUBROUTINE: std::os::raw::c_uint = 0x92ED;
    pub const GL_COMPUTE_SUBROUTINE_UNIFORM: std::os::raw::c_uint = 0x92F3;
    pub const GL_COMPUTE_TEXTURE: std::os::raw::c_uint = 0x82A0;
    pub const GL_COMPUTE_WORK_GROUP_SIZE: std::os::raw::c_uint = 0x8267;
    pub const GL_CONDITION_SATISFIED: std::os::raw::c_uint = 0x911C;
    pub const GL_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8003;
    pub const GL_CONSTANT_COLOR: std::os::raw::c_uint = 0x8001;
    pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_CONTEXT_CORE_PROFILE_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_CONTEXT_FLAGS: std::os::raw::c_uint = 0x821E;
    pub const GL_CONTEXT_FLAG_DEBUG_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: std::os::raw::c_uint = 0x00000004;
    pub const GL_CONTEXT_LOST: std::os::raw::c_uint = 0x0507;
    pub const GL_CONTEXT_PROFILE_MASK: std::os::raw::c_uint = 0x9126;
    pub const GL_CONTEXT_RELEASE_BEHAVIOR: std::os::raw::c_uint = 0x82FB;
    pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: std::os::raw::c_uint = 0x82FC;
    pub const GL_COPY: std::os::raw::c_uint = 0x1503;
    pub const GL_COPY_INVERTED: std::os::raw::c_uint = 0x150C;
    pub const GL_COPY_READ_BUFFER: std::os::raw::c_uint = 0x8F36;
    pub const GL_COPY_READ_BUFFER_BINDING: std::os::raw::c_uint = 0x8F36;
    pub const GL_COPY_WRITE_BUFFER: std::os::raw::c_uint = 0x8F37;
    pub const GL_COPY_WRITE_BUFFER_BINDING: std::os::raw::c_uint = 0x8F37;
    pub const GL_CULL_FACE: std::os::raw::c_uint = 0x0B44;
    pub const GL_CULL_FACE_MODE: std::os::raw::c_uint = 0x0B45;
    pub const GL_CURRENT_PROGRAM: std::os::raw::c_uint = 0x8B8D;
    pub const GL_CURRENT_QUERY: std::os::raw::c_uint = 0x8865;
    pub const GL_CURRENT_VERTEX_ATTRIB: std::os::raw::c_uint = 0x8626;
    pub const GL_CW: std::os::raw::c_uint = 0x0900;
    pub const GL_DEBUG_CALLBACK_FUNCTION: std::os::raw::c_uint = 0x8244;
    pub const GL_DEBUG_CALLBACK_USER_PARAM: std::os::raw::c_uint = 0x8245;
    pub const GL_DEBUG_GROUP_STACK_DEPTH: std::os::raw::c_uint = 0x826D;
    pub const GL_DEBUG_LOGGED_MESSAGES: std::os::raw::c_uint = 0x9145;
    pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: std::os::raw::c_uint = 0x8243;
    pub const GL_DEBUG_OUTPUT: std::os::raw::c_uint = 0x92E0;
    pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: std::os::raw::c_uint = 0x8242;
    pub const GL_DEBUG_SEVERITY_HIGH: std::os::raw::c_uint = 0x9146;
    pub const GL_DEBUG_SEVERITY_LOW: std::os::raw::c_uint = 0x9148;
    pub const GL_DEBUG_SEVERITY_MEDIUM: std::os::raw::c_uint = 0x9147;
    pub const GL_DEBUG_SEVERITY_NOTIFICATION: std::os::raw::c_uint = 0x826B;
    pub const GL_DEBUG_SOURCE_API: std::os::raw::c_uint = 0x8246;
    pub const GL_DEBUG_SOURCE_APPLICATION: std::os::raw::c_uint = 0x824A;
    pub const GL_DEBUG_SOURCE_OTHER: std::os::raw::c_uint = 0x824B;
    pub const GL_DEBUG_SOURCE_SHADER_COMPILER: std::os::raw::c_uint = 0x8248;
    pub const GL_DEBUG_SOURCE_THIRD_PARTY: std::os::raw::c_uint = 0x8249;
    pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: std::os::raw::c_uint = 0x8247;
    pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: std::os::raw::c_uint = 0x824D;
    pub const GL_DEBUG_TYPE_ERROR: std::os::raw::c_uint = 0x824C;
    pub const GL_DEBUG_TYPE_MARKER: std::os::raw::c_uint = 0x8268;
    pub const GL_DEBUG_TYPE_OTHER: std::os::raw::c_uint = 0x8251;
    pub const GL_DEBUG_TYPE_PERFORMANCE: std::os::raw::c_uint = 0x8250;
    pub const GL_DEBUG_TYPE_POP_GROUP: std::os::raw::c_uint = 0x826A;
    pub const GL_DEBUG_TYPE_PORTABILITY: std::os::raw::c_uint = 0x824F;
    pub const GL_DEBUG_TYPE_PUSH_GROUP: std::os::raw::c_uint = 0x8269;
    pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: std::os::raw::c_uint = 0x824E;
    pub const GL_DECR: std::os::raw::c_uint = 0x1E03;
    pub const GL_DECR_WRAP: std::os::raw::c_uint = 0x8508;
    pub const GL_DELETE_STATUS: std::os::raw::c_uint = 0x8B80;
    pub const GL_DEPTH: std::os::raw::c_uint = 0x1801;
    pub const GL_DEPTH24_STENCIL8: std::os::raw::c_uint = 0x88F0;
    pub const GL_DEPTH32F_STENCIL8: std::os::raw::c_uint = 0x8CAD;
    pub const GL_DEPTH_ATTACHMENT: std::os::raw::c_uint = 0x8D00;
    pub const GL_DEPTH_BUFFER_BIT: std::os::raw::c_uint = 0x00000100;
    pub const GL_DEPTH_CLAMP: std::os::raw::c_uint = 0x864F;
    pub const GL_DEPTH_CLEAR_VALUE: std::os::raw::c_uint = 0x0B73;
    pub const GL_DEPTH_COMPONENT: std::os::raw::c_uint = 0x1902;
    pub const GL_DEPTH_COMPONENT16: std::os::raw::c_uint = 0x81A5;
    pub const GL_DEPTH_COMPONENT24: std::os::raw::c_uint = 0x81A6;
    pub const GL_DEPTH_COMPONENT32: std::os::raw::c_uint = 0x81A7;
    pub const GL_DEPTH_COMPONENT32F: std::os::raw::c_uint = 0x8CAC;
    pub const GL_DEPTH_COMPONENTS: std::os::raw::c_uint = 0x8284;
    pub const GL_DEPTH_FUNC: std::os::raw::c_uint = 0x0B74;
    pub const GL_DEPTH_RANGE: std::os::raw::c_uint = 0x0B70;
    pub const GL_DEPTH_RENDERABLE: std::os::raw::c_uint = 0x8287;
    pub const GL_DEPTH_STENCIL: std::os::raw::c_uint = 0x84F9;
    pub const GL_DEPTH_STENCIL_ATTACHMENT: std::os::raw::c_uint = 0x821A;
    pub const GL_DEPTH_STENCIL_TEXTURE_MODE: std::os::raw::c_uint = 0x90EA;
    pub const GL_DEPTH_TEST: std::os::raw::c_uint = 0x0B71;
    pub const GL_DEPTH_WRITEMASK: std::os::raw::c_uint = 0x0B72;
    pub const GL_DISPATCH_INDIRECT_BUFFER: std::os::raw::c_uint = 0x90EE;
    pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: std::os::raw::c_uint = 0x90EF;
    pub const GL_DITHER: std::os::raw::c_uint = 0x0BD0;
    pub const GL_DONT_CARE: std::os::raw::c_uint = 0x1100;
    pub const GL_DOUBLE: std::os::raw::c_uint = 0x140A;
    pub const GL_DOUBLEBUFFER: std::os::raw::c_uint = 0x0C32;
    pub const GL_DOUBLE_MAT2: std::os::raw::c_uint = 0x8F46;
    pub const GL_DOUBLE_MAT2x3: std::os::raw::c_uint = 0x8F49;
    pub const GL_DOUBLE_MAT2x4: std::os::raw::c_uint = 0x8F4A;
    pub const GL_DOUBLE_MAT3: std::os::raw::c_uint = 0x8F47;
    pub const GL_DOUBLE_MAT3x2: std::os::raw::c_uint = 0x8F4B;
    pub const GL_DOUBLE_MAT3x4: std::os::raw::c_uint = 0x8F4C;
    pub const GL_DOUBLE_MAT4: std::os::raw::c_uint = 0x8F48;
    pub const GL_DOUBLE_MAT4x2: std::os::raw::c_uint = 0x8F4D;
    pub const GL_DOUBLE_MAT4x3: std::os::raw::c_uint = 0x8F4E;
    pub const GL_DOUBLE_VEC2: std::os::raw::c_uint = 0x8FFC;
    pub const GL_DOUBLE_VEC3: std::os::raw::c_uint = 0x8FFD;
    pub const GL_DOUBLE_VEC4: std::os::raw::c_uint = 0x8FFE;
    pub const GL_DRAW_BUFFER: std::os::raw::c_uint = 0x0C01;
    pub const GL_DRAW_BUFFER0: std::os::raw::c_uint = 0x8825;
    pub const GL_DRAW_BUFFER1: std::os::raw::c_uint = 0x8826;
    pub const GL_DRAW_BUFFER10: std::os::raw::c_uint = 0x882F;
    pub const GL_DRAW_BUFFER11: std::os::raw::c_uint = 0x8830;
    pub const GL_DRAW_BUFFER12: std::os::raw::c_uint = 0x8831;
    pub const GL_DRAW_BUFFER13: std::os::raw::c_uint = 0x8832;
    pub const GL_DRAW_BUFFER14: std::os::raw::c_uint = 0x8833;
    pub const GL_DRAW_BUFFER15: std::os::raw::c_uint = 0x8834;
    pub const GL_DRAW_BUFFER2: std::os::raw::c_uint = 0x8827;
    pub const GL_DRAW_BUFFER3: std::os::raw::c_uint = 0x8828;
    pub const GL_DRAW_BUFFER4: std::os::raw::c_uint = 0x8829;
    pub const GL_DRAW_BUFFER5: std::os::raw::c_uint = 0x882A;
    pub const GL_DRAW_BUFFER6: std::os::raw::c_uint = 0x882B;
    pub const GL_DRAW_BUFFER7: std::os::raw::c_uint = 0x882C;
    pub const GL_DRAW_BUFFER8: std::os::raw::c_uint = 0x882D;
    pub const GL_DRAW_BUFFER9: std::os::raw::c_uint = 0x882E;
    pub const GL_DRAW_FRAMEBUFFER: std::os::raw::c_uint = 0x8CA9;
    pub const GL_DRAW_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CA6;
    pub const GL_DRAW_INDIRECT_BUFFER: std::os::raw::c_uint = 0x8F3F;
    pub const GL_DRAW_INDIRECT_BUFFER_BINDING: std::os::raw::c_uint = 0x8F43;
    pub const GL_DST_ALPHA: std::os::raw::c_uint = 0x0304;
    pub const GL_DST_COLOR: std::os::raw::c_uint = 0x0306;
    pub const GL_DYNAMIC_COPY: std::os::raw::c_uint = 0x88EA;
    pub const GL_DYNAMIC_DRAW: std::os::raw::c_uint = 0x88E8;
    pub const GL_DYNAMIC_READ: std::os::raw::c_uint = 0x88E9;
    pub const GL_DYNAMIC_STORAGE_BIT: std::os::raw::c_uint = 0x0100;
    pub const GL_ELEMENT_ARRAY_BARRIER_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_ELEMENT_ARRAY_BUFFER: std::os::raw::c_uint = 0x8893;
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8895;
    pub const GL_EQUAL: std::os::raw::c_uint = 0x0202;
    pub const GL_EQUIV: std::os::raw::c_uint = 0x1509;
    pub const GL_EXTENSIONS: std::os::raw::c_uint = 0x1F03;
    pub const GL_FALSE: std::os::raw::c_uchar = 0;
    pub const GL_FASTEST: std::os::raw::c_uint = 0x1101;
    pub const GL_FILL: std::os::raw::c_uint = 0x1B02;
    pub const GL_FILTER: std::os::raw::c_uint = 0x829A;
    pub const GL_FIRST_VERTEX_CONVENTION: std::os::raw::c_uint = 0x8E4D;
    pub const GL_FIXED: std::os::raw::c_uint = 0x140C;
    pub const GL_FIXED_ONLY: std::os::raw::c_uint = 0x891D;
    pub const GL_FLOAT: std::os::raw::c_uint = 0x1406;
    pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: std::os::raw::c_uint = 0x8DAD;
    pub const GL_FLOAT_MAT2: std::os::raw::c_uint = 0x8B5A;
    pub const GL_FLOAT_MAT2x3: std::os::raw::c_uint = 0x8B65;
    pub const GL_FLOAT_MAT2x4: std::os::raw::c_uint = 0x8B66;
    pub const GL_FLOAT_MAT3: std::os::raw::c_uint = 0x8B5B;
    pub const GL_FLOAT_MAT3x2: std::os::raw::c_uint = 0x8B67;
    pub const GL_FLOAT_MAT3x4: std::os::raw::c_uint = 0x8B68;
    pub const GL_FLOAT_MAT4: std::os::raw::c_uint = 0x8B5C;
    pub const GL_FLOAT_MAT4x2: std::os::raw::c_uint = 0x8B69;
    pub const GL_FLOAT_MAT4x3: std::os::raw::c_uint = 0x8B6A;
    pub const GL_FLOAT_VEC2: std::os::raw::c_uint = 0x8B50;
    pub const GL_FLOAT_VEC3: std::os::raw::c_uint = 0x8B51;
    pub const GL_FLOAT_VEC4: std::os::raw::c_uint = 0x8B52;
    pub const GL_FRACTIONAL_EVEN: std::os::raw::c_uint = 0x8E7C;
    pub const GL_FRACTIONAL_ODD: std::os::raw::c_uint = 0x8E7B;
    pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: std::os::raw::c_uint = 0x8E5D;
    pub const GL_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8B30;
    pub const GL_FRAGMENT_SHADER_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: std::os::raw::c_uint = 0x8B8B;
    pub const GL_FRAGMENT_SUBROUTINE: std::os::raw::c_uint = 0x92EC;
    pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: std::os::raw::c_uint = 0x92F2;
    pub const GL_FRAGMENT_TEXTURE: std::os::raw::c_uint = 0x829F;
    pub const GL_FRAMEBUFFER: std::os::raw::c_uint = 0x8D40;
    pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: std::os::raw::c_uint = 0x8215;
    pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: std::os::raw::c_uint = 0x8214;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: std::os::raw::c_uint = 0x8210;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: std::os::raw::c_uint = 0x8211;
    pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: std::os::raw::c_uint = 0x8216;
    pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: std::os::raw::c_uint = 0x8213;
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: std::os::raw::c_uint = 0x8DA7;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: std::os::raw::c_uint = 0x8CD1;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: std::os::raw::c_uint = 0x8CD0;
    pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: std::os::raw::c_uint = 0x8212;
    pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: std::os::raw::c_uint = 0x8217;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: std::os::raw::c_uint = 0x8CD3;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: std::os::raw::c_uint = 0x8CD4;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: std::os::raw::c_uint = 0x8CD2;
    pub const GL_FRAMEBUFFER_BARRIER_BIT: std::os::raw::c_uint = 0x00000400;
    pub const GL_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CA6;
    pub const GL_FRAMEBUFFER_BLEND: std::os::raw::c_uint = 0x828B;
    pub const GL_FRAMEBUFFER_COMPLETE: std::os::raw::c_uint = 0x8CD5;
    pub const GL_FRAMEBUFFER_DEFAULT: std::os::raw::c_uint = 0x8218;
    pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: std::os::raw::c_uint = 0x9314;
    pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: std::os::raw::c_uint = 0x9311;
    pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: std::os::raw::c_uint = 0x9312;
    pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: std::os::raw::c_uint = 0x9313;
    pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: std::os::raw::c_uint = 0x9310;
    pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: std::os::raw::c_uint = 0x8CD6;
    pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: std::os::raw::c_uint = 0x8CDB;
    pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: std::os::raw::c_uint = 0x8DA8;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: std::os::raw::c_uint = 0x8CD7;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: std::os::raw::c_uint = 0x8D56;
    pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: std::os::raw::c_uint = 0x8CDC;
    pub const GL_FRAMEBUFFER_RENDERABLE: std::os::raw::c_uint = 0x8289;
    pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: std::os::raw::c_uint = 0x828A;
    pub const GL_FRAMEBUFFER_SRGB: std::os::raw::c_uint = 0x8DB9;
    pub const GL_FRAMEBUFFER_UNDEFINED: std::os::raw::c_uint = 0x8219;
    pub const GL_FRAMEBUFFER_UNSUPPORTED: std::os::raw::c_uint = 0x8CDD;
    pub const GL_FRONT: std::os::raw::c_uint = 0x0404;
    pub const GL_FRONT_AND_BACK: std::os::raw::c_uint = 0x0408;
    pub const GL_FRONT_FACE: std::os::raw::c_uint = 0x0B46;
    pub const GL_FRONT_LEFT: std::os::raw::c_uint = 0x0400;
    pub const GL_FRONT_RIGHT: std::os::raw::c_uint = 0x0401;
    pub const GL_FULL_SUPPORT: std::os::raw::c_uint = 0x82B7;
    pub const GL_FUNC_ADD: std::os::raw::c_uint = 0x8006;
    pub const GL_FUNC_REVERSE_SUBTRACT: std::os::raw::c_uint = 0x800B;
    pub const GL_FUNC_SUBTRACT: std::os::raw::c_uint = 0x800A;
    pub const GL_GEOMETRY_INPUT_TYPE: std::os::raw::c_uint = 0x8917;
    pub const GL_GEOMETRY_OUTPUT_TYPE: std::os::raw::c_uint = 0x8918;
    pub const GL_GEOMETRY_SHADER: std::os::raw::c_uint = 0x8DD9;
    pub const GL_GEOMETRY_SHADER_BIT: std::os::raw::c_uint = 0x00000004;
    pub const GL_GEOMETRY_SHADER_INVOCATIONS: std::os::raw::c_uint = 0x887F;
    pub const GL_GEOMETRY_SUBROUTINE: std::os::raw::c_uint = 0x92EB;
    pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: std::os::raw::c_uint = 0x92F1;
    pub const GL_GEOMETRY_TEXTURE: std::os::raw::c_uint = 0x829E;
    pub const GL_GEOMETRY_VERTICES_OUT: std::os::raw::c_uint = 0x8916;
    pub const GL_GEQUAL: std::os::raw::c_uint = 0x0206;
    pub const GL_GET_TEXTURE_IMAGE_FORMAT: std::os::raw::c_uint = 0x8291;
    pub const GL_GET_TEXTURE_IMAGE_TYPE: std::os::raw::c_uint = 0x8292;
    pub const GL_GREATER: std::os::raw::c_uint = 0x0204;
    pub const GL_GREEN: std::os::raw::c_uint = 0x1904;
    pub const GL_GREEN_INTEGER: std::os::raw::c_uint = 0x8D95;
    pub const GL_GUILTY_CONTEXT_RESET: std::os::raw::c_uint = 0x8253;
    pub const GL_HALF_FLOAT: std::os::raw::c_uint = 0x140B;
    pub const GL_HIGH_FLOAT: std::os::raw::c_uint = 0x8DF2;
    pub const GL_HIGH_INT: std::os::raw::c_uint = 0x8DF5;
    pub const GL_IMAGE_1D: std::os::raw::c_uint = 0x904C;
    pub const GL_IMAGE_1D_ARRAY: std::os::raw::c_uint = 0x9052;
    pub const GL_IMAGE_2D: std::os::raw::c_uint = 0x904D;
    pub const GL_IMAGE_2D_ARRAY: std::os::raw::c_uint = 0x9053;
    pub const GL_IMAGE_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9055;
    pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9056;
    pub const GL_IMAGE_2D_RECT: std::os::raw::c_uint = 0x904F;
    pub const GL_IMAGE_3D: std::os::raw::c_uint = 0x904E;
    pub const GL_IMAGE_BINDING_ACCESS: std::os::raw::c_uint = 0x8F3E;
    pub const GL_IMAGE_BINDING_FORMAT: std::os::raw::c_uint = 0x906E;
    pub const GL_IMAGE_BINDING_LAYER: std::os::raw::c_uint = 0x8F3D;
    pub const GL_IMAGE_BINDING_LAYERED: std::os::raw::c_uint = 0x8F3C;
    pub const GL_IMAGE_BINDING_LEVEL: std::os::raw::c_uint = 0x8F3B;
    pub const GL_IMAGE_BINDING_NAME: std::os::raw::c_uint = 0x8F3A;
    pub const GL_IMAGE_BUFFER: std::os::raw::c_uint = 0x9051;
    pub const GL_IMAGE_CLASS_10_10_10_2: std::os::raw::c_uint = 0x82C3;
    pub const GL_IMAGE_CLASS_11_11_10: std::os::raw::c_uint = 0x82C2;
    pub const GL_IMAGE_CLASS_1_X_16: std::os::raw::c_uint = 0x82BE;
    pub const GL_IMAGE_CLASS_1_X_32: std::os::raw::c_uint = 0x82BB;
    pub const GL_IMAGE_CLASS_1_X_8: std::os::raw::c_uint = 0x82C1;
    pub const GL_IMAGE_CLASS_2_X_16: std::os::raw::c_uint = 0x82BD;
    pub const GL_IMAGE_CLASS_2_X_32: std::os::raw::c_uint = 0x82BA;
    pub const GL_IMAGE_CLASS_2_X_8: std::os::raw::c_uint = 0x82C0;
    pub const GL_IMAGE_CLASS_4_X_16: std::os::raw::c_uint = 0x82BC;
    pub const GL_IMAGE_CLASS_4_X_32: std::os::raw::c_uint = 0x82B9;
    pub const GL_IMAGE_CLASS_4_X_8: std::os::raw::c_uint = 0x82BF;
    pub const GL_IMAGE_COMPATIBILITY_CLASS: std::os::raw::c_uint = 0x82A8;
    pub const GL_IMAGE_CUBE: std::os::raw::c_uint = 0x9050;
    pub const GL_IMAGE_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x9054;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: std::os::raw::c_uint = 0x90C9;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: std::os::raw::c_uint = 0x90C8;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: std::os::raw::c_uint = 0x90C7;
    pub const GL_IMAGE_PIXEL_FORMAT: std::os::raw::c_uint = 0x82A9;
    pub const GL_IMAGE_PIXEL_TYPE: std::os::raw::c_uint = 0x82AA;
    pub const GL_IMAGE_TEXEL_SIZE: std::os::raw::c_uint = 0x82A7;
    pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: std::os::raw::c_uint = 0x8B9B;
    pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: std::os::raw::c_uint = 0x8B9A;
    pub const GL_INCR: std::os::raw::c_uint = 0x1E02;
    pub const GL_INCR_WRAP: std::os::raw::c_uint = 0x8507;
    pub const GL_INFO_LOG_LENGTH: std::os::raw::c_uint = 0x8B84;
    pub const GL_INNOCENT_CONTEXT_RESET: std::os::raw::c_uint = 0x8254;
    pub const GL_INT: std::os::raw::c_uint = 0x1404;
    pub const GL_INTERLEAVED_ATTRIBS: std::os::raw::c_uint = 0x8C8C;
    pub const GL_INTERNALFORMAT_ALPHA_SIZE: std::os::raw::c_uint = 0x8274;
    pub const GL_INTERNALFORMAT_ALPHA_TYPE: std::os::raw::c_uint = 0x827B;
    pub const GL_INTERNALFORMAT_BLUE_SIZE: std::os::raw::c_uint = 0x8273;
    pub const GL_INTERNALFORMAT_BLUE_TYPE: std::os::raw::c_uint = 0x827A;
    pub const GL_INTERNALFORMAT_DEPTH_SIZE: std::os::raw::c_uint = 0x8275;
    pub const GL_INTERNALFORMAT_DEPTH_TYPE: std::os::raw::c_uint = 0x827C;
    pub const GL_INTERNALFORMAT_GREEN_SIZE: std::os::raw::c_uint = 0x8272;
    pub const GL_INTERNALFORMAT_GREEN_TYPE: std::os::raw::c_uint = 0x8279;
    pub const GL_INTERNALFORMAT_PREFERRED: std::os::raw::c_uint = 0x8270;
    pub const GL_INTERNALFORMAT_RED_SIZE: std::os::raw::c_uint = 0x8271;
    pub const GL_INTERNALFORMAT_RED_TYPE: std::os::raw::c_uint = 0x8278;
    pub const GL_INTERNALFORMAT_SHARED_SIZE: std::os::raw::c_uint = 0x8277;
    pub const GL_INTERNALFORMAT_STENCIL_SIZE: std::os::raw::c_uint = 0x8276;
    pub const GL_INTERNALFORMAT_STENCIL_TYPE: std::os::raw::c_uint = 0x827D;
    pub const GL_INTERNALFORMAT_SUPPORTED: std::os::raw::c_uint = 0x826F;
    pub const GL_INT_2_10_10_10_REV: std::os::raw::c_uint = 0x8D9F;
    pub const GL_INT_IMAGE_1D: std::os::raw::c_uint = 0x9057;
    pub const GL_INT_IMAGE_1D_ARRAY: std::os::raw::c_uint = 0x905D;
    pub const GL_INT_IMAGE_2D: std::os::raw::c_uint = 0x9058;
    pub const GL_INT_IMAGE_2D_ARRAY: std::os::raw::c_uint = 0x905E;
    pub const GL_INT_IMAGE_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9060;
    pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9061;
    pub const GL_INT_IMAGE_2D_RECT: std::os::raw::c_uint = 0x905A;
    pub const GL_INT_IMAGE_3D: std::os::raw::c_uint = 0x9059;
    pub const GL_INT_IMAGE_BUFFER: std::os::raw::c_uint = 0x905C;
    pub const GL_INT_IMAGE_CUBE: std::os::raw::c_uint = 0x905B;
    pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x905F;
    pub const GL_INT_SAMPLER_1D: std::os::raw::c_uint = 0x8DC9;
    pub const GL_INT_SAMPLER_1D_ARRAY: std::os::raw::c_uint = 0x8DCE;
    pub const GL_INT_SAMPLER_2D: std::os::raw::c_uint = 0x8DCA;
    pub const GL_INT_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DCF;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9109;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x910C;
    pub const GL_INT_SAMPLER_2D_RECT: std::os::raw::c_uint = 0x8DCD;
    pub const GL_INT_SAMPLER_3D: std::os::raw::c_uint = 0x8DCB;
    pub const GL_INT_SAMPLER_BUFFER: std::os::raw::c_uint = 0x8DD0;
    pub const GL_INT_SAMPLER_CUBE: std::os::raw::c_uint = 0x8DCC;
    pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x900E;
    pub const GL_INT_VEC2: std::os::raw::c_uint = 0x8B53;
    pub const GL_INT_VEC3: std::os::raw::c_uint = 0x8B54;
    pub const GL_INT_VEC4: std::os::raw::c_uint = 0x8B55;
    pub const GL_INVALID_ENUM: std::os::raw::c_uint = 0x0500;
    pub const GL_INVALID_FRAMEBUFFER_OPERATION: std::os::raw::c_uint = 0x0506;
    pub const GL_INVALID_INDEX: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_INVALID_OPERATION: std::os::raw::c_uint = 0x0502;
    pub const GL_INVALID_VALUE: std::os::raw::c_uint = 0x0501;
    pub const GL_INVERT: std::os::raw::c_uint = 0x150A;
    pub const GL_ISOLINES: std::os::raw::c_uint = 0x8E7A;
    pub const GL_IS_PER_PATCH: std::os::raw::c_uint = 0x92E7;
    pub const GL_IS_ROW_MAJOR: std::os::raw::c_uint = 0x9300;
    pub const GL_KEEP: std::os::raw::c_uint = 0x1E00;
    pub const GL_LAST_VERTEX_CONVENTION: std::os::raw::c_uint = 0x8E4E;
    pub const GL_LAYER_PROVOKING_VERTEX: std::os::raw::c_uint = 0x825E;
    pub const GL_LEFT: std::os::raw::c_uint = 0x0406;
    pub const GL_LEQUAL: std::os::raw::c_uint = 0x0203;
    pub const GL_LESS: std::os::raw::c_uint = 0x0201;
    pub const GL_LINE: std::os::raw::c_uint = 0x1B01;
    pub const GL_LINEAR: std::os::raw::c_uint = 0x2601;
    pub const GL_LINEAR_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2703;
    pub const GL_LINEAR_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2701;
    pub const GL_LINES: std::os::raw::c_uint = 0x0001;
    pub const GL_LINES_ADJACENCY: std::os::raw::c_uint = 0x000A;
    pub const GL_LINE_LOOP: std::os::raw::c_uint = 0x0002;
    pub const GL_LINE_SMOOTH: std::os::raw::c_uint = 0x0B20;
    pub const GL_LINE_SMOOTH_HINT: std::os::raw::c_uint = 0x0C52;
    pub const GL_LINE_STRIP: std::os::raw::c_uint = 0x0003;
    pub const GL_LINE_STRIP_ADJACENCY: std::os::raw::c_uint = 0x000B;
    pub const GL_LINE_WIDTH: std::os::raw::c_uint = 0x0B21;
    pub const GL_LINE_WIDTH_GRANULARITY: std::os::raw::c_uint = 0x0B23;
    pub const GL_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x0B22;
    pub const GL_LINK_STATUS: std::os::raw::c_uint = 0x8B82;
    pub const GL_LOCATION: std::os::raw::c_uint = 0x930E;
    pub const GL_LOCATION_COMPONENT: std::os::raw::c_uint = 0x934A;
    pub const GL_LOCATION_INDEX: std::os::raw::c_uint = 0x930F;
    pub const GL_LOGIC_OP_MODE: std::os::raw::c_uint = 0x0BF0;
    pub const GL_LOSE_CONTEXT_ON_RESET: std::os::raw::c_uint = 0x8252;
    pub const GL_LOWER_LEFT: std::os::raw::c_uint = 0x8CA1;
    pub const GL_LOW_FLOAT: std::os::raw::c_uint = 0x8DF0;
    pub const GL_LOW_INT: std::os::raw::c_uint = 0x8DF3;
    pub const GL_MAJOR_VERSION: std::os::raw::c_uint = 0x821B;
    pub const GL_MANUAL_GENERATE_MIPMAP: std::os::raw::c_uint = 0x8294;
    pub const GL_MAP_COHERENT_BIT: std::os::raw::c_uint = 0x0080;
    pub const GL_MAP_FLUSH_EXPLICIT_BIT: std::os::raw::c_uint = 0x0010;
    pub const GL_MAP_INVALIDATE_BUFFER_BIT: std::os::raw::c_uint = 0x0008;
    pub const GL_MAP_INVALIDATE_RANGE_BIT: std::os::raw::c_uint = 0x0004;
    pub const GL_MAP_PERSISTENT_BIT: std::os::raw::c_uint = 0x0040;
    pub const GL_MAP_READ_BIT: std::os::raw::c_uint = 0x0001;
    pub const GL_MAP_UNSYNCHRONIZED_BIT: std::os::raw::c_uint = 0x0020;
    pub const GL_MAP_WRITE_BIT: std::os::raw::c_uint = 0x0002;
    pub const GL_MATRIX_STRIDE: std::os::raw::c_uint = 0x92FF;
    pub const GL_MAX: std::os::raw::c_uint = 0x8008;
    pub const GL_MAX_3D_TEXTURE_SIZE: std::os::raw::c_uint = 0x8073;
    pub const GL_MAX_ARRAY_TEXTURE_LAYERS: std::os::raw::c_uint = 0x88FF;
    pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: std::os::raw::c_uint = 0x92DC;
    pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: std::os::raw::c_uint = 0x92D8;
    pub const GL_MAX_CLIP_DISTANCES: std::os::raw::c_uint = 0x0D32;
    pub const GL_MAX_COLOR_ATTACHMENTS: std::os::raw::c_uint = 0x8CDF;
    pub const GL_MAX_COLOR_TEXTURE_SAMPLES: std::os::raw::c_uint = 0x910E;
    pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x92D7;
    pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x92D1;
    pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES: std::os::raw::c_uint = 0x82FA;
    pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8266;
    pub const GL_MAX_COMBINED_DIMENSIONS: std::os::raw::c_uint = 0x8282;
    pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A33;
    pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A32;
    pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: std::os::raw::c_uint = 0x90CF;
    pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: std::os::raw::c_uint = 0x8F39;
    pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: std::os::raw::c_uint = 0x8F39;
    pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: std::os::raw::c_uint = 0x90DC;
    pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8E1E;
    pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8E1F;
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4D;
    pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2E;
    pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A31;
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x8265;
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x8264;
    pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: std::os::raw::c_uint = 0x91BD;
    pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: std::os::raw::c_uint = 0x90DB;
    pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: std::os::raw::c_uint = 0x8262;
    pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x91BC;
    pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x91BB;
    pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8263;
    pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: std::os::raw::c_uint = 0x91BE;
    pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: std::os::raw::c_uint = 0x90EB;
    pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: std::os::raw::c_uint = 0x91BF;
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: std::os::raw::c_uint = 0x851C;
    pub const GL_MAX_CULL_DISTANCES: std::os::raw::c_uint = 0x82F9;
    pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: std::os::raw::c_uint = 0x826C;
    pub const GL_MAX_DEBUG_LOGGED_MESSAGES: std::os::raw::c_uint = 0x9144;
    pub const GL_MAX_DEBUG_MESSAGE_LENGTH: std::os::raw::c_uint = 0x9143;
    pub const GL_MAX_DEPTH: std::os::raw::c_uint = 0x8280;
    pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: std::os::raw::c_uint = 0x910F;
    pub const GL_MAX_DRAW_BUFFERS: std::os::raw::c_uint = 0x8824;
    pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: std::os::raw::c_uint = 0x88FC;
    pub const GL_MAX_ELEMENTS_INDICES: std::os::raw::c_uint = 0x80E9;
    pub const GL_MAX_ELEMENTS_VERTICES: std::os::raw::c_uint = 0x80E8;
    pub const GL_MAX_ELEMENT_INDEX: std::os::raw::c_uint = 0x8D6B;
    pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x92D6;
    pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x92D0;
    pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: std::os::raw::c_uint = 0x90CE;
    pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: std::os::raw::c_uint = 0x9125;
    pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: std::os::raw::c_uint = 0x8E5C;
    pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: std::os::raw::c_uint = 0x90DA;
    pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2D;
    pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B49;
    pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: std::os::raw::c_uint = 0x8DFD;
    pub const GL_MAX_FRAMEBUFFER_HEIGHT: std::os::raw::c_uint = 0x9316;
    pub const GL_MAX_FRAMEBUFFER_LAYERS: std::os::raw::c_uint = 0x9317;
    pub const GL_MAX_FRAMEBUFFER_SAMPLES: std::os::raw::c_uint = 0x9318;
    pub const GL_MAX_FRAMEBUFFER_WIDTH: std::os::raw::c_uint = 0x9315;
    pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x92D5;
    pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x92CF;
    pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: std::os::raw::c_uint = 0x90CD;
    pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: std::os::raw::c_uint = 0x9123;
    pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x9124;
    pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: std::os::raw::c_uint = 0x8DE0;
    pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: std::os::raw::c_uint = 0x8E5A;
    pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: std::os::raw::c_uint = 0x90D7;
    pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8C29;
    pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x8DE1;
    pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2C;
    pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8DDF;
    pub const GL_MAX_HEIGHT: std::os::raw::c_uint = 0x827F;
    pub const GL_MAX_IMAGE_SAMPLES: std::os::raw::c_uint = 0x906D;
    pub const GL_MAX_IMAGE_UNITS: std::os::raw::c_uint = 0x8F38;
    pub const GL_MAX_INTEGER_SAMPLES: std::os::raw::c_uint = 0x9110;
    pub const GL_MAX_LABEL_LENGTH: std::os::raw::c_uint = 0x82E8;
    pub const GL_MAX_LAYERS: std::os::raw::c_uint = 0x8281;
    pub const GL_MAX_NAME_LENGTH: std::os::raw::c_uint = 0x92F6;
    pub const GL_MAX_NUM_ACTIVE_VARIABLES: std::os::raw::c_uint = 0x92F7;
    pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: std::os::raw::c_uint = 0x92F8;
    pub const GL_MAX_PATCH_VERTICES: std::os::raw::c_uint = 0x8E7D;
    pub const GL_MAX_PROGRAM_TEXEL_OFFSET: std::os::raw::c_uint = 0x8905;
    pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: std::os::raw::c_uint = 0x8E5F;
    pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: std::os::raw::c_uint = 0x84F8;
    pub const GL_MAX_RENDERBUFFER_SIZE: std::os::raw::c_uint = 0x84E8;
    pub const GL_MAX_SAMPLES: std::os::raw::c_uint = 0x8D57;
    pub const GL_MAX_SAMPLE_MASK_WORDS: std::os::raw::c_uint = 0x8E59;
    pub const GL_MAX_SERVER_WAIT_TIMEOUT: std::os::raw::c_uint = 0x9111;
    pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: std::os::raw::c_uint = 0x90DE;
    pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: std::os::raw::c_uint = 0x90DD;
    pub const GL_MAX_SUBROUTINES: std::os::raw::c_uint = 0x8DE7;
    pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: std::os::raw::c_uint = 0x8DE8;
    pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x92D3;
    pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x92CD;
    pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: std::os::raw::c_uint = 0x90CB;
    pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: std::os::raw::c_uint = 0x886C;
    pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x8E83;
    pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: std::os::raw::c_uint = 0x90D8;
    pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8E81;
    pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x8E85;
    pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8E89;
    pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8E7F;
    pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x92D4;
    pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x92CE;
    pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: std::os::raw::c_uint = 0x90CC;
    pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: std::os::raw::c_uint = 0x886D;
    pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x8E86;
    pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: std::os::raw::c_uint = 0x90D9;
    pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8E82;
    pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8E8A;
    pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8E80;
    pub const GL_MAX_TESS_GEN_LEVEL: std::os::raw::c_uint = 0x8E7E;
    pub const GL_MAX_TESS_PATCH_COMPONENTS: std::os::raw::c_uint = 0x8E84;
    pub const GL_MAX_TEXTURE_BUFFER_SIZE: std::os::raw::c_uint = 0x8C2B;
    pub const GL_MAX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8872;
    pub const GL_MAX_TEXTURE_LOD_BIAS: std::os::raw::c_uint = 0x84FD;
    pub const GL_MAX_TEXTURE_SIZE: std::os::raw::c_uint = 0x0D33;
    pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS: std::os::raw::c_uint = 0x8E70;
    pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: std::os::raw::c_uint = 0x8C8A;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: std::os::raw::c_uint = 0x8C8B;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: std::os::raw::c_uint = 0x8C80;
    pub const GL_MAX_UNIFORM_BLOCK_SIZE: std::os::raw::c_uint = 0x8A30;
    pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: std::os::raw::c_uint = 0x8A2F;
    pub const GL_MAX_UNIFORM_LOCATIONS: std::os::raw::c_uint = 0x826E;
    pub const GL_MAX_VARYING_COMPONENTS: std::os::raw::c_uint = 0x8B4B;
    pub const GL_MAX_VARYING_FLOATS: std::os::raw::c_uint = 0x8B4B;
    pub const GL_MAX_VARYING_VECTORS: std::os::raw::c_uint = 0x8DFC;
    pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: std::os::raw::c_uint = 0x92D2;
    pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: std::os::raw::c_uint = 0x92CC;
    pub const GL_MAX_VERTEX_ATTRIBS: std::os::raw::c_uint = 0x8869;
    pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: std::os::raw::c_uint = 0x82DA;
    pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: std::os::raw::c_uint = 0x82D9;
    pub const GL_MAX_VERTEX_ATTRIB_STRIDE: std::os::raw::c_uint = 0x82E5;
    pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: std::os::raw::c_uint = 0x90CA;
    pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x9122;
    pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: std::os::raw::c_uint = 0x90D6;
    pub const GL_MAX_VERTEX_STREAMS: std::os::raw::c_uint = 0x8E71;
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4C;
    pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2B;
    pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B4A;
    pub const GL_MAX_VERTEX_UNIFORM_VECTORS: std::os::raw::c_uint = 0x8DFB;
    pub const GL_MAX_VIEWPORTS: std::os::raw::c_uint = 0x825B;
    pub const GL_MAX_VIEWPORT_DIMS: std::os::raw::c_uint = 0x0D3A;
    pub const GL_MAX_WIDTH: std::os::raw::c_uint = 0x827E;
    pub const GL_MEDIUM_FLOAT: std::os::raw::c_uint = 0x8DF1;
    pub const GL_MEDIUM_INT: std::os::raw::c_uint = 0x8DF4;
    pub const GL_MIN: std::os::raw::c_uint = 0x8007;
    pub const GL_MINOR_VERSION: std::os::raw::c_uint = 0x821C;
    pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: std::os::raw::c_uint = 0x8E5B;
    pub const GL_MIN_MAP_BUFFER_ALIGNMENT: std::os::raw::c_uint = 0x90BC;
    pub const GL_MIN_PROGRAM_TEXEL_OFFSET: std::os::raw::c_uint = 0x8904;
    pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: std::os::raw::c_uint = 0x8E5E;
    pub const GL_MIN_SAMPLE_SHADING_VALUE: std::os::raw::c_uint = 0x8C37;
    pub const GL_MIPMAP: std::os::raw::c_uint = 0x8293;
    pub const GL_MIRRORED_REPEAT: std::os::raw::c_uint = 0x8370;
    pub const GL_MIRROR_CLAMP_TO_EDGE: std::os::raw::c_uint = 0x8743;
    pub const GL_MULTISAMPLE: std::os::raw::c_uint = 0x809D;
    pub const GL_NAME_LENGTH: std::os::raw::c_uint = 0x92F9;
    pub const GL_NAND: std::os::raw::c_uint = 0x150E;
    pub const GL_NEAREST: std::os::raw::c_uint = 0x2600;
    pub const GL_NEAREST_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2702;
    pub const GL_NEAREST_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2700;
    pub const GL_NEGATIVE_ONE_TO_ONE: std::os::raw::c_uint = 0x935E;
    pub const GL_NEVER: std::os::raw::c_uint = 0x0200;
    pub const GL_NICEST: std::os::raw::c_uint = 0x1102;
    pub const GL_NONE: std::os::raw::c_uint = 0;
    pub const GL_NOOP: std::os::raw::c_uint = 0x1505;
    pub const GL_NOR: std::os::raw::c_uint = 0x1508;
    pub const GL_NOTEQUAL: std::os::raw::c_uint = 0x0205;
    pub const GL_NO_ERROR: std::os::raw::c_uint = 0;
    pub const GL_NO_RESET_NOTIFICATION: std::os::raw::c_uint = 0x8261;
    pub const GL_NUM_ACTIVE_VARIABLES: std::os::raw::c_uint = 0x9304;
    pub const GL_NUM_COMPATIBLE_SUBROUTINES: std::os::raw::c_uint = 0x8E4A;
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A2;
    pub const GL_NUM_EXTENSIONS: std::os::raw::c_uint = 0x821D;
    pub const GL_NUM_PROGRAM_BINARY_FORMATS: std::os::raw::c_uint = 0x87FE;
    pub const GL_NUM_SAMPLE_COUNTS: std::os::raw::c_uint = 0x9380;
    pub const GL_NUM_SHADER_BINARY_FORMATS: std::os::raw::c_uint = 0x8DF9;
    pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: std::os::raw::c_uint = 0x82E9;
    pub const GL_OBJECT_TYPE: std::os::raw::c_uint = 0x9112;
    pub const GL_OFFSET: std::os::raw::c_uint = 0x92FC;
    pub const GL_ONE: std::os::raw::c_uint = 1;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8004;
    pub const GL_ONE_MINUS_CONSTANT_COLOR: std::os::raw::c_uint = 0x8002;
    pub const GL_ONE_MINUS_DST_ALPHA: std::os::raw::c_uint = 0x0305;
    pub const GL_ONE_MINUS_DST_COLOR: std::os::raw::c_uint = 0x0307;
    pub const GL_ONE_MINUS_SRC1_ALPHA: std::os::raw::c_uint = 0x88FB;
    pub const GL_ONE_MINUS_SRC1_COLOR: std::os::raw::c_uint = 0x88FA;
    pub const GL_ONE_MINUS_SRC_ALPHA: std::os::raw::c_uint = 0x0303;
    pub const GL_ONE_MINUS_SRC_COLOR: std::os::raw::c_uint = 0x0301;
    pub const GL_OR: std::os::raw::c_uint = 0x1507;
    pub const GL_OR_INVERTED: std::os::raw::c_uint = 0x150D;
    pub const GL_OR_REVERSE: std::os::raw::c_uint = 0x150B;
    pub const GL_OUT_OF_MEMORY: std::os::raw::c_uint = 0x0505;
    pub const GL_PACK_ALIGNMENT: std::os::raw::c_uint = 0x0D05;
    pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: std::os::raw::c_uint = 0x912D;
    pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: std::os::raw::c_uint = 0x912C;
    pub const GL_PACK_COMPRESSED_BLOCK_SIZE: std::os::raw::c_uint = 0x912E;
    pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: std::os::raw::c_uint = 0x912B;
    pub const GL_PACK_IMAGE_HEIGHT: std::os::raw::c_uint = 0x806C;
    pub const GL_PACK_LSB_FIRST: std::os::raw::c_uint = 0x0D01;
    pub const GL_PACK_ROW_LENGTH: std::os::raw::c_uint = 0x0D02;
    pub const GL_PACK_SKIP_IMAGES: std::os::raw::c_uint = 0x806B;
    pub const GL_PACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0D04;
    pub const GL_PACK_SKIP_ROWS: std::os::raw::c_uint = 0x0D03;
    pub const GL_PACK_SWAP_BYTES: std::os::raw::c_uint = 0x0D00;
    pub const GL_PATCHES: std::os::raw::c_uint = 0x000E;
    pub const GL_PATCH_DEFAULT_INNER_LEVEL: std::os::raw::c_uint = 0x8E73;
    pub const GL_PATCH_DEFAULT_OUTER_LEVEL: std::os::raw::c_uint = 0x8E74;
    pub const GL_PATCH_VERTICES: std::os::raw::c_uint = 0x8E72;
    pub const GL_PIXEL_BUFFER_BARRIER_BIT: std::os::raw::c_uint = 0x00000080;
    pub const GL_PIXEL_PACK_BUFFER: std::os::raw::c_uint = 0x88EB;
    pub const GL_PIXEL_PACK_BUFFER_BINDING: std::os::raw::c_uint = 0x88ED;
    pub const GL_PIXEL_UNPACK_BUFFER: std::os::raw::c_uint = 0x88EC;
    pub const GL_PIXEL_UNPACK_BUFFER_BINDING: std::os::raw::c_uint = 0x88EF;
    pub const GL_POINT: std::os::raw::c_uint = 0x1B00;
    pub const GL_POINTS: std::os::raw::c_uint = 0x0000;
    pub const GL_POINT_FADE_THRESHOLD_SIZE: std::os::raw::c_uint = 0x8128;
    pub const GL_POINT_SIZE: std::os::raw::c_uint = 0x0B11;
    pub const GL_POINT_SIZE_GRANULARITY: std::os::raw::c_uint = 0x0B13;
    pub const GL_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x0B12;
    pub const GL_POINT_SPRITE_COORD_ORIGIN: std::os::raw::c_uint = 0x8CA0;
    pub const GL_POLYGON_MODE: std::os::raw::c_uint = 0x0B40;
    pub const GL_POLYGON_OFFSET_FACTOR: std::os::raw::c_uint = 0x8038;
    pub const GL_POLYGON_OFFSET_FILL: std::os::raw::c_uint = 0x8037;
    pub const GL_POLYGON_OFFSET_LINE: std::os::raw::c_uint = 0x2A02;
    pub const GL_POLYGON_OFFSET_POINT: std::os::raw::c_uint = 0x2A01;
    pub const GL_POLYGON_OFFSET_UNITS: std::os::raw::c_uint = 0x2A00;
    pub const GL_POLYGON_SMOOTH: std::os::raw::c_uint = 0x0B41;
    pub const GL_POLYGON_SMOOTH_HINT: std::os::raw::c_uint = 0x0C53;
    pub const GL_PRIMITIVES_GENERATED: std::os::raw::c_uint = 0x8C87;
    pub const GL_PRIMITIVE_RESTART: std::os::raw::c_uint = 0x8F9D;
    pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: std::os::raw::c_uint = 0x8D69;
    pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: std::os::raw::c_uint = 0x8221;
    pub const GL_PRIMITIVE_RESTART_INDEX: std::os::raw::c_uint = 0x8F9E;
    pub const GL_PROGRAM: std::os::raw::c_uint = 0x82E2;
    pub const GL_PROGRAM_BINARY_FORMATS: std::os::raw::c_uint = 0x87FF;
    pub const GL_PROGRAM_BINARY_LENGTH: std::os::raw::c_uint = 0x8741;
    pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: std::os::raw::c_uint = 0x8257;
    pub const GL_PROGRAM_INPUT: std::os::raw::c_uint = 0x92E3;
    pub const GL_PROGRAM_OUTPUT: std::os::raw::c_uint = 0x92E4;
    pub const GL_PROGRAM_PIPELINE: std::os::raw::c_uint = 0x82E4;
    pub const GL_PROGRAM_PIPELINE_BINDING: std::os::raw::c_uint = 0x825A;
    pub const GL_PROGRAM_POINT_SIZE: std::os::raw::c_uint = 0x8642;
    pub const GL_PROGRAM_SEPARABLE: std::os::raw::c_uint = 0x8258;
    pub const GL_PROVOKING_VERTEX: std::os::raw::c_uint = 0x8E4F;
    pub const GL_PROXY_TEXTURE_1D: std::os::raw::c_uint = 0x8063;
    pub const GL_PROXY_TEXTURE_1D_ARRAY: std::os::raw::c_uint = 0x8C19;
    pub const GL_PROXY_TEXTURE_2D: std::os::raw::c_uint = 0x8064;
    pub const GL_PROXY_TEXTURE_2D_ARRAY: std::os::raw::c_uint = 0x8C1B;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9101;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9103;
    pub const GL_PROXY_TEXTURE_3D: std::os::raw::c_uint = 0x8070;
    pub const GL_PROXY_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x851B;
    pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x900B;
    pub const GL_PROXY_TEXTURE_RECTANGLE: std::os::raw::c_uint = 0x84F7;
    pub const GL_QUADS: std::os::raw::c_uint = 0x0007;
    pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: std::os::raw::c_uint = 0x8E4C;
    pub const GL_QUERY: std::os::raw::c_uint = 0x82E3;
    pub const GL_QUERY_BUFFER: std::os::raw::c_uint = 0x9192;
    pub const GL_QUERY_BUFFER_BARRIER_BIT: std::os::raw::c_uint = 0x00008000;
    pub const GL_QUERY_BUFFER_BINDING: std::os::raw::c_uint = 0x9193;
    pub const GL_QUERY_BY_REGION_NO_WAIT: std::os::raw::c_uint = 0x8E16;
    pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: std::os::raw::c_uint = 0x8E1A;
    pub const GL_QUERY_BY_REGION_WAIT: std::os::raw::c_uint = 0x8E15;
    pub const GL_QUERY_BY_REGION_WAIT_INVERTED: std::os::raw::c_uint = 0x8E19;
    pub const GL_QUERY_COUNTER_BITS: std::os::raw::c_uint = 0x8864;
    pub const GL_QUERY_NO_WAIT: std::os::raw::c_uint = 0x8E14;
    pub const GL_QUERY_NO_WAIT_INVERTED: std::os::raw::c_uint = 0x8E18;
    pub const GL_QUERY_RESULT: std::os::raw::c_uint = 0x8866;
    pub const GL_QUERY_RESULT_AVAILABLE: std::os::raw::c_uint = 0x8867;
    pub const GL_QUERY_RESULT_NO_WAIT: std::os::raw::c_uint = 0x9194;
    pub const GL_QUERY_TARGET: std::os::raw::c_uint = 0x82EA;
    pub const GL_QUERY_WAIT: std::os::raw::c_uint = 0x8E13;
    pub const GL_QUERY_WAIT_INVERTED: std::os::raw::c_uint = 0x8E17;
    pub const GL_R11F_G11F_B10F: std::os::raw::c_uint = 0x8C3A;
    pub const GL_R16: std::os::raw::c_uint = 0x822A;
    pub const GL_R16F: std::os::raw::c_uint = 0x822D;
    pub const GL_R16I: std::os::raw::c_uint = 0x8233;
    pub const GL_R16UI: std::os::raw::c_uint = 0x8234;
    pub const GL_R16_SNORM: std::os::raw::c_uint = 0x8F98;
    pub const GL_R32F: std::os::raw::c_uint = 0x822E;
    pub const GL_R32I: std::os::raw::c_uint = 0x8235;
    pub const GL_R32UI: std::os::raw::c_uint = 0x8236;
    pub const GL_R3_G3_B2: std::os::raw::c_uint = 0x2A10;
    pub const GL_R8: std::os::raw::c_uint = 0x8229;
    pub const GL_R8I: std::os::raw::c_uint = 0x8231;
    pub const GL_R8UI: std::os::raw::c_uint = 0x8232;
    pub const GL_R8_SNORM: std::os::raw::c_uint = 0x8F94;
    pub const GL_RASTERIZER_DISCARD: std::os::raw::c_uint = 0x8C89;
    pub const GL_READ_BUFFER: std::os::raw::c_uint = 0x0C02;
    pub const GL_READ_FRAMEBUFFER: std::os::raw::c_uint = 0x8CA8;
    pub const GL_READ_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CAA;
    pub const GL_READ_ONLY: std::os::raw::c_uint = 0x88B8;
    pub const GL_READ_PIXELS: std::os::raw::c_uint = 0x828C;
    pub const GL_READ_PIXELS_FORMAT: std::os::raw::c_uint = 0x828D;
    pub const GL_READ_PIXELS_TYPE: std::os::raw::c_uint = 0x828E;
    pub const GL_READ_WRITE: std::os::raw::c_uint = 0x88BA;
    pub const GL_RED: std::os::raw::c_uint = 0x1903;
    pub const GL_RED_INTEGER: std::os::raw::c_uint = 0x8D94;
    pub const GL_REFERENCED_BY_COMPUTE_SHADER: std::os::raw::c_uint = 0x930B;
    pub const GL_REFERENCED_BY_FRAGMENT_SHADER: std::os::raw::c_uint = 0x930A;
    pub const GL_REFERENCED_BY_GEOMETRY_SHADER: std::os::raw::c_uint = 0x9309;
    pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: std::os::raw::c_uint = 0x9307;
    pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: std::os::raw::c_uint = 0x9308;
    pub const GL_REFERENCED_BY_VERTEX_SHADER: std::os::raw::c_uint = 0x9306;
    pub const GL_RENDERBUFFER: std::os::raw::c_uint = 0x8D41;
    pub const GL_RENDERBUFFER_ALPHA_SIZE: std::os::raw::c_uint = 0x8D53;
    pub const GL_RENDERBUFFER_BINDING: std::os::raw::c_uint = 0x8CA7;
    pub const GL_RENDERBUFFER_BLUE_SIZE: std::os::raw::c_uint = 0x8D52;
    pub const GL_RENDERBUFFER_DEPTH_SIZE: std::os::raw::c_uint = 0x8D54;
    pub const GL_RENDERBUFFER_GREEN_SIZE: std::os::raw::c_uint = 0x8D51;
    pub const GL_RENDERBUFFER_HEIGHT: std::os::raw::c_uint = 0x8D43;
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT: std::os::raw::c_uint = 0x8D44;
    pub const GL_RENDERBUFFER_RED_SIZE: std::os::raw::c_uint = 0x8D50;
    pub const GL_RENDERBUFFER_SAMPLES: std::os::raw::c_uint = 0x8CAB;
    pub const GL_RENDERBUFFER_STENCIL_SIZE: std::os::raw::c_uint = 0x8D55;
    pub const GL_RENDERBUFFER_WIDTH: std::os::raw::c_uint = 0x8D42;
    pub const GL_RENDERER: std::os::raw::c_uint = 0x1F01;
    pub const GL_REPEAT: std::os::raw::c_uint = 0x2901;
    pub const GL_REPLACE: std::os::raw::c_uint = 0x1E01;
    pub const GL_RESET_NOTIFICATION_STRATEGY: std::os::raw::c_uint = 0x8256;
    pub const GL_RG: std::os::raw::c_uint = 0x8227;
    pub const GL_RG16: std::os::raw::c_uint = 0x822C;
    pub const GL_RG16F: std::os::raw::c_uint = 0x822F;
    pub const GL_RG16I: std::os::raw::c_uint = 0x8239;
    pub const GL_RG16UI: std::os::raw::c_uint = 0x823A;
    pub const GL_RG16_SNORM: std::os::raw::c_uint = 0x8F99;
    pub const GL_RG32F: std::os::raw::c_uint = 0x8230;
    pub const GL_RG32I: std::os::raw::c_uint = 0x823B;
    pub const GL_RG32UI: std::os::raw::c_uint = 0x823C;
    pub const GL_RG8: std::os::raw::c_uint = 0x822B;
    pub const GL_RG8I: std::os::raw::c_uint = 0x8237;
    pub const GL_RG8UI: std::os::raw::c_uint = 0x8238;
    pub const GL_RG8_SNORM: std::os::raw::c_uint = 0x8F95;
    pub const GL_RGB: std::os::raw::c_uint = 0x1907;
    pub const GL_RGB10: std::os::raw::c_uint = 0x8052;
    pub const GL_RGB10_A2: std::os::raw::c_uint = 0x8059;
    pub const GL_RGB10_A2UI: std::os::raw::c_uint = 0x906F;
    pub const GL_RGB12: std::os::raw::c_uint = 0x8053;
    pub const GL_RGB16: std::os::raw::c_uint = 0x8054;
    pub const GL_RGB16F: std::os::raw::c_uint = 0x881B;
    pub const GL_RGB16I: std::os::raw::c_uint = 0x8D89;
    pub const GL_RGB16UI: std::os::raw::c_uint = 0x8D77;
    pub const GL_RGB16_SNORM: std::os::raw::c_uint = 0x8F9A;
    pub const GL_RGB32F: std::os::raw::c_uint = 0x8815;
    pub const GL_RGB32I: std::os::raw::c_uint = 0x8D83;
    pub const GL_RGB32UI: std::os::raw::c_uint = 0x8D71;
    pub const GL_RGB4: std::os::raw::c_uint = 0x804F;
    pub const GL_RGB5: std::os::raw::c_uint = 0x8050;
    pub const GL_RGB565: std::os::raw::c_uint = 0x8D62;
    pub const GL_RGB5_A1: std::os::raw::c_uint = 0x8057;
    pub const GL_RGB8: std::os::raw::c_uint = 0x8051;
    pub const GL_RGB8I: std::os::raw::c_uint = 0x8D8F;
    pub const GL_RGB8UI: std::os::raw::c_uint = 0x8D7D;
    pub const GL_RGB8_SNORM: std::os::raw::c_uint = 0x8F96;
    pub const GL_RGB9_E5: std::os::raw::c_uint = 0x8C3D;
    pub const GL_RGBA: std::os::raw::c_uint = 0x1908;
    pub const GL_RGBA12: std::os::raw::c_uint = 0x805A;
    pub const GL_RGBA16: std::os::raw::c_uint = 0x805B;
    pub const GL_RGBA16F: std::os::raw::c_uint = 0x881A;
    pub const GL_RGBA16I: std::os::raw::c_uint = 0x8D88;
    pub const GL_RGBA16UI: std::os::raw::c_uint = 0x8D76;
    pub const GL_RGBA16_SNORM: std::os::raw::c_uint = 0x8F9B;
    pub const GL_RGBA2: std::os::raw::c_uint = 0x8055;
    pub const GL_RGBA32F: std::os::raw::c_uint = 0x8814;
    pub const GL_RGBA32I: std::os::raw::c_uint = 0x8D82;
    pub const GL_RGBA32UI: std::os::raw::c_uint = 0x8D70;
    pub const GL_RGBA4: std::os::raw::c_uint = 0x8056;
    pub const GL_RGBA8: std::os::raw::c_uint = 0x8058;
    pub const GL_RGBA8I: std::os::raw::c_uint = 0x8D8E;
    pub const GL_RGBA8UI: std::os::raw::c_uint = 0x8D7C;
    pub const GL_RGBA8_SNORM: std::os::raw::c_uint = 0x8F97;
    pub const GL_RGBA_INTEGER: std::os::raw::c_uint = 0x8D99;
    pub const GL_RGB_INTEGER: std::os::raw::c_uint = 0x8D98;
    pub const GL_RG_INTEGER: std::os::raw::c_uint = 0x8228;
    pub const GL_RIGHT: std::os::raw::c_uint = 0x0407;
    pub const GL_SAMPLER: std::os::raw::c_uint = 0x82E6;
    pub const GL_SAMPLER_1D: std::os::raw::c_uint = 0x8B5D;
    pub const GL_SAMPLER_1D_ARRAY: std::os::raw::c_uint = 0x8DC0;
    pub const GL_SAMPLER_1D_ARRAY_SHADOW: std::os::raw::c_uint = 0x8DC3;
    pub const GL_SAMPLER_1D_SHADOW: std::os::raw::c_uint = 0x8B61;
    pub const GL_SAMPLER_2D: std::os::raw::c_uint = 0x8B5E;
    pub const GL_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DC1;
    pub const GL_SAMPLER_2D_ARRAY_SHADOW: std::os::raw::c_uint = 0x8DC4;
    pub const GL_SAMPLER_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9108;
    pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x910B;
    pub const GL_SAMPLER_2D_RECT: std::os::raw::c_uint = 0x8B63;
    pub const GL_SAMPLER_2D_RECT_SHADOW: std::os::raw::c_uint = 0x8B64;
    pub const GL_SAMPLER_2D_SHADOW: std::os::raw::c_uint = 0x8B62;
    pub const GL_SAMPLER_3D: std::os::raw::c_uint = 0x8B5F;
    pub const GL_SAMPLER_BINDING: std::os::raw::c_uint = 0x8919;
    pub const GL_SAMPLER_BUFFER: std::os::raw::c_uint = 0x8DC2;
    pub const GL_SAMPLER_CUBE: std::os::raw::c_uint = 0x8B60;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x900C;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: std::os::raw::c_uint = 0x900D;
    pub const GL_SAMPLER_CUBE_SHADOW: std::os::raw::c_uint = 0x8DC5;
    pub const GL_SAMPLES: std::os::raw::c_uint = 0x80A9;
    pub const GL_SAMPLES_PASSED: std::os::raw::c_uint = 0x8914;
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE: std::os::raw::c_uint = 0x809E;
    pub const GL_SAMPLE_ALPHA_TO_ONE: std::os::raw::c_uint = 0x809F;
    pub const GL_SAMPLE_BUFFERS: std::os::raw::c_uint = 0x80A8;
    pub const GL_SAMPLE_COVERAGE: std::os::raw::c_uint = 0x80A0;
    pub const GL_SAMPLE_COVERAGE_INVERT: std::os::raw::c_uint = 0x80AB;
    pub const GL_SAMPLE_COVERAGE_VALUE: std::os::raw::c_uint = 0x80AA;
    pub const GL_SAMPLE_MASK: std::os::raw::c_uint = 0x8E51;
    pub const GL_SAMPLE_MASK_VALUE: std::os::raw::c_uint = 0x8E52;
    pub const GL_SAMPLE_POSITION: std::os::raw::c_uint = 0x8E50;
    pub const GL_SAMPLE_SHADING: std::os::raw::c_uint = 0x8C36;
    pub const GL_SCISSOR_BOX: std::os::raw::c_uint = 0x0C10;
    pub const GL_SCISSOR_TEST: std::os::raw::c_uint = 0x0C11;
    pub const GL_SEPARATE_ATTRIBS: std::os::raw::c_uint = 0x8C8D;
    pub const GL_SET: std::os::raw::c_uint = 0x150F;
    pub const GL_SHADER: std::os::raw::c_uint = 0x82E1;
    pub const GL_SHADER_BINARY_FORMATS: std::os::raw::c_uint = 0x8DF8;
    pub const GL_SHADER_COMPILER: std::os::raw::c_uint = 0x8DFA;
    pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: std::os::raw::c_uint = 0x00000020;
    pub const GL_SHADER_IMAGE_ATOMIC: std::os::raw::c_uint = 0x82A6;
    pub const GL_SHADER_IMAGE_LOAD: std::os::raw::c_uint = 0x82A4;
    pub const GL_SHADER_IMAGE_STORE: std::os::raw::c_uint = 0x82A5;
    pub const GL_SHADER_SOURCE_LENGTH: std::os::raw::c_uint = 0x8B88;
    pub const GL_SHADER_STORAGE_BARRIER_BIT: std::os::raw::c_uint = 0x00002000;
    pub const GL_SHADER_STORAGE_BLOCK: std::os::raw::c_uint = 0x92E6;
    pub const GL_SHADER_STORAGE_BUFFER: std::os::raw::c_uint = 0x90D2;
    pub const GL_SHADER_STORAGE_BUFFER_BINDING: std::os::raw::c_uint = 0x90D3;
    pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: std::os::raw::c_uint = 0x90DF;
    pub const GL_SHADER_STORAGE_BUFFER_SIZE: std::os::raw::c_uint = 0x90D5;
    pub const GL_SHADER_STORAGE_BUFFER_START: std::os::raw::c_uint = 0x90D4;
    pub const GL_SHADER_TYPE: std::os::raw::c_uint = 0x8B4F;
    pub const GL_SHADING_LANGUAGE_VERSION: std::os::raw::c_uint = 0x8B8C;
    pub const GL_SHORT: std::os::raw::c_uint = 0x1402;
    pub const GL_SIGNALED: std::os::raw::c_uint = 0x9119;
    pub const GL_SIGNED_NORMALIZED: std::os::raw::c_uint = 0x8F9C;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: std::os::raw::c_uint = 0x82AC;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: std::os::raw::c_uint = 0x82AE;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: std::os::raw::c_uint = 0x82AD;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: std::os::raw::c_uint = 0x82AF;
    pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: std::os::raw::c_uint = 0x0B23;
    pub const GL_SMOOTH_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x0B22;
    pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: std::os::raw::c_uint = 0x0B13;
    pub const GL_SMOOTH_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x0B12;
    pub const GL_SRC1_ALPHA: std::os::raw::c_uint = 0x8589;
    pub const GL_SRC1_COLOR: std::os::raw::c_uint = 0x88F9;
    pub const GL_SRC_ALPHA: std::os::raw::c_uint = 0x0302;
    pub const GL_SRC_ALPHA_SATURATE: std::os::raw::c_uint = 0x0308;
    pub const GL_SRC_COLOR: std::os::raw::c_uint = 0x0300;
    pub const GL_SRGB: std::os::raw::c_uint = 0x8C40;
    pub const GL_SRGB8: std::os::raw::c_uint = 0x8C41;
    pub const GL_SRGB8_ALPHA8: std::os::raw::c_uint = 0x8C43;
    pub const GL_SRGB_ALPHA: std::os::raw::c_uint = 0x8C42;
    pub const GL_SRGB_READ: std::os::raw::c_uint = 0x8297;
    pub const GL_SRGB_WRITE: std::os::raw::c_uint = 0x8298;
    pub const GL_STACK_OVERFLOW: std::os::raw::c_uint = 0x0503;
    pub const GL_STACK_UNDERFLOW: std::os::raw::c_uint = 0x0504;
    pub const GL_STATIC_COPY: std::os::raw::c_uint = 0x88E6;
    pub const GL_STATIC_DRAW: std::os::raw::c_uint = 0x88E4;
    pub const GL_STATIC_READ: std::os::raw::c_uint = 0x88E5;
    pub const GL_STENCIL: std::os::raw::c_uint = 0x1802;
    pub const GL_STENCIL_ATTACHMENT: std::os::raw::c_uint = 0x8D20;
    pub const GL_STENCIL_BACK_FAIL: std::os::raw::c_uint = 0x8801;
    pub const GL_STENCIL_BACK_FUNC: std::os::raw::c_uint = 0x8800;
    pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x8802;
    pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x8803;
    pub const GL_STENCIL_BACK_REF: std::os::raw::c_uint = 0x8CA3;
    pub const GL_STENCIL_BACK_VALUE_MASK: std::os::raw::c_uint = 0x8CA4;
    pub const GL_STENCIL_BACK_WRITEMASK: std::os::raw::c_uint = 0x8CA5;
    pub const GL_STENCIL_BUFFER_BIT: std::os::raw::c_uint = 0x00000400;
    pub const GL_STENCIL_CLEAR_VALUE: std::os::raw::c_uint = 0x0B91;
    pub const GL_STENCIL_COMPONENTS: std::os::raw::c_uint = 0x8285;
    pub const GL_STENCIL_FAIL: std::os::raw::c_uint = 0x0B94;
    pub const GL_STENCIL_FUNC: std::os::raw::c_uint = 0x0B92;
    pub const GL_STENCIL_INDEX: std::os::raw::c_uint = 0x1901;
    pub const GL_STENCIL_INDEX1: std::os::raw::c_uint = 0x8D46;
    pub const GL_STENCIL_INDEX16: std::os::raw::c_uint = 0x8D49;
    pub const GL_STENCIL_INDEX4: std::os::raw::c_uint = 0x8D47;
    pub const GL_STENCIL_INDEX8: std::os::raw::c_uint = 0x8D48;
    pub const GL_STENCIL_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x0B96;
    pub const GL_STENCIL_REF: std::os::raw::c_uint = 0x0B97;
    pub const GL_STENCIL_RENDERABLE: std::os::raw::c_uint = 0x8288;
    pub const GL_STENCIL_TEST: std::os::raw::c_uint = 0x0B90;
    pub const GL_STENCIL_VALUE_MASK: std::os::raw::c_uint = 0x0B93;
    pub const GL_STENCIL_WRITEMASK: std::os::raw::c_uint = 0x0B98;
    pub const GL_STEREO: std::os::raw::c_uint = 0x0C33;
    pub const GL_STREAM_COPY: std::os::raw::c_uint = 0x88E2;
    pub const GL_STREAM_DRAW: std::os::raw::c_uint = 0x88E0;
    pub const GL_STREAM_READ: std::os::raw::c_uint = 0x88E1;
    pub const GL_SUBPIXEL_BITS: std::os::raw::c_uint = 0x0D50;
    pub const GL_SYNC_CONDITION: std::os::raw::c_uint = 0x9113;
    pub const GL_SYNC_FENCE: std::os::raw::c_uint = 0x9116;
    pub const GL_SYNC_FLAGS: std::os::raw::c_uint = 0x9115;
    pub const GL_SYNC_FLUSH_COMMANDS_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_SYNC_GPU_COMMANDS_COMPLETE: std::os::raw::c_uint = 0x9117;
    pub const GL_SYNC_STATUS: std::os::raw::c_uint = 0x9114;
    pub const GL_TESS_CONTROL_OUTPUT_VERTICES: std::os::raw::c_uint = 0x8E75;
    pub const GL_TESS_CONTROL_SHADER: std::os::raw::c_uint = 0x8E88;
    pub const GL_TESS_CONTROL_SHADER_BIT: std::os::raw::c_uint = 0x00000008;
    pub const GL_TESS_CONTROL_SUBROUTINE: std::os::raw::c_uint = 0x92E9;
    pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: std::os::raw::c_uint = 0x92EF;
    pub const GL_TESS_CONTROL_TEXTURE: std::os::raw::c_uint = 0x829C;
    pub const GL_TESS_EVALUATION_SHADER: std::os::raw::c_uint = 0x8E87;
    pub const GL_TESS_EVALUATION_SHADER_BIT: std::os::raw::c_uint = 0x00000010;
    pub const GL_TESS_EVALUATION_SUBROUTINE: std::os::raw::c_uint = 0x92EA;
    pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: std::os::raw::c_uint = 0x92F0;
    pub const GL_TESS_EVALUATION_TEXTURE: std::os::raw::c_uint = 0x829D;
    pub const GL_TESS_GEN_MODE: std::os::raw::c_uint = 0x8E76;
    pub const GL_TESS_GEN_POINT_MODE: std::os::raw::c_uint = 0x8E79;
    pub const GL_TESS_GEN_SPACING: std::os::raw::c_uint = 0x8E77;
    pub const GL_TESS_GEN_VERTEX_ORDER: std::os::raw::c_uint = 0x8E78;
    pub const GL_TEXTURE: std::os::raw::c_uint = 0x1702;
    pub const GL_TEXTURE0: std::os::raw::c_uint = 0x84C0;
    pub const GL_TEXTURE1: std::os::raw::c_uint = 0x84C1;
    pub const GL_TEXTURE10: std::os::raw::c_uint = 0x84CA;
    pub const GL_TEXTURE11: std::os::raw::c_uint = 0x84CB;
    pub const GL_TEXTURE12: std::os::raw::c_uint = 0x84CC;
    pub const GL_TEXTURE13: std::os::raw::c_uint = 0x84CD;
    pub const GL_TEXTURE14: std::os::raw::c_uint = 0x84CE;
    pub const GL_TEXTURE15: std::os::raw::c_uint = 0x84CF;
    pub const GL_TEXTURE16: std::os::raw::c_uint = 0x84D0;
    pub const GL_TEXTURE17: std::os::raw::c_uint = 0x84D1;
    pub const GL_TEXTURE18: std::os::raw::c_uint = 0x84D2;
    pub const GL_TEXTURE19: std::os::raw::c_uint = 0x84D3;
    pub const GL_TEXTURE2: std::os::raw::c_uint = 0x84C2;
    pub const GL_TEXTURE20: std::os::raw::c_uint = 0x84D4;
    pub const GL_TEXTURE21: std::os::raw::c_uint = 0x84D5;
    pub const GL_TEXTURE22: std::os::raw::c_uint = 0x84D6;
    pub const GL_TEXTURE23: std::os::raw::c_uint = 0x84D7;
    pub const GL_TEXTURE24: std::os::raw::c_uint = 0x84D8;
    pub const GL_TEXTURE25: std::os::raw::c_uint = 0x84D9;
    pub const GL_TEXTURE26: std::os::raw::c_uint = 0x84DA;
    pub const GL_TEXTURE27: std::os::raw::c_uint = 0x84DB;
    pub const GL_TEXTURE28: std::os::raw::c_uint = 0x84DC;
    pub const GL_TEXTURE29: std::os::raw::c_uint = 0x84DD;
    pub const GL_TEXTURE3: std::os::raw::c_uint = 0x84C3;
    pub const GL_TEXTURE30: std::os::raw::c_uint = 0x84DE;
    pub const GL_TEXTURE31: std::os::raw::c_uint = 0x84DF;
    pub const GL_TEXTURE4: std::os::raw::c_uint = 0x84C4;
    pub const GL_TEXTURE5: std::os::raw::c_uint = 0x84C5;
    pub const GL_TEXTURE6: std::os::raw::c_uint = 0x84C6;
    pub const GL_TEXTURE7: std::os::raw::c_uint = 0x84C7;
    pub const GL_TEXTURE8: std::os::raw::c_uint = 0x84C8;
    pub const GL_TEXTURE9: std::os::raw::c_uint = 0x84C9;
    pub const GL_TEXTURE_1D: std::os::raw::c_uint = 0x0DE0;
    pub const GL_TEXTURE_1D_ARRAY: std::os::raw::c_uint = 0x8C18;
    pub const GL_TEXTURE_2D: std::os::raw::c_uint = 0x0DE1;
    pub const GL_TEXTURE_2D_ARRAY: std::os::raw::c_uint = 0x8C1A;
    pub const GL_TEXTURE_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9100;
    pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9102;
    pub const GL_TEXTURE_3D: std::os::raw::c_uint = 0x806F;
    pub const GL_TEXTURE_ALPHA_SIZE: std::os::raw::c_uint = 0x805F;
    pub const GL_TEXTURE_ALPHA_TYPE: std::os::raw::c_uint = 0x8C13;
    pub const GL_TEXTURE_BASE_LEVEL: std::os::raw::c_uint = 0x813C;
    pub const GL_TEXTURE_BINDING_1D: std::os::raw::c_uint = 0x8068;
    pub const GL_TEXTURE_BINDING_1D_ARRAY: std::os::raw::c_uint = 0x8C1C;
    pub const GL_TEXTURE_BINDING_2D: std::os::raw::c_uint = 0x8069;
    pub const GL_TEXTURE_BINDING_2D_ARRAY: std::os::raw::c_uint = 0x8C1D;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9104;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9105;
    pub const GL_TEXTURE_BINDING_3D: std::os::raw::c_uint = 0x806A;
    pub const GL_TEXTURE_BINDING_BUFFER: std::os::raw::c_uint = 0x8C2C;
    pub const GL_TEXTURE_BINDING_CUBE_MAP: std::os::raw::c_uint = 0x8514;
    pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x900A;
    pub const GL_TEXTURE_BINDING_RECTANGLE: std::os::raw::c_uint = 0x84F6;
    pub const GL_TEXTURE_BLUE_SIZE: std::os::raw::c_uint = 0x805E;
    pub const GL_TEXTURE_BLUE_TYPE: std::os::raw::c_uint = 0x8C12;
    pub const GL_TEXTURE_BORDER_COLOR: std::os::raw::c_uint = 0x1004;
    pub const GL_TEXTURE_BUFFER: std::os::raw::c_uint = 0x8C2A;
    pub const GL_TEXTURE_BUFFER_BINDING: std::os::raw::c_uint = 0x8C2A;
    pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: std::os::raw::c_uint = 0x8C2D;
    pub const GL_TEXTURE_BUFFER_OFFSET: std::os::raw::c_uint = 0x919D;
    pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: std::os::raw::c_uint = 0x919F;
    pub const GL_TEXTURE_BUFFER_SIZE: std::os::raw::c_uint = 0x919E;
    pub const GL_TEXTURE_COMPARE_FUNC: std::os::raw::c_uint = 0x884D;
    pub const GL_TEXTURE_COMPARE_MODE: std::os::raw::c_uint = 0x884C;
    pub const GL_TEXTURE_COMPRESSED: std::os::raw::c_uint = 0x86A1;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: std::os::raw::c_uint = 0x82B2;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: std::os::raw::c_uint = 0x82B3;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: std::os::raw::c_uint = 0x82B1;
    pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: std::os::raw::c_uint = 0x86A0;
    pub const GL_TEXTURE_COMPRESSION_HINT: std::os::raw::c_uint = 0x84EF;
    pub const GL_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x8513;
    pub const GL_TEXTURE_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x9009;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: std::os::raw::c_uint = 0x8516;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: std::os::raw::c_uint = 0x8518;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: std::os::raw::c_uint = 0x851A;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: std::os::raw::c_uint = 0x8515;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: std::os::raw::c_uint = 0x8517;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: std::os::raw::c_uint = 0x8519;
    pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: std::os::raw::c_uint = 0x884F;
    pub const GL_TEXTURE_DEPTH: std::os::raw::c_uint = 0x8071;
    pub const GL_TEXTURE_DEPTH_SIZE: std::os::raw::c_uint = 0x884A;
    pub const GL_TEXTURE_DEPTH_TYPE: std::os::raw::c_uint = 0x8C16;
    pub const GL_TEXTURE_FETCH_BARRIER_BIT: std::os::raw::c_uint = 0x00000008;
    pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: std::os::raw::c_uint = 0x9107;
    pub const GL_TEXTURE_GATHER: std::os::raw::c_uint = 0x82A2;
    pub const GL_TEXTURE_GATHER_SHADOW: std::os::raw::c_uint = 0x82A3;
    pub const GL_TEXTURE_GREEN_SIZE: std::os::raw::c_uint = 0x805D;
    pub const GL_TEXTURE_GREEN_TYPE: std::os::raw::c_uint = 0x8C11;
    pub const GL_TEXTURE_HEIGHT: std::os::raw::c_uint = 0x1001;
    pub const GL_TEXTURE_IMAGE_FORMAT: std::os::raw::c_uint = 0x828F;
    pub const GL_TEXTURE_IMAGE_TYPE: std::os::raw::c_uint = 0x8290;
    pub const GL_TEXTURE_IMMUTABLE_FORMAT: std::os::raw::c_uint = 0x912F;
    pub const GL_TEXTURE_IMMUTABLE_LEVELS: std::os::raw::c_uint = 0x82DF;
    pub const GL_TEXTURE_INTERNAL_FORMAT: std::os::raw::c_uint = 0x1003;
    pub const GL_TEXTURE_LOD_BIAS: std::os::raw::c_uint = 0x8501;
    pub const GL_TEXTURE_MAG_FILTER: std::os::raw::c_uint = 0x2800;
    pub const GL_TEXTURE_MAX_LEVEL: std::os::raw::c_uint = 0x813D;
    pub const GL_TEXTURE_MAX_LOD: std::os::raw::c_uint = 0x813B;
    pub const GL_TEXTURE_MIN_FILTER: std::os::raw::c_uint = 0x2801;
    pub const GL_TEXTURE_MIN_LOD: std::os::raw::c_uint = 0x813A;
    pub const GL_TEXTURE_RECTANGLE: std::os::raw::c_uint = 0x84F5;
    pub const GL_TEXTURE_RED_SIZE: std::os::raw::c_uint = 0x805C;
    pub const GL_TEXTURE_RED_TYPE: std::os::raw::c_uint = 0x8C10;
    pub const GL_TEXTURE_SAMPLES: std::os::raw::c_uint = 0x9106;
    pub const GL_TEXTURE_SHADOW: std::os::raw::c_uint = 0x82A1;
    pub const GL_TEXTURE_SHARED_SIZE: std::os::raw::c_uint = 0x8C3F;
    pub const GL_TEXTURE_STENCIL_SIZE: std::os::raw::c_uint = 0x88F1;
    pub const GL_TEXTURE_SWIZZLE_A: std::os::raw::c_uint = 0x8E45;
    pub const GL_TEXTURE_SWIZZLE_B: std::os::raw::c_uint = 0x8E44;
    pub const GL_TEXTURE_SWIZZLE_G: std::os::raw::c_uint = 0x8E43;
    pub const GL_TEXTURE_SWIZZLE_R: std::os::raw::c_uint = 0x8E42;
    pub const GL_TEXTURE_SWIZZLE_RGBA: std::os::raw::c_uint = 0x8E46;
    pub const GL_TEXTURE_TARGET: std::os::raw::c_uint = 0x1006;
    pub const GL_TEXTURE_UPDATE_BARRIER_BIT: std::os::raw::c_uint = 0x00000100;
    pub const GL_TEXTURE_VIEW: std::os::raw::c_uint = 0x82B5;
    pub const GL_TEXTURE_VIEW_MIN_LAYER: std::os::raw::c_uint = 0x82DD;
    pub const GL_TEXTURE_VIEW_MIN_LEVEL: std::os::raw::c_uint = 0x82DB;
    pub const GL_TEXTURE_VIEW_NUM_LAYERS: std::os::raw::c_uint = 0x82DE;
    pub const GL_TEXTURE_VIEW_NUM_LEVELS: std::os::raw::c_uint = 0x82DC;
    pub const GL_TEXTURE_WIDTH: std::os::raw::c_uint = 0x1000;
    pub const GL_TEXTURE_WRAP_R: std::os::raw::c_uint = 0x8072;
    pub const GL_TEXTURE_WRAP_S: std::os::raw::c_uint = 0x2802;
    pub const GL_TEXTURE_WRAP_T: std::os::raw::c_uint = 0x2803;
    pub const GL_TIMEOUT_EXPIRED: std::os::raw::c_uint = 0x911B;
    pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const GL_TIMESTAMP: std::os::raw::c_uint = 0x8E28;
    pub const GL_TIME_ELAPSED: std::os::raw::c_uint = 0x88BF;
    pub const GL_TOP_LEVEL_ARRAY_SIZE: std::os::raw::c_uint = 0x930C;
    pub const GL_TOP_LEVEL_ARRAY_STRIDE: std::os::raw::c_uint = 0x930D;
    pub const GL_TRANSFORM_FEEDBACK: std::os::raw::c_uint = 0x8E22;
    pub const GL_TRANSFORM_FEEDBACK_ACTIVE: std::os::raw::c_uint = 0x8E24;
    pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: std::os::raw::c_uint = 0x00000800;
    pub const GL_TRANSFORM_FEEDBACK_BINDING: std::os::raw::c_uint = 0x8E25;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: std::os::raw::c_uint = 0x8C8E;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: std::os::raw::c_uint = 0x8E24;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: std::os::raw::c_uint = 0x8C8F;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: std::os::raw::c_uint = 0x934B;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: std::os::raw::c_uint = 0x8C7F;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: std::os::raw::c_uint = 0x8E23;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: std::os::raw::c_uint = 0x8C85;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: std::os::raw::c_uint = 0x8C84;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: std::os::raw::c_uint = 0x934C;
    pub const GL_TRANSFORM_FEEDBACK_PAUSED: std::os::raw::c_uint = 0x8E23;
    pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: std::os::raw::c_uint = 0x8C88;
    pub const GL_TRANSFORM_FEEDBACK_VARYING: std::os::raw::c_uint = 0x92F4;
    pub const GL_TRANSFORM_FEEDBACK_VARYINGS: std::os::raw::c_uint = 0x8C83;
    pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: std::os::raw::c_uint = 0x8C76;
    pub const GL_TRIANGLES: std::os::raw::c_uint = 0x0004;
    pub const GL_TRIANGLES_ADJACENCY: std::os::raw::c_uint = 0x000C;
    pub const GL_TRIANGLE_FAN: std::os::raw::c_uint = 0x0006;
    pub const GL_TRIANGLE_STRIP: std::os::raw::c_uint = 0x0005;
    pub const GL_TRIANGLE_STRIP_ADJACENCY: std::os::raw::c_uint = 0x000D;
    pub const GL_TRUE: std::os::raw::c_uchar = 1;
    pub const GL_TYPE: std::os::raw::c_uint = 0x92FA;
    pub const GL_UNDEFINED_VERTEX: std::os::raw::c_uint = 0x8260;
    pub const GL_UNIFORM: std::os::raw::c_uint = 0x92E1;
    pub const GL_UNIFORM_ARRAY_STRIDE: std::os::raw::c_uint = 0x8A3C;
    pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: std::os::raw::c_uint = 0x92DA;
    pub const GL_UNIFORM_BARRIER_BIT: std::os::raw::c_uint = 0x00000004;
    pub const GL_UNIFORM_BLOCK: std::os::raw::c_uint = 0x92E2;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8A42;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: std::os::raw::c_uint = 0x8A43;
    pub const GL_UNIFORM_BLOCK_BINDING: std::os::raw::c_uint = 0x8A3F;
    pub const GL_UNIFORM_BLOCK_DATA_SIZE: std::os::raw::c_uint = 0x8A40;
    pub const GL_UNIFORM_BLOCK_INDEX: std::os::raw::c_uint = 0x8A3A;
    pub const GL_UNIFORM_BLOCK_NAME_LENGTH: std::os::raw::c_uint = 0x8A41;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: std::os::raw::c_uint = 0x90EC;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8A46;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: std::os::raw::c_uint = 0x8A45;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: std::os::raw::c_uint = 0x84F0;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: std::os::raw::c_uint = 0x84F1;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: std::os::raw::c_uint = 0x8A44;
    pub const GL_UNIFORM_BUFFER: std::os::raw::c_uint = 0x8A11;
    pub const GL_UNIFORM_BUFFER_BINDING: std::os::raw::c_uint = 0x8A28;
    pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: std::os::raw::c_uint = 0x8A34;
    pub const GL_UNIFORM_BUFFER_SIZE: std::os::raw::c_uint = 0x8A2A;
    pub const GL_UNIFORM_BUFFER_START: std::os::raw::c_uint = 0x8A29;
    pub const GL_UNIFORM_IS_ROW_MAJOR: std::os::raw::c_uint = 0x8A3E;
    pub const GL_UNIFORM_MATRIX_STRIDE: std::os::raw::c_uint = 0x8A3D;
    pub const GL_UNIFORM_NAME_LENGTH: std::os::raw::c_uint = 0x8A39;
    pub const GL_UNIFORM_OFFSET: std::os::raw::c_uint = 0x8A3B;
    pub const GL_UNIFORM_SIZE: std::os::raw::c_uint = 0x8A38;
    pub const GL_UNIFORM_TYPE: std::os::raw::c_uint = 0x8A37;
    pub const GL_UNKNOWN_CONTEXT_RESET: std::os::raw::c_uint = 0x8255;
    pub const GL_UNPACK_ALIGNMENT: std::os::raw::c_uint = 0x0CF5;
    pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: std::os::raw::c_uint = 0x9129;
    pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: std::os::raw::c_uint = 0x9128;
    pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: std::os::raw::c_uint = 0x912A;
    pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: std::os::raw::c_uint = 0x9127;
    pub const GL_UNPACK_IMAGE_HEIGHT: std::os::raw::c_uint = 0x806E;
    pub const GL_UNPACK_LSB_FIRST: std::os::raw::c_uint = 0x0CF1;
    pub const GL_UNPACK_ROW_LENGTH: std::os::raw::c_uint = 0x0CF2;
    pub const GL_UNPACK_SKIP_IMAGES: std::os::raw::c_uint = 0x806D;
    pub const GL_UNPACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0CF4;
    pub const GL_UNPACK_SKIP_ROWS: std::os::raw::c_uint = 0x0CF3;
    pub const GL_UNPACK_SWAP_BYTES: std::os::raw::c_uint = 0x0CF0;
    pub const GL_UNSIGNALED: std::os::raw::c_uint = 0x9118;
    pub const GL_UNSIGNED_BYTE: std::os::raw::c_uint = 0x1401;
    pub const GL_UNSIGNED_BYTE_2_3_3_REV: std::os::raw::c_uint = 0x8362;
    pub const GL_UNSIGNED_BYTE_3_3_2: std::os::raw::c_uint = 0x8032;
    pub const GL_UNSIGNED_INT: std::os::raw::c_uint = 0x1405;
    pub const GL_UNSIGNED_INT_10F_11F_11F_REV: std::os::raw::c_uint = 0x8C3B;
    pub const GL_UNSIGNED_INT_10_10_10_2: std::os::raw::c_uint = 0x8036;
    pub const GL_UNSIGNED_INT_24_8: std::os::raw::c_uint = 0x84FA;
    pub const GL_UNSIGNED_INT_2_10_10_10_REV: std::os::raw::c_uint = 0x8368;
    pub const GL_UNSIGNED_INT_5_9_9_9_REV: std::os::raw::c_uint = 0x8C3E;
    pub const GL_UNSIGNED_INT_8_8_8_8: std::os::raw::c_uint = 0x8035;
    pub const GL_UNSIGNED_INT_8_8_8_8_REV: std::os::raw::c_uint = 0x8367;
    pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: std::os::raw::c_uint = 0x92DB;
    pub const GL_UNSIGNED_INT_IMAGE_1D: std::os::raw::c_uint = 0x9062;
    pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: std::os::raw::c_uint = 0x9068;
    pub const GL_UNSIGNED_INT_IMAGE_2D: std::os::raw::c_uint = 0x9063;
    pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: std::os::raw::c_uint = 0x9069;
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: std::os::raw::c_uint = 0x906B;
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x906C;
    pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: std::os::raw::c_uint = 0x9065;
    pub const GL_UNSIGNED_INT_IMAGE_3D: std::os::raw::c_uint = 0x9064;
    pub const GL_UNSIGNED_INT_IMAGE_BUFFER: std::os::raw::c_uint = 0x9067;
    pub const GL_UNSIGNED_INT_IMAGE_CUBE: std::os::raw::c_uint = 0x9066;
    pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x906A;
    pub const GL_UNSIGNED_INT_SAMPLER_1D: std::os::raw::c_uint = 0x8DD1;
    pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: std::os::raw::c_uint = 0x8DD6;
    pub const GL_UNSIGNED_INT_SAMPLER_2D: std::os::raw::c_uint = 0x8DD2;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DD7;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: std::os::raw::c_uint = 0x910A;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x910D;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: std::os::raw::c_uint = 0x8DD5;
    pub const GL_UNSIGNED_INT_SAMPLER_3D: std::os::raw::c_uint = 0x8DD3;
    pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: std::os::raw::c_uint = 0x8DD8;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE: std::os::raw::c_uint = 0x8DD4;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: std::os::raw::c_uint = 0x900F;
    pub const GL_UNSIGNED_INT_VEC2: std::os::raw::c_uint = 0x8DC6;
    pub const GL_UNSIGNED_INT_VEC3: std::os::raw::c_uint = 0x8DC7;
    pub const GL_UNSIGNED_INT_VEC4: std::os::raw::c_uint = 0x8DC8;
    pub const GL_UNSIGNED_NORMALIZED: std::os::raw::c_uint = 0x8C17;
    pub const GL_UNSIGNED_SHORT: std::os::raw::c_uint = 0x1403;
    pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: std::os::raw::c_uint = 0x8366;
    pub const GL_UNSIGNED_SHORT_4_4_4_4: std::os::raw::c_uint = 0x8033;
    pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: std::os::raw::c_uint = 0x8365;
    pub const GL_UNSIGNED_SHORT_5_5_5_1: std::os::raw::c_uint = 0x8034;
    pub const GL_UNSIGNED_SHORT_5_6_5: std::os::raw::c_uint = 0x8363;
    pub const GL_UNSIGNED_SHORT_5_6_5_REV: std::os::raw::c_uint = 0x8364;
    pub const GL_UPPER_LEFT: std::os::raw::c_uint = 0x8CA2;
    pub const GL_VALIDATE_STATUS: std::os::raw::c_uint = 0x8B83;
    pub const GL_VENDOR: std::os::raw::c_uint = 0x1F00;
    pub const GL_VERSION: std::os::raw::c_uint = 0x1F02;
    pub const GL_VERTEX_ARRAY: std::os::raw::c_uint = 0x8074;
    pub const GL_VERTEX_ARRAY_BINDING: std::os::raw::c_uint = 0x85B5;
    pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889F;
    pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: std::os::raw::c_uint = 0x88FE;
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: std::os::raw::c_uint = 0x8622;
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: std::os::raw::c_uint = 0x88FD;
    pub const GL_VERTEX_ATTRIB_ARRAY_LONG: std::os::raw::c_uint = 0x874E;
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: std::os::raw::c_uint = 0x886A;
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: std::os::raw::c_uint = 0x8645;
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: std::os::raw::c_uint = 0x8623;
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: std::os::raw::c_uint = 0x8624;
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: std::os::raw::c_uint = 0x8625;
    pub const GL_VERTEX_ATTRIB_BINDING: std::os::raw::c_uint = 0x82D4;
    pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: std::os::raw::c_uint = 0x82D5;
    pub const GL_VERTEX_BINDING_BUFFER: std::os::raw::c_uint = 0x8F4F;
    pub const GL_VERTEX_BINDING_DIVISOR: std::os::raw::c_uint = 0x82D6;
    pub const GL_VERTEX_BINDING_OFFSET: std::os::raw::c_uint = 0x82D7;
    pub const GL_VERTEX_BINDING_STRIDE: std::os::raw::c_uint = 0x82D8;
    pub const GL_VERTEX_PROGRAM_POINT_SIZE: std::os::raw::c_uint = 0x8642;
    pub const GL_VERTEX_SHADER: std::os::raw::c_uint = 0x8B31;
    pub const GL_VERTEX_SHADER_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_VERTEX_SUBROUTINE: std::os::raw::c_uint = 0x92E8;
    pub const GL_VERTEX_SUBROUTINE_UNIFORM: std::os::raw::c_uint = 0x92EE;
    pub const GL_VERTEX_TEXTURE: std::os::raw::c_uint = 0x829B;
    pub const GL_VIEWPORT: std::os::raw::c_uint = 0x0BA2;
    pub const GL_VIEWPORT_BOUNDS_RANGE: std::os::raw::c_uint = 0x825D;
    pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: std::os::raw::c_uint = 0x825F;
    pub const GL_VIEWPORT_SUBPIXEL_BITS: std::os::raw::c_uint = 0x825C;
    pub const GL_VIEW_CLASS_128_BITS: std::os::raw::c_uint = 0x82C4;
    pub const GL_VIEW_CLASS_16_BITS: std::os::raw::c_uint = 0x82CA;
    pub const GL_VIEW_CLASS_24_BITS: std::os::raw::c_uint = 0x82C9;
    pub const GL_VIEW_CLASS_32_BITS: std::os::raw::c_uint = 0x82C8;
    pub const GL_VIEW_CLASS_48_BITS: std::os::raw::c_uint = 0x82C7;
    pub const GL_VIEW_CLASS_64_BITS: std::os::raw::c_uint = 0x82C6;
    pub const GL_VIEW_CLASS_8_BITS: std::os::raw::c_uint = 0x82CB;
    pub const GL_VIEW_CLASS_96_BITS: std::os::raw::c_uint = 0x82C5;
    pub const GL_VIEW_CLASS_BPTC_FLOAT: std::os::raw::c_uint = 0x82D3;
    pub const GL_VIEW_CLASS_BPTC_UNORM: std::os::raw::c_uint = 0x82D2;
    pub const GL_VIEW_CLASS_RGTC1_RED: std::os::raw::c_uint = 0x82D0;
    pub const GL_VIEW_CLASS_RGTC2_RG: std::os::raw::c_uint = 0x82D1;
    pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: std::os::raw::c_uint = 0x82CC;
    pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: std::os::raw::c_uint = 0x82CD;
    pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: std::os::raw::c_uint = 0x82CE;
    pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: std::os::raw::c_uint = 0x82CF;
    pub const GL_VIEW_COMPATIBILITY_CLASS: std::os::raw::c_uint = 0x82B6;
    pub const GL_WAIT_FAILED: std::os::raw::c_uint = 0x911D;
    pub const GL_WRITE_ONLY: std::os::raw::c_uint = 0x88B9;
    pub const GL_XOR: std::os::raw::c_uint = 0x1506;
    pub const GL_ZERO: std::os::raw::c_uint = 0;
    pub const GL_ZERO_TO_ONE: std::os::raw::c_uint = 0x935F;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code)]

    use std;
    use std::mem;
    use super::storage;
    use super::types::*;

     #[inline] pub unsafe fn ActiveShaderProgram(pipeline: GLuint, program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::ActiveShaderProgram.ptr)(pipeline, program) }
     #[inline] pub unsafe fn ActiveTexture(texture: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ActiveTexture.ptr)(texture) }
     #[inline] pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::AttachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn BeginConditionalRender(id: GLuint, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::BeginConditionalRender.ptr)(id, mode) }
     #[inline] pub unsafe fn BeginQuery(target: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BeginQuery.ptr)(target, id) }
     #[inline] pub unsafe fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(storage::BeginQueryIndexed.ptr)(target, index, id) }
     #[inline] pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BeginTransformFeedback.ptr)(primitiveMode) }
     #[inline] pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindAttribLocation.ptr)(program, index, name) }
     #[inline] pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindBuffer.ptr)(target, buffer) }
     #[inline] pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(storage::BindBufferBase.ptr)(target, index, buffer) }
     #[inline] pub unsafe fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::BindBufferRange.ptr)(target, index, buffer, offset, size) }
     #[inline] pub unsafe fn BindBuffersBase(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint) -> ()>(storage::BindBuffersBase.ptr)(target, first, count, buffers) }
     #[inline] pub unsafe fn BindBuffersRange(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizeiptr) -> ()>(storage::BindBuffersRange.ptr)(target, first, count, buffers, offsets, sizes) }
     #[inline] pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindFragDataLocation.ptr)(program, color, name) }
     #[inline] pub unsafe fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, *const GLchar) -> ()>(storage::BindFragDataLocationIndexed.ptr)(program, colorNumber, index, name) }
     #[inline] pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindFramebuffer.ptr)(target, framebuffer) }
     #[inline] pub unsafe fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum) -> ()>(storage::BindImageTexture.ptr)(unit, texture, level, layered, layer, access, format) }
     #[inline] pub unsafe fn BindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(storage::BindImageTextures.ptr)(first, count, textures) }
     #[inline] pub unsafe fn BindProgramPipeline(pipeline: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::BindProgramPipeline.ptr)(pipeline) }
     #[inline] pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindRenderbuffer.ptr)(target, renderbuffer) }
     #[inline] pub unsafe fn BindSampler(unit: GLuint, sampler: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::BindSampler.ptr)(unit, sampler) }
     #[inline] pub unsafe fn BindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(storage::BindSamplers.ptr)(first, count, samplers) }
     #[inline] pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTexture.ptr)(target, texture) }
     #[inline] pub unsafe fn BindTextureUnit(unit: GLuint, texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::BindTextureUnit.ptr)(unit, texture) }
     #[inline] pub unsafe fn BindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(storage::BindTextures.ptr)(first, count, textures) }
     #[inline] pub unsafe fn BindTransformFeedback(target: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTransformFeedback.ptr)(target, id) }
     #[inline] pub unsafe fn BindVertexArray(array: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::BindVertexArray.ptr)(array) }
     #[inline] pub unsafe fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) -> ()>(storage::BindVertexBuffer.ptr)(bindingindex, buffer, offset, stride) }
     #[inline] pub unsafe fn BindVertexBuffers(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) -> ()>(storage::BindVertexBuffers.ptr)(first, count, buffers, offsets, strides) }
     #[inline] pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::BlendColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn BlendEquation(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BlendEquation.ptr)(mode) }
     #[inline] pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendEquationSeparate.ptr)(modeRGB, modeAlpha) }
     #[inline] pub unsafe fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(storage::BlendEquationSeparatei.ptr)(buf, modeRGB, modeAlpha) }
     #[inline] pub unsafe fn BlendEquationi(buf: GLuint, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::BlendEquationi.ptr)(buf, mode) }
     #[inline] pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendFunc.ptr)(sfactor, dfactor) }
     #[inline] pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::BlendFuncSeparate.ptr)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
     #[inline] pub unsafe fn BlendFuncSeparatei(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) -> ()>(storage::BlendFuncSeparatei.ptr)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
     #[inline] pub unsafe fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(storage::BlendFunci.ptr)(buf, src, dst) }
     #[inline] pub unsafe fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>(storage::BlitFramebuffer.ptr)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
     #[inline] pub unsafe fn BlitNamedFramebuffer(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>(storage::BlitNamedFramebuffer.ptr)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
     #[inline] pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const std::os::raw::c_void, usage: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const std::os::raw::c_void, GLenum) -> ()>(storage::BufferData.ptr)(target, size, data, usage) }
     #[inline] pub unsafe fn BufferStorage(target: GLenum, size: GLsizeiptr, data: *const std::os::raw::c_void, flags: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const std::os::raw::c_void, GLbitfield) -> ()>(storage::BufferStorage.ptr)(target, size, data, flags) }
     #[inline] pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const std::os::raw::c_void) -> ()>(storage::BufferSubData.ptr)(target, offset, size, data) }
     #[inline] pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum { mem::transmute::<_, extern "system" fn(GLenum) -> GLenum>(storage::CheckFramebufferStatus.ptr)(target) }
     #[inline] pub unsafe fn CheckNamedFramebufferStatus(framebuffer: GLuint, target: GLenum) -> GLenum { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> GLenum>(storage::CheckNamedFramebufferStatus.ptr)(framebuffer, target) }
     #[inline] pub unsafe fn ClampColor(target: GLenum, clamp: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::ClampColor.ptr)(target, clamp) }
     #[inline] pub unsafe fn Clear(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::Clear.ptr)(mask) }
     #[inline] pub unsafe fn ClearBufferData(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::ClearBufferData.ptr)(target, internalformat, format, type_, data) }
     #[inline] pub unsafe fn ClearBufferSubData(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::ClearBufferSubData.ptr)(target, internalformat, offset, size, format, type_, data) }
     #[inline] pub unsafe fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>(storage::ClearBufferfi.ptr)(buffer, drawbuffer, depth, stencil) }
     #[inline] pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(storage::ClearBufferfv.ptr)(buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>(storage::ClearBufferiv.ptr)(buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>(storage::ClearBufferuiv.ptr)(buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ClearDepth(depth: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::ClearDepth.ptr)(depth) }
     #[inline] pub unsafe fn ClearDepthf(d: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::ClearDepthf.ptr)(d) }
     #[inline] pub unsafe fn ClearNamedBufferData(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::ClearNamedBufferData.ptr)(buffer, internalformat, format, type_, data) }
     #[inline] pub unsafe fn ClearNamedBufferSubData(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::ClearNamedBufferSubData.ptr)(buffer, internalformat, offset, size, format, type_, data) }
     #[inline] pub unsafe fn ClearNamedFramebufferfi(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, GLfloat, GLint) -> ()>(storage::ClearNamedFramebufferfi.ptr)(framebuffer, buffer, drawbuffer, depth, stencil) }
     #[inline] pub unsafe fn ClearNamedFramebufferfv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLfloat) -> ()>(storage::ClearNamedFramebufferfv.ptr)(framebuffer, buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearNamedFramebufferiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLint) -> ()>(storage::ClearNamedFramebufferiv.ptr)(framebuffer, buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearNamedFramebufferuiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLuint) -> ()>(storage::ClearNamedFramebufferuiv.ptr)(framebuffer, buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearStencil(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ClearStencil.ptr)(s) }
     #[inline] pub unsafe fn ClearTexImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::ClearTexImage.ptr)(texture, level, format, type_, data) }
     #[inline] pub unsafe fn ClearTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::ClearTexSubImage.ptr)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data) }
     #[inline] pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum>(storage::ClientWaitSync.ptr)(sync, flags, timeout) }
     #[inline] pub unsafe fn ClipControl(origin: GLenum, depth: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::ClipControl.ptr)(origin, depth) }
     #[inline] pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMask.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMaski.ptr)(index, r, g, b, a) }
     #[inline] pub unsafe fn CompileShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CompileShader.ptr)(shader) }
     #[inline] pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage1D.ptr)(target, level, internalformat, width, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage2D.ptr)(target, level, internalformat, width, height, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage3D.ptr)(target, level, internalformat, width, height, depth, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage1D.ptr)(target, level, xoffset, width, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTextureSubImage1D.ptr)(texture, level, xoffset, width, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTextureSubImage2D.ptr)(texture, level, xoffset, yoffset, width, height, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTextureSubImage3D.ptr)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
     #[inline] pub unsafe fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> ()>(storage::CopyBufferSubData.ptr)(readTarget, writeTarget, readOffset, writeOffset, size) }
     #[inline] pub unsafe fn CopyImageSubData(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, GLint, GLint, GLint, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> ()>(storage::CopyImageSubData.ptr)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) }
     #[inline] pub unsafe fn CopyNamedBufferSubData(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr) -> ()>(storage::CopyNamedBufferSubData.ptr)(readBuffer, writeBuffer, readOffset, writeOffset, size) }
     #[inline] pub unsafe fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint) -> ()>(storage::CopyTexImage1D.ptr)(target, level, internalformat, x, y, width, border) }
     #[inline] pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> ()>(storage::CopyTexImage2D.ptr)(target, level, internalformat, x, y, width, height, border) }
     #[inline] pub unsafe fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei) -> ()>(storage::CopyTexSubImage1D.ptr)(target, level, xoffset, x, y, width) }
     #[inline] pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage2D.ptr)(target, level, xoffset, yoffset, x, y, width, height) }
     #[inline] pub unsafe fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
     #[inline] pub unsafe fn CopyTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei) -> ()>(storage::CopyTextureSubImage1D.ptr)(texture, level, xoffset, x, y, width) }
     #[inline] pub unsafe fn CopyTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTextureSubImage2D.ptr)(texture, level, xoffset, yoffset, x, y, width, height) }
     #[inline] pub unsafe fn CopyTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTextureSubImage3D.ptr)(texture, level, xoffset, yoffset, zoffset, x, y, width, height) }
     #[inline] pub unsafe fn CreateBuffers(n: GLsizei, buffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn CreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateFramebuffers.ptr)(n, framebuffers) }
     #[inline] pub unsafe fn CreateProgram() -> GLuint { mem::transmute::<_, extern "system" fn() -> GLuint>(storage::CreateProgram.ptr)() }
     #[inline] pub unsafe fn CreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateProgramPipelines.ptr)(n, pipelines) }
     #[inline] pub unsafe fn CreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(storage::CreateQueries.ptr)(target, n, ids) }
     #[inline] pub unsafe fn CreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateRenderbuffers.ptr)(n, renderbuffers) }
     #[inline] pub unsafe fn CreateSamplers(n: GLsizei, samplers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateSamplers.ptr)(n, samplers) }
     #[inline] pub unsafe fn CreateShader(type_: GLenum) -> GLuint { mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(storage::CreateShader.ptr)(type_) }
     #[inline] pub unsafe fn CreateShaderProgramv(type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint>(storage::CreateShaderProgramv.ptr)(type_, count, strings) }
     #[inline] pub unsafe fn CreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(storage::CreateTextures.ptr)(target, n, textures) }
     #[inline] pub unsafe fn CreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateTransformFeedbacks.ptr)(n, ids) }
     #[inline] pub unsafe fn CreateVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateVertexArrays.ptr)(n, arrays) }
     #[inline] pub unsafe fn CullFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::CullFace.ptr)(mode) }
     #[inline] pub unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const std::os::raw::c_void) -> ()>(storage::DebugMessageCallback.ptr)(callback, userParam) }
     #[inline] pub unsafe fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) -> ()>(storage::DebugMessageControl.ptr)(source, type_, severity, count, ids, enabled) }
     #[inline] pub unsafe fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar) -> ()>(storage::DebugMessageInsert.ptr)(source, type_, id, severity, length, buf) }
     #[inline] pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteFramebuffers.ptr)(n, framebuffers) }
     #[inline] pub unsafe fn DeleteProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteProgram.ptr)(program) }
     #[inline] pub unsafe fn DeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteProgramPipelines.ptr)(n, pipelines) }
     #[inline] pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteQueries.ptr)(n, ids) }
     #[inline] pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteRenderbuffers.ptr)(n, renderbuffers) }
     #[inline] pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteSamplers.ptr)(count, samplers) }
     #[inline] pub unsafe fn DeleteShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteShader.ptr)(shader) }
     #[inline] pub unsafe fn DeleteSync(sync: GLsync) -> () { mem::transmute::<_, extern "system" fn(GLsync) -> ()>(storage::DeleteSync.ptr)(sync) }
     #[inline] pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteTransformFeedbacks.ptr)(n, ids) }
     #[inline] pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteVertexArrays.ptr)(n, arrays) }
     #[inline] pub unsafe fn DepthFunc(func: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DepthFunc.ptr)(func) }
     #[inline] pub unsafe fn DepthMask(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::DepthMask.ptr)(flag) }
     #[inline] pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::DepthRange.ptr)(n, f) }
     #[inline] pub unsafe fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLdouble) -> ()>(storage::DepthRangeArrayv.ptr)(first, count, v) }
     #[inline] pub unsafe fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::DepthRangeIndexed.ptr)(index, n, f) }
     #[inline] pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::DepthRangef.ptr)(n, f) }
     #[inline] pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DetachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn Disable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Disable.ptr)(cap) }
     #[inline] pub unsafe fn DisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DisableVertexArrayAttrib.ptr)(vaobj, index) }
     #[inline] pub unsafe fn DisableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DisableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn Disablei(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::Disablei.ptr)(target, index) }
     #[inline] pub unsafe fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::DispatchCompute.ptr)(num_groups_x, num_groups_y, num_groups_z) }
     #[inline] pub unsafe fn DispatchComputeIndirect(indirect: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLintptr) -> ()>(storage::DispatchComputeIndirect.ptr)(indirect) }
     #[inline] pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(storage::DrawArrays.ptr)(mode, first, count) }
     #[inline] pub unsafe fn DrawArraysIndirect(mode: GLenum, indirect: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawArraysIndirect.ptr)(mode, indirect) }
     #[inline] pub unsafe fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> ()>(storage::DrawArraysInstanced.ptr)(mode, first, count, instancecount) }
     #[inline] pub unsafe fn DrawArraysInstancedBaseInstance(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint) -> ()>(storage::DrawArraysInstancedBaseInstance.ptr)(mode, first, count, instancecount, baseinstance) }
     #[inline] pub unsafe fn DrawBuffer(buf: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DrawBuffer.ptr)(buf) }
     #[inline] pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(storage::DrawBuffers.ptr)(n, bufs) }
     #[inline] pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawElements.ptr)(mode, count, type_, indices) }
     #[inline] pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLint) -> ()>(storage::DrawElementsBaseVertex.ptr)(mode, count, type_, indices, basevertex) }
     #[inline] pub unsafe fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawElementsIndirect.ptr)(mode, type_, indirect) }
     #[inline] pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLsizei) -> ()>(storage::DrawElementsInstanced.ptr)(mode, count, type_, indices, instancecount) }
     #[inline] pub unsafe fn DrawElementsInstancedBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, instancecount: GLsizei, baseinstance: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLsizei, GLuint) -> ()>(storage::DrawElementsInstancedBaseInstance.ptr)(mode, count, type_, indices, instancecount, baseinstance) }
     #[inline] pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, instancecount: GLsizei, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLsizei, GLint) -> ()>(storage::DrawElementsInstancedBaseVertex.ptr)(mode, count, type_, indices, instancecount, basevertex) }
     #[inline] pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLsizei, GLint, GLuint) -> ()>(storage::DrawElementsInstancedBaseVertexBaseInstance.ptr)(mode, count, type_, indices, instancecount, basevertex, baseinstance) }
     #[inline] pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawRangeElements.ptr)(mode, start, end, count, type_, indices) }
     #[inline] pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const std::os::raw::c_void, GLint) -> ()>(storage::DrawRangeElementsBaseVertex.ptr)(mode, start, end, count, type_, indices, basevertex) }
     #[inline] pub unsafe fn DrawTransformFeedback(mode: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::DrawTransformFeedback.ptr)(mode, id) }
     #[inline] pub unsafe fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei) -> ()>(storage::DrawTransformFeedbackInstanced.ptr)(mode, id, instancecount) }
     #[inline] pub unsafe fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(storage::DrawTransformFeedbackStream.ptr)(mode, id, stream) }
     #[inline] pub unsafe fn DrawTransformFeedbackStreamInstanced(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei) -> ()>(storage::DrawTransformFeedbackStreamInstanced.ptr)(mode, id, stream, instancecount) }
     #[inline] pub unsafe fn Enable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Enable.ptr)(cap) }
     #[inline] pub unsafe fn EnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::EnableVertexArrayAttrib.ptr)(vaobj, index) }
     #[inline] pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::EnableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn Enablei(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::Enablei.ptr)(target, index) }
     #[inline] pub unsafe fn EndConditionalRender() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndConditionalRender.ptr)() }
     #[inline] pub unsafe fn EndQuery(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::EndQuery.ptr)(target) }
     #[inline] pub unsafe fn EndQueryIndexed(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::EndQueryIndexed.ptr)(target, index) }
     #[inline] pub unsafe fn EndTransformFeedback() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndTransformFeedback.ptr)() }
     #[inline] pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync { mem::transmute::<_, extern "system" fn(GLenum, GLbitfield) -> GLsync>(storage::FenceSync.ptr)(condition, flags) }
     #[inline] pub unsafe fn Finish() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.ptr)() }
     #[inline] pub unsafe fn Flush() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.ptr)() }
     #[inline] pub unsafe fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>(storage::FlushMappedBufferRange.ptr)(target, offset, length) }
     #[inline] pub unsafe fn FlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>(storage::FlushMappedNamedBufferRange.ptr)(buffer, offset, length) }
     #[inline] pub unsafe fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::FramebufferParameteri.ptr)(target, pname, param) }
     #[inline] pub unsafe fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> ()>(storage::FramebufferRenderbuffer.ptr)(target, attachment, renderbuffertarget, renderbuffer) }
     #[inline] pub unsafe fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture.ptr)(target, attachment, texture, level) }
     #[inline] pub unsafe fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture1D.ptr)(target, attachment, textarget, texture, level) }
     #[inline] pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture2D.ptr)(target, attachment, textarget, texture, level) }
     #[inline] pub unsafe fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint) -> ()>(storage::FramebufferTexture3D.ptr)(target, attachment, textarget, texture, level, zoffset) }
     #[inline] pub unsafe fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> ()>(storage::FramebufferTextureLayer.ptr)(target, attachment, texture, level, layer) }
     #[inline] pub unsafe fn FrontFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::FrontFace.ptr)(mode) }
     #[inline] pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenFramebuffers.ptr)(n, framebuffers) }
     #[inline] pub unsafe fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenProgramPipelines.ptr)(n, pipelines) }
     #[inline] pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenQueries.ptr)(n, ids) }
     #[inline] pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenRenderbuffers.ptr)(n, renderbuffers) }
     #[inline] pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenSamplers.ptr)(count, samplers) }
     #[inline] pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenTransformFeedbacks.ptr)(n, ids) }
     #[inline] pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenVertexArrays.ptr)(n, arrays) }
     #[inline] pub unsafe fn GenerateMipmap(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::GenerateMipmap.ptr)(target) }
     #[inline] pub unsafe fn GenerateTextureMipmap(texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::GenerateTextureMipmap.ptr)(texture) }
     #[inline] pub unsafe fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveAtomicCounterBufferiv.ptr)(program, bufferIndex, pname, params) }
     #[inline] pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveAttrib.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetActiveSubroutineName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveSubroutineName.ptr)(program, shadertype, index, bufsize, length, name) }
     #[inline] pub unsafe fn GetActiveSubroutineUniformName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveSubroutineUniformName.ptr)(program, shadertype, index, bufsize, length, name) }
     #[inline] pub unsafe fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveSubroutineUniformiv.ptr)(program, shadertype, index, pname, values) }
     #[inline] pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveUniform.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveUniformBlockName.ptr)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
     #[inline] pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveUniformBlockiv.ptr)(program, uniformBlockIndex, pname, params) }
     #[inline] pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveUniformName.ptr)(program, uniformIndex, bufSize, length, uniformName) }
     #[inline] pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveUniformsiv.ptr)(program, uniformCount, uniformIndices, pname, params) }
     #[inline] pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>(storage::GetAttachedShaders.ptr)(program, maxCount, count, shaders) }
     #[inline] pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetAttribLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLboolean) -> ()>(storage::GetBooleani_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(storage::GetBooleanv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>(storage::GetBufferParameteri64v.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetBufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetBufferPointerv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut std::os::raw::c_void) -> ()>(storage::GetBufferSubData.ptr)(target, offset, size, data) }
     #[inline] pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut std::os::raw::c_void) -> ()>(storage::GetCompressedTexImage.ptr)(target, level, img) }
     #[inline] pub unsafe fn GetCompressedTextureImage(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut std::os::raw::c_void) -> ()>(storage::GetCompressedTextureImage.ptr)(texture, level, bufSize, pixels) }
     #[inline] pub unsafe fn GetCompressedTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLsizei, *mut std::os::raw::c_void) -> ()>(storage::GetCompressedTextureSubImage.ptr)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels) }
     #[inline] pub unsafe fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint>(storage::GetDebugMessageLog.ptr)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
     #[inline] pub unsafe fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLdouble) -> ()>(storage::GetDoublei_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(storage::GetDoublev.ptr)(pname, data) }
     #[inline] pub unsafe fn GetError() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetError.ptr)() }
     #[inline] pub unsafe fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(storage::GetFloati_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetFloatv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetFragDataIndex.ptr)(program, name) }
     #[inline] pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetFragDataLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) -> ()>(storage::GetFramebufferAttachmentParameteriv.ptr)(target, attachment, pname, params) }
     #[inline] pub unsafe fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetFramebufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetGraphicsResetStatus() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetGraphicsResetStatus.ptr)() }
     #[inline] pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>(storage::GetInteger64i_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>(storage::GetInteger64v.ptr)(pname, data) }
     #[inline] pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>(storage::GetIntegeri_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(storage::GetIntegerv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetInternalformati64v(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64) -> ()>(storage::GetInternalformati64v.ptr)(target, internalformat, pname, bufSize, params) }
     #[inline] pub unsafe fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) -> ()>(storage::GetInternalformativ.ptr)(target, internalformat, pname, bufSize, params) }
     #[inline] pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(storage::GetMultisamplefv.ptr)(pname, index, val) }
     #[inline] pub unsafe fn GetNamedBufferParameteri64v(buffer: GLuint, pname: GLenum, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>(storage::GetNamedBufferParameteri64v.ptr)(buffer, pname, params) }
     #[inline] pub unsafe fn GetNamedBufferParameteriv(buffer: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetNamedBufferParameteriv.ptr)(buffer, pname, params) }
     #[inline] pub unsafe fn GetNamedBufferPointerv(buffer: GLuint, pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetNamedBufferPointerv.ptr)(buffer, pname, params) }
     #[inline] pub unsafe fn GetNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut std::os::raw::c_void) -> ()>(storage::GetNamedBufferSubData.ptr)(buffer, offset, size, data) }
     #[inline] pub unsafe fn GetNamedFramebufferAttachmentParameteriv(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(storage::GetNamedFramebufferAttachmentParameteriv.ptr)(framebuffer, attachment, pname, params) }
     #[inline] pub unsafe fn GetNamedFramebufferParameteriv(framebuffer: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetNamedFramebufferParameteriv.ptr)(framebuffer, pname, param) }
     #[inline] pub unsafe fn GetNamedRenderbufferParameteriv(renderbuffer: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetNamedRenderbufferParameteriv.ptr)(renderbuffer, pname, params) }
     #[inline] pub unsafe fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetObjectLabel.ptr)(identifier, name, bufSize, length, label) }
     #[inline] pub unsafe fn GetObjectPtrLabel(ptr: *const std::os::raw::c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(*const std::os::raw::c_void, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetObjectPtrLabel.ptr)(ptr, bufSize, length, label) }
     #[inline] pub unsafe fn GetPointerv(pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetPointerv.ptr)(pname, params) }
     #[inline] pub unsafe fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut std::os::raw::c_void) -> ()>(storage::GetProgramBinary.ptr)(program, bufSize, length, binaryFormat, binary) }
     #[inline] pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramInfoLog.ptr)(program, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetProgramInterfaceiv(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(storage::GetProgramInterfaceiv.ptr)(program, programInterface, pname, params) }
     #[inline] pub unsafe fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramPipelineInfoLog.ptr)(pipeline, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetProgramPipelineiv.ptr)(pipeline, pname, params) }
     #[inline] pub unsafe fn GetProgramResourceIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>(storage::GetProgramResourceIndex.ptr)(program, programInterface, name) }
     #[inline] pub unsafe fn GetProgramResourceLocation(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(storage::GetProgramResourceLocation.ptr)(program, programInterface, name) }
     #[inline] pub unsafe fn GetProgramResourceLocationIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(storage::GetProgramResourceLocationIndex.ptr)(program, programInterface, name) }
     #[inline] pub unsafe fn GetProgramResourceName(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramResourceName.ptr)(program, programInterface, index, bufSize, length, name) }
     #[inline] pub unsafe fn GetProgramResourceiv(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *const GLenum, GLsizei, *mut GLsizei, *mut GLint) -> ()>(storage::GetProgramResourceiv.ptr)(program, programInterface, index, propCount, props, bufSize, length, params) }
     #[inline] pub unsafe fn GetProgramStageiv(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(storage::GetProgramStageiv.ptr)(program, shadertype, pname, values) }
     #[inline] pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetProgramiv.ptr)(program, pname, params) }
     #[inline] pub unsafe fn GetQueryBufferObjecti64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjecti64v.ptr)(id, buffer, pname, offset) }
     #[inline] pub unsafe fn GetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjectiv.ptr)(id, buffer, pname, offset) }
     #[inline] pub unsafe fn GetQueryBufferObjectui64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjectui64v.ptr)(id, buffer, pname, offset) }
     #[inline] pub unsafe fn GetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjectuiv.ptr)(id, buffer, pname, offset) }
     #[inline] pub unsafe fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLenum, *mut GLint) -> ()>(storage::GetQueryIndexediv.ptr)(target, index, pname, params) }
     #[inline] pub unsafe fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>(storage::GetQueryObjecti64v.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetQueryObjectiv.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint64) -> ()>(storage::GetQueryObjectui64v.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetQueryObjectuiv.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetQueryiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetRenderbufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetSamplerParameterIiv.ptr)(sampler, pname, params) }
     #[inline] pub unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetSamplerParameterIuiv.ptr)(sampler, pname, params) }
     #[inline] pub unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetSamplerParameterfv.ptr)(sampler, pname, params) }
     #[inline] pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetSamplerParameteriv.ptr)(sampler, pname, params) }
     #[inline] pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderInfoLog.ptr)(shader, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint) -> ()>(storage::GetShaderPrecisionFormat.ptr)(shadertype, precisiontype, range, precision) }
     #[inline] pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderSource.ptr)(shader, bufSize, length, source) }
     #[inline] pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetShaderiv.ptr)(shader, pname, params) }
     #[inline] pub unsafe fn GetString(name: GLenum) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(storage::GetString.ptr)(name) }
     #[inline] pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> *const GLubyte>(storage::GetStringi.ptr)(name, index) }
     #[inline] pub unsafe fn GetSubroutineIndex(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>(storage::GetSubroutineIndex.ptr)(program, shadertype, name) }
     #[inline] pub unsafe fn GetSubroutineUniformLocation(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(storage::GetSubroutineUniformLocation.ptr)(program, shadertype, name) }
     #[inline] pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> ()>(storage::GetSynciv.ptr)(sync, pname, bufSize, length, values) }
     #[inline] pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::GetTexImage.ptr)(target, level, format, type_, pixels) }
     #[inline] pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> ()>(storage::GetTexLevelParameterfv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(storage::GetTexLevelParameteriv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameterIiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>(storage::GetTexParameterIuiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTextureImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut std::os::raw::c_void) -> ()>(storage::GetTextureImage.ptr)(texture, level, format, type_, bufSize, pixels) }
     #[inline] pub unsafe fn GetTextureLevelParameterfv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat) -> ()>(storage::GetTextureLevelParameterfv.ptr)(texture, level, pname, params) }
     #[inline] pub unsafe fn GetTextureLevelParameteriv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLint) -> ()>(storage::GetTextureLevelParameteriv.ptr)(texture, level, pname, params) }
     #[inline] pub unsafe fn GetTextureParameterIiv(texture: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetTextureParameterIiv.ptr)(texture, pname, params) }
     #[inline] pub unsafe fn GetTextureParameterIuiv(texture: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetTextureParameterIuiv.ptr)(texture, pname, params) }
     #[inline] pub unsafe fn GetTextureParameterfv(texture: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetTextureParameterfv.ptr)(texture, pname, params) }
     #[inline] pub unsafe fn GetTextureParameteriv(texture: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetTextureParameteriv.ptr)(texture, pname, params) }
     #[inline] pub unsafe fn GetTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut std::os::raw::c_void) -> ()>(storage::GetTextureSubImage.ptr)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels) }
     #[inline] pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar) -> ()>(storage::GetTransformFeedbackVarying.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetTransformFeedbacki64_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64) -> ()>(storage::GetTransformFeedbacki64_v.ptr)(xfb, pname, index, param) }
     #[inline] pub unsafe fn GetTransformFeedbacki_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint) -> ()>(storage::GetTransformFeedbacki_v.ptr)(xfb, pname, index, param) }
     #[inline] pub unsafe fn GetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetTransformFeedbackiv.ptr)(xfb, pname, param) }
     #[inline] pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLuint>(storage::GetUniformBlockIndex.ptr)(program, uniformBlockName) }
     #[inline] pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) -> ()>(storage::GetUniformIndices.ptr)(program, uniformCount, uniformNames, uniformIndices) }
     #[inline] pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetUniformLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut GLuint) -> ()>(storage::GetUniformSubroutineuiv.ptr)(shadertype, location, params) }
     #[inline] pub unsafe fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLdouble) -> ()>(storage::GetUniformdv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>(storage::GetUniformfv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>(storage::GetUniformiv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>(storage::GetUniformuiv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetVertexArrayIndexed64iv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64) -> ()>(storage::GetVertexArrayIndexed64iv.ptr)(vaobj, index, pname, param) }
     #[inline] pub unsafe fn GetVertexArrayIndexediv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexArrayIndexediv.ptr)(vaobj, index, pname, param) }
     #[inline] pub unsafe fn GetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexArrayiv.ptr)(vaobj, pname, param) }
     #[inline] pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribIiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetVertexAttribIuiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(storage::GetVertexAttribLdv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetVertexAttribPointerv.ptr)(index, pname, pointer) }
     #[inline] pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(storage::GetVertexAttribdv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetVertexAttribfv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetnCompressedTexImage(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, *mut std::os::raw::c_void) -> ()>(storage::GetnCompressedTexImage.ptr)(target, lod, bufSize, pixels) }
     #[inline] pub unsafe fn GetnTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut std::os::raw::c_void) -> ()>(storage::GetnTexImage.ptr)(target, level, format, type_, bufSize, pixels) }
     #[inline] pub unsafe fn GetnUniformdv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble) -> ()>(storage::GetnUniformdv.ptr)(program, location, bufSize, params) }
     #[inline] pub unsafe fn GetnUniformfv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) -> ()>(storage::GetnUniformfv.ptr)(program, location, bufSize, params) }
     #[inline] pub unsafe fn GetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) -> ()>(storage::GetnUniformiv.ptr)(program, location, bufSize, params) }
     #[inline] pub unsafe fn GetnUniformuiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) -> ()>(storage::GetnUniformuiv.ptr)(program, location, bufSize, params) }
     #[inline] pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::Hint.ptr)(target, mode) }
     #[inline] pub unsafe fn InvalidateBufferData(buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::InvalidateBufferData.ptr)(buffer) }
     #[inline] pub unsafe fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>(storage::InvalidateBufferSubData.ptr)(buffer, offset, length) }
     #[inline] pub unsafe fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum) -> ()>(storage::InvalidateFramebuffer.ptr)(target, numAttachments, attachments) }
     #[inline] pub unsafe fn InvalidateNamedFramebufferData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>(storage::InvalidateNamedFramebufferData.ptr)(framebuffer, numAttachments, attachments) }
     #[inline] pub unsafe fn InvalidateNamedFramebufferSubData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::InvalidateNamedFramebufferSubData.ptr)(framebuffer, numAttachments, attachments, x, y, width, height) }
     #[inline] pub unsafe fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::InvalidateSubFramebuffer.ptr)(target, numAttachments, attachments, x, y, width, height) }
     #[inline] pub unsafe fn InvalidateTexImage(texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>(storage::InvalidateTexImage.ptr)(texture, level) }
     #[inline] pub unsafe fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> ()>(storage::InvalidateTexSubImage.ptr)(texture, level, xoffset, yoffset, zoffset, width, height, depth) }
     #[inline] pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsBuffer.ptr)(buffer) }
     #[inline] pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::IsEnabled.ptr)(cap) }
     #[inline] pub unsafe fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> GLboolean>(storage::IsEnabledi.ptr)(target, index) }
     #[inline] pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsFramebuffer.ptr)(framebuffer) }
     #[inline] pub unsafe fn IsProgram(program: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgram.ptr)(program) }
     #[inline] pub unsafe fn IsProgramPipeline(pipeline: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgramPipeline.ptr)(pipeline) }
     #[inline] pub unsafe fn IsQuery(id: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsQuery.ptr)(id) }
     #[inline] pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsRenderbuffer.ptr)(renderbuffer) }
     #[inline] pub unsafe fn IsSampler(sampler: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsSampler.ptr)(sampler) }
     #[inline] pub unsafe fn IsShader(shader: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsShader.ptr)(shader) }
     #[inline] pub unsafe fn IsSync(sync: GLsync) -> GLboolean { mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>(storage::IsSync.ptr)(sync) }
     #[inline] pub unsafe fn IsTexture(texture: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTexture.ptr)(texture) }
     #[inline] pub unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTransformFeedback.ptr)(id) }
     #[inline] pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsVertexArray.ptr)(array) }
     #[inline] pub unsafe fn LineWidth(width: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::LineWidth.ptr)(width) }
     #[inline] pub unsafe fn LinkProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LinkProgram.ptr)(program) }
     #[inline] pub unsafe fn LogicOp(opcode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::LogicOp.ptr)(opcode) }
     #[inline] pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut std::os::raw::c_void { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> *mut std::os::raw::c_void>(storage::MapBuffer.ptr)(target, access) }
     #[inline] pub unsafe fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut std::os::raw::c_void { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut std::os::raw::c_void>(storage::MapBufferRange.ptr)(target, offset, length, access) }
     #[inline] pub unsafe fn MapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut std::os::raw::c_void { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> *mut std::os::raw::c_void>(storage::MapNamedBuffer.ptr)(buffer, access) }
     #[inline] pub unsafe fn MapNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut std::os::raw::c_void { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield) -> *mut std::os::raw::c_void>(storage::MapNamedBufferRange.ptr)(buffer, offset, length, access) }
     #[inline] pub unsafe fn MemoryBarrier(barriers: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::MemoryBarrier.ptr)(barriers) }
     #[inline] pub unsafe fn MemoryBarrierByRegion(barriers: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::MemoryBarrierByRegion.ptr)(barriers) }
     #[inline] pub unsafe fn MinSampleShading(value: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::MinSampleShading.ptr)(value) }
     #[inline] pub unsafe fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei) -> ()>(storage::MultiDrawArrays.ptr)(mode, first, count, drawcount) }
     #[inline] pub unsafe fn MultiDrawArraysIndirect(mode: GLenum, indirect: *const std::os::raw::c_void, drawcount: GLsizei, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const std::os::raw::c_void, GLsizei, GLsizei) -> ()>(storage::MultiDrawArraysIndirect.ptr)(mode, indirect, drawcount, stride) }
     #[inline] pub unsafe fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const std::os::raw::c_void, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const std::os::raw::c_void, GLsizei) -> ()>(storage::MultiDrawElements.ptr)(mode, count, type_, indices, drawcount) }
     #[inline] pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const std::os::raw::c_void, drawcount: GLsizei, basevertex: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const std::os::raw::c_void, GLsizei, *const GLint) -> ()>(storage::MultiDrawElementsBaseVertex.ptr)(mode, count, type_, indices, drawcount, basevertex) }
     #[inline] pub unsafe fn MultiDrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const std::os::raw::c_void, drawcount: GLsizei, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const std::os::raw::c_void, GLsizei, GLsizei) -> ()>(storage::MultiDrawElementsIndirect.ptr)(mode, type_, indirect, drawcount, stride) }
     #[inline] pub unsafe fn NamedBufferData(buffer: GLuint, size: GLsizeiptr, data: *const std::os::raw::c_void, usage: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const std::os::raw::c_void, GLenum) -> ()>(storage::NamedBufferData.ptr)(buffer, size, data, usage) }
     #[inline] pub unsafe fn NamedBufferStorage(buffer: GLuint, size: GLsizeiptr, data: *const std::os::raw::c_void, flags: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const std::os::raw::c_void, GLbitfield) -> ()>(storage::NamedBufferStorage.ptr)(buffer, size, data, flags) }
     #[inline] pub unsafe fn NamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const std::os::raw::c_void) -> ()>(storage::NamedBufferSubData.ptr)(buffer, offset, size, data) }
     #[inline] pub unsafe fn NamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::NamedFramebufferDrawBuffer.ptr)(framebuffer, buf) }
     #[inline] pub unsafe fn NamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>(storage::NamedFramebufferDrawBuffers.ptr)(framebuffer, n, bufs) }
     #[inline] pub unsafe fn NamedFramebufferParameteri(framebuffer: GLuint, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::NamedFramebufferParameteri.ptr)(framebuffer, pname, param) }
     #[inline] pub unsafe fn NamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::NamedFramebufferReadBuffer.ptr)(framebuffer, src) }
     #[inline] pub unsafe fn NamedFramebufferRenderbuffer(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLuint) -> ()>(storage::NamedFramebufferRenderbuffer.ptr)(framebuffer, attachment, renderbuffertarget, renderbuffer) }
     #[inline] pub unsafe fn NamedFramebufferTexture(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint) -> ()>(storage::NamedFramebufferTexture.ptr)(framebuffer, attachment, texture, level) }
     #[inline] pub unsafe fn NamedFramebufferTextureLayer(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint) -> ()>(storage::NamedFramebufferTextureLayer.ptr)(framebuffer, attachment, texture, level, layer) }
     #[inline] pub unsafe fn NamedRenderbufferStorage(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLsizei, GLsizei) -> ()>(storage::NamedRenderbufferStorage.ptr)(renderbuffer, internalformat, width, height) }
     #[inline] pub unsafe fn NamedRenderbufferStorageMultisample(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::NamedRenderbufferStorageMultisample.ptr)(renderbuffer, samples, internalformat, width, height) }
     #[inline] pub unsafe fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(storage::ObjectLabel.ptr)(identifier, name, length, label) }
     #[inline] pub unsafe fn ObjectPtrLabel(ptr: *const std::os::raw::c_void, length: GLsizei, label: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(*const std::os::raw::c_void, GLsizei, *const GLchar) -> ()>(storage::ObjectPtrLabel.ptr)(ptr, length, label) }
     #[inline] pub unsafe fn PatchParameterfv(pname: GLenum, values: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::PatchParameterfv.ptr)(pname, values) }
     #[inline] pub unsafe fn PatchParameteri(pname: GLenum, value: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PatchParameteri.ptr)(pname, value) }
     #[inline] pub unsafe fn PauseTransformFeedback() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PauseTransformFeedback.ptr)() }
     #[inline] pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PixelStoref.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelStorei.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PointParameterf.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::PointParameterfv.ptr)(pname, params) }
     #[inline] pub unsafe fn PointParameteri(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PointParameteri.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::PointParameteriv.ptr)(pname, params) }
     #[inline] pub unsafe fn PointSize(size: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::PointSize.ptr)(size) }
     #[inline] pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::PolygonMode.ptr)(face, mode) }
     #[inline] pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PolygonOffset.ptr)(factor, units) }
     #[inline] pub unsafe fn PopDebugGroup() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopDebugGroup.ptr)() }
     #[inline] pub unsafe fn PrimitiveRestartIndex(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::PrimitiveRestartIndex.ptr)(index) }
     #[inline] pub unsafe fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const std::os::raw::c_void, length: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const std::os::raw::c_void, GLsizei) -> ()>(storage::ProgramBinary.ptr)(program, binaryFormat, binary, length) }
     #[inline] pub unsafe fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::ProgramParameteri.ptr)(program, pname, value) }
     #[inline] pub unsafe fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble) -> ()>(storage::ProgramUniform1d.ptr)(program, location, v0) }
     #[inline] pub unsafe fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform1dv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat) -> ()>(storage::ProgramUniform1f.ptr)(program, location, v0) }
     #[inline] pub unsafe fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform1fv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(storage::ProgramUniform1i.ptr)(program, location, v0) }
     #[inline] pub unsafe fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform1iv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint) -> ()>(storage::ProgramUniform1ui.ptr)(program, location, v0) }
     #[inline] pub unsafe fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform1uiv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble) -> ()>(storage::ProgramUniform2d.ptr)(program, location, v0, v1) }
     #[inline] pub unsafe fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform2dv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat) -> ()>(storage::ProgramUniform2f.ptr)(program, location, v0, v1) }
     #[inline] pub unsafe fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform2fv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(storage::ProgramUniform2i.ptr)(program, location, v0, v1) }
     #[inline] pub unsafe fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform2iv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint) -> ()>(storage::ProgramUniform2ui.ptr)(program, location, v0, v1) }
     #[inline] pub unsafe fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform2uiv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble) -> ()>(storage::ProgramUniform3d.ptr)(program, location, v0, v1, v2) }
     #[inline] pub unsafe fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform3dv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) -> ()>(storage::ProgramUniform3f.ptr)(program, location, v0, v1, v2) }
     #[inline] pub unsafe fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform3fv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(storage::ProgramUniform3i.ptr)(program, location, v0, v1, v2) }
     #[inline] pub unsafe fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform3iv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) -> ()>(storage::ProgramUniform3ui.ptr)(program, location, v0, v1, v2) }
     #[inline] pub unsafe fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform3uiv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::ProgramUniform4d.ptr)(program, location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform4dv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ProgramUniform4f.ptr)(program, location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform4fv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) -> ()>(storage::ProgramUniform4i.ptr)(program, location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform4iv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::ProgramUniform4ui.ptr)(program, location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform4uiv.ptr)(program, location, count, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix2dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix2fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix2x3dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix2x3fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix2x4dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix2x4fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix3dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix3fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix3x2dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix3x2fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix3x4dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix3x4fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix4dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix4fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix4x2dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix4x2fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix4x3dv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix4x3fv.ptr)(program, location, count, transpose, value) }
     #[inline] pub unsafe fn ProvokingVertex(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ProvokingVertex.ptr)(mode) }
     #[inline] pub unsafe fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(storage::PushDebugGroup.ptr)(source, id, length, message) }
     #[inline] pub unsafe fn QueryCounter(id: GLuint, target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::QueryCounter.ptr)(id, target) }
     #[inline] pub unsafe fn ReadBuffer(src: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ReadBuffer.ptr)(src) }
     #[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::ReadPixels.ptr)(x, y, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn ReadnPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut std::os::raw::c_void) -> ()>(storage::ReadnPixels.ptr)(x, y, width, height, format, type_, bufSize, data) }
     #[inline] pub unsafe fn ReleaseShaderCompiler() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::ReleaseShaderCompiler.ptr)() }
     #[inline] pub unsafe fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) -> ()>(storage::RenderbufferStorage.ptr)(target, internalformat, width, height) }
     #[inline] pub unsafe fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::RenderbufferStorageMultisample.ptr)(target, samples, internalformat, width, height) }
     #[inline] pub unsafe fn ResumeTransformFeedback() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::ResumeTransformFeedback.ptr)() }
     #[inline] pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>(storage::SampleCoverage.ptr)(value, invert) }
     #[inline] pub unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>(storage::SampleMaski.ptr)(maskNumber, mask) }
     #[inline] pub unsafe fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::SamplerParameterIiv.ptr)(sampler, pname, param) }
     #[inline] pub unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(storage::SamplerParameterIuiv.ptr)(sampler, pname, param) }
     #[inline] pub unsafe fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(storage::SamplerParameterf.ptr)(sampler, pname, param) }
     #[inline] pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(storage::SamplerParameterfv.ptr)(sampler, pname, param) }
     #[inline] pub unsafe fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::SamplerParameteri.ptr)(sampler, pname, param) }
     #[inline] pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::SamplerParameteriv.ptr)(sampler, pname, param) }
     #[inline] pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Scissor.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn ScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLint) -> ()>(storage::ScissorArrayv.ptr)(first, count, v) }
     #[inline] pub unsafe fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::ScissorIndexed.ptr)(index, left, bottom, width, height) }
     #[inline] pub unsafe fn ScissorIndexedv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::ScissorIndexedv.ptr)(index, v) }
     #[inline] pub unsafe fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const std::os::raw::c_void, length: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint, GLenum, *const std::os::raw::c_void, GLsizei) -> ()>(storage::ShaderBinary.ptr)(count, shaders, binaryformat, binary, length) }
     #[inline] pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>(storage::ShaderSource.ptr)(shader, count, string, length) }
     #[inline] pub unsafe fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::ShaderStorageBlockBinding.ptr)(program, storageBlockIndex, storageBlockBinding) }
     #[inline] pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(storage::StencilFunc.ptr)(func, ref_, mask) }
     #[inline] pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>(storage::StencilFuncSeparate.ptr)(face, func, ref_, mask) }
     #[inline] pub unsafe fn StencilMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::StencilMask.ptr)(mask) }
     #[inline] pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::StencilMaskSeparate.ptr)(face, mask) }
     #[inline] pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(storage::StencilOp.ptr)(fail, zfail, zpass) }
     #[inline] pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::StencilOpSeparate.ptr)(face, sfail, dpfail, dppass) }
     #[inline] pub unsafe fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(storage::TexBuffer.ptr)(target, internalformat, buffer) }
     #[inline] pub unsafe fn TexBufferRange(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::TexBufferRange.ptr)(target, internalformat, buffer, offset, size) }
     #[inline] pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage1D.ptr)(target, level, internalformat, width, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage2D.ptr)(target, level, internalformat, width, height, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexImage2DMultisample.ptr)(target, samples, internalformat, width, height, fixedsamplelocations) }
     #[inline] pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage3D.ptr)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexImage3DMultisample.ptr)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
     #[inline] pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameterIiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(storage::TexParameterIuiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexParameterf.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexParameteri.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexStorage1D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei) -> ()>(storage::TexStorage1D.ptr)(target, levels, internalformat, width) }
     #[inline] pub unsafe fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::TexStorage2D.ptr)(target, levels, internalformat, width, height) }
     #[inline] pub unsafe fn TexStorage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexStorage2DMultisample.ptr)(target, samples, internalformat, width, height, fixedsamplelocations) }
     #[inline] pub unsafe fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> ()>(storage::TexStorage3D.ptr)(target, levels, internalformat, width, height, depth) }
     #[inline] pub unsafe fn TexStorage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexStorage3DMultisample.ptr)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
     #[inline] pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage1D.ptr)(target, level, xoffset, width, format, type_, pixels) }
     #[inline] pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
     #[inline] pub unsafe fn TextureBarrier() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::TextureBarrier.ptr)() }
     #[inline] pub unsafe fn TextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint) -> ()>(storage::TextureBuffer.ptr)(texture, internalformat, buffer) }
     #[inline] pub unsafe fn TextureBufferRange(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::TextureBufferRange.ptr)(texture, internalformat, buffer, offset, size) }
     #[inline] pub unsafe fn TextureParameterIiv(texture: GLuint, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::TextureParameterIiv.ptr)(texture, pname, params) }
     #[inline] pub unsafe fn TextureParameterIuiv(texture: GLuint, pname: GLenum, params: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(storage::TextureParameterIuiv.ptr)(texture, pname, params) }
     #[inline] pub unsafe fn TextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(storage::TextureParameterf.ptr)(texture, pname, param) }
     #[inline] pub unsafe fn TextureParameterfv(texture: GLuint, pname: GLenum, param: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(storage::TextureParameterfv.ptr)(texture, pname, param) }
     #[inline] pub unsafe fn TextureParameteri(texture: GLuint, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::TextureParameteri.ptr)(texture, pname, param) }
     #[inline] pub unsafe fn TextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::TextureParameteriv.ptr)(texture, pname, param) }
     #[inline] pub unsafe fn TextureStorage1D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei) -> ()>(storage::TextureStorage1D.ptr)(texture, levels, internalformat, width) }
     #[inline] pub unsafe fn TextureStorage2D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::TextureStorage2D.ptr)(texture, levels, internalformat, width, height) }
     #[inline] pub unsafe fn TextureStorage2DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>(storage::TextureStorage2DMultisample.ptr)(texture, samples, internalformat, width, height, fixedsamplelocations) }
     #[inline] pub unsafe fn TextureStorage3D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> ()>(storage::TextureStorage3D.ptr)(texture, levels, internalformat, width, height, depth) }
     #[inline] pub unsafe fn TextureStorage3DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>(storage::TextureStorage3DMultisample.ptr)(texture, samples, internalformat, width, height, depth, fixedsamplelocations) }
     #[inline] pub unsafe fn TextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TextureSubImage1D.ptr)(texture, level, xoffset, width, format, type_, pixels) }
     #[inline] pub unsafe fn TextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TextureSubImage2D.ptr)(texture, level, xoffset, yoffset, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn TextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TextureSubImage3D.ptr)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
     #[inline] pub unsafe fn TextureView(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::TextureView.ptr)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) }
     #[inline] pub unsafe fn TransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::TransformFeedbackBufferBase.ptr)(xfb, index, buffer) }
     #[inline] pub unsafe fn TransformFeedbackBufferRange(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::TransformFeedbackBufferRange.ptr)(xfb, index, buffer, offset, size) }
     #[inline] pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> ()>(storage::TransformFeedbackVaryings.ptr)(program, count, varyings, bufferMode) }
     #[inline] pub unsafe fn Uniform1d(location: GLint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble) -> ()>(storage::Uniform1d.ptr)(location, x) }
     #[inline] pub unsafe fn Uniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform1dv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(storage::Uniform1f.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform1fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform1i(location: GLint, v0: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::Uniform1i.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform1iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>(storage::Uniform1ui.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform1uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble) -> ()>(storage::Uniform2d.ptr)(location, x, y) }
     #[inline] pub unsafe fn Uniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform2dv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(storage::Uniform2f.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform2fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Uniform2i.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform2iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>(storage::Uniform2ui.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform2uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble) -> ()>(storage::Uniform3d.ptr)(location, x, y, z) }
     #[inline] pub unsafe fn Uniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform3dv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform3f.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform3fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Uniform3i.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform3iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>(storage::Uniform3ui.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform3uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Uniform4d.ptr)(location, x, y, z, w) }
     #[inline] pub unsafe fn Uniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform4dv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform4f.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform4fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint) -> ()>(storage::Uniform4i.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform4iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::Uniform4ui.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform4uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::UniformBlockBinding.ptr)(program, uniformBlockIndex, uniformBlockBinding) }
     #[inline] pub unsafe fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix2dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix2x3dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2x3fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix2x4dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2x4fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix3dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix3x2dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3x2fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix3x4dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3x4fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix4dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix4x2dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4x2fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix4x3dv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4x3fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLuint) -> ()>(storage::UniformSubroutinesuiv.ptr)(shadertype, count, indices) }
     #[inline] pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::UnmapBuffer.ptr)(target) }
     #[inline] pub unsafe fn UnmapNamedBuffer(buffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::UnmapNamedBuffer.ptr)(buffer) }
     #[inline] pub unsafe fn UseProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::UseProgram.ptr)(program) }
     #[inline] pub unsafe fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLbitfield, GLuint) -> ()>(storage::UseProgramStages.ptr)(pipeline, stages, program) }
     #[inline] pub unsafe fn ValidateProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgram.ptr)(program) }
     #[inline] pub unsafe fn ValidateProgramPipeline(pipeline: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgramPipeline.ptr)(pipeline) }
     #[inline] pub unsafe fn VertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::VertexArrayAttribBinding.ptr)(vaobj, attribindex, bindingindex) }
     #[inline] pub unsafe fn VertexArrayAttribFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexArrayAttribFormat.ptr)(vaobj, attribindex, size, type_, normalized, relativeoffset) }
     #[inline] pub unsafe fn VertexArrayAttribIFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexArrayAttribIFormat.ptr)(vaobj, attribindex, size, type_, relativeoffset) }
     #[inline] pub unsafe fn VertexArrayAttribLFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexArrayAttribLFormat.ptr)(vaobj, attribindex, size, type_, relativeoffset) }
     #[inline] pub unsafe fn VertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::VertexArrayBindingDivisor.ptr)(vaobj, bindingindex, divisor) }
     #[inline] pub unsafe fn VertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexArrayElementBuffer.ptr)(vaobj, buffer) }
     #[inline] pub unsafe fn VertexArrayVertexBuffer(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei) -> ()>(storage::VertexArrayVertexBuffer.ptr)(vaobj, bindingindex, buffer, offset, stride) }
     #[inline] pub unsafe fn VertexArrayVertexBuffers(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) -> ()>(storage::VertexArrayVertexBuffers.ptr)(vaobj, first, count, buffers, offsets, strides) }
     #[inline] pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(storage::VertexAttrib1d.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib1dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>(storage::VertexAttrib1f.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib1fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort) -> ()>(storage::VertexAttrib1s.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib1sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::VertexAttrib2d.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib2dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>(storage::VertexAttrib2f.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib2fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort) -> ()>(storage::VertexAttrib2s.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib2sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib3d.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib3dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib3f.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib3fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib3s.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib3sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4Nbv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4Niv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4Nsv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte) -> ()>(storage::VertexAttrib4Nub.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4Nubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4Nuiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4Nusv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4bv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib4d.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib4dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib4f.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib4fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib4s.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4ubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4usv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexAttribBinding.ptr)(attribindex, bindingindex) }
     #[inline] pub unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexAttribDivisor.ptr)(index, divisor) }
     #[inline] pub unsafe fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribFormat.ptr)(attribindex, size, type_, normalized, relativeoffset) }
     #[inline] pub unsafe fn VertexAttribI1i(index: GLuint, x: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>(storage::VertexAttribI1i.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI1iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexAttribI1ui.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI1uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(storage::VertexAttribI2i.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttribI2iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI2iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI2ui.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI2uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(storage::VertexAttribI3i.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttribI3iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI3iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI3ui.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI3uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttribI4bv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(storage::VertexAttribI4i.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI4iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttribI4sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttribI4ubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI4ui.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI4uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4usv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttribI4usv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexAttribIFormat.ptr)(attribindex, size, type_, relativeoffset) }
     #[inline] pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexAttribIPointer.ptr)(index, size, type_, stride, pointer) }
     #[inline] pub unsafe fn VertexAttribL1d(index: GLuint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(storage::VertexAttribL1d.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttribL1dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL1dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::VertexAttribL2d.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttribL2dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL2dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttribL3d.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttribL3dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL3dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttribL4d.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttribL4dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL4dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexAttribLFormat.ptr)(attribindex, size, type_, relativeoffset) }
     #[inline] pub unsafe fn VertexAttribLPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexAttribLPointer.ptr)(index, size, type_, stride, pointer) }
     #[inline] pub unsafe fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP1ui.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP1uiv.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP2ui.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP2uiv.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP3ui.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP3uiv.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP4ui.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP4uiv.ptr)(index, type_, normalized, value) }
     #[inline] pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexAttribPointer.ptr)(index, size, type_, normalized, stride, pointer) }
     #[inline] pub unsafe fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexBindingDivisor.ptr)(bindingindex, divisor) }
     #[inline] pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Viewport.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn ViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLfloat) -> ()>(storage::ViewportArrayv.ptr)(first, count, v) }
     #[inline] pub unsafe fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ViewportIndexedf.ptr)(index, x, y, w, h) }
     #[inline] pub unsafe fn ViewportIndexedfv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::ViewportIndexedfv.ptr)(index, v) }
     #[inline] pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> () { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>(storage::WaitSync.ptr)(sync, flags, timeout) }
}

mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw;

     pub static mut ActiveShaderProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ActiveTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut AttachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginConditionalRender: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginQueryIndexed: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBufferBase: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBuffersBase: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBuffersRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindFragDataLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindFragDataLocationIndexed: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindImageTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindImageTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindProgramPipeline: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindSampler: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindSamplers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindTextureUnit: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindVertexArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindVertexBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindVertexBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquationSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquationSeparatei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquationi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFuncSeparatei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFunci: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlitFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlitNamedFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferStorage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CheckFramebufferStatus: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CheckNamedFramebufferStatus: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClampColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Clear: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferfi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearDepth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearDepthf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearNamedBufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearNamedBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearNamedFramebufferfi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearNamedFramebufferfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearNamedFramebufferiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearNamedFramebufferuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearStencil: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearTexSubImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClientWaitSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClipControl: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMaski: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompileShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTextureSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTextureSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTextureSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyImageSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyNamedBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTextureSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTextureSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTextureSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateFramebuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateProgramPipelines: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateQueries: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateRenderbuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateSamplers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateShaderProgramv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateTransformFeedbacks: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateVertexArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CullFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DebugMessageCallback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DebugMessageControl: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DebugMessageInsert: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteFramebuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteProgramPipelines: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteQueries: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteRenderbuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteSamplers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteTransformFeedbacks: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteVertexArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRangeArrayv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRangeIndexed: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRangef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DetachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Disable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DisableVertexArrayAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DisableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Disablei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DispatchCompute: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DispatchComputeIndirect: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArraysIndirect: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArraysInstanced: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArraysInstancedBaseInstance: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsIndirect: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsInstanced: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsInstancedBaseInstance: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsInstancedBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsInstancedBaseVertexBaseInstance: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawRangeElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawRangeElementsBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawTransformFeedbackInstanced: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawTransformFeedbackStream: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawTransformFeedbackStreamInstanced: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Enable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EnableVertexArrayAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EnableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Enablei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndConditionalRender: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndQueryIndexed: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FenceSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Finish: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Flush: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FlushMappedBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FlushMappedNamedBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTextureLayer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FrontFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenFramebuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenProgramPipelines: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenQueries: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenRenderbuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenSamplers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenTransformFeedbacks: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenVertexArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenerateMipmap: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenerateTextureMipmap: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveAtomicCounterBufferiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveSubroutineName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveSubroutineUniformName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveSubroutineUniformiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniform: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformBlockName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformBlockiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformsiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttachedShaders: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBooleani_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBooleanv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferParameteri64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetCompressedTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetCompressedTextureImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetCompressedTextureSubImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetDebugMessageLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetDoublei_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetDoublev: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetError: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFloati_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFloatv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFragDataIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFragDataLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFramebufferAttachmentParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFramebufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetGraphicsResetStatus: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetInteger64i_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetInteger64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetIntegeri_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetIntegerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetInternalformati64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetInternalformativ: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMultisamplefv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetNamedBufferParameteri64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetNamedBufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetNamedBufferPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetNamedBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetNamedFramebufferAttachmentParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetNamedFramebufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetNamedRenderbufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetObjectLabel: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetObjectPtrLabel: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramBinary: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramInterfaceiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramPipelineInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramPipelineiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramResourceIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramResourceLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramResourceLocationIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramResourceName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramResourceiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramStageiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryBufferObjecti64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryBufferObjectiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryBufferObjectui64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryBufferObjectuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryIndexediv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjecti64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjectiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjectui64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjectuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetRenderbufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSamplerParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSamplerParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSamplerParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSamplerParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderPrecisionFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetString: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetStringi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSubroutineIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSubroutineUniformLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSynciv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureLevelParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureLevelParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTextureSubImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTransformFeedbackVarying: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTransformFeedbacki64_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTransformFeedbacki_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTransformFeedbackiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformBlockIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformIndices: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformSubroutineuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexArrayIndexed64iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexArrayIndexediv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexArrayiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribLdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetnCompressedTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetnTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetnUniformdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetnUniformfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetnUniformiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetnUniformuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Hint: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateBufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateNamedFramebufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateNamedFramebufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateSubFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InvalidateTexSubImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsEnabled: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsEnabledi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsProgramPipeline: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsSampler: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsVertexArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LineWidth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LinkProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LogicOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapNamedBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapNamedBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MemoryBarrier: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MemoryBarrierByRegion: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MinSampleShading: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawArraysIndirect: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawElementsBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawElementsIndirect: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedBufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedBufferStorage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedFramebufferDrawBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedFramebufferDrawBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedFramebufferParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedFramebufferReadBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedFramebufferRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedFramebufferTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedFramebufferTextureLayer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedRenderbufferStorage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NamedRenderbufferStorageMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ObjectLabel: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ObjectPtrLabel: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PatchParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PatchParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PauseTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStoref: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStorei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointSize: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonOffset: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopDebugGroup: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PrimitiveRestartIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramBinary: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform1uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform2uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniform4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix2x3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix2x3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix2x4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix2x4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix3x2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix3x2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix3x4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix3x4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix4x2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix4x2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix4x3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProgramUniformMatrix4x3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProvokingVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushDebugGroup: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut QueryCounter: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadnPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReleaseShaderCompiler: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RenderbufferStorage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RenderbufferStorageMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ResumeTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SampleCoverage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SampleMaski: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SamplerParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SamplerParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SamplerParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SamplerParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SamplerParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SamplerParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scissor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ScissorArrayv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ScissorIndexed: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ScissorIndexedv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShaderBinary: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShaderStorageBlockBinding: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMaskSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOpSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage2DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage3DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexStorage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexStorage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexStorage2DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexStorage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexStorage3DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureBarrier: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureStorage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureStorage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureStorage2DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureStorage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureStorage3DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TextureView: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TransformFeedbackBufferBase: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TransformFeedbackBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TransformFeedbackVaryings: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformBlockBinding: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2x3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2x3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2x4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2x4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3x2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3x2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3x4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3x4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4x2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4x2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4x3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4x3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformSubroutinesuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UnmapBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UnmapNamedBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UseProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UseProgramStages: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ValidateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ValidateProgramPipeline: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayAttribBinding: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayAttribFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayAttribIFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayAttribLFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayBindingDivisor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayElementBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayVertexBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexArrayVertexBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nbv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Niv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nsv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nusv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribBinding: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribDivisor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribIFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribIPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribL4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribLFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribLPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP1ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP1uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP2ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP2uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribP4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexBindingDivisor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Viewport: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ViewportArrayv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ViewportIndexedf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ViewportIndexedfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WaitSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
}

pub fn load<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
    unsafe {
         storage::ActiveShaderProgram.load(&mut loadfn, "glActiveShaderProgram");
         storage::ActiveTexture.load(&mut loadfn, "glActiveTexture");
         storage::AttachShader.load(&mut loadfn, "glAttachShader");
         storage::BeginConditionalRender.load(&mut loadfn, "glBeginConditionalRender");
         storage::BeginQuery.load(&mut loadfn, "glBeginQuery");
         storage::BeginQueryIndexed.load(&mut loadfn, "glBeginQueryIndexed");
         storage::BeginTransformFeedback.load(&mut loadfn, "glBeginTransformFeedback");
         storage::BindAttribLocation.load(&mut loadfn, "glBindAttribLocation");
         storage::BindBuffer.load(&mut loadfn, "glBindBuffer");
         storage::BindBufferBase.load(&mut loadfn, "glBindBufferBase");
         storage::BindBufferRange.load(&mut loadfn, "glBindBufferRange");
         storage::BindBuffersBase.load(&mut loadfn, "glBindBuffersBase");
         storage::BindBuffersRange.load(&mut loadfn, "glBindBuffersRange");
         storage::BindFragDataLocation.load(&mut loadfn, "glBindFragDataLocation");
         storage::BindFragDataLocationIndexed.load(&mut loadfn, "glBindFragDataLocationIndexed");
         storage::BindFramebuffer.load(&mut loadfn, "glBindFramebuffer");
         storage::BindImageTexture.load(&mut loadfn, "glBindImageTexture");
         storage::BindImageTextures.load(&mut loadfn, "glBindImageTextures");
         storage::BindProgramPipeline.load(&mut loadfn, "glBindProgramPipeline");
         storage::BindRenderbuffer.load(&mut loadfn, "glBindRenderbuffer");
         storage::BindSampler.load(&mut loadfn, "glBindSampler");
         storage::BindSamplers.load(&mut loadfn, "glBindSamplers");
         storage::BindTexture.load(&mut loadfn, "glBindTexture");
         storage::BindTextureUnit.load(&mut loadfn, "glBindTextureUnit");
         storage::BindTextures.load(&mut loadfn, "glBindTextures");
         storage::BindTransformFeedback.load(&mut loadfn, "glBindTransformFeedback");
         storage::BindVertexArray.load(&mut loadfn, "glBindVertexArray");
         storage::BindVertexBuffer.load(&mut loadfn, "glBindVertexBuffer");
         storage::BindVertexBuffers.load(&mut loadfn, "glBindVertexBuffers");
         storage::BlendColor.load(&mut loadfn, "glBlendColor");
         storage::BlendEquation.load(&mut loadfn, "glBlendEquation");
         storage::BlendEquationSeparate.load(&mut loadfn, "glBlendEquationSeparate");
         storage::BlendEquationSeparatei.load(&mut loadfn, "glBlendEquationSeparatei");
         storage::BlendEquationi.load(&mut loadfn, "glBlendEquationi");
         storage::BlendFunc.load(&mut loadfn, "glBlendFunc");
         storage::BlendFuncSeparate.load(&mut loadfn, "glBlendFuncSeparate");
         storage::BlendFuncSeparatei.load(&mut loadfn, "glBlendFuncSeparatei");
         storage::BlendFunci.load(&mut loadfn, "glBlendFunci");
         storage::BlitFramebuffer.load(&mut loadfn, "glBlitFramebuffer");
         storage::BlitNamedFramebuffer.load(&mut loadfn, "glBlitNamedFramebuffer");
         storage::BufferData.load(&mut loadfn, "glBufferData");
         storage::BufferStorage.load(&mut loadfn, "glBufferStorage");
         storage::BufferSubData.load(&mut loadfn, "glBufferSubData");
         storage::CheckFramebufferStatus.load(&mut loadfn, "glCheckFramebufferStatus");
         storage::CheckNamedFramebufferStatus.load(&mut loadfn, "glCheckNamedFramebufferStatus");
         storage::ClampColor.load(&mut loadfn, "glClampColor");
         storage::Clear.load(&mut loadfn, "glClear");
         storage::ClearBufferData.load(&mut loadfn, "glClearBufferData");
         storage::ClearBufferSubData.load(&mut loadfn, "glClearBufferSubData");
         storage::ClearBufferfi.load(&mut loadfn, "glClearBufferfi");
         storage::ClearBufferfv.load(&mut loadfn, "glClearBufferfv");
         storage::ClearBufferiv.load(&mut loadfn, "glClearBufferiv");
         storage::ClearBufferuiv.load(&mut loadfn, "glClearBufferuiv");
         storage::ClearColor.load(&mut loadfn, "glClearColor");
         storage::ClearDepth.load(&mut loadfn, "glClearDepth");
         storage::ClearDepthf.load(&mut loadfn, "glClearDepthf");
         storage::ClearNamedBufferData.load(&mut loadfn, "glClearNamedBufferData");
         storage::ClearNamedBufferSubData.load(&mut loadfn, "glClearNamedBufferSubData");
         storage::ClearNamedFramebufferfi.load(&mut loadfn, "glClearNamedFramebufferfi");
         storage::ClearNamedFramebufferfv.load(&mut loadfn, "glClearNamedFramebufferfv");
         storage::ClearNamedFramebufferiv.load(&mut loadfn, "glClearNamedFramebufferiv");
         storage::ClearNamedFramebufferuiv.load(&mut loadfn, "glClearNamedFramebufferuiv");
         storage::ClearStencil.load(&mut loadfn, "glClearStencil");
         storage::ClearTexImage.load(&mut loadfn, "glClearTexImage");
         storage::ClearTexSubImage.load(&mut loadfn, "glClearTexSubImage");
         storage::ClientWaitSync.load(&mut loadfn, "glClientWaitSync");
         storage::ClipControl.load(&mut loadfn, "glClipControl");
         storage::ColorMask.load(&mut loadfn, "glColorMask");
         storage::ColorMaski.load(&mut loadfn, "glColorMaski");
         storage::CompileShader.load(&mut loadfn, "glCompileShader");
         storage::CompressedTexImage1D.load(&mut loadfn, "glCompressedTexImage1D");
         storage::CompressedTexImage2D.load(&mut loadfn, "glCompressedTexImage2D");
         storage::CompressedTexImage3D.load(&mut loadfn, "glCompressedTexImage3D");
         storage::CompressedTexSubImage1D.load(&mut loadfn, "glCompressedTexSubImage1D");
         storage::CompressedTexSubImage2D.load(&mut loadfn, "glCompressedTexSubImage2D");
         storage::CompressedTexSubImage3D.load(&mut loadfn, "glCompressedTexSubImage3D");
         storage::CompressedTextureSubImage1D.load(&mut loadfn, "glCompressedTextureSubImage1D");
         storage::CompressedTextureSubImage2D.load(&mut loadfn, "glCompressedTextureSubImage2D");
         storage::CompressedTextureSubImage3D.load(&mut loadfn, "glCompressedTextureSubImage3D");
         storage::CopyBufferSubData.load(&mut loadfn, "glCopyBufferSubData");
         storage::CopyImageSubData.load(&mut loadfn, "glCopyImageSubData");
         storage::CopyNamedBufferSubData.load(&mut loadfn, "glCopyNamedBufferSubData");
         storage::CopyTexImage1D.load(&mut loadfn, "glCopyTexImage1D");
         storage::CopyTexImage2D.load(&mut loadfn, "glCopyTexImage2D");
         storage::CopyTexSubImage1D.load(&mut loadfn, "glCopyTexSubImage1D");
         storage::CopyTexSubImage2D.load(&mut loadfn, "glCopyTexSubImage2D");
         storage::CopyTexSubImage3D.load(&mut loadfn, "glCopyTexSubImage3D");
         storage::CopyTextureSubImage1D.load(&mut loadfn, "glCopyTextureSubImage1D");
         storage::CopyTextureSubImage2D.load(&mut loadfn, "glCopyTextureSubImage2D");
         storage::CopyTextureSubImage3D.load(&mut loadfn, "glCopyTextureSubImage3D");
         storage::CreateBuffers.load(&mut loadfn, "glCreateBuffers");
         storage::CreateFramebuffers.load(&mut loadfn, "glCreateFramebuffers");
         storage::CreateProgram.load(&mut loadfn, "glCreateProgram");
         storage::CreateProgramPipelines.load(&mut loadfn, "glCreateProgramPipelines");
         storage::CreateQueries.load(&mut loadfn, "glCreateQueries");
         storage::CreateRenderbuffers.load(&mut loadfn, "glCreateRenderbuffers");
         storage::CreateSamplers.load(&mut loadfn, "glCreateSamplers");
         storage::CreateShader.load(&mut loadfn, "glCreateShader");
         storage::CreateShaderProgramv.load(&mut loadfn, "glCreateShaderProgramv");
         storage::CreateTextures.load(&mut loadfn, "glCreateTextures");
         storage::CreateTransformFeedbacks.load(&mut loadfn, "glCreateTransformFeedbacks");
         storage::CreateVertexArrays.load(&mut loadfn, "glCreateVertexArrays");
         storage::CullFace.load(&mut loadfn, "glCullFace");
         storage::DebugMessageCallback.load(&mut loadfn, "glDebugMessageCallback");
         storage::DebugMessageControl.load(&mut loadfn, "glDebugMessageControl");
         storage::DebugMessageInsert.load(&mut loadfn, "glDebugMessageInsert");
         storage::DeleteBuffers.load(&mut loadfn, "glDeleteBuffers");
         storage::DeleteFramebuffers.load(&mut loadfn, "glDeleteFramebuffers");
         storage::DeleteProgram.load(&mut loadfn, "glDeleteProgram");
         storage::DeleteProgramPipelines.load(&mut loadfn, "glDeleteProgramPipelines");
         storage::DeleteQueries.load(&mut loadfn, "glDeleteQueries");
         storage::DeleteRenderbuffers.load(&mut loadfn, "glDeleteRenderbuffers");
         storage::DeleteSamplers.load(&mut loadfn, "glDeleteSamplers");
         storage::DeleteShader.load(&mut loadfn, "glDeleteShader");
         storage::DeleteSync.load(&mut loadfn, "glDeleteSync");
         storage::DeleteTextures.load(&mut loadfn, "glDeleteTextures");
         storage::DeleteTransformFeedbacks.load(&mut loadfn, "glDeleteTransformFeedbacks");
         storage::DeleteVertexArrays.load(&mut loadfn, "glDeleteVertexArrays");
         storage::DepthFunc.load(&mut loadfn, "glDepthFunc");
         storage::DepthMask.load(&mut loadfn, "glDepthMask");
         storage::DepthRange.load(&mut loadfn, "glDepthRange");
         storage::DepthRangeArrayv.load(&mut loadfn, "glDepthRangeArrayv");
         storage::DepthRangeIndexed.load(&mut loadfn, "glDepthRangeIndexed");
         storage::DepthRangef.load(&mut loadfn, "glDepthRangef");
         storage::DetachShader.load(&mut loadfn, "glDetachShader");
         storage::Disable.load(&mut loadfn, "glDisable");
         storage::DisableVertexArrayAttrib.load(&mut loadfn, "glDisableVertexArrayAttrib");
         storage::DisableVertexAttribArray.load(&mut loadfn, "glDisableVertexAttribArray");
         storage::Disablei.load(&mut loadfn, "glDisablei");
         storage::DispatchCompute.load(&mut loadfn, "glDispatchCompute");
         storage::DispatchComputeIndirect.load(&mut loadfn, "glDispatchComputeIndirect");
         storage::DrawArrays.load(&mut loadfn, "glDrawArrays");
         storage::DrawArraysIndirect.load(&mut loadfn, "glDrawArraysIndirect");
         storage::DrawArraysInstanced.load(&mut loadfn, "glDrawArraysInstanced");
         storage::DrawArraysInstancedBaseInstance.load(&mut loadfn, "glDrawArraysInstancedBaseInstance");
         storage::DrawBuffer.load(&mut loadfn, "glDrawBuffer");
         storage::DrawBuffers.load(&mut loadfn, "glDrawBuffers");
         storage::DrawElements.load(&mut loadfn, "glDrawElements");
         storage::DrawElementsBaseVertex.load(&mut loadfn, "glDrawElementsBaseVertex");
         storage::DrawElementsIndirect.load(&mut loadfn, "glDrawElementsIndirect");
         storage::DrawElementsInstanced.load(&mut loadfn, "glDrawElementsInstanced");
         storage::DrawElementsInstancedBaseInstance.load(&mut loadfn, "glDrawElementsInstancedBaseInstance");
         storage::DrawElementsInstancedBaseVertex.load(&mut loadfn, "glDrawElementsInstancedBaseVertex");
         storage::DrawElementsInstancedBaseVertexBaseInstance.load(&mut loadfn, "glDrawElementsInstancedBaseVertexBaseInstance");
         storage::DrawRangeElements.load(&mut loadfn, "glDrawRangeElements");
         storage::DrawRangeElementsBaseVertex.load(&mut loadfn, "glDrawRangeElementsBaseVertex");
         storage::DrawTransformFeedback.load(&mut loadfn, "glDrawTransformFeedback");
         storage::DrawTransformFeedbackInstanced.load(&mut loadfn, "glDrawTransformFeedbackInstanced");
         storage::DrawTransformFeedbackStream.load(&mut loadfn, "glDrawTransformFeedbackStream");
         storage::DrawTransformFeedbackStreamInstanced.load(&mut loadfn, "glDrawTransformFeedbackStreamInstanced");
         storage::Enable.load(&mut loadfn, "glEnable");
         storage::EnableVertexArrayAttrib.load(&mut loadfn, "glEnableVertexArrayAttrib");
         storage::EnableVertexAttribArray.load(&mut loadfn, "glEnableVertexAttribArray");
         storage::Enablei.load(&mut loadfn, "glEnablei");
         storage::EndConditionalRender.load(&mut loadfn, "glEndConditionalRender");
         storage::EndQuery.load(&mut loadfn, "glEndQuery");
         storage::EndQueryIndexed.load(&mut loadfn, "glEndQueryIndexed");
         storage::EndTransformFeedback.load(&mut loadfn, "glEndTransformFeedback");
         storage::FenceSync.load(&mut loadfn, "glFenceSync");
         storage::Finish.load(&mut loadfn, "glFinish");
         storage::Flush.load(&mut loadfn, "glFlush");
         storage::FlushMappedBufferRange.load(&mut loadfn, "glFlushMappedBufferRange");
         storage::FlushMappedNamedBufferRange.load(&mut loadfn, "glFlushMappedNamedBufferRange");
         storage::FramebufferParameteri.load(&mut loadfn, "glFramebufferParameteri");
         storage::FramebufferRenderbuffer.load(&mut loadfn, "glFramebufferRenderbuffer");
         storage::FramebufferTexture.load(&mut loadfn, "glFramebufferTexture");
         storage::FramebufferTexture1D.load(&mut loadfn, "glFramebufferTexture1D");
         storage::FramebufferTexture2D.load(&mut loadfn, "glFramebufferTexture2D");
         storage::FramebufferTexture3D.load(&mut loadfn, "glFramebufferTexture3D");
         storage::FramebufferTextureLayer.load(&mut loadfn, "glFramebufferTextureLayer");
         storage::FrontFace.load(&mut loadfn, "glFrontFace");
         storage::GenBuffers.load(&mut loadfn, "glGenBuffers");
         storage::GenFramebuffers.load(&mut loadfn, "glGenFramebuffers");
         storage::GenProgramPipelines.load(&mut loadfn, "glGenProgramPipelines");
         storage::GenQueries.load(&mut loadfn, "glGenQueries");
         storage::GenRenderbuffers.load(&mut loadfn, "glGenRenderbuffers");
         storage::GenSamplers.load(&mut loadfn, "glGenSamplers");
         storage::GenTextures.load(&mut loadfn, "glGenTextures");
         storage::GenTransformFeedbacks.load(&mut loadfn, "glGenTransformFeedbacks");
         storage::GenVertexArrays.load(&mut loadfn, "glGenVertexArrays");
         storage::GenerateMipmap.load(&mut loadfn, "glGenerateMipmap");
         storage::GenerateTextureMipmap.load(&mut loadfn, "glGenerateTextureMipmap");
         storage::GetActiveAtomicCounterBufferiv.load(&mut loadfn, "glGetActiveAtomicCounterBufferiv");
         storage::GetActiveAttrib.load(&mut loadfn, "glGetActiveAttrib");
         storage::GetActiveSubroutineName.load(&mut loadfn, "glGetActiveSubroutineName");
         storage::GetActiveSubroutineUniformName.load(&mut loadfn, "glGetActiveSubroutineUniformName");
         storage::GetActiveSubroutineUniformiv.load(&mut loadfn, "glGetActiveSubroutineUniformiv");
         storage::GetActiveUniform.load(&mut loadfn, "glGetActiveUniform");
         storage::GetActiveUniformBlockName.load(&mut loadfn, "glGetActiveUniformBlockName");
         storage::GetActiveUniformBlockiv.load(&mut loadfn, "glGetActiveUniformBlockiv");
         storage::GetActiveUniformName.load(&mut loadfn, "glGetActiveUniformName");
         storage::GetActiveUniformsiv.load(&mut loadfn, "glGetActiveUniformsiv");
         storage::GetAttachedShaders.load(&mut loadfn, "glGetAttachedShaders");
         storage::GetAttribLocation.load(&mut loadfn, "glGetAttribLocation");
         storage::GetBooleani_v.load(&mut loadfn, "glGetBooleani_v");
         storage::GetBooleanv.load(&mut loadfn, "glGetBooleanv");
         storage::GetBufferParameteri64v.load(&mut loadfn, "glGetBufferParameteri64v");
         storage::GetBufferParameteriv.load(&mut loadfn, "glGetBufferParameteriv");
         storage::GetBufferPointerv.load(&mut loadfn, "glGetBufferPointerv");
         storage::GetBufferSubData.load(&mut loadfn, "glGetBufferSubData");
         storage::GetCompressedTexImage.load(&mut loadfn, "glGetCompressedTexImage");
         storage::GetCompressedTextureImage.load(&mut loadfn, "glGetCompressedTextureImage");
         storage::GetCompressedTextureSubImage.load(&mut loadfn, "glGetCompressedTextureSubImage");
         storage::GetDebugMessageLog.load(&mut loadfn, "glGetDebugMessageLog");
         storage::GetDoublei_v.load(&mut loadfn, "glGetDoublei_v");
         storage::GetDoublev.load(&mut loadfn, "glGetDoublev");
         storage::GetError.load(&mut loadfn, "glGetError");
         storage::GetFloati_v.load(&mut loadfn, "glGetFloati_v");
         storage::GetFloatv.load(&mut loadfn, "glGetFloatv");
         storage::GetFragDataIndex.load(&mut loadfn, "glGetFragDataIndex");
         storage::GetFragDataLocation.load(&mut loadfn, "glGetFragDataLocation");
         storage::GetFramebufferAttachmentParameteriv.load(&mut loadfn, "glGetFramebufferAttachmentParameteriv");
         storage::GetFramebufferParameteriv.load(&mut loadfn, "glGetFramebufferParameteriv");
         storage::GetGraphicsResetStatus.load(&mut loadfn, "glGetGraphicsResetStatus");
         storage::GetInteger64i_v.load(&mut loadfn, "glGetInteger64i_v");
         storage::GetInteger64v.load(&mut loadfn, "glGetInteger64v");
         storage::GetIntegeri_v.load(&mut loadfn, "glGetIntegeri_v");
         storage::GetIntegerv.load(&mut loadfn, "glGetIntegerv");
         storage::GetInternalformati64v.load(&mut loadfn, "glGetInternalformati64v");
         storage::GetInternalformativ.load(&mut loadfn, "glGetInternalformativ");
         storage::GetMultisamplefv.load(&mut loadfn, "glGetMultisamplefv");
         storage::GetNamedBufferParameteri64v.load(&mut loadfn, "glGetNamedBufferParameteri64v");
         storage::GetNamedBufferParameteriv.load(&mut loadfn, "glGetNamedBufferParameteriv");
         storage::GetNamedBufferPointerv.load(&mut loadfn, "glGetNamedBufferPointerv");
         storage::GetNamedBufferSubData.load(&mut loadfn, "glGetNamedBufferSubData");
         storage::GetNamedFramebufferAttachmentParameteriv.load(&mut loadfn, "glGetNamedFramebufferAttachmentParameteriv");
         storage::GetNamedFramebufferParameteriv.load(&mut loadfn, "glGetNamedFramebufferParameteriv");
         storage::GetNamedRenderbufferParameteriv.load(&mut loadfn, "glGetNamedRenderbufferParameteriv");
         storage::GetObjectLabel.load(&mut loadfn, "glGetObjectLabel");
         storage::GetObjectPtrLabel.load(&mut loadfn, "glGetObjectPtrLabel");
         storage::GetPointerv.load(&mut loadfn, "glGetPointerv");
         storage::GetProgramBinary.load(&mut loadfn, "glGetProgramBinary");
         storage::GetProgramInfoLog.load(&mut loadfn, "glGetProgramInfoLog");
         storage::GetProgramInterfaceiv.load(&mut loadfn, "glGetProgramInterfaceiv");
         storage::GetProgramPipelineInfoLog.load(&mut loadfn, "glGetProgramPipelineInfoLog");
         storage::GetProgramPipelineiv.load(&mut loadfn, "glGetProgramPipelineiv");
         storage::GetProgramResourceIndex.load(&mut loadfn, "glGetProgramResourceIndex");
         storage::GetProgramResourceLocation.load(&mut loadfn, "glGetProgramResourceLocation");
         storage::GetProgramResourceLocationIndex.load(&mut loadfn, "glGetProgramResourceLocationIndex");
         storage::GetProgramResourceName.load(&mut loadfn, "glGetProgramResourceName");
         storage::GetProgramResourceiv.load(&mut loadfn, "glGetProgramResourceiv");
         storage::GetProgramStageiv.load(&mut loadfn, "glGetProgramStageiv");
         storage::GetProgramiv.load(&mut loadfn, "glGetProgramiv");
         storage::GetQueryBufferObjecti64v.load(&mut loadfn, "glGetQueryBufferObjecti64v");
         storage::GetQueryBufferObjectiv.load(&mut loadfn, "glGetQueryBufferObjectiv");
         storage::GetQueryBufferObjectui64v.load(&mut loadfn, "glGetQueryBufferObjectui64v");
         storage::GetQueryBufferObjectuiv.load(&mut loadfn, "glGetQueryBufferObjectuiv");
         storage::GetQueryIndexediv.load(&mut loadfn, "glGetQueryIndexediv");
         storage::GetQueryObjecti64v.load(&mut loadfn, "glGetQueryObjecti64v");
         storage::GetQueryObjectiv.load(&mut loadfn, "glGetQueryObjectiv");
         storage::GetQueryObjectui64v.load(&mut loadfn, "glGetQueryObjectui64v");
         storage::GetQueryObjectuiv.load(&mut loadfn, "glGetQueryObjectuiv");
         storage::GetQueryiv.load(&mut loadfn, "glGetQueryiv");
         storage::GetRenderbufferParameteriv.load(&mut loadfn, "glGetRenderbufferParameteriv");
         storage::GetSamplerParameterIiv.load(&mut loadfn, "glGetSamplerParameterIiv");
         storage::GetSamplerParameterIuiv.load(&mut loadfn, "glGetSamplerParameterIuiv");
         storage::GetSamplerParameterfv.load(&mut loadfn, "glGetSamplerParameterfv");
         storage::GetSamplerParameteriv.load(&mut loadfn, "glGetSamplerParameteriv");
         storage::GetShaderInfoLog.load(&mut loadfn, "glGetShaderInfoLog");
         storage::GetShaderPrecisionFormat.load(&mut loadfn, "glGetShaderPrecisionFormat");
         storage::GetShaderSource.load(&mut loadfn, "glGetShaderSource");
         storage::GetShaderiv.load(&mut loadfn, "glGetShaderiv");
         storage::GetString.load(&mut loadfn, "glGetString");
         storage::GetStringi.load(&mut loadfn, "glGetStringi");
         storage::GetSubroutineIndex.load(&mut loadfn, "glGetSubroutineIndex");
         storage::GetSubroutineUniformLocation.load(&mut loadfn, "glGetSubroutineUniformLocation");
         storage::GetSynciv.load(&mut loadfn, "glGetSynciv");
         storage::GetTexImage.load(&mut loadfn, "glGetTexImage");
         storage::GetTexLevelParameterfv.load(&mut loadfn, "glGetTexLevelParameterfv");
         storage::GetTexLevelParameteriv.load(&mut loadfn, "glGetTexLevelParameteriv");
         storage::GetTexParameterIiv.load(&mut loadfn, "glGetTexParameterIiv");
         storage::GetTexParameterIuiv.load(&mut loadfn, "glGetTexParameterIuiv");
         storage::GetTexParameterfv.load(&mut loadfn, "glGetTexParameterfv");
         storage::GetTexParameteriv.load(&mut loadfn, "glGetTexParameteriv");
         storage::GetTextureImage.load(&mut loadfn, "glGetTextureImage");
         storage::GetTextureLevelParameterfv.load(&mut loadfn, "glGetTextureLevelParameterfv");
         storage::GetTextureLevelParameteriv.load(&mut loadfn, "glGetTextureLevelParameteriv");
         storage::GetTextureParameterIiv.load(&mut loadfn, "glGetTextureParameterIiv");
         storage::GetTextureParameterIuiv.load(&mut loadfn, "glGetTextureParameterIuiv");
         storage::GetTextureParameterfv.load(&mut loadfn, "glGetTextureParameterfv");
         storage::GetTextureParameteriv.load(&mut loadfn, "glGetTextureParameteriv");
         storage::GetTextureSubImage.load(&mut loadfn, "glGetTextureSubImage");
         storage::GetTransformFeedbackVarying.load(&mut loadfn, "glGetTransformFeedbackVarying");
         storage::GetTransformFeedbacki64_v.load(&mut loadfn, "glGetTransformFeedbacki64_v");
         storage::GetTransformFeedbacki_v.load(&mut loadfn, "glGetTransformFeedbacki_v");
         storage::GetTransformFeedbackiv.load(&mut loadfn, "glGetTransformFeedbackiv");
         storage::GetUniformBlockIndex.load(&mut loadfn, "glGetUniformBlockIndex");
         storage::GetUniformIndices.load(&mut loadfn, "glGetUniformIndices");
         storage::GetUniformLocation.load(&mut loadfn, "glGetUniformLocation");
         storage::GetUniformSubroutineuiv.load(&mut loadfn, "glGetUniformSubroutineuiv");
         storage::GetUniformdv.load(&mut loadfn, "glGetUniformdv");
         storage::GetUniformfv.load(&mut loadfn, "glGetUniformfv");
         storage::GetUniformiv.load(&mut loadfn, "glGetUniformiv");
         storage::GetUniformuiv.load(&mut loadfn, "glGetUniformuiv");
         storage::GetVertexArrayIndexed64iv.load(&mut loadfn, "glGetVertexArrayIndexed64iv");
         storage::GetVertexArrayIndexediv.load(&mut loadfn, "glGetVertexArrayIndexediv");
         storage::GetVertexArrayiv.load(&mut loadfn, "glGetVertexArrayiv");
         storage::GetVertexAttribIiv.load(&mut loadfn, "glGetVertexAttribIiv");
         storage::GetVertexAttribIuiv.load(&mut loadfn, "glGetVertexAttribIuiv");
         storage::GetVertexAttribLdv.load(&mut loadfn, "glGetVertexAttribLdv");
         storage::GetVertexAttribPointerv.load(&mut loadfn, "glGetVertexAttribPointerv");
         storage::GetVertexAttribdv.load(&mut loadfn, "glGetVertexAttribdv");
         storage::GetVertexAttribfv.load(&mut loadfn, "glGetVertexAttribfv");
         storage::GetVertexAttribiv.load(&mut loadfn, "glGetVertexAttribiv");
         storage::GetnCompressedTexImage.load(&mut loadfn, "glGetnCompressedTexImage");
         storage::GetnTexImage.load(&mut loadfn, "glGetnTexImage");
         storage::GetnUniformdv.load(&mut loadfn, "glGetnUniformdv");
         storage::GetnUniformfv.load(&mut loadfn, "glGetnUniformfv");
         storage::GetnUniformiv.load(&mut loadfn, "glGetnUniformiv");
         storage::GetnUniformuiv.load(&mut loadfn, "glGetnUniformuiv");
         storage::Hint.load(&mut loadfn, "glHint");
         storage::InvalidateBufferData.load(&mut loadfn, "glInvalidateBufferData");
         storage::InvalidateBufferSubData.load(&mut loadfn, "glInvalidateBufferSubData");
         storage::InvalidateFramebuffer.load(&mut loadfn, "glInvalidateFramebuffer");
         storage::InvalidateNamedFramebufferData.load(&mut loadfn, "glInvalidateNamedFramebufferData");
         storage::InvalidateNamedFramebufferSubData.load(&mut loadfn, "glInvalidateNamedFramebufferSubData");
         storage::InvalidateSubFramebuffer.load(&mut loadfn, "glInvalidateSubFramebuffer");
         storage::InvalidateTexImage.load(&mut loadfn, "glInvalidateTexImage");
         storage::InvalidateTexSubImage.load(&mut loadfn, "glInvalidateTexSubImage");
         storage::IsBuffer.load(&mut loadfn, "glIsBuffer");
         storage::IsEnabled.load(&mut loadfn, "glIsEnabled");
         storage::IsEnabledi.load(&mut loadfn, "glIsEnabledi");
         storage::IsFramebuffer.load(&mut loadfn, "glIsFramebuffer");
         storage::IsProgram.load(&mut loadfn, "glIsProgram");
         storage::IsProgramPipeline.load(&mut loadfn, "glIsProgramPipeline");
         storage::IsQuery.load(&mut loadfn, "glIsQuery");
         storage::IsRenderbuffer.load(&mut loadfn, "glIsRenderbuffer");
         storage::IsSampler.load(&mut loadfn, "glIsSampler");
         storage::IsShader.load(&mut loadfn, "glIsShader");
         storage::IsSync.load(&mut loadfn, "glIsSync");
         storage::IsTexture.load(&mut loadfn, "glIsTexture");
         storage::IsTransformFeedback.load(&mut loadfn, "glIsTransformFeedback");
         storage::IsVertexArray.load(&mut loadfn, "glIsVertexArray");
         storage::LineWidth.load(&mut loadfn, "glLineWidth");
         storage::LinkProgram.load(&mut loadfn, "glLinkProgram");
         storage::LogicOp.load(&mut loadfn, "glLogicOp");
         storage::MapBuffer.load(&mut loadfn, "glMapBuffer");
         storage::MapBufferRange.load(&mut loadfn, "glMapBufferRange");
         storage::MapNamedBuffer.load(&mut loadfn, "glMapNamedBuffer");
         storage::MapNamedBufferRange.load(&mut loadfn, "glMapNamedBufferRange");
         storage::MemoryBarrier.load(&mut loadfn, "glMemoryBarrier");
         storage::MemoryBarrierByRegion.load(&mut loadfn, "glMemoryBarrierByRegion");
         storage::MinSampleShading.load(&mut loadfn, "glMinSampleShading");
         storage::MultiDrawArrays.load(&mut loadfn, "glMultiDrawArrays");
         storage::MultiDrawArraysIndirect.load(&mut loadfn, "glMultiDrawArraysIndirect");
         storage::MultiDrawElements.load(&mut loadfn, "glMultiDrawElements");
         storage::MultiDrawElementsBaseVertex.load(&mut loadfn, "glMultiDrawElementsBaseVertex");
         storage::MultiDrawElementsIndirect.load(&mut loadfn, "glMultiDrawElementsIndirect");
         storage::NamedBufferData.load(&mut loadfn, "glNamedBufferData");
         storage::NamedBufferStorage.load(&mut loadfn, "glNamedBufferStorage");
         storage::NamedBufferSubData.load(&mut loadfn, "glNamedBufferSubData");
         storage::NamedFramebufferDrawBuffer.load(&mut loadfn, "glNamedFramebufferDrawBuffer");
         storage::NamedFramebufferDrawBuffers.load(&mut loadfn, "glNamedFramebufferDrawBuffers");
         storage::NamedFramebufferParameteri.load(&mut loadfn, "glNamedFramebufferParameteri");
         storage::NamedFramebufferReadBuffer.load(&mut loadfn, "glNamedFramebufferReadBuffer");
         storage::NamedFramebufferRenderbuffer.load(&mut loadfn, "glNamedFramebufferRenderbuffer");
         storage::NamedFramebufferTexture.load(&mut loadfn, "glNamedFramebufferTexture");
         storage::NamedFramebufferTextureLayer.load(&mut loadfn, "glNamedFramebufferTextureLayer");
         storage::NamedRenderbufferStorage.load(&mut loadfn, "glNamedRenderbufferStorage");
         storage::NamedRenderbufferStorageMultisample.load(&mut loadfn, "glNamedRenderbufferStorageMultisample");
         storage::ObjectLabel.load(&mut loadfn, "glObjectLabel");
         storage::ObjectPtrLabel.load(&mut loadfn, "glObjectPtrLabel");
         storage::PatchParameterfv.load(&mut loadfn, "glPatchParameterfv");
         storage::PatchParameteri.load(&mut loadfn, "glPatchParameteri");
         storage::PauseTransformFeedback.load(&mut loadfn, "glPauseTransformFeedback");
         storage::PixelStoref.load(&mut loadfn, "glPixelStoref");
         storage::PixelStorei.load(&mut loadfn, "glPixelStorei");
         storage::PointParameterf.load(&mut loadfn, "glPointParameterf");
         storage::PointParameterfv.load(&mut loadfn, "glPointParameterfv");
         storage::PointParameteri.load(&mut loadfn, "glPointParameteri");
         storage::PointParameteriv.load(&mut loadfn, "glPointParameteriv");
         storage::PointSize.load(&mut loadfn, "glPointSize");
         storage::PolygonMode.load(&mut loadfn, "glPolygonMode");
         storage::PolygonOffset.load(&mut loadfn, "glPolygonOffset");
         storage::PopDebugGroup.load(&mut loadfn, "glPopDebugGroup");
         storage::PrimitiveRestartIndex.load(&mut loadfn, "glPrimitiveRestartIndex");
         storage::ProgramBinary.load(&mut loadfn, "glProgramBinary");
         storage::ProgramParameteri.load(&mut loadfn, "glProgramParameteri");
         storage::ProgramUniform1d.load(&mut loadfn, "glProgramUniform1d");
         storage::ProgramUniform1dv.load(&mut loadfn, "glProgramUniform1dv");
         storage::ProgramUniform1f.load(&mut loadfn, "glProgramUniform1f");
         storage::ProgramUniform1fv.load(&mut loadfn, "glProgramUniform1fv");
         storage::ProgramUniform1i.load(&mut loadfn, "glProgramUniform1i");
         storage::ProgramUniform1iv.load(&mut loadfn, "glProgramUniform1iv");
         storage::ProgramUniform1ui.load(&mut loadfn, "glProgramUniform1ui");
         storage::ProgramUniform1uiv.load(&mut loadfn, "glProgramUniform1uiv");
         storage::ProgramUniform2d.load(&mut loadfn, "glProgramUniform2d");
         storage::ProgramUniform2dv.load(&mut loadfn, "glProgramUniform2dv");
         storage::ProgramUniform2f.load(&mut loadfn, "glProgramUniform2f");
         storage::ProgramUniform2fv.load(&mut loadfn, "glProgramUniform2fv");
         storage::ProgramUniform2i.load(&mut loadfn, "glProgramUniform2i");
         storage::ProgramUniform2iv.load(&mut loadfn, "glProgramUniform2iv");
         storage::ProgramUniform2ui.load(&mut loadfn, "glProgramUniform2ui");
         storage::ProgramUniform2uiv.load(&mut loadfn, "glProgramUniform2uiv");
         storage::ProgramUniform3d.load(&mut loadfn, "glProgramUniform3d");
         storage::ProgramUniform3dv.load(&mut loadfn, "glProgramUniform3dv");
         storage::ProgramUniform3f.load(&mut loadfn, "glProgramUniform3f");
         storage::ProgramUniform3fv.load(&mut loadfn, "glProgramUniform3fv");
         storage::ProgramUniform3i.load(&mut loadfn, "glProgramUniform3i");
         storage::ProgramUniform3iv.load(&mut loadfn, "glProgramUniform3iv");
         storage::ProgramUniform3ui.load(&mut loadfn, "glProgramUniform3ui");
         storage::ProgramUniform3uiv.load(&mut loadfn, "glProgramUniform3uiv");
         storage::ProgramUniform4d.load(&mut loadfn, "glProgramUniform4d");
         storage::ProgramUniform4dv.load(&mut loadfn, "glProgramUniform4dv");
         storage::ProgramUniform4f.load(&mut loadfn, "glProgramUniform4f");
         storage::ProgramUniform4fv.load(&mut loadfn, "glProgramUniform4fv");
         storage::ProgramUniform4i.load(&mut loadfn, "glProgramUniform4i");
         storage::ProgramUniform4iv.load(&mut loadfn, "glProgramUniform4iv");
         storage::ProgramUniform4ui.load(&mut loadfn, "glProgramUniform4ui");
         storage::ProgramUniform4uiv.load(&mut loadfn, "glProgramUniform4uiv");
         storage::ProgramUniformMatrix2dv.load(&mut loadfn, "glProgramUniformMatrix2dv");
         storage::ProgramUniformMatrix2fv.load(&mut loadfn, "glProgramUniformMatrix2fv");
         storage::ProgramUniformMatrix2x3dv.load(&mut loadfn, "glProgramUniformMatrix2x3dv");
         storage::ProgramUniformMatrix2x3fv.load(&mut loadfn, "glProgramUniformMatrix2x3fv");
         storage::ProgramUniformMatrix2x4dv.load(&mut loadfn, "glProgramUniformMatrix2x4dv");
         storage::ProgramUniformMatrix2x4fv.load(&mut loadfn, "glProgramUniformMatrix2x4fv");
         storage::ProgramUniformMatrix3dv.load(&mut loadfn, "glProgramUniformMatrix3dv");
         storage::ProgramUniformMatrix3fv.load(&mut loadfn, "glProgramUniformMatrix3fv");
         storage::ProgramUniformMatrix3x2dv.load(&mut loadfn, "glProgramUniformMatrix3x2dv");
         storage::ProgramUniformMatrix3x2fv.load(&mut loadfn, "glProgramUniformMatrix3x2fv");
         storage::ProgramUniformMatrix3x4dv.load(&mut loadfn, "glProgramUniformMatrix3x4dv");
         storage::ProgramUniformMatrix3x4fv.load(&mut loadfn, "glProgramUniformMatrix3x4fv");
         storage::ProgramUniformMatrix4dv.load(&mut loadfn, "glProgramUniformMatrix4dv");
         storage::ProgramUniformMatrix4fv.load(&mut loadfn, "glProgramUniformMatrix4fv");
         storage::ProgramUniformMatrix4x2dv.load(&mut loadfn, "glProgramUniformMatrix4x2dv");
         storage::ProgramUniformMatrix4x2fv.load(&mut loadfn, "glProgramUniformMatrix4x2fv");
         storage::ProgramUniformMatrix4x3dv.load(&mut loadfn, "glProgramUniformMatrix4x3dv");
         storage::ProgramUniformMatrix4x3fv.load(&mut loadfn, "glProgramUniformMatrix4x3fv");
         storage::ProvokingVertex.load(&mut loadfn, "glProvokingVertex");
         storage::PushDebugGroup.load(&mut loadfn, "glPushDebugGroup");
         storage::QueryCounter.load(&mut loadfn, "glQueryCounter");
         storage::ReadBuffer.load(&mut loadfn, "glReadBuffer");
         storage::ReadPixels.load(&mut loadfn, "glReadPixels");
         storage::ReadnPixels.load(&mut loadfn, "glReadnPixels");
         storage::ReleaseShaderCompiler.load(&mut loadfn, "glReleaseShaderCompiler");
         storage::RenderbufferStorage.load(&mut loadfn, "glRenderbufferStorage");
         storage::RenderbufferStorageMultisample.load(&mut loadfn, "glRenderbufferStorageMultisample");
         storage::ResumeTransformFeedback.load(&mut loadfn, "glResumeTransformFeedback");
         storage::SampleCoverage.load(&mut loadfn, "glSampleCoverage");
         storage::SampleMaski.load(&mut loadfn, "glSampleMaski");
         storage::SamplerParameterIiv.load(&mut loadfn, "glSamplerParameterIiv");
         storage::SamplerParameterIuiv.load(&mut loadfn, "glSamplerParameterIuiv");
         storage::SamplerParameterf.load(&mut loadfn, "glSamplerParameterf");
         storage::SamplerParameterfv.load(&mut loadfn, "glSamplerParameterfv");
         storage::SamplerParameteri.load(&mut loadfn, "glSamplerParameteri");
         storage::SamplerParameteriv.load(&mut loadfn, "glSamplerParameteriv");
         storage::Scissor.load(&mut loadfn, "glScissor");
         storage::ScissorArrayv.load(&mut loadfn, "glScissorArrayv");
         storage::ScissorIndexed.load(&mut loadfn, "glScissorIndexed");
         storage::ScissorIndexedv.load(&mut loadfn, "glScissorIndexedv");
         storage::ShaderBinary.load(&mut loadfn, "glShaderBinary");
         storage::ShaderSource.load(&mut loadfn, "glShaderSource");
         storage::ShaderStorageBlockBinding.load(&mut loadfn, "glShaderStorageBlockBinding");
         storage::StencilFunc.load(&mut loadfn, "glStencilFunc");
         storage::StencilFuncSeparate.load(&mut loadfn, "glStencilFuncSeparate");
         storage::StencilMask.load(&mut loadfn, "glStencilMask");
         storage::StencilMaskSeparate.load(&mut loadfn, "glStencilMaskSeparate");
         storage::StencilOp.load(&mut loadfn, "glStencilOp");
         storage::StencilOpSeparate.load(&mut loadfn, "glStencilOpSeparate");
         storage::TexBuffer.load(&mut loadfn, "glTexBuffer");
         storage::TexBufferRange.load(&mut loadfn, "glTexBufferRange");
         storage::TexImage1D.load(&mut loadfn, "glTexImage1D");
         storage::TexImage2D.load(&mut loadfn, "glTexImage2D");
         storage::TexImage2DMultisample.load(&mut loadfn, "glTexImage2DMultisample");
         storage::TexImage3D.load(&mut loadfn, "glTexImage3D");
         storage::TexImage3DMultisample.load(&mut loadfn, "glTexImage3DMultisample");
         storage::TexParameterIiv.load(&mut loadfn, "glTexParameterIiv");
         storage::TexParameterIuiv.load(&mut loadfn, "glTexParameterIuiv");
         storage::TexParameterf.load(&mut loadfn, "glTexParameterf");
         storage::TexParameterfv.load(&mut loadfn, "glTexParameterfv");
         storage::TexParameteri.load(&mut loadfn, "glTexParameteri");
         storage::TexParameteriv.load(&mut loadfn, "glTexParameteriv");
         storage::TexStorage1D.load(&mut loadfn, "glTexStorage1D");
         storage::TexStorage2D.load(&mut loadfn, "glTexStorage2D");
         storage::TexStorage2DMultisample.load(&mut loadfn, "glTexStorage2DMultisample");
         storage::TexStorage3D.load(&mut loadfn, "glTexStorage3D");
         storage::TexStorage3DMultisample.load(&mut loadfn, "glTexStorage3DMultisample");
         storage::TexSubImage1D.load(&mut loadfn, "glTexSubImage1D");
         storage::TexSubImage2D.load(&mut loadfn, "glTexSubImage2D");
         storage::TexSubImage3D.load(&mut loadfn, "glTexSubImage3D");
         storage::TextureBarrier.load(&mut loadfn, "glTextureBarrier");
         storage::TextureBuffer.load(&mut loadfn, "glTextureBuffer");
         storage::TextureBufferRange.load(&mut loadfn, "glTextureBufferRange");
         storage::TextureParameterIiv.load(&mut loadfn, "glTextureParameterIiv");
         storage::TextureParameterIuiv.load(&mut loadfn, "glTextureParameterIuiv");
         storage::TextureParameterf.load(&mut loadfn, "glTextureParameterf");
         storage::TextureParameterfv.load(&mut loadfn, "glTextureParameterfv");
         storage::TextureParameteri.load(&mut loadfn, "glTextureParameteri");
         storage::TextureParameteriv.load(&mut loadfn, "glTextureParameteriv");
         storage::TextureStorage1D.load(&mut loadfn, "glTextureStorage1D");
         storage::TextureStorage2D.load(&mut loadfn, "glTextureStorage2D");
         storage::TextureStorage2DMultisample.load(&mut loadfn, "glTextureStorage2DMultisample");
         storage::TextureStorage3D.load(&mut loadfn, "glTextureStorage3D");
         storage::TextureStorage3DMultisample.load(&mut loadfn, "glTextureStorage3DMultisample");
         storage::TextureSubImage1D.load(&mut loadfn, "glTextureSubImage1D");
         storage::TextureSubImage2D.load(&mut loadfn, "glTextureSubImage2D");
         storage::TextureSubImage3D.load(&mut loadfn, "glTextureSubImage3D");
         storage::TextureView.load(&mut loadfn, "glTextureView");
         storage::TransformFeedbackBufferBase.load(&mut loadfn, "glTransformFeedbackBufferBase");
         storage::TransformFeedbackBufferRange.load(&mut loadfn, "glTransformFeedbackBufferRange");
         storage::TransformFeedbackVaryings.load(&mut loadfn, "glTransformFeedbackVaryings");
         storage::Uniform1d.load(&mut loadfn, "glUniform1d");
         storage::Uniform1dv.load(&mut loadfn, "glUniform1dv");
         storage::Uniform1f.load(&mut loadfn, "glUniform1f");
         storage::Uniform1fv.load(&mut loadfn, "glUniform1fv");
         storage::Uniform1i.load(&mut loadfn, "glUniform1i");
         storage::Uniform1iv.load(&mut loadfn, "glUniform1iv");
         storage::Uniform1ui.load(&mut loadfn, "glUniform1ui");
         storage::Uniform1uiv.load(&mut loadfn, "glUniform1uiv");
         storage::Uniform2d.load(&mut loadfn, "glUniform2d");
         storage::Uniform2dv.load(&mut loadfn, "glUniform2dv");
         storage::Uniform2f.load(&mut loadfn, "glUniform2f");
         storage::Uniform2fv.load(&mut loadfn, "glUniform2fv");
         storage::Uniform2i.load(&mut loadfn, "glUniform2i");
         storage::Uniform2iv.load(&mut loadfn, "glUniform2iv");
         storage::Uniform2ui.load(&mut loadfn, "glUniform2ui");
         storage::Uniform2uiv.load(&mut loadfn, "glUniform2uiv");
         storage::Uniform3d.load(&mut loadfn, "glUniform3d");
         storage::Uniform3dv.load(&mut loadfn, "glUniform3dv");
         storage::Uniform3f.load(&mut loadfn, "glUniform3f");
         storage::Uniform3fv.load(&mut loadfn, "glUniform3fv");
         storage::Uniform3i.load(&mut loadfn, "glUniform3i");
         storage::Uniform3iv.load(&mut loadfn, "glUniform3iv");
         storage::Uniform3ui.load(&mut loadfn, "glUniform3ui");
         storage::Uniform3uiv.load(&mut loadfn, "glUniform3uiv");
         storage::Uniform4d.load(&mut loadfn, "glUniform4d");
         storage::Uniform4dv.load(&mut loadfn, "glUniform4dv");
         storage::Uniform4f.load(&mut loadfn, "glUniform4f");
         storage::Uniform4fv.load(&mut loadfn, "glUniform4fv");
         storage::Uniform4i.load(&mut loadfn, "glUniform4i");
         storage::Uniform4iv.load(&mut loadfn, "glUniform4iv");
         storage::Uniform4ui.load(&mut loadfn, "glUniform4ui");
         storage::Uniform4uiv.load(&mut loadfn, "glUniform4uiv");
         storage::UniformBlockBinding.load(&mut loadfn, "glUniformBlockBinding");
         storage::UniformMatrix2dv.load(&mut loadfn, "glUniformMatrix2dv");
         storage::UniformMatrix2fv.load(&mut loadfn, "glUniformMatrix2fv");
         storage::UniformMatrix2x3dv.load(&mut loadfn, "glUniformMatrix2x3dv");
         storage::UniformMatrix2x3fv.load(&mut loadfn, "glUniformMatrix2x3fv");
         storage::UniformMatrix2x4dv.load(&mut loadfn, "glUniformMatrix2x4dv");
         storage::UniformMatrix2x4fv.load(&mut loadfn, "glUniformMatrix2x4fv");
         storage::UniformMatrix3dv.load(&mut loadfn, "glUniformMatrix3dv");
         storage::UniformMatrix3fv.load(&mut loadfn, "glUniformMatrix3fv");
         storage::UniformMatrix3x2dv.load(&mut loadfn, "glUniformMatrix3x2dv");
         storage::UniformMatrix3x2fv.load(&mut loadfn, "glUniformMatrix3x2fv");
         storage::UniformMatrix3x4dv.load(&mut loadfn, "glUniformMatrix3x4dv");
         storage::UniformMatrix3x4fv.load(&mut loadfn, "glUniformMatrix3x4fv");
         storage::UniformMatrix4dv.load(&mut loadfn, "glUniformMatrix4dv");
         storage::UniformMatrix4fv.load(&mut loadfn, "glUniformMatrix4fv");
         storage::UniformMatrix4x2dv.load(&mut loadfn, "glUniformMatrix4x2dv");
         storage::UniformMatrix4x2fv.load(&mut loadfn, "glUniformMatrix4x2fv");
         storage::UniformMatrix4x3dv.load(&mut loadfn, "glUniformMatrix4x3dv");
         storage::UniformMatrix4x3fv.load(&mut loadfn, "glUniformMatrix4x3fv");
         storage::UniformSubroutinesuiv.load(&mut loadfn, "glUniformSubroutinesuiv");
         storage::UnmapBuffer.load(&mut loadfn, "glUnmapBuffer");
         storage::UnmapNamedBuffer.load(&mut loadfn, "glUnmapNamedBuffer");
         storage::UseProgram.load(&mut loadfn, "glUseProgram");
         storage::UseProgramStages.load(&mut loadfn, "glUseProgramStages");
         storage::ValidateProgram.load(&mut loadfn, "glValidateProgram");
         storage::ValidateProgramPipeline.load(&mut loadfn, "glValidateProgramPipeline");
         storage::VertexArrayAttribBinding.load(&mut loadfn, "glVertexArrayAttribBinding");
         storage::VertexArrayAttribFormat.load(&mut loadfn, "glVertexArrayAttribFormat");
         storage::VertexArrayAttribIFormat.load(&mut loadfn, "glVertexArrayAttribIFormat");
         storage::VertexArrayAttribLFormat.load(&mut loadfn, "glVertexArrayAttribLFormat");
         storage::VertexArrayBindingDivisor.load(&mut loadfn, "glVertexArrayBindingDivisor");
         storage::VertexArrayElementBuffer.load(&mut loadfn, "glVertexArrayElementBuffer");
         storage::VertexArrayVertexBuffer.load(&mut loadfn, "glVertexArrayVertexBuffer");
         storage::VertexArrayVertexBuffers.load(&mut loadfn, "glVertexArrayVertexBuffers");
         storage::VertexAttrib1d.load(&mut loadfn, "glVertexAttrib1d");
         storage::VertexAttrib1dv.load(&mut loadfn, "glVertexAttrib1dv");
         storage::VertexAttrib1f.load(&mut loadfn, "glVertexAttrib1f");
         storage::VertexAttrib1fv.load(&mut loadfn, "glVertexAttrib1fv");
         storage::VertexAttrib1s.load(&mut loadfn, "glVertexAttrib1s");
         storage::VertexAttrib1sv.load(&mut loadfn, "glVertexAttrib1sv");
         storage::VertexAttrib2d.load(&mut loadfn, "glVertexAttrib2d");
         storage::VertexAttrib2dv.load(&mut loadfn, "glVertexAttrib2dv");
         storage::VertexAttrib2f.load(&mut loadfn, "glVertexAttrib2f");
         storage::VertexAttrib2fv.load(&mut loadfn, "glVertexAttrib2fv");
         storage::VertexAttrib2s.load(&mut loadfn, "glVertexAttrib2s");
         storage::VertexAttrib2sv.load(&mut loadfn, "glVertexAttrib2sv");
         storage::VertexAttrib3d.load(&mut loadfn, "glVertexAttrib3d");
         storage::VertexAttrib3dv.load(&mut loadfn, "glVertexAttrib3dv");
         storage::VertexAttrib3f.load(&mut loadfn, "glVertexAttrib3f");
         storage::VertexAttrib3fv.load(&mut loadfn, "glVertexAttrib3fv");
         storage::VertexAttrib3s.load(&mut loadfn, "glVertexAttrib3s");
         storage::VertexAttrib3sv.load(&mut loadfn, "glVertexAttrib3sv");
         storage::VertexAttrib4Nbv.load(&mut loadfn, "glVertexAttrib4Nbv");
         storage::VertexAttrib4Niv.load(&mut loadfn, "glVertexAttrib4Niv");
         storage::VertexAttrib4Nsv.load(&mut loadfn, "glVertexAttrib4Nsv");
         storage::VertexAttrib4Nub.load(&mut loadfn, "glVertexAttrib4Nub");
         storage::VertexAttrib4Nubv.load(&mut loadfn, "glVertexAttrib4Nubv");
         storage::VertexAttrib4Nuiv.load(&mut loadfn, "glVertexAttrib4Nuiv");
         storage::VertexAttrib4Nusv.load(&mut loadfn, "glVertexAttrib4Nusv");
         storage::VertexAttrib4bv.load(&mut loadfn, "glVertexAttrib4bv");
         storage::VertexAttrib4d.load(&mut loadfn, "glVertexAttrib4d");
         storage::VertexAttrib4dv.load(&mut loadfn, "glVertexAttrib4dv");
         storage::VertexAttrib4f.load(&mut loadfn, "glVertexAttrib4f");
         storage::VertexAttrib4fv.load(&mut loadfn, "glVertexAttrib4fv");
         storage::VertexAttrib4iv.load(&mut loadfn, "glVertexAttrib4iv");
         storage::VertexAttrib4s.load(&mut loadfn, "glVertexAttrib4s");
         storage::VertexAttrib4sv.load(&mut loadfn, "glVertexAttrib4sv");
         storage::VertexAttrib4ubv.load(&mut loadfn, "glVertexAttrib4ubv");
         storage::VertexAttrib4uiv.load(&mut loadfn, "glVertexAttrib4uiv");
         storage::VertexAttrib4usv.load(&mut loadfn, "glVertexAttrib4usv");
         storage::VertexAttribBinding.load(&mut loadfn, "glVertexAttribBinding");
         storage::VertexAttribDivisor.load(&mut loadfn, "glVertexAttribDivisor");
         storage::VertexAttribFormat.load(&mut loadfn, "glVertexAttribFormat");
         storage::VertexAttribI1i.load(&mut loadfn, "glVertexAttribI1i");
         storage::VertexAttribI1iv.load(&mut loadfn, "glVertexAttribI1iv");
         storage::VertexAttribI1ui.load(&mut loadfn, "glVertexAttribI1ui");
         storage::VertexAttribI1uiv.load(&mut loadfn, "glVertexAttribI1uiv");
         storage::VertexAttribI2i.load(&mut loadfn, "glVertexAttribI2i");
         storage::VertexAttribI2iv.load(&mut loadfn, "glVertexAttribI2iv");
         storage::VertexAttribI2ui.load(&mut loadfn, "glVertexAttribI2ui");
         storage::VertexAttribI2uiv.load(&mut loadfn, "glVertexAttribI2uiv");
         storage::VertexAttribI3i.load(&mut loadfn, "glVertexAttribI3i");
         storage::VertexAttribI3iv.load(&mut loadfn, "glVertexAttribI3iv");
         storage::VertexAttribI3ui.load(&mut loadfn, "glVertexAttribI3ui");
         storage::VertexAttribI3uiv.load(&mut loadfn, "glVertexAttribI3uiv");
         storage::VertexAttribI4bv.load(&mut loadfn, "glVertexAttribI4bv");
         storage::VertexAttribI4i.load(&mut loadfn, "glVertexAttribI4i");
         storage::VertexAttribI4iv.load(&mut loadfn, "glVertexAttribI4iv");
         storage::VertexAttribI4sv.load(&mut loadfn, "glVertexAttribI4sv");
         storage::VertexAttribI4ubv.load(&mut loadfn, "glVertexAttribI4ubv");
         storage::VertexAttribI4ui.load(&mut loadfn, "glVertexAttribI4ui");
         storage::VertexAttribI4uiv.load(&mut loadfn, "glVertexAttribI4uiv");
         storage::VertexAttribI4usv.load(&mut loadfn, "glVertexAttribI4usv");
         storage::VertexAttribIFormat.load(&mut loadfn, "glVertexAttribIFormat");
         storage::VertexAttribIPointer.load(&mut loadfn, "glVertexAttribIPointer");
         storage::VertexAttribL1d.load(&mut loadfn, "glVertexAttribL1d");
         storage::VertexAttribL1dv.load(&mut loadfn, "glVertexAttribL1dv");
         storage::VertexAttribL2d.load(&mut loadfn, "glVertexAttribL2d");
         storage::VertexAttribL2dv.load(&mut loadfn, "glVertexAttribL2dv");
         storage::VertexAttribL3d.load(&mut loadfn, "glVertexAttribL3d");
         storage::VertexAttribL3dv.load(&mut loadfn, "glVertexAttribL3dv");
         storage::VertexAttribL4d.load(&mut loadfn, "glVertexAttribL4d");
         storage::VertexAttribL4dv.load(&mut loadfn, "glVertexAttribL4dv");
         storage::VertexAttribLFormat.load(&mut loadfn, "glVertexAttribLFormat");
         storage::VertexAttribLPointer.load(&mut loadfn, "glVertexAttribLPointer");
         storage::VertexAttribP1ui.load(&mut loadfn, "glVertexAttribP1ui");
         storage::VertexAttribP1uiv.load(&mut loadfn, "glVertexAttribP1uiv");
         storage::VertexAttribP2ui.load(&mut loadfn, "glVertexAttribP2ui");
         storage::VertexAttribP2uiv.load(&mut loadfn, "glVertexAttribP2uiv");
         storage::VertexAttribP3ui.load(&mut loadfn, "glVertexAttribP3ui");
         storage::VertexAttribP3uiv.load(&mut loadfn, "glVertexAttribP3uiv");
         storage::VertexAttribP4ui.load(&mut loadfn, "glVertexAttribP4ui");
         storage::VertexAttribP4uiv.load(&mut loadfn, "glVertexAttribP4uiv");
         storage::VertexAttribPointer.load(&mut loadfn, "glVertexAttribPointer");
         storage::VertexBindingDivisor.load(&mut loadfn, "glVertexBindingDivisor");
         storage::Viewport.load(&mut loadfn, "glViewport");
         storage::ViewportArrayv.load(&mut loadfn, "glViewportArrayv");
         storage::ViewportIndexedf.load(&mut loadfn, "glViewportIndexedf");
         storage::ViewportIndexedfv.load(&mut loadfn, "glViewportIndexedfv");
         storage::WaitSync.load(&mut loadfn, "glWaitSync");

    }
}


extern crate gl;
extern crate gl_generator;
extern crate nalgebra_glm as glm;
use super::vbo;
use super::gl_error;

pub struct Shader{
    program : gl::types::GLuint,
}


fn GetError(id : gl::types::GLuint) -> String { 
    let mut len_glint : gl::types::GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len_glint);
    }
    let len : usize = len_glint as usize;

    let mut buffer: Vec<u8> = Vec::with_capacity( len + 1 );
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe {
        gl::GetShaderInfoLog(
        id,
        len as gl::types::GLint,
        std::ptr::null_mut(),
        buffer.as_ptr() as *mut gl::types::GLchar);
    }
    let as_string : CString =  unsafe { CString::from_vec_unchecked(buffer) };
    return as_string.to_string_lossy().into_owned();
}



fn CompileShader(shader : &CStr, id : gl::types::GLuint) -> bool{
    unsafe{
        gl::ShaderSource(id, 1, &shader.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        let mut success: gl::types::GLint = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        return success != 0; 
    }
}
 
fn Compile(vert : &CStr, frag : &CStr) -> Result<gl::types::GLuint,String> {
    let vert_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let frag_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    if !CompileShader(vert, vert_id) {
        let err = "Vertex Compilation Failed:\n ".to_string() + &GetError(vert_id);
        return Err(err.to_string());
    }
    if !CompileShader(frag, frag_id) {
        let err = "Fragment Compilation Failed:\n ".to_string() + &GetError(frag_id);
        return Err(err.to_string());
    }
    let program = unsafe {gl::CreateProgram()};
    
    let mut link_status : gl::types::GLint = 0;
    unsafe{
        gl::AttachShader(program, vert_id);
        gl::AttachShader(program, frag_id);
        gl::LinkProgram(program);
        gl::GetProgramiv(program, gl::LINK_STATUS,&mut link_status );
    }
    if link_status == 0 {
        let err = "Linking Failed: ".to_string() + &GetError(program);
        return Err(err);
    }

    match gl_error::GetError() {
        Some(x) => return Err(x),
        _ => (),
    };
    return Ok(program);
}





use std::ffi::{CString, CStr};
impl Shader{
    pub fn new(vert : &str, frag : &str) -> Result<Shader,String> {
        let vert_str = CString::new(vert).unwrap();
        let frag_str = CString::new(frag).unwrap();
        let program  = Compile(&vert_str, &frag_str)?;
        return Ok(Self{program : program})
    }
    
    pub fn Enable(&self){
        unsafe{
            gl::UseProgram(self.program);
        }
    }

     pub fn FindUniform(&self, name : &str) -> Option<gl::types::GLint>{
         unsafe{
            let name_str = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program,name_str.as_ptr());
            if location == -1 {
                return None;
            }
            return Some(location);
        }
    }
    
     pub fn FindAttribute(&self, name : &str) -> Option<gl::types::GLint>{
         unsafe{
             let name_str = CString::new(name).unwrap();
             let location = gl::GetAttribLocation(self.program,name_str.as_ptr());
             if location == -1{
                 return None;
             }
             return Some(location);
         }
     }

     pub fn UniformMat4(&self,u : gl::types::GLint, mat : &glm::Mat4 ){
         unsafe{
             gl::UniformMatrix4fv(u,1,gl::FALSE, glm::value_ptr(mat).as_ptr() );
         }
         gl_error::PrintError();
     }
    
     pub fn UniformVec4(&self, u : gl::types::GLint, vec : &glm::Vec4){
         unsafe{
             gl::Uniform4fv(u,1,glm::value_ptr(vec).as_ptr());
         }
         gl_error::PrintError();
     }
     
     pub fn Attribute(&self,a : gl::types::GLuint, buffer : &vbo::VboView ){
         unsafe{
             gl::EnableVertexAttribArray(a); 
             gl::VertexAttribPointer(a,buffer.width as i32,gl::FLOAT,gl::FALSE,buffer.stride as i32, 
                 buffer.start as *const gl::types::GLvoid);
             //gl::VertexAttribPointer(a,buffer.Width(),gl::FLOAT,gl::FALSE,0,std::ptr::null()); 
         } 
         gl_error::PrintError();
     }
}

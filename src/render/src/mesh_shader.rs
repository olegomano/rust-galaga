extern crate gl;
extern crate nalgebra_glm as glm;
use super::shader;
use super::vbo;
use super::gl_error;
use super::texture;
use super::material;

fn GetUniformLocation( program_id : gl::types::GLuint, name : &str) -> Option<gl::types::GLint> {
    return shader::FindUniform(program_id,name);
} 


fn GetAttributeLocation( program_id : gl::types::GLuint, name : &str) -> Option<gl::types::GLint> {
    return shader::FindAttribute(program_id,name);
} 

fn CompileShader(vertex : &str,  fragment : &str) -> Result<gl::types::GLuint, String> {
    return shader::Compile(vertex, fragment);
}

static vert_shader : &str =  
    "#version 300 es
     uniform mat4 u_mat4x4_modelmat;
     uniform mat4 u_mat4x4_camera;
     uniform vec4 u_vec4_camera_params;
     in vec4 a_vec4_vertex;
     in vec4 a_vec4_normal;
     
     out vec4 v_normal;

     void main() {
        mat4 camera_space = u_mat4x4_camera * u_mat4x4_modelmat;
        vec4 vert_pos = a_vec4_vertex;
        v_normal = camera_space * a_vec4_normal;
        gl_Position = camera_space * vert_pos;
        gl_Position.w = (u_vec4_camera_params.x + gl_Position.z) / u_vec4_camera_params.x;
        gl_Position.z = (vert_pos.z + 1.0) / 100.0;
     }";

static frag_shader : &str =
    "#version 300 es
    precision mediump float;
     in vec4  v_normal;
     out vec4 FragColor;
     void main() { 
        FragColor = vec4(1.0,1.0,1.0,1.0) * dot(v_normal.xyz, vec3(1,0,0)) ;
        FragColor.a  = 1.0;
     }";


pub struct MeshInstace{
    gl_buffer : vbo::Vbo,
    
    vertex : vbo::VboView,
    normal : vbo::VboView,
    uv : vbo::VboView,

    pub transform : glm::Mat4,
}

impl MeshInstace {
    pub fn new(buffer : &[f32]) -> Self{
        let gl_buffer = vbo::Vbo::new(buffer);
        
        let vertex  = gl_buffer.MakeView(0, 4, 32, (buffer.len() / 4) as u32  );
        let normal  = gl_buffer.MakeView(4, 4, 32, (buffer.len() / 4) as u32  );
        let uv      = gl_buffer.MakeView(0, 4, 0, (buffer.len() / 4) as u32  );

        return Self{
            transform : glm::identity(),
            gl_buffer : gl_buffer,
            vertex : vertex,
            normal : normal,
            uv : uv,
        }
    }

    pub fn Info() -> String {
        let mut result : String = "".to_owned();
        return result;
    }
}

#[derive(Default)]
#[derive(asset_gen::GlBinding)]
pub struct GLBindings{
    u_mat4x4_modelmat : gl::types::GLint,
    u_mat4x4_camera : gl::types::GLint,
    u_vec4_camera_params : gl::types::GLint,
    a_vec4_vertex : gl::types::GLint,
    a_vec4_normal : gl::types::GLint,
    program_id : gl::types::GLuint,
}

pub struct MeshShader {
    binding : GLBindings,
    camera : glm::Mat4,
}


impl MeshShader{
    pub fn new() -> Result<MeshShader,String>{
        return Ok(Self{    
            binding : GLBindings::new(vert_shader,frag_shader).unwrap(),
            camera : glm::identity(),
        })
    }

    pub fn UpdateCamera(&mut self, camera : glm::Mat4){
        self.camera = camera;
    }

    pub fn Render(&mut self, mesh : &MeshInstace){
        let camera_params : glm::Vec4 = glm::vec4(3.0,0.0,0.0,0.0);
        mesh.gl_buffer.Bind();
        shader::UseProgram(self.binding.program_id);
        shader::UniformMat4(self.binding.u_mat4x4_camera, &self.camera);
        shader::UniformMat4(self.binding.u_mat4x4_modelmat, &mesh.transform);
        shader::UniformVec4(self.binding.u_vec4_camera_params, &camera_params);
        
        println!("Rendering {} tris",mesh.vertex.count);
        shader::Attribute(self.binding.a_vec4_vertex as gl::types::GLuint , &mesh.vertex); 
        shader::Attribute(self.binding.a_vec4_normal as gl::types::GLuint , &mesh.normal); 
        unsafe{
            gl::DrawArrays(gl::TRIANGLES,0,mesh.vertex.count as i32);
            gl_error::PrintError(); 
        }
        mesh.gl_buffer.Unbind();

    }

}


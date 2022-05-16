extern crate gl;
extern crate nalgebra_glm as glm;
use super::shader;
use super::vbo;
use super::gl_error;
use super::texture;
use super::material;

static vert_shader : &str =  
    "#version 300 es
     uniform mat4 u_mat4x4_modelmat;
     in vec4 a_vec4_vertex;
     in vec2 a_vec2_uv;

     out vec2 uv;
     void main() {
        uv = a_vec2_uv;
        gl_Position = u_mat4x4_modelmat * a_vec4_vertex;
     }";

static frag_shader : &str =
    "#version 300 es
     precision mediump float;
     uniform sampler2D t_diffuse;
     uniform sampler2D t_displace;

     in vec2 uv;
     out vec4 FragColor;
     void main() { 
        FragColor = texture(t_diffuse,uv) + texture(t_displace,uv);
     }";

static quad_vert : &[f32] = &[
    -0.5,-0.5,0.0,1.0,  0.0,0.0,
    -0.5, 0.5,0.0,1.0,  0.0,1.0,
     0.5, 0.5,0.0,1.0,  1.0,1.0,

     0.5, 0.5,0.0,1.0,  1.0, 1.0,
     0.5,-0.5,0.0,1.0,  1.0, 0.0,
    -0.5,-0.5,0.0,1.0,  0.0, 0.0
];


fn GetUniformLocation( program_id : gl::types::GLuint, name : &str) -> Option<gl::types::GLint> {
    return shader::FindUniform(program_id,name);
} 


fn GetAttributeLocation( program_id : gl::types::GLuint, name : &str) -> Option<gl::types::GLint> {
    return shader::FindAttribute(program_id,name);
} 

fn CompileShader(vertex : &str,  fragment : &str) -> Result<gl::types::GLuint, String> {
    return shader::Compile(vertex, fragment);
}

#[derive(Default)]
#[derive(asset_gen::GlBinding)]
pub struct GLBindings{
    u_mat4x4_modelmat : gl::types::GLint,
    t_diffuse : gl::types::GLint,
    t_displace : gl::types::GLint,
    a_vec4_vertex : gl::types::GLint,
    a_vec2_uv : gl::types::GLint,
    program_id : gl::types::GLuint,
}

pub struct SpriteShader{
    quad_vbo : vbo::Vbo,
    binding : GLBindings,
}

impl SpriteShader{
    pub fn new() -> Result<SpriteShader,String>
    {
        let vbo = vbo::Vbo::new(quad_vert);
        let mut binding : GLBindings = GLBindings::new(vert_shader,frag_shader).unwrap();
        println!("{}",binding.GetGlInfo());
        return Ok(Self{
            quad_vbo : vbo,
            binding : binding,
        })
    }

    pub fn Render( 
            &mut self,
            transform : &glm::Mat4, 
            texture: &material::Material){

        shader::UseProgram(self.binding.program_id);
        shader::UniformMat4(self.binding.u_mat4x4_modelmat, transform);
        self.quad_vbo.Bind(); 
        
        texture.Diffuse().Bind(gl::TEXTURE0);
        texture.Displace().Bind(gl::TEXTURE1);
        shader::Uniform1i(self.binding.t_diffuse,0);
        shader::Uniform1i(self.binding.t_displace,1);

        shader::Attribute(self.binding.a_vec4_vertex as gl::types::GLuint , &self.quad_vbo.MakeView(0,4,24,146));
        shader::Attribute(self.binding.a_vec2_uv as gl::types::GLuint , &self.quad_vbo.MakeView(16,2,24,146));
        
        unsafe{
            gl::DrawArrays(gl::TRIANGLES,0,6);
            gl_error::PrintError(); 
        }
    }
}

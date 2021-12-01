extern crate gl;
extern crate nalgebra_glm as glm;
use super::shader;
use super::vbo;
use super::gl_error;

static vert_shader : &str =  
    "#version 300 es
     uniform mat4 MVPMatrix;
     in vec4 vPosition;
     in vec2 vUv;

     out vec2 uv;
     void main() {
        uv = vUv;
        gl_Position = MVPMatrix * vPosition;
     }";

static frag_shader : &str =
    "#version 300 es
     precision mediump float;
     uniform vec4 vColor;
     in vec2 uv;
     out vec4 FragColor;
     void main() { 
        FragColor = vColor;
        FragColor.x = uv.x;
        FragColor.y = uv.y;
        FragColor.z = uv.x * uv.y;
     }";

static quad_vert : &[f32] = &[
    -1.0,-1.0,0.0,1.0,  0.0,0.0,
    -1.0, 1.0,0.0,1.0,  0.0,1.0,
     1.0, 1.0,0.0,1.0,  1.0,1.0,

     1.0, 1.0,0.0,1.0,  1.0, 1.0,
     1.0,-1.0,0.0,1.0,  1.0, 0.0,
    -1.0,-1.0,0.0,1.0,  0.0, 0.0
];

pub struct SpriteShader{
    shader : shader::Shader,
    u_model_matrix : gl::types::GLint,
    u_color : gl::types::GLint,
    a_vertex : gl::types::GLint,
    a_uv : gl::types::GLint,
    quad_vbo : vbo::Vbo,
}

impl SpriteShader{
    pub fn new() -> Result<SpriteShader,String>
    {
        let shader = shader::Shader::new(vert_shader,frag_shader)?;
        let u_model_matrix = match shader.FindUniform("MVPMatrix"){
            Some(l) => l,
            None => return Err("Failed to find MVPMatrix".to_string()),
        };
        let u_color = match shader.FindUniform("vColor"){
            Some(l) => l,
            None => return Err("Failed to find vColor".to_string()),
        };
        let a_vertex = match shader.FindAttribute("vPosition"){
            Some(l) => l,
            None => return Err("Failed to find Attrbute vPosition".to_string()),
        };
        let a_uv = match shader.FindAttribute("vUv") {
            Some(l) => l,
            None => return Err("Failed to find Attrbute vUv".to_string()),
        };
        
        let vbo = vbo::Vbo::new(quad_vert);
        return Ok(Self{
            shader : shader,
            u_model_matrix : u_model_matrix,
            u_color : u_color,
            a_vertex : a_vertex,
            a_uv : a_uv,
            quad_vbo : vbo
        })
    }

    pub fn Render( &mut self,transform : &glm::Mat4){
        self.shader.Enable();
        self.shader.UniformMat4(self.u_model_matrix, transform);
        let color : glm::Vec4= glm::Vec4::new(1.0,1.0,1.0,1.0);
        self.quad_vbo.Bind();
        self.shader.UniformVec4(self.u_color, &color);
        self.shader.Attribute(self.a_vertex as gl::types::GLuint , &self.quad_vbo.MakeView(0,4,24,146));
        self.shader.Attribute(self.a_uv as gl::types::GLuint , &self.quad_vbo.MakeView(16,2,24,146));

        unsafe{
            gl::DrawArrays(gl::TRIANGLES,0,6);
        }
        gl_error::PrintError(); 
    }
}

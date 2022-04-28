use std::time::{Duration, Instant};

#[macro_use]
extern crate asset_gen;


#[cfg(test)]
mod gl_test{
    #[derive(asset_gen::GlBinding)]
    struct test_gl_struct{
        u_model_matrix : i32,
        u_camera_matrix : i32,
        a_vec4_vertex : i32,
        a_vec4_normal : i32,
        a_vec2_uv : i32,
        some_member  : i32,
        u_uniform : i32,
        program_id : i32,
    }

    fn CompileShader(vert_shader : &str, frag_shader : &str) -> Result<i32,String>{
        println!("Compiling Shader {} {}",vert_shader,frag_shader);
        return Ok(0);
    }
    
    fn GetAttributeLocation(program_id : i32, name : &str) -> Option<i32> {
        println!("GetAttributeLocation {} {}",program_id,name);
        return Some(0);
    }

    fn GetUniformLocation(program_id : i32, name : &str) -> Option<i32>{
        println!("GetUniformLocation {} {}",program_id,name);
        return Some(0);
    }

    #[test]
    fn test_GetInfo(){
        let mut a = test_gl_struct::new("aa","bb").unwrap(); 
        println!("Struct Info: {}",a.GetInfo());
    }

    #[test]
    fn test_GetGlInfo(){
        let mut a = test_gl_struct::new("aa","bb").unwrap(); 
        a.GlBind();
        println!("Gl Info: {}",a.GetGlInfo());
    }
}



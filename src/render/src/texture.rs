extern crate asset_gen;

extern crate gl;
extern crate image;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
pub struct Texture{
    handle : gl::types::GLuint
}


fn CreateTexture() -> Option<gl::types::GLuint> {
    unsafe{
    let mut texture_id : gl::types::GLuint = 0;
        gl::GenTextures(1,&mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D,texture_id);
        //gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        //gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        //gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32); 
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); 
        return Some(texture_id);    
    }
}


fn UpdateTexture(texture : gl::types::GLuint, w : i32, h : i32,buffer : &[u8]){
    unsafe{
        println!("Updating texture {} (w,h): {} {}", texture, w,h);
        gl::BindTexture(gl::TEXTURE_2D,texture);
        gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32, w,h,0,gl::RGBA,gl::UNSIGNED_BYTE,buffer.as_ptr() as *const std::ffi::c_void);
    }   
}


impl Texture{     
    pub fn from_buffer(width : u32, height : u32, buffer : &[u8]) -> Result<Self,String>{
        let texture = CreateTexture().unwrap();
        UpdateTexture(texture,width as i32,height as i32,buffer);
        return Ok(Self{
            handle : texture
        });
    }
    
   #[asset_gen::timed]
   pub fn from_path(path : &str) -> Result<Self,String>{
        let image = image::open(path).unwrap().into_rgba8();
        let buffer : &[u8] = image.as_raw();  
        println!("Loaded image {} of size {} {}", path, image.width(), image.height());
        return Texture::from_buffer(image.width(),image.height(),buffer);
    }

    pub fn Bind(&self, texture_index : gl::types::GLuint){
        unsafe{
            gl::ActiveTexture(texture_index);
            gl::BindTexture(gl::TEXTURE_2D,self.handle);
        }
    }
}

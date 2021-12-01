extern crate gl;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

struct Texture{
    handle : gl::types::GLuint
}


fn CreateTexture() -> Option<gl::types::GLuint> {
    unsafe{
    let mut texture_id : gl::types::GLuint = 0;
        gl::GenTextures(1,&mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D,texture_id);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32); 
        return Some(texture_id);    
    }
}


fn UpdateTexture(texture : gl::types::GLuint, w : i32, h : i32,buffer : &[u8]){
    unsafe{
        gl::BindTexture(gl::TEXTURE_2D,texture);
        gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32, w,h,0,gl::RGBA,gl::UNSIGNED_BYTE,buffer.as_ptr() as *const std::ffi::c_void);
    }   
}

static sample_texture : &[u8] = &[
    255,0,0,255, 0,255,0,255,
    0,0,255,255, 255,0,0,255
];


fn SampleTexture() -> Result<Texture,String> {
   return Texture::from_buffer(2,2,sample_texture);
}

impl Texture{
    fn from_path(path : &str) -> Result<Self,String>{
      let file = File::open(path).unwrap();
      let mut reader = BufReader::new(file);
      let mut buffer : Vec<u8> = Vec::new();
      return Texture::from_buffer(128,128,&buffer);
      //reader.read_to_end(&mut buffer);
      //return from_buffer(&buffer);
    }

    fn from_buffer(width : i32, height : i32, buffer : &[u8]) -> Result<Self,String>{
        let texture = CreateTexture().unwrap();
        UpdateTexture(texture,width,height,buffer);
        return Ok(Self{
            handle : texture
        });
    }
}

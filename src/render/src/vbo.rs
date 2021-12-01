extern crate gl;

#[derive(Copy,Clone,Debug)]
pub struct Vbo{
    handle : gl::types::GLuint,
    size : u32,
}

#[derive(Copy,Clone,Debug)]
pub struct VboView{
    owner : Vbo,
    pub width : u32,
    pub start : u32,
    pub stride : u32,
    pub count : u32,
}

impl Vbo{

    pub fn new<T>(buffer: &[T]) -> Vbo {
        let mut handle : gl::types::GLuint= 0;
        unsafe{
            gl::GenBuffers(1, &mut handle as *mut _ );  
        }
        let mut res =  Self {handle, size : (  buffer.len() * std::mem::size_of::<T>()  )as u32};
        res.Write(buffer);
        return res;
    }
    
    pub fn DefaultView(&self, width : u32) -> VboView{
        return self.MakeView(0,width,0,self.size);
    }

    pub fn MakeView(&self, start : u32, width : u32, stride : u32, count : u32) -> VboView{
        return VboView{
            owner : *self,
            start  : start,
            width : width,
            stride : stride,
            count : count,
        }
    }

    pub fn Bind(&self){
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER,self.handle); 
        }
    }

    pub fn Unbind(&self){
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER,0);
        }
    }

    pub fn Size(&self) -> i32{
        return self.size as i32;
    }

    fn Write<T>(&mut self, buffer : &[T]) {
        let len  = buffer.len();
        let size = buffer.len() * std::mem::size_of::<T>(); 

        println!("Writing vbo {},  len: {}, size: {}",self.handle, len,size);
        unsafe{
        gl::BindBuffer(gl::ARRAY_BUFFER, self.handle);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            size as gl::types::GLsizeiptr, // size of data in bytes
            buffer.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

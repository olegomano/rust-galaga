extern crate gl;

pub fn GetError() -> Option<String>{

    let mut has_error = false;
    let mut error_string : String = "".to_string();

    unsafe{
        loop{
            let error = gl::GetError();
            if(error == gl::NO_ERROR){
                if(has_error){
                    return Some(error_string);
                }else{
                    return None
                }
            }
            
            has_error = true;
            let error = match error {
                    gl::INVALID_ENUM => "GL_INVALID_ENUM",
                    gl::INVALID_VALUE => "GL_INVALID_VALUE",
                    gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
                    gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                    gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
                    gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
                    gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
                    _ => "unknown error"
            };
            error_string.push_str(error)

        }
    }
}

pub fn PrintError() {
    match GetError(){
        Some(x) => println!("GLError: {}", x),
        None => {},
    };
}

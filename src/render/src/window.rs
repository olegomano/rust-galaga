extern crate gl;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use std::process;

pub trait FrameHandler{
    fn HandleFrame(&mut self);
}

pub trait InputHandler{
    fn HandleInput(&mut self, event : &sdl2::event::Event);
}

pub struct SDLContext {
    context : sdl2::Sdl,
    events : sdl2::EventPump,
    window : sdl2::video::Window,
    gl : sdl2::video::GLContext    
}

impl SDLContext {
    pub fn new() -> Self{
        let w = 200;
        let h = 400;

        let context = sdl2::init().expect("SDL initialization failed");
        let video_subsystem = context.video().expect("Couldn't get SDL video subsystem");
        ///video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
        video_subsystem.gl_attr().set_context_major_version(3);
        video_subsystem.gl_attr().set_context_minor_version(0);

        let mut window = video_subsystem.window("sdl2 window",w,h)
            .position_centered()
            .opengl()
            .build()
            .unwrap(); 
         
        let gl_context = window.gl_create_context().unwrap();
        assert!(gl_context.is_current()); //don't think this should ever happen
        
        let gl = std::rc::Rc::new(gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const _
        }));

        unsafe{
            gl::Viewport(0, 0, w as i32, h as i32); // set viewport
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        let mut events = context.event_pump().expect("Failed to get SDL event pump");
        let mut vao : gl::types::GLuint = 0; 
        unsafe {
            gl::GenVertexArrays(1, &mut vao as *mut _); 
            gl::BindVertexArray(vao);
        }
        //window.maximize();
        //window.set_fullscreen(sdl2::video::FullscreenType::Desktop);
        return Self{
            context,
            events,
            window,
            gl : gl_context,
        }
    }

    pub fn update(&mut self, frame_handler :  &mut dyn  FrameHandler, event_handler : &mut dyn InputHandler){
        for event in self.events.poll_iter(){
            match event {
                Event::Quit {..} => {  process::exit(0x100) },
                _=>{ 
                    event_handler.HandleInput(&event);
                }
            }
        }
        unsafe{
            gl::Clear(gl::COLOR_BUFFER_BIT);
        } 
        frame_handler.HandleFrame();
        self.window.gl_swap_window();
        sleep(Duration::new(0,1_000_000_000u32/60));
    }
}



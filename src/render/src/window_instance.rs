use super::window;
use std::pin::Pin;
use std::marker::PhantomPinned;


#[macro_export]
macro_rules! Build {
    ($type_name:ty) => {
        struct WindowInstance {
            this : *mut WindowInstance,
            sdl  : window::SDLContext,
            instance : $type_name,
            _pin : PhantomPinned,
        }

        impl WindowInstance{
            fn OnFrame(&mut self){
                unsafe{
                    self.sdl.update(&mut *self.this,&mut *self.this);
                }
            }
        }
 
        impl render::window::FrameHandler for WindowInstance{
            fn HandleFrame(&mut self){
                self.instance.HandleFrame();
            }
        }

        impl render::window::InputHandler for WindowInstance{
            fn HandleInput(&mut self, event: &sdl2::event::Event) {
                self.instance.HandleInput(event);
            }
        }


        struct Window{
            instance : Pin<Box<WindowInstance>>,
        }

        impl Window{
            fn new() -> Self{
                let mut instance : WindowInstance = WindowInstance{
                    this : std::ptr::null_mut(),
                    sdl  : window::SDLContext::new(),
                    instance : <$type_name>::new(),
                    _pin : PhantomPinned,
                };
                
                let mut boxed = Box::pin(instance);
                unsafe{
                    let mut_ref = Pin::as_mut(&mut boxed);
                    let ptr = Pin::get_unchecked_mut(mut_ref);
                    (*ptr).this = ptr;
                }
                
                return Self{
                    instance : boxed
                };
            }

            fn Instance(&mut self) -> &mut WindowInstance{
                unsafe{
                    let mut_ref = Pin::as_mut(&mut self.instance);
                    let ptr  = Pin::get_unchecked_mut(mut_ref);
                    return ptr;
                }
            }
        }

        impl emscripten_main_loop::MainLoop for Window {
            fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
                let instance = self.Instance();
                instance.OnFrame();
                return emscripten_main_loop::MainLoopEvent::Continue;     
            }       
        }
   
   }
}


        /*
              */


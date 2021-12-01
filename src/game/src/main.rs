use render;
use render::window;
use render::sprite_shader::SpriteShader;
use sdl2::event::Event;
use std::pin::Pin;
use std::marker::PhantomPinned;
extern crate nalgebra_glm as glm;

struct Game{
    sprite_shader : render::sprite_shader::SpriteShader,
    transform : glm::Mat4,
    count : f32
}

impl Game {
    fn new() -> Self{
        let mut transform = glm::identity();
        transform = glm::scale(&transform, &glm::vec3(0.5, 0.5,0.5));

        return Self{
            sprite_shader : render::sprite_shader::SpriteShader::new().unwrap(),
            transform : transform,
            count : 0.0,
        }
    }
}

impl render::window::FrameHandler for Game{
    fn HandleFrame(&mut self){
        self.transform = glm::rotate_z(&self.transform,0.001);
        self.sprite_shader.Render(&self.transform);      
        self.count = self.count + 1.0;
    }
}

impl render::window::InputHandler for Game{
    fn HandleInput(&mut self, event: &sdl2::event::Event) {
    }
}


render::Build!(Game);

fn main() {
    let mut game = Window::new();
    println!("Hello, world!");
    emscripten_main_loop::run( game );
}

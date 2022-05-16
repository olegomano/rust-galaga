use render;
use render::material;
use render::texture;
use render::window;
use render::sprite_shader::SpriteShader;
use sdl2::event::Event;
use std::pin::Pin;
use std::marker::PhantomPinned;
use render::asset_index;
use render::asset;
use std::rc::Rc;

extern crate nalgebra_glm as glm;

struct Game{
    sprite_shader : render::sprite_shader::SpriteShader,
    transform : glm::Mat4,
    count : f32,
    texture : material::Material,
    asset_index : Rc<asset_index::AssetIndex>,
}


impl Game {
    fn new() -> Self{
        let mut transform = glm::identity();
        transform = glm::scale(&transform, &glm::vec3(0.8, 0.8,0.8));
        
        let asset_index = Rc::new(asset_index::AssetIndex::new());
        let asset = asset::Asset::new("txt_diffuse.rbga",  asset_index.clone());

        return Self{
            sprite_shader : render::sprite_shader::SpriteShader::new().unwrap(),
            transform : transform,
            count : 0.0,
            texture : material::Material::from_asset(&asset).unwrap(),
            asset_index : asset_index,
        }
    }
}

impl render::window::FrameHandler for Game{
    fn HandleFrame(&mut self){
        self.transform = glm::rotate_z(&self.transform,0.001);
        self.sprite_shader.Render(&self.transform, &self.texture);      
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

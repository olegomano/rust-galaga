use render;
use render::material;
use render::texture;
use render::window;
use render::sprite_shader::SpriteShader;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::pin::Pin;
use std::marker::PhantomPinned;
use render::asset_index;
use render::asset;
use std::rc::Rc;
use std::cell::RefCell;
use byteorder::{BigEndian, LittleEndian,ReadBytesExt}; // 1.2.7
use byte_slice_cast::*;
use std::collections::HashMap;

extern crate nalgebra_glm as glm;

struct Game{
    sprite_shader : render::sprite_shader::SpriteShader,
    mesh_shader : render::mesh_shader::MeshShader,
    transform : glm::Mat4,
    count : f32,
    texture : material::Material,
    asset_index : Rc<asset_index::AssetIndex>,
    mesh_map : HashMap<String,Rc<RefCell<render::mesh_shader::MeshInstace>>>,
}


impl Game {
    fn new() -> Self{
        let mut transform = glm::identity();
        //transform = glm::scale(&transform, &glm::vec3(0.01, 0.01,0.01));
        
        let asset_index = Rc::new(asset_index::AssetIndex::new());
        let asset = asset::Asset::new("txt_diffuse.rbga",  asset_index.clone());
        let mut mesh_map : HashMap<String,Rc<RefCell<render::mesh_shader::MeshInstace>>> = HashMap::new();
 
        let cube_mesh = asset::Asset::new("tavern.mesh", asset_index.clone());
        let mut mesh = render::mesh_shader::MeshInstace::new(cube_mesh.Buffer().as_slice_of::<f32>().unwrap()); 
        mesh_map.insert("tavern.mesh".to_owned(), Rc::new(RefCell::new(mesh) ));

        return Self{
            sprite_shader : render::sprite_shader::SpriteShader::new().unwrap(),
            mesh_shader : render::mesh_shader::MeshShader::new().unwrap(),
            transform : transform,
            count : 0.0,
            texture : material::Material::from_asset(&asset).unwrap(),
            asset_index : asset_index,
            mesh_map : mesh_map, 
        }
    }
}

impl render::window::FrameHandler for Game{
    fn HandleFrame(&mut self){
        self.transform = glm::rotate_z(&self.transform,0.01);
        self.transform = glm::rotate_x(&self.transform, 0.01);
        //self.sprite_shader.Render(&self.transform, &self.texture);      
        self.count = self.count + 1.0;
        for (path,mesh) in &mut self.mesh_map {
            mesh.borrow_mut().transform = self.transform;
            self.mesh_shader.Render(&mesh.borrow_mut());
        }
    }
}


impl render::window::InputHandler for Game{
    fn HandleInput(&mut self, event: &Event) {
        match event{
            Event::KeyDown{
                keycode : Some(Keycode::W), ..
            } => {
                self.transform = glm::translate(&self.transform, &glm::vec3(0.01,0.00,0.00));
            },
            Event::KeyDown{
                keycode : Some(Keycode::S), ..
            } => {
                self.transform = glm::translate(&self.transform, &glm::vec3(-0.01,0.00,0.00));
            }, 
            Event::KeyDown{
                keycode  : Some(Keycode::LShift), ..
            } => {
                self.transform = glm::scale(&self.transform, &glm::vec3(0.95, 0.95,0.95));
            },
            Event::KeyDown{
                keycode  : Some(Keycode::LCtrl), ..
            } => {
                self.transform = glm::scale(&self.transform, &glm::vec3(1.11, 1.11,1.11));
            },
            _=>{}
        }
    }
}


render::Build!(Game);

fn main() {
    let mut game = Window::new();
    println!("Hello, world!");
    emscripten_main_loop::run( game );
}

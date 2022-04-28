extern crate gl;
extern crate gl_generator;
extern crate nalgebra_glm as glm;

use super::texture;
use super::gl_error;

#[derive(Debug, Copy, Clone)]
pub struct Material{
    diffuse : texture::Texture,
    displace : texture::Texture
}


impl Material{
    /*
     * Loads material with name *name* from dir *dir*
     *
     * Assumes naming convention name_diffuse.ext, name_displace.ext
     */
    pub fn from_dir(dir : &str, name : &str, ext : &str) -> Option<Self>{
        let diffuse  = texture::Texture::from_path(&format!("{}/{}_diffuse.{}",dir,name,ext)).expect("");
        let displace = texture::Texture::from_path(&format!("{}/{}_displace.{}",dir,name,ext)).expect("");
        return Some(Self{
            diffuse : diffuse,
            displace : displace
        })
    }

    pub fn Diffuse(&self) -> texture::Texture {
        return self.diffuse;
    }

    pub fn Displace(&self) -> texture::Texture{
        return self.displace;
    }
}

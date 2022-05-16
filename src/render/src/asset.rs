use super::asset_index;
use std::rc::Rc;

pub struct Asset{
    asset_index : Rc<asset_index::AssetIndex>,
    name : String,
}

impl Asset{
    pub fn new(name : &str,asset_index : Rc<asset_index::AssetIndex>) -> Self{
        return Self{
            name : name.to_owned(),
            asset_index : asset_index
        }
    }
    
    pub fn Buffer(&self) -> &'static [u8] {
        return self.asset_index.index[&self.name]
    }

    pub fn Name(&self) -> &str{
        return &self.name;
    }
}

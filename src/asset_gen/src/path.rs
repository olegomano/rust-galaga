use std::fs;

fn AssetFolder() -> String {
    return "./asset".to_owned();
}

fn Asset(name : &str) -> String {
    return format!("{}/{}", AssetFolder(), name);
}

fn LoadFile(path : &str) -> Option<Vec<u8>> {
    match fs::read(str){
        Ok(r) => return Some(r),
        Err(e) => {
            print!("Error reading {}: {}",path,e);
            return None
        }
        
    }
}

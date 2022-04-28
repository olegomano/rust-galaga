fn AssetFolder() -> String {
    return "./asset".to_owned();
}

fn Asset(name : &str) -> String {
    return format!("{}/{}", AssetFolder(), name);
}



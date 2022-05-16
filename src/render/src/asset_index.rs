const txt_displace:&[u8]=include_bytes!("/home/oleg/Documents/dev/galaga/out/asset/txt_displace.rbga");
const txt_diffuse:&[u8]=include_bytes!("/home/oleg/Documents/dev/galaga/out/asset/txt_diffuse.rbga");


        use std::collections::HashMap;
        pub struct AssetIndex{
            pub index : HashMap<String,&'static [u8]> 
        }
impl AssetIndex{
pub fn new() -> Self{
    let mut asset_index : HashMap<String,&[u8]> = HashMap::new();
    asset_index.insert("txt_displace.rbga".to_owned(),txt_displace);
    asset_index.insert("txt_diffuse.rbga".to_owned(),txt_diffuse);

    return Self{
        index : asset_index,
    }
}
}

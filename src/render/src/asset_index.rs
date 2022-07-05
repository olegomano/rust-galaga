
        use std::collections::HashMap;
        use include_bytes_aligned::include_bytes_aligned;
        pub struct AssetIndex{
            pub index : HashMap<String,&'static [u8]> 
        }
static cube_mesh:&'static [u8]=include_bytes_aligned!(4,"/home/oleg/Documents/dev/galaga/out/asset/cube.mesh");
static txt_displace_rbga:&'static [u8]=include_bytes_aligned!(4,"/home/oleg/Documents/dev/galaga/out/asset/txt_displace.rbga");
static tavern_mesh:&'static [u8]=include_bytes_aligned!(4,"/home/oleg/Documents/dev/galaga/out/asset/tavern.mesh");
static plane_mesh:&'static [u8]=include_bytes_aligned!(4,"/home/oleg/Documents/dev/galaga/out/asset/plane.mesh");
static txt_diffuse_rbga:&'static [u8]=include_bytes_aligned!(4,"/home/oleg/Documents/dev/galaga/out/asset/txt_diffuse.rbga");
static folder_txt_diffuse_rbga:&'static [u8]=include_bytes_aligned!(4,"/home/oleg/Documents/dev/galaga/out/asset/folder/txt_diffuse.rbga");


        impl AssetIndex{ 
            pub fn new() -> Self {
                let mut asset_index : HashMap<String,&[u8]> = HashMap::new();
                asset_index.insert("cube.mesh".to_owned(),cube_mesh);
asset_index.insert("txt_displace.rbga".to_owned(),txt_displace_rbga);
asset_index.insert("tavern.mesh".to_owned(),tavern_mesh);
asset_index.insert("plane.mesh".to_owned(),plane_mesh);
asset_index.insert("txt_diffuse.rbga".to_owned(),txt_diffuse_rbga);
asset_index.insert("folder/txt_diffuse.rbga".to_owned(),folder_txt_diffuse_rbga);

                return Self{
                    index : asset_index
                }
            }
        }
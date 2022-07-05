import os
import pathlib
import packager

class RustAssetIndex:
    def __init__(self,asset_list,dst_file):
        self._dst_file = dst_file
        self._asset_list = asset_list

    def run(self):
        rust_string = self.generate_rust(self._asset_list)
        f = open(self._dst_file,"w")
        f.write(rust_string)
        f.close()


    def generate_rust(self,asset_list):
        struct_decl = """
        use std::collections::HashMap;
        use include_bytes_aligned::include_bytes_aligned;
        pub struct AssetIndex{
            pub index : HashMap<String,&'static [u8]> 
        }"""

        map_insert = ""
        index_decl = ""
        for asset in asset_list:
            for f in asset.output_files:
                local_path = f[len(asset.dst_root) + 1:]
                var_name = local_path.replace(".","_")
                var_name = var_name.replace("/","_")

                index_decl = index_decl + "static {}:&'static [u8]=include_bytes_aligned!(4,\"{}\");\n".format(var_name,f)
                map_insert = map_insert +"asset_index.insert(\"{}\".to_owned(),{});".format(local_path,var_name) + "\n"

        constructor = """
        impl AssetIndex{ 
            pub fn new() -> Self {
                let mut asset_index : HashMap<String,&[u8]> = HashMap::new();
                {}
                return Self{
                    index : asset_index
                }
            }
        }"""
        constructor = constructor.replace("{}",map_insert)
        return struct_decl + "\n" + index_decl + "\n" + constructor

if __name__ == "__main__":
    a = packager.AssetPackager("../asset","../out/asset/")
    a.run()
    i = RustAssetIndex(a._tracked_assets,"../src/render/src/asset_index.rs")    
    i.run()

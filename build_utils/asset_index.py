import os
import pathlib

class RustAssetIndex:
    def __init__(self,src_dir,dst_file):
        self._src_dir = str(pathlib.Path(src_dir).resolve())
        self._dst_file = str(pathlib.Path(dst_file).resolve())
    
    def run(self):
        all_assets = self.collect_assets()
        print(all_assets)
        rust_string = self.generate_rust(all_assets)
        f = open(self._dst_file,"w")
        f.write(rust_string)
        f.close()
    
    def collect_assets(self):
        all_assets = {}

        asset_root = str(pathlib.Path(self._src_dir).resolve())

        for root, dirs, files in os.walk(self._src_dir):
            for file in files:
                asset_path = pathlib.Path( str(os.path.join(root,file) ))
                asset_relative_path = str(asset_path.resolve())[len(asset_root) + 1:]
                all_assets[asset_relative_path] = asset_relative_path.replace("/","_").split(".")[0]
        return all_assets



    def generate_rust(self,asset_map):
        struct_decl = """
        use std::collections::HashMap;
        pub struct AssetIndex{
            pub index : HashMap<String,&'static [u8]> 
        }"""

        map_insert = ""
        index_decl = ""
        for f in asset_map:

            #pub const txt_displace:&[u8]=include_bytes!("/home/oleg/Documents/dev/galaga/asset/txt_displace.png");
            full_asset_path = os.path.join(self._src_dir,f)
            index_decl = index_decl + "const {}:&[u8]=include_bytes!(\"{}\");\n".format(asset_map[f],full_asset_path)
            map_insert = map_insert+"asset_index.insert(\"{}\".to_owned(),{});".format(f,asset_map[f]) + "\n"

        constructor = "impl AssetIndex{\n"
        constructor = constructor + "pub fn new() -> Self{\n"
        constructor = constructor + "let mut asset_index : HashMap<String,&[u8]> = HashMap::new();\n"
        constructor = constructor + map_insert + "\n"
        constructor = constructor + "return Self{\n"
        constructor = constructor + "   index : asset_index,\n"
        constructor = constructor + "}\n}\n}\n"
        final_string = index_decl + "\n" + struct_decl + "\n" + constructor
        return final_string

if __name__ == "__main__":
    a = RustAssetIndex("../out/asset","../src/render/src/asset_index.rs")    
    a.run()

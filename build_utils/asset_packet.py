import os
import pathlib
from dataclasses import dataclass
from PIL import Image

@dataclass
class Asset:
    name : str
    src_path : str
    dst_path : str
    ext : str
    output_files : object

    
class PngDecoder():
    def __init__(self):
        pass

    def run(self,asset : Asset):
        png = Image.open(asset.src_path).convert('RGBA')
        dst = str(os.path.join(asset.dst_path,asset.name+".rbga")) 
        f = open(dst,"wb")

        for pixel in png.getdata():
            for p in pixel:
                f.write(p.to_bytes(1,"big"))
    
class AssetPacker():
    def __init__(self,source_dir,dst_dir):
        self._source_dir = str(pathlib.Path(source_dir).resolve())
        self._dst_dir = str(pathlib.Path(dst_dir).resolve())
        self._transform_table = {
            ".png" : [PngDecoder()]
        }
        self._tracked_assets = []
        self._tracked_assets_by_ext = {}

        print(self._source_dir)
        print(self._dst_dir)

    #path is global path
    #returns path relative to the asset root
    def asset_path(self,path):
        return path[len(self._source_dir) + 1:]

    def run(self):
        for dir_path, dirs, files in os.walk(self._source_dir):
            for f in files:
                asset_path_src = pathlib.Path(os.path.join(dir_path,f))
                asset_local_path = self.asset_path(str(asset_path_src.resolve()))

                dst_path = pathlib.Path(os.path.join(self._dst_dir,asset_local_path)) 
                
                asset = Asset(
                            name = asset_path_src.stem,
                            src_path = str(asset_path_src.resolve()),
                            dst_path = str(dst_path.parents[0].resolve()),
                            ext = asset_path_src.suffix,
                            output_files = [],
                        ) 
                #os.makedirs(asset.dst_path)
                self._tracked_assets.append(asset)
                if asset.ext not in self._tracked_assets_by_ext:
                    self._tracked_assets_by_ext[asset.ext] = []
                self._tracked_assets_by_ext[asset.ext].append(asset)

        for transform_ext in self._transform_table:
            asset_list = self._tracked_assets_by_ext[transform_ext]
            if asset_list is not None:
                transform_list = self._transform_table[transform_ext]
                for asset in asset_list:
                    for transform in transform_list:
                        print("Running transform {} for asset{}".format(transform,asset))
                        transform.run(asset)

if __name__ == "__main__":
    a = AssetPacker("../asset","../out/asset/")
    a.run()

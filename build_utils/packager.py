import os
import pathlib
from dataclasses import dataclass
from PIL import Image
import collada
import numpy as np
from array import array

@dataclass
class Asset:
    name : str
    src_path : str
    local_parent :str
    ext : str
    
    dst_root : str
    output_files : object


class ColladaDecoder():
    def __init__(self):
        pass

    def run(self,asset : Asset):
        mesh = collada.Collada(asset.src_path)
        scene = mesh.scene
        
        mesh_array = array('f',[])
        for node in scene.nodes:
            for geometry in node.objects("geometry"):
                for primitive in geometry.primitives():
                    for shape in primitive.shapes():
                        vertex = []
                        normal = []

                        for v in shape.vertices:
                            global_vertex = np.matmul(node.matrix,np.append(v,1))
                            vertex.append(global_vertex)

                        if shape.normals is not None:
                            for n in shape.normals:
                                global_normal = np.matmul(node.matrix,np.append(n,1))
                                normal.append(global_normal)
                        else:
                            n = np.cross(shape.vertices[1],shape.vertices[0])
                            global_normal = np.matmul(node.matrix,np.append(n,1))
                            global_normal = global_normal / np.linalg.norm(global_normal)
                            normal.append(global_normal)
                            normal.append(global_normal)
                            normal.append(global_normal)
                        
                        for i in range(0,len(vertex)):
                            for v in vertex[i]:
                                mesh_array.append(v)
                            for n in normal[i]:
                                mesh_array.append(n)
                            
        dst = str(os.path.join(asset.dst_root, asset.local_parent,asset.name+".mesh")) 
        out_file = open(dst,"wb")
        mesh_array.tofile(out_file)
        out_file.close()
        asset.output_files.append(dst)

class PngDecoder():
    def __init__(self):
        pass

    def run(self,asset : Asset):
        png = Image.open(asset.src_path).convert('RGBA')
        dst = str(os.path.join(asset.dst_root, asset.local_parent, asset.name+".rbga")) 
        
        if not os.path.exists(os.path.dirname(dst)):   # create folders if not exists
            os.makedirs(os.path.dirname(dst))

        f = open(dst,"wb")
        asset.output_files.append(dst)
        for pixel in png.getdata():
            for p in pixel:
                f.write(p.to_bytes(1,"big"))
    
class AssetPackager():
    def __init__(self,source_dir,dst_dir):
        self._source_dir = str(pathlib.Path(source_dir).resolve())
        self._dst_dir = str(pathlib.Path(dst_dir).resolve())
        self._transform_table = {
            ".png" : [PngDecoder()],
            ".dae" : [ColladaDecoder()],
        }
        self._tracked_assets = []
        self._tracked_assets_by_ext = {}
        self._dir_index = {}
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
                asset_local_path = self.asset_path(str(asset_path_src.resolve().parents[0]))
                dst_path = pathlib.Path(os.path.join(self._dst_dir,asset_local_path)) 
                
                asset = Asset(
                            name = asset_path_src.stem,
                            src_path   = str(asset_path_src.resolve()),
                            local_parent = asset_local_path,

                            dst_root = self._dst_dir,
                            ext = asset_path_src.suffix,
                            output_files = [],
                        ) 
                #os.makedirs(asset.dst_path)
                self._tracked_assets.append(asset)
                if asset.ext not in self._tracked_assets_by_ext:
                    self._tracked_assets_by_ext[asset.ext] = []
                self._tracked_assets_by_ext[asset.ext].append(asset)        
            local_dir_path = self.asset_path(str(pathlib.Path(dir_path).resolve()))
        
        for transform_ext in self._transform_table:
            asset_list = self._tracked_assets_by_ext[transform_ext]
            if asset_list is not None:
                transform_list = self._transform_table[transform_ext]
                for asset in asset_list:
                    for transform in transform_list:
                        print("Running transform {} for asset{}".format(transform,asset))
                        transform.run(asset)

if __name__ == "__main__":
    a = AssetPackager("../asset","../out/asset/")
    a.run()
    for asset in a._tracked_assets:
        print(asset)

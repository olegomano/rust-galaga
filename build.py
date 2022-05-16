import argparse
import subprocess
import os
from shutil import copy2
from shutil import copytree
from pathlib import Path

def exec(command):
    p = subprocess.Popen(
            command)
    p.communicate()
    return True,None

def package_assets(dst_folder):
    for root, subdirs, files in os.walk("./asset"):
        for f in files:
            copy2( Path("./asset",f),  Path(dst_folder,f))

#pub const BYTES: &[u8] = include_bytes!("large-binary-blob");
def build_web():
    package_assets("./out/web")
    pwd = os.getcwd()
    os.chdir("./src/")
    r,m = exec(["cargo","build", "--target=wasm32-unknown-emscripten"])
    os.chdir(pwd)

    Path("./out/web/").mkdir(parents=True, exist_ok=True)
    copy2("./index.html","./out/web/index.html")
    copy2("./src/target/wasm32-unknown-emscripten/debug/game.js","./out/web/game.js")
    copy2("./src/target/wasm32-unknown-emscripten/debug/game.wasm","./out/web/game.wasm")
    copy2("./src/target/wasm32-unknown-emscripten/debug/game.wasm.map","./out/web/game.wasm.map")
    pass

def run_web():
    exec(["python3","-m","http.server","--directory","./out/web/"]);

def build_native():
    package_assets("./out/native")
    pwd = os.getcwd()
    os.chdir("./src/")
    r,m = exec(["cargo","build"])
    os.chdir(pwd)
    

    Path("./out/native/").mkdir(parents=True, exist_ok=True)
    copy2("./src/target/debug/game","./out/native/game")

def run_native():
    exec(["./out/native/game"]);
    pass

BUILD_TARGETS = {
    "web":{
        "build" : build_web,
        "run" : run_web,
    },
    "native":{
        "build": build_native,
        "run"  : run_native,
    },
}

def create_build_subparser(parser):
    parser.set_defaults(which="build")
    parser.add_argument("--target", choices=BUILD_TARGETS.keys(),default="native")
    parser.add_argument("--skip-build", action="store_true", default = False)
    parser.add_argument("--skip-run", action="store_true", default = False)

def do_build(args):
    print("Runnning build")
    if not args.skip_build:
        BUILD_TARGETS[args.target]["build"]()
    if not args.skip_run:
        BUILD_TARGETS[args.target]["run"]()

    
def create_test_subparser(parser):
    parser.set_defaults(which="test")
    parser.add_argument("-l",nargs="+",required=False)
    pass

def do_test(args):
    pwd = os.getcwd()
    files = os.listdir("./src")
    filter_list = args.l

    for f in files:
        file_path = "./src/" + str(f)
        if os.path.isdir(file_path):
            os.chdir(file_path)
            if filter_list is None or len(filter_list) == 0 or f in filter_list:
                exec(["cargo","test","test","--","--nocapture"])
        os.chdir(pwd)
    
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Compiles and packages the game')
    subparsers = parser.add_subparsers(help="commands")
    create_build_subparser(  subparsers.add_parser("build") );
    create_test_subparser(   subparsers.add_parser("test") );

    args = parser.parse_args()
    print(args)
    if args.which == "build":
        do_build(args)
    if args.which == "test":
        do_test(args)


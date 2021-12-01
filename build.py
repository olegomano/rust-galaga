import argparse
import subprocess
import os
from shutil import copy2
from pathlib import Path

def exec(command):
    p = subprocess.Popen(
            command)
    p.communicate()
    return True,None

def build_web():
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

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Compiles and packages the game')
    parser.add_argument("--target", choices=BUILD_TARGETS.keys(),default="native")
    parser.add_argument("--action", choices=["build","run"],default="build")
    args = parser.parse_args()

    BUILD_TARGETS[args.target][args.action]() 

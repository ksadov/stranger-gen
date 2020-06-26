#!/bin/sh

wasm-pack build --target web
python3 -m http.server
rm pkg/.gitignore

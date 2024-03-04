#!/bin/bash

wasm-pack build --release --target web --out-dir www/release
basic-http-server www/
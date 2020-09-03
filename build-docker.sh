#!/usr/bin/env bash

cd frontend
npm run build

cd ..
cross build --target x86_64-unknown-linux-musl --release

docker build --rm --no-cache -t pegasis0/rust_todo:latest .

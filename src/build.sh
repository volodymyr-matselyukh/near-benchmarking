#!/bin/sh

echo ">> Building contract"

cargo build --target wasm32-unknown-unknown --release
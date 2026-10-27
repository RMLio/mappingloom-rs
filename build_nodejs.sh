#!/usr/bin/env bash

set -e

# Rust build
echo "==> Rust building and bindings"
cargo build -p translator --lib --release --features=neon

# NodeJS bindings
echo "==> NodeJS bindings"
# Install NodeJS NEON dependencies
npm i
# Execute NEON on Rust library to generate index.node file for NodeJS dynamic library
./node_modules/.bin/neon dist -n translator -v -f target/release/libtranslator.so
# Move index.node to the right folder after generation
mv index.node crates/translator/src/nodejs

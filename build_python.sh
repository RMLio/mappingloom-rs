#!/usr/bin/env bash

set -e

# Rust build
echo "==> Rust building and bindings"
cargo build -p translator --lib --release --features=pyo3

# Python bindings
echo "==> Python bindings"
# Native import, but requires renaming
cp target/release/libtranslator.so crates/translator/src/python/ltranslator.so

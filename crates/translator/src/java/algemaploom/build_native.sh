#!/bin/bash
#
# AlgeMapLoom JNI bindings native Rust library building script
# License: MIT (2024)
#
# NATIVE_APPLE=false skips the macOS libraries, for a build whose jar is only
# tested on Linux.
set -e

# Remove Rust library as resource
echo "==> Cleaning up old Rust library"
rm -rf src/main/resources/Linux/*
rm -rf src/main/resources/Windows/*
rm -rf src/main/resources/Apple-x86_64/*
rm -rf src/main/resources/Apple-aarch64/*
RESOURCES=crates/translator/src/java/algemaploom/src/main/resources
cd ../../../../../
rm -rf target

# Builds the library for a target with the given cargo command, and copies it
# into the jar's resource folder.
function build {
	echo "==> Building Rust library for $2 (log: target/$2/log.txt)"
	mkdir -p target/$2
	$1 -p translator --lib --release --features=jni --target=$2 >target/$2/log.txt 2>&1
	cp target/$2/release/$3 $RESOURCES/$4/
}

build "cargo build" x86_64-unknown-linux-gnu libtranslator.so Linux
build "cargo build" x86_64-pc-windows-gnu translator.dll Windows

# The library links no Apple framework, so off a Mac cargo-zigbuild builds it
# without the Apple SDK.
if [[ "$NATIVE_APPLE" != false ]]; then
	if [[ "$OSTYPE" == "darwin"* ]]; then
		CARGO_APPLE="cargo build"
	else
		CARGO_APPLE="cargo zigbuild"
	fi
	build "$CARGO_APPLE" x86_64-apple-darwin libtranslator.dylib Apple-x86_64
	build "$CARGO_APPLE" aarch64-apple-darwin libtranslator.dylib Apple-aarch64
fi

echo "Done!"

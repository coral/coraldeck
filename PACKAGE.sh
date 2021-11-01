#!/bin/bash
cargo build --release
mkdir -p package
cp target/release/coraldeck package/coraldeck
cp -r files package/files
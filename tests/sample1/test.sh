#!/usr/bin/env sh

rm -rf ./tmp
mkdir -p ./tmp/sample1
cargo run -- tests/sample1/src.yaml tmp/sample1/dst.rs
rustc tmp/sample1/dst.rs --crate-type lib --out-dir ./tmp/sample1

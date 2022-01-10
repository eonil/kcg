#!/usr/bin/env sh

rm -rf ./tmp
mkdir -p ./tmp/sample1/src
cargo run -- tests/sample1/src.yaml tmp/sample1/src/lib.rs --include ./tests/sample1/prelude.rs --skip Ship

cd tmp/sample1
cargo init --lib
echo 'serde = "1.0.133"' >> Cargo.toml
echo 'serde_derive = "1.0.133"' >> Cargo.toml
echo 'serde_json = "1.0.74"' >> Cargo.toml
echo 'serde_yaml = "0.8.23"' >> Cargo.toml
cargo build


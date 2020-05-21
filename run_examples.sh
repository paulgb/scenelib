#!/bin/sh

set -e

mkdir -p examples_svg
cd examples_svg

for ex in $(ls ../examples); do
    echo "${ex%%.*}"
    cargo run --manifest-path=../Cargo.toml --release --example "${ex%%.*}"
done;

#! /bin/bash
cargo build --release
time ./target/release/ray-tracing
open ./image.ppm

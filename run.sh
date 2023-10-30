#! /bin/bash
cargo build --release
time ./target/release/ray-tracing
eog ./image.ppm

#!/bin/sh
cargo afl build --release
cargo afl fuzz -i input -o output target/release/fuzzer

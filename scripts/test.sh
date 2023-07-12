#!/usr/bin/env bash

export RUST_BACKTRACE=${RUST_BACKTRACE:-0}
export RUST_LOG=${RUST_BACKTRACE:-debug}

# Run all tests and show their output
cargo test --all -- --nocapture

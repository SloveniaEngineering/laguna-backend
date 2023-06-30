#!/usr/bin/env bash

RUST_BACKTRACE=full RUST_LOG=debug HOST=127.0.0.1 PORT=8080 cargo watch -x run

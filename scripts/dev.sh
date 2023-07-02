#!/usr/bin/env bash

export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/laguna_db 
export RUST_BACKTRACE=full 
export RUST_LOG=debug 
export HOST=127.0.0.1 
export PORT=8080 

# Runs in development enviornment and reruns for every change
cargo watch -x run

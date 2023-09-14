#!/usr/bin/env bash

export SQLX_OFFLINE=true
export DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/laguna_db}

# Run all tests and show their output
cargo test --all --features testx -- --nocapture

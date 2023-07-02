#!/usr/bin/env bash

export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/laguna_test_db
export RUST_BACKTRACE=full
export RUST_LOG=debug

# Create test database
sqlx database create

# Run all tests and show their output
cargo test --all -- --nocapture

# Automatically drop test database
sqlx database drop -y

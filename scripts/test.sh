#!/usr/bin/env bash

export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/laguna_dev_db

sqlx database setup

# Run all tests and show their output
cargo test --all --features testx -- --nocapture

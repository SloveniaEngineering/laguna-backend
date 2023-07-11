#!/usr/bin/env bash

export DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/laguna_db}
export RUST_BACKTRACE=${RUST_BACKTRACE:-full}
export RUST_LOG=${RUST_LOG:-debug}
export HOST=${HOST:-127.0.0.1}
export PORT=${PORT:-6969}
export FRONTEND_HOST=${FRONTEND_HOST:-127.0.0.1}
export FRONTEND_PORT=${FRONTEND_PORT:-4200}

# Runs in development enviornment and reruns for every change
cargo watch -x run

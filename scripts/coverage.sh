#!/usr/bin/env bash

export DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/laguna_dev_db}

cargo tarpaulin --workspace --timeout 120 --features testx --skip-clean --target-dir=target/coverage

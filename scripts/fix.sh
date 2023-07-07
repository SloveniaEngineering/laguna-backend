#!/usr/bin/env bash

# Fixes all warnings (inplace).
cargo fix --tests --allow-dirty --workspace

# Format after fixing.
cargo fmt

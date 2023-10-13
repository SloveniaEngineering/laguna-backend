#!/usr/bin/env bash

cargo clippy --fix --workspace --tests --allow-dirty --allow-staged

cargo fmt
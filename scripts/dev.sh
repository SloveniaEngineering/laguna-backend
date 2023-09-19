#!/usr/bin/env bash

# Runs in development environment and reruns for every change
RUST_LOG=info cargo watch -x run

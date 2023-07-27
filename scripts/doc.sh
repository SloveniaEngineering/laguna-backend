#!/usr/bin/env bash

# Workaround for detecting CARGO_CFG_DOC.
# See: https://github.com/rust-lang/cargo/issues/8811
# See: https://github.com/rust-lang/cargo/issues/8944


# Don't doc deps, only doc all private items in workspace.
cargo doc --no-deps --document-private-items --workspace --open --features dox

#!/usr/bin/env bash

# Don't doc deps, only doc all private items in workspace.
cargo doc --no-deps --document-private-items --workspace --open

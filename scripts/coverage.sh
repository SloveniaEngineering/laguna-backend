#!/usr/bin/env bash

cargo tarpaulin --workspace --timeout 120 --features testx --skip-clean

#!/usr/bin/env bash

cargo sqlx prepare --workspace --database-url=postgres://postgres:postgres@127.0.0.1:5432/laguna_dev_db

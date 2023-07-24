#!/usr/bin/env bash

export DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/laguna_db}
export SQLX_OFFLINE=${SQLX_OFFLINE:-1}

cargo sqlx prepare --merged -- --workspace

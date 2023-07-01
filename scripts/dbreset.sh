#!/usr/bin/env bash

# Example usage: scripts/dbreset.sh laguna_test_db

# Clears data in local database $1 and re-runs the migrations
sqlx database reset --database-url=postgres://postgres:postgres@127.0.0.1:5432/$1

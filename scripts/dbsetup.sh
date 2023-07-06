#!/usr/bin/env bash

# Example usage: scripts/dbsetup.sh laguna_test_db

# Creates local database $1 (if not exists), runs the migrations
sqlx database setup --database-url=postgres://postgres:postgres@127.0.0.1:5432/$1

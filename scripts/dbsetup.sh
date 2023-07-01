#!/usr/bin/env bash

# Example usage: scripts/dbsetup.sh laguna_test_db

# Drops local database $1, re-creates it and re-runs the migrations
sqlx database setup --database-url=postgres://postgres:postgres@127.0.0.1:5432/$1


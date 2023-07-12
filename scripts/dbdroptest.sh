#!/usr/bin/env bash

# Drop all databases with the name *_laguna_test_db
for db in `psql -c '\l' | grep *_laguna_test_db | cut -d '|' -f 1`; do psql -c "DROP DATABASE $db"; done

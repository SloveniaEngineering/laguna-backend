#!/usr/bin/env bash

# Usage: scripts/dbdroptest.sh
# Usage with non-tty terminals: scripts/dbdroptest_fixtty.sh scripts/dbdroptest.sh

# Drop all databases with laguna_test_db in name.
# WARNING: this does not work with non-tty terminals
for db in $(PGPASSWORD=postgres psql -U postgres -c '\l' | grep $1 | cut -d '|' -f 1); do
    # Set password so that psql does not prompt for it.
    # Quote database name so that it works with names containing UUIDv4.
    PGPASSWORD=postgres psql -U postgres -q -c "DROP DATABASE \"${db}\"";
    echo "DROPPED DATABASE ${db@Q}";
done

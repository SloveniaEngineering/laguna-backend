# Example usage: scripts/dbreset-win.ps1 laguna_test_db

$dbname = $args[0]

# Drops local database $dbname, re-creates it, and re-runs the migrations
sqlx database reset --database-url="postgres://postgres:postgres@127.0.0.1:5432/$dbname"


# Example usage: scripts/dbsetup-win.ps1 laguna_test_db

$dbname = $args[0]

# Creates local database $dbname (if not exists), runs the migrations
sqlx database setup --database-url="postgres://postgres:postgres@127.0.0.1:5432/$dbname"


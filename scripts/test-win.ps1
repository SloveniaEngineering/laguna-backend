# Clean any previous default values
$PSDefaultParameterValues.Remove("env:DATABASE_URL")
$PSDefaultParameterValues.Remove("env:RUST_BACKTRACE")
$PSDefaultParameterValues.Remove("env:RUST_LOG")
$PSDefaultParameterValues.Remove("env:HOST")
$PSDefaultParameterValues.Remove("env:PORT")

$PSDefaultParameterValues = @{
    "env:DATABASE_URL"="postgres://postgres:postgres@127.0.0.1:5432/laguna_test_db";
    "env:RUST_BACKTRACE"="0";
    "env:RUST_LOG"="debug"
}

# Drop existing test DB (if it exists)
# This is because in case of error, database is never dropped at the end, so we drop it in the beginning as well.
sqlx database drop -y

# Create test database
sqlx database create

# Run all tests and show their output
# Use single thread because many tests will use laguna_test_db at same time when multithreaded.
cargo test --all -- --nocapture --test-threads=1

# Automatically drop test database
sqlx database drop -y

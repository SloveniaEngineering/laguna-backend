# Clean any previous default values
$PSDefaultParameterValues.Remove("env:TEST_DATABASE_BASE_URL");

$PSDefaultParameterValues = @{
    "env:TEST_DATABASE_BASE_URL"="postgres://postgres:postgres@127.0.0.1:5432/laguna_test_db"
}

# Run all tests and show their output
cargo test --all --features testx -- --nocapture

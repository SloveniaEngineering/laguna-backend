$PSDefaultParameterValues.Remove("env:SQLX_OFFLINE")
$PSDefaultParameterValues.Remove("env:DATABASE_URL")


$PSDefaultParameterValues = @{
  "env:SQLX_OFFLINE"="true"
  "env:DATABASE_URL"="postgres://postgres:postgres@127.0.0.1:5432/laguna_dev_db"
}

# Run all tests and show their output
cargo test --all --features testx -- --nocapture

$PSDefaultParameterValues.Remove("env:DATABASE_URL")

$PSDefaultParameterValues = @{
    "env:DATABASE_URL"="postgres://postgres:postgres@127.0.0.1/laguna_dev_db"
}

cargo tarpaulin --workspace --timeout 120 --features testx --skip-clean

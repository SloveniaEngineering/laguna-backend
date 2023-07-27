$PSDefaultParameterValues.Remove("env:DATABASE_URL")
$PSDefaultParameterValues.Remove("env:SQLX_OFFLINE")

$PSDefaultParameterValues = @{
    "env:DATABASE_URL"="postgres://postgres:postgres@127.0.0.1:5432/laguna_db";
    "env:SQLX_OFFLINE"="1"
}

cargo sqlx prepare --merged -- --workspace

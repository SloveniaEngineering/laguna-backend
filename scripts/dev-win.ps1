# Clean any previous default values
$PSDefaultParameterValues.Remove("env:DATABASE_URL")
$PSDefaultParameterValues.Remove("env:RUST_BACKTRACE")
$PSDefaultParameterValues.Remove("env:RUST_LOG")
$PSDefaultParameterValues.Remove("env:HOST")
$PSDefaultParameterValues.Remove("env:PORT")
$PSDefaultParameterValues.Remove("env:FRONTEND_HOST")
$PSDefaultParameterValues.Remove("env:FRONTEND_PORT")

$PSDefaultParameterValues = @{
    "env:DATABASE_URL"="postgres://postgres:postgres@127.0.0.1:5432/laguna_db";
    "env:RUST_BACKTRACE"="full";
    "env:RUST_LOG"="debug";
    "env:HOST"="127.0.0.1";
    "env:PORT"="6969";
    "env:FRONTEND_HOST"="127.0.0.1";
    "env:FRONTEND_PORT"="4200"
}

# Runs in development enviornment and reruns for every change
cargo watch -x run

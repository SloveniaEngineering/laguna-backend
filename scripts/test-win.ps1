# Clean any previous default values
$PSDefaultParameterValues.Remove("env:RUST_BACKTRACE")
$PSDefaultParameterValues.Remove("env:RUST_LOG")

$PSDefaultParameterValues = @{
    "env:RUST_BACKTRACE"="0";
    "env:RUST_LOG"="debug"
}

# Run all tests and show their output
cargo test --all -- --nocapture

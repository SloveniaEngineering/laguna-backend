# Don't doc deps, only doc all private items in workspace.
$env:RUST_LOG="info"; cargo doc --no-deps --workspace --open --features dox


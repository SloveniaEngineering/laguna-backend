# Fixes all warnings (inplace).
cargo fix --tests --allow-dirty --allow-staged --workspace

# Format after fixing.
cargo fmt


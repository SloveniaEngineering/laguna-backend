# Tool for DB management via command line
cargo install sqlx-cli --no-default-features --features rustls,postgres

# Tools for:
# 1. Watching for changes in the code and recompiling
# 2. Generating code coverage reports
# 3. OPTIONAL: Expanding macros
# 4. OPTIONAL: Detecting unused deps
cargo install cargo-watch cargo-tarpaulin
#             cargo-expand
#             cargo-udeps


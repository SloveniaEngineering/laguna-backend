$env:DATABASE_URL="postgres://postgres:postgres@localhost:5432/laguna_dev_db";
    sqlx database setup;
    # Run all tests and show their output
    cargo test --all --features testx -- --nocapture

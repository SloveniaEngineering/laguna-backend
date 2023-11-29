Common development scripts.

> [!WARNING]
> These scripts are NOT meant to be used in production environment.

> [!NOTE]
> Below are only `.sh` scripts (that use `bash`), windows scripts (`.ps1`) should behave the same.

| Script                 | Description                                                                                                                             |
|------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| `coverage.sh`          | Generate codecov on tests. This will use local database specified in `DATABASE_URL`.                                                    |
| `dbdroptest.sh`        | Remove all test databases where tests failed. When test fails, the database isn't removed so this script provides a way to delete them. |
| `dbdroptest_fixtty.sh` | Put in front of `dbdroptest.sh` if you are running a non-tty terminal, eg. WSL2 or Git Bash MingW in windows.                           |
| `dbreset.sh`           | Drops and recreates a databased pointed to by `DATABASE_URL`, it also migrates tables.                                                  |
| `dev.sh`               | Runs a development server with `RUST_LOG=info` level logging.                                                                           |
| `doc.sh`               | Generates documentation locally without external libraries.                                                                             |
| `fix.sh`               | Runs `cargo fix` which removes unnecessary imports and other things, it also formats code.                                              |
| `lintfix.sh`           | Just like `fix.sh` but considers clippy (the linter) as well as compiler.                                                               |
| `prepare.sh`           | Run this if you changed any prepared compile-time query macros (`query_*!(something)`).                                                 |
| `setversion.sh`        | Sets version specified as command argument to all `Cargo.toml` files and all MIME types found in `laguna-backend-api`.                  |
| `test.sh`              | Runs all tests and show their output.                                                                                                   |
| `tools.sh`             | Installs common tools for effective development environment.                                                                            |

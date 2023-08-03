# Table of contents

> **Warning**
> This document is WIP (work in progress) and will likely never be complete.
> This document is also **not meant to be read linearly** but rather as a reference.

1. [Requirements](#requirements)
2. [Setup & First run](#setup--first-run)
3. [Testing](#testing)
4. [Configuration](#configuration)
5. [Generating documentation](#generating-documentation)
6. [Generating coverage](#generating-coverage)
7. [Fixing warnings](#fixing-warnings)
8. [Migrations and model changes](#migrations-and-model-changes)
9. [CI Workflow](#ci-workflow)
10. [Performance optimization](#performance-optimization)
11. [Project structure](#project-structure)
12. [Submitting changes](#submitting-changes)

## Requirements

1. Rust (https://www.rust-lang.org/tools/install)
2. Postgres (https://www.postgresql.org/download/)

It is recommended to run Linux (either WSL or VM if you are not on Linux) for development.
This is because `scripts/*` are written in bash and because backend will be deployed on linux.
In the future we will add powershell scripts for Windows.

## Setup & First run

> **Note**
> This guide uses Linux `scripts/*.sh`, but scrips for Windows are also available via `scripts/*-win.ps1`.

1. Clone this repo `git clone --recurse-submodules https://github.com/SloveniaEngineering/laguna-backend` and `cd` into it.
2. Run `cargo install sqlx-cli --no-default-features --features rustls,postgres`.
3. Run `cargo install cargo-watch`.
4. Make sure Postgres daemon is running, then do `scripts/dbsetup.sh laguna_db` to create `laguna_db` local DB with tables.
5. Run **and watch for changes** with `scripts/dev.sh` or just run with `cargo run`.

> **Note** > `scripts/dev.sh` watches for changes in source code and if change is detected automatically recompiles and restarts the server.

## Testing

1. Run `scripts/test.sh` to run all tests.

To delete test zombie databases if tests failed use `scripts/dbdroptest.sh`.

## Configuration

Most of the configuration can be done via config files in `configs/` directory. Both testing and development use `dev.toml` config.

When testing, however, `application.database.name` is always overriden to `laguna_test_db + <random UUIDv4>` for each test ensuring clean DB per test.

To extend config with custom fields see `crates/laguna-backend-config` crate.

## Generating Documentation

Documentation is auto-generated on push to `master`.
It can be accessed via GitHub Pages at https://sloveniaengineering.github.io/laguna-backend.

To generate and open identical local documentation run `scripts/doc.sh`.

## Generating Coverage

Coverage can be generated using `scripts/coverage.sh`.

## Fixing warnings

1. Run `scripts/fix.sh` to fix most warnings automatically + format code.

## Migrations and model changes

Here is a scenario.

1. You change a model in `laguna-backend-model` crate.
2. You also change the corresponding migration in `migrations` dir.
3. Your migration is now out of sync with DB.
4. Run `scripts/dbreset.sh laguna_dev_db` which drops current `laguna_dev_db`, recreates new `laguna_dev_db` and runs all migrations.

Here is another way to do it (without dropping DB):

1. You change a model in `laguna-backend-model` crate.
2. You create a new migration in `migrations` crate using `sqlx migrate add <migration_name>` which contains some `ALTER` statements which probably have `DEFAULT`s.
3. You run `sqlx migrate run` to run all migrations.

It is also possible to create "reversible" migrations with `sqlx migrate add -r <migration_name>`
which will create an `up` and `down` migration files and can be reverted with `sqlx migrate revert`.

## Changing queries and using prepared statements

Always prefer compile-time query to runtime query, so that errors are caught at compile time.

- If a compile time (ie. `query_*!`) is changed (even if just spacing is changed (because of underlying hash of query)) it needs to be re-prepared with `scripts/prepare.sh`.
  This generates `sqlx-data.json` in workspace root.

## CI Workflow

To skip CI for a commit, add [One of the these keywords to your commit message](https://docs.github.com/en/actions/managing-workflow-runs/skipping-workflow-runs).

## Performance optimization

See `.cargo/config.toml` for more info.

## Project structure

> **Note**
> Only important files and dirs are listed here.

- `.github/` contains GitHub Actions definitions for CI/CD.
  - `dependabot.yml` contains config for automatic dependency updates.
  - `workflows/` contains CI/CD workflows.
    - `rust.yml` contains CI workflow for Rust.
- `.cargo/config.toml` contains GLOBAL project config for Cargo and Rust. This is because we have a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and its easier to have global config.
- `/` root directory contains root Cargo Crate `laguna-backend` and definition of [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
- `src/main.rs` is server entry point.
- `crates/` contains Cargo Workspace members (sub-Crates) of the project.
  - `laguna-backend-internal/` is a crate that contains re-exports of all other `crates/*` and is used by `laguna-backend` (root crate) to access all other crates.
    - `laguna-backend-internal/src/lib.rs` re-exporting can be seen here.
  - `laguna-backend-api/` contains API endpoints.
  - `laguna-backend-dto/` contains DTOs (data-transfer-objects) used by [laguna-frontend](https://github.com/SloveniaEngineering/laguna-frontend).
  - `laguna-backend-model/` contains DB models and relations.
  - `laguna-backend-config/` contains custom config structs and functions that work with `configs/` directory and `actix-settings`. `laguna-backend-config` is not a workspace member.
  - `laguna-backend-middleware/` contains application logic from API to DB.
- `migrations/` contains SQL migrations for DB.
- `configs/` contains config files for development, testing and deploy.
- `scripts/` contains scripts for development, testing and deploy.

## Submitting changes

> **Warning** > **Don't fork** and contribute, just clone and contribute.
> This is because some token CI permissions are acting weird with forks.

Because of that, if you want to contribute you have to be in the `SloveniaEngineering` GitHub organization.
Message someone from the organization to add you to the organization or create an issue.

### Label guide

There are many types of labels, the general syntax for them is `<TYPE>-<SUBTYPE>`

Descriptions can be found at: https://github.com/SloveniaEngineering/laguna-backend/labels.

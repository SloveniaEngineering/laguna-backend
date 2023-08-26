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
8. [CI Workflow](#ci-workflow)
9. [Performance optimization](#performance-optimization)
10. [SQL (Squeal) Notes](#sql-notes)
    1. [Changing queries and using prepared statements](#changing-queries-and-using-prepared-statements)
11. [Project structure](#project-structure)
12. [Submitting changes](#submitting-changes)
    1. [Label guide](#label-guide)
    2. [Versioning](#versioning)
    3. [Branching](#branching)
       1. [Naming rules](#naming-rules)

## Requirements

1. Rust (https://www.rust-lang.org/tools/install)
2. Postgres (https://www.postgresql.org/download/)
3. Git (https://git-scm.com/downloads)

It is recommended to run Linux (either WSL or VM if you are not on Linux) for development.
This is because `scripts/*` are written in bash and because backend will be deployed on linux.

## Setup & First run

> **Note**
> This guide uses Linux `scripts/*.sh`, but scrips for Windows are also available via `scripts/*-win.ps1`.

1. Clone this repo `git clone --recurse-submodules https://github.com/SloveniaEngineering/laguna-backend` and `cd` into it.
2. Run `scripts/tools.sh` to install project tools that simplify development.
   This can be quite expensive so if you don't need all the tools you can install them manually (see `scripts/tools.sh`).
3. Make sure Postgres daemon is running, then do `sqlx database setup --database-url=postgres://postgres:postgres@127.0.0.1/laguna_dev_db` to create `laguna_dev_db` local DB with tables.
4. Run with `scripts/dev.sh` or with `cargo run`.

> **Note**
> `scripts/dev.sh` watches for changes in source code and if change is detected automatically recompiles and restarts the server.

## Testing

> **Note**
> In the future we will likely test validation and **important** logic with unit tests separately from integration tests that work with DB.

1. Run `scripts/test.sh` to run all tests using `_sqlx_test` databases and store test infos in `laguna_dev_db` in `_sqlx_test` schema rather than `public` used for local development.

To delete zombie test databases if tests failed use `scripts/dbdroptest.sh _sqlx_test`.

> **Note**
> On WSL or Mingw64 you likely don't have a tty terminal. For that you can use `scripts/dbdroptest_fixtty.sh scripts/dbdroptest.sh _sqlx_test`.

> **Note**
> You probably don't want to delete test databases, because they are useful for debugging/inspection.

## Configuration

Most of the configuration can be done in `Laguna.toml` file.

You can override config with environment variables (in order of precedence, highest first):

1. `DATABASE_URL` environment variable overrides `application.database`.
2. For example `application.database.name` can be overriden by `APPLICATION_DATABASE_NAME` environment variable.

For more info and to extend config with custom fields see `crates/laguna-backend-config` crate.

## Generating Documentation

Documentation is auto-generated on push to `master`.
It can be accessed via GitHub Pages at https://sloveniaengineering.github.io/laguna-backend.

To generate and open a preview of identical local documentation run `scripts/doc.sh`.

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

## CI Workflow

To skip CI for a commit, add [One of the these keywords to your commit message](https://docs.github.com/en/actions/managing-workflow-runs/skipping-workflow-runs).
Only do this if you are resolving `T-Stupid` issue/PR.

## Performance optimization

See `.cargo/config.toml` for more info.

# SQL Notes

- In stored functions, prefer `RETURNS TABLE (LIKE <table_name>)` over `RETURNS SETOF <table_name>`. 
  See other ways: https://dba.stackexchange.com/questions/135378/how-to-use-returns-table-with-an-existing-table-in-postgresql

- Stored functions return `NULL` rows if rows are not found. This is the only reason why we can't have compile-time queries for now. 
  See: https://stackoverflow.com/questions/60222263/why-do-postgresql-functions-return-null-columns-instead-of-no-rows-when-the-retu. 
  That SO doesn't fix the issue.

- Second blocker to compile-time queries is the fact that `sqlx` doesn't work nice with custom types (even though it should, which is what they claim). 
  For example, role must be used in select like this: `SELECT role AS "role: _" FROM users WHERE id = $1`. 
  The same goes with update where you may need to do `UPDATE "User" SET role = $1 WHERE id = $2` and then in compile-time `sqlx::query!` pass `role as _`.

- Prefer sending empty strings to stored procedures (rather than `NULL` due to usage of `STRICT`, unless ALL parameters are `NULL`-able. 
  See: https://www.postgresql.org/docs/current/sql-createfunction.html specifically `CALLED ON NULL INPUT` (default mode)). 
  Then in stored functions use `NULLIF`, `TRIM` and `COALESCE` to convert empty strings to `NULL` if needed.

- See also https://github.com/launchbadge/sqlx/pull/2670 as a possible fix to the third point.

### Changing queries and using prepared statements

Always prefer compile-time query to runtime query, so that errors are caught at compile time.

- If a compile time (ie. `query_*!`) is changed (even if just spacing is changed (because of underlying hash of query)) it needs to be re-prepared with `scripts/prepare.sh`.
  This generates `.sqlx/*` files.
  Use `--merged` to genertae `sqlx-data.json` in workspace root.

- If after compiling, **compile time query is valid and works correctly** you might consider moving it into stored procedure/function. Using `sqlx = { ..., git = ..., rev = ... }` in `Cargo.toml` didn't solve the issue though.

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
  - `laguna-backend-config/` contains custom config structs for `Laguna.toml` config.
  - `laguna-backend-setup/` contains App setup logic (for tests, dev and production) and relies heavily from definitions in `laguna-backend-config`.
  - `laguna-backend-middleware/` contains application logic from API to DB.
  - `laguna-backend-tracker/` contains torrent tracking system commons.
  - `laguna-backend-tracker-http` implements TCP-based (HTTP) tracker.
  - `laguna-backend-tracker-udp` implements UDP-based tracker.
  - `laguna-backend-tracker-ws` implements WebRTC-based (WebSocker) tracker.
- `migrations/` contains SQL migrations for DB.
- `configs/` contains config files for development, testing and deploy.
- `scripts/` contains scripts for development, testing and deploy.

## Submitting changes

> **Warning**
> **Don't fork** and contribute, just clone and contribute.
> This is because some token CI permissions are acting weird with forks.
> This will be fixed.

Because of that, if you want to contribute you have to be in the `SloveniaEngineering` GitHub organization.
Message someone from the organization to add you to the organization or create an issue.

### Label guide

There are many types of labels, the general syntax for them is `<TYPE>-<SUBTYPE>`.

Descriptions can be found at: https://github.com/SloveniaEngineering/laguna-backend/labels.

Basic types are:

1. `A` - Area
2. `C` - Challenge
3. `D` - Difficulty
4. `M` - Special type for unsorted
5. `N` - Needs
6. `P` - Priority
7. `S` - Status
8. `T` - Type of issue/PR
9. `V` - Type of SemVer version to bump after resolve

### Versioning

This project uses [Semantic Versioning](https://semver.org/) for releases.
Releases occur when `dev` is merged into `master` (aka. Git Flow).

- Patch version is incremented on merge of `patch-*` into `dev`.
- Minor version is incremented on merge of `impl-*` into `dev`.
- Major version is set manually.

This way `dev` serves as a buffer for review and testing.

- Version applies when `dev` is merged into `master` and Release is created with appropriate tag.

### Branching

- Always branch of off `dev` branch.
- Always rebase your branch to lastest `dev`.

#### Naming rules

- If you are fixing/refactoring anything name your branch `patch-<something that is being fixed>`.
- If you are implementing anything name your branch `impl-<something that is being implemented>`.

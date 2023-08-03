#!/usr/bin/env bash

cargo sqlx prepare --merged --database-url=postgress://postgres:postgres@localhost:5432/laguna_dev_db -- --workspace

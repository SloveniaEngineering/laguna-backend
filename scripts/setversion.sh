#!/usr/bin/env bash

# Find all Cargo.toml files and set version = $1 in each of them
# Usage: ./setversionall.sh 1.2.3

set -e

VERSION=$1

echo "Setting version in Cargo.toml files"

for file in $(find . -name Cargo.toml); do
    echo "Setting version in $file"
    sed -i "s/^version = .*/version = \"$VERSION\"/" $file
done


# Find all of the form "application/vnd.sloveniaengineering.laguna.0.1.0+json" and set the version to $1

echo "Setting version in API docs (utoipa)"

for file in $(find crates/laguna-backend-api/src -name '*.rs'); do
    echo "Setting version in $file"
    # TODO: This is long, make it shorter
    sed -i "s/application\/vnd.sloveniaengineering.laguna.[0-9].[0-9].[0-9]-alpha+json/application\/vnd.sloveniaengineering.laguna.$VERSION+json/g;s/application\/vnd.sloveniaengineering.laguna.[0-9].[0-9].[0-9]-beta+json/application\/vnd.sloveniaengineering.laguna.$VERSION+json/g;s/application\/vnd.sloveniaengineering.laguna.[0-9].[0-9].[0-9]+json/application\/vnd.sloveniaengineering.laguna.$VERSION+json/g" $file
done

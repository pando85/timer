#!/bin/bash
set -e

# bump version
vim timer_core/Cargo.toml

# update lock file
cargo update -p timer_core

# update CHANGELOG.md
make update-changelog

# merge PR
git add .
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' timer_core/Cargo.toml | head -n1)
git commit -m "release: Version $VERSION"

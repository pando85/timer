# release workflow

- Bump version in `Cargo.toml`.
- Update lock file: `cargo update`.
- Update `CHANGELOG.md`.
- Merge PR
- Tag version in main branch: `make tag`
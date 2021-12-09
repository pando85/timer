# release workflow

- Bump version in `timer_core/Cargo.toml`.
- Update lock file: `cargo update -p timer_core`.
- Update `CHANGELOG.md`.
- Merge PR.
- Tag version in main branch, add header from changelog and body without `#`: `make tag`

# Release workflow

- Bump version in `timer_core/Cargo.toml`.
- Update lock file: `cargo update -p timer_core`.
- Update `CHANGELOG.md` with `make update-changelog`.
- Merge PR.

## Upgrade dependencies

Requirements:

- `cargo-edit`: `cargo install cargo-edit`

Upgrade dependencies:

- `cargo upgrade` or `cargo upgrade --incompatible`

Update cargo lock dependencies:

- `cargo update`

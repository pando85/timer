# Release workflow

```bash
.ci/release.sh
```

This script **automatically**:
1. Detects and fixes shallow clones (fetches full history + tags)
2. Bumps version in `Cargo.toml`
3. Updates the lock file
4. Runs `make update-changelog`
5. Creates a release commit

## Upgrade dependencies

Requirements:

- `cargo-edit`: `cargo install cargo-edit`

Upgrade dependencies:

- `cargo upgrade` or `cargo upgrade --incompatible`

Update cargo lock dependencies:

- `cargo update`

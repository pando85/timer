# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.10.0](https://github.com/pando85/timer/tree/v0.10.0) - 2025-10-31

### Fixed

- Make clippy 1.89 happy ([ecb355b](https://github.com/pando85/timer/commit/ecb355b0eb699314f0acd074bdc372830f6985be))

### Documentation

- Add copilot instructions ([8b2f980](https://github.com/pando85/timer/commit/8b2f98024257d2ead5e69380f473ba7ec7e74c38))
- Add commit ID links in CHANGELOG.md ([3a4089a](https://github.com/pando85/timer/commit/3a4089a6054900883263fe1d1c5c188db01fd653))

### Build

- ci: Fix cargo login token ([267e44a](https://github.com/pando85/timer/commit/267e44ac1b5287e5c0e617709251bc8ce27f7634))
- ci: Change Apple build to arm64 and update to macos-15 ([7a1d24f](https://github.com/pando85/timer/commit/7a1d24f95c0d6a78115c84e5a2887a5687458a60))
  - **BREAKING**: Apple x86_64 binary is deprecated.
- deps: Update Rust crate rodio to v0.21.1 ([140ac00](https://github.com/pando85/timer/commit/140ac005c0b9ae878a2f4a05904437f3ccad8a06))
- deps: Update Rust crate clap to v4.5.42 ([ffd3d34](https://github.com/pando85/timer/commit/ffd3d34c3bb3dfeaeada76056e70b3763dea1cd8))
- deps: Update Rust crate clap to v4.5.43 ([7b77a73](https://github.com/pando85/timer/commit/7b77a7395c0b1edb396ffffcd72d77f57347fc1a))
- deps: Update Rust crate clap to v4.5.44 ([6c2125b](https://github.com/pando85/timer/commit/6c2125b5686a0a3468e8cd46e9d25d68dac71694))
- deps: Update actions/checkout action to v5 ([e32c0e9](https://github.com/pando85/timer/commit/e32c0e9e98102cc0a565f729fd63a61ee7440412))
- deps: Update Rust crate glob to v0.3.3 ([8184a8a](https://github.com/pando85/timer/commit/8184a8a398afe33308a929e43e77f7a24455d4a9))
- deps: Update Rust crate libc to v0.2.175 ([4fe9427](https://github.com/pando85/timer/commit/4fe9427c5a5990345200422160f9bc0c99da10a9))
- deps: Update pre-commit hook pre-commit/pre-commit-hooks to v6 ([c6945dc](https://github.com/pando85/timer/commit/c6945dc0a08b92ecb69ccdf1e1d4362846b05b4e))
- deps: Update Rust crate clap to v4.5.45 ([65c570a](https://github.com/pando85/timer/commit/65c570a62c40235ef23cfe3c25f9a8f4640331c2))
- deps: Update Rust crate regex to v1.11.2 ([738d03f](https://github.com/pando85/timer/commit/738d03fe8e9c6c9f32ec9fa8609c9a5a859f2a38))
- deps: Update Rust crate clap to v4.5.46 ([a969efc](https://github.com/pando85/timer/commit/a969efc5893c9dc5a2c76a55f3b4eda5bfd90998))
- deps: Update Rust crate clap to v4.5.47 ([6636fd6](https://github.com/pando85/timer/commit/6636fd685d3e3b9f6daa694da282c738c004f52d))
- deps: Update Rust crate time to v0.3.43 ([4dc9a6a](https://github.com/pando85/timer/commit/4dc9a6ae7c577f2c4fc719495c84e9fe7754c15b))
- deps: Update actions/stale action to v10 ([9f84d36](https://github.com/pando85/timer/commit/9f84d36a9c3a3b78108fb07812ffe606b22509d3))
- deps: Update clechasseur/rs-clippy-check action to v5 ([c0798b2](https://github.com/pando85/timer/commit/c0798b2babb7f839319408ac637ac3518e9acdfe))
- deps: Update actions/setup-python action to v6 ([b7cab2f](https://github.com/pando85/timer/commit/b7cab2fcd8e5505dd6378bb37b9918e9a9a472c2))
- deps: Update Rust crate clap to v4.5.48 ([fa2c44d](https://github.com/pando85/timer/commit/fa2c44d659b0fbc26e473eb66cb89241943e6e64))
- deps: Update Rust crate libc to v0.2.176 ([a47cb26](https://github.com/pando85/timer/commit/a47cb2666c5fc282a0b21d32fc35dfdc025da0e2))
- deps: Update Rust crate regex to v1.11.3 ([4c81137](https://github.com/pando85/timer/commit/4c811371425ddf858744ff80b31aae5fd96fb2f4))
- deps: Update Rust crate time to v0.3.44 ([96a2104](https://github.com/pando85/timer/commit/96a21044a418403ee4b71484c5c2fa850ffd3891))
- deps: Update pre-commit hook alessandrojcm/commitlint-pre-commit-hook to v9.23.0 ([7fee635](https://github.com/pando85/timer/commit/7fee635cbd7b1a177758caa60a6e5c9d4da7668f))
- deps: Update Rust crate regex to v1.12.1 ([fd0c7e2](https://github.com/pando85/timer/commit/fd0c7e216b6d90906a21a634ee53b5d2dd33c881))
- deps: Update Rust crate regex to v1.12.2 ([7883edc](https://github.com/pando85/timer/commit/7883edc465f134ade613d04fae90c8078e0418f2))
- deps: Update Rust crate libc to v0.2.177 ([945cadd](https://github.com/pando85/timer/commit/945cadd09aca7db9bbd552b5d59e5338d6cccd66))
- deps: Update Rust crate clap to v4.5.49 ([974ce25](https://github.com/pando85/timer/commit/974ce250fb175cd76f6591bb9ef5e0aadc8c0445))
- deps: Update Rust crate clap to v4.5.50 ([ce43cde](https://github.com/pando85/timer/commit/ce43cde10b1c36b08077b3080fc90f69af0ddaad))
- deps: Update Rust crate clap to v4.5.51 ([d05b0b5](https://github.com/pando85/timer/commit/d05b0b5208fbc0995cfd28ad71d510a8bee2ce5d))

## [v0.9.0](https://github.com/pando85/timer/tree/v0.9.0) - 2025-07-13

### Added

- Add hms parser

### Fixed

- ci: Fix SHA256 URL on AUR pkgbuild generator
- Cargo clippy errors 1.88

### Build

- ci: Update AUR rash script with uri module
- ci: Auto update renovate pre-commit once a month automatically
- deps: Update Rust crate libc to v0.2.174
- deps: Update Rust crate clap to v4.5.41
- deps: Update Rust crate rodio to 0.21
- deps: Update API of rodio 0.21

### Testing

- ci: Deprecate commitlint workflow
- Re-add macos timer unit tests
- Fix macos undefined `time` macro

## [v0.8.10](https://github.com/pando85/timer/tree/v0.8.10) - 2025-06-15

### Added

- ci: Add pre-commit and deprecate cargo-husky

### Fixed

- ci: Disable rust jobs in pre-commit workflows

### Documentation

- Update README with `timer-rs-bin` package and fix curl install
- Fix logo links in README

### Build

- ci: Update ubuntu runners
- deps: Update Rust crate crossterm to 0.29
- deps: Update Rust crate clap to v4.5.36
- deps: Update Rust crate clap to v4.5.37
- deps: Update Rust crate libc to v0.2.172
- deps: Update Rust crate nix to 0.30
- deps: Update Rust crate nix to v0.30.1
- deps: Update Rust crate clap to v4.5.38
- deps: Update Rust crate clap to v4.5.39
- deps: Update Rust crate clap to v4.5.40
- deps: Update Rust crate signal-hook to v0.3.18
- deps: Update Rust crate libc to v0.2.173

## [v0.8.9](https://github.com/pando85/timer/tree/v0.8.9) - 2025-04-01

### Build

- renovate: Migrate config renovate.json5

## [v0.8.8](https://github.com/pando85/timer/tree/v0.8.8) - 2025-04-01

### Build

- ci: Fix AUR bin package install command

## [v0.8.7](https://github.com/pando85/timer/tree/v0.8.7) - 2025-04-01

### Build

- ci: Disable AUR bin package
- ci: Update rash install
- ci: Enable bin AUR package
- deps: Update Rust crate clap to v4.5.35
- deps: Update Rust crate time to v0.3.41

## [v0.8.6](https://github.com/pando85/timer/tree/v0.8.6) - 2025-04-01

### Build

- ci: Add sha256sum to release binaries
- ci: Change from main to master
- ci: Add AUR bin package
- deps: Update Rust crate libc to v0.2.170
- deps: Update Rust crate clap to v4.5.31
- deps: Update Rust crate time to v0.3.39
- deps: Update Rust crate clap to v4.5.32
- deps: Update Rust crate libc to v0.2.171
- deps: Update Rust crate clap to v4.5.33
- deps: Update Rust crate clap to v4.5.34
- Update to 2024 edition

## [v0.8.5](https://github.com/pando85/timer/tree/v0.8.5) - 2025-02-18

### Build

- deps: Update Rust crate clap to v4.5.29
- deps: Update Rust crate clap to v4.5.30
- deps: Update KSXGitHub/github-actions-deploy-aur action to v4

## [v0.8.4](https://github.com/pando85/timer/tree/v0.8.4) - 2025-02-09

### Build

- Fix time version to 0.37

## [v0.8.3](https://github.com/pando85/timer/tree/v0.8.3) - 2025-02-09

### Fixed

- ci: Clippy Github Action name typo
- ci: Remove `token` deprecated attr from `rs-clippy-check`
- ci: Update deprecated `macos-12` runner for `macos-13`
- Remove deprecated `set_soundness` from time 0.3.37

### Build

- deps: Update Rust crate clap to v4.5.20
- deps: Update Rust crate libc to v0.2.160
- deps: Update Rust crate libc to v0.2.161
- deps: Update Rust crate regex to v1.11.1
- deps: Update Rust crate rodio to 0.20.0
- deps: Update Rust crate rodio to v0.20.1
- deps: Update Rust crate clap to v4.5.21
- deps: Update Rust crate libc to v0.2.167
- deps: Update Rust crate time to v0.3.37
- deps: Keep versions to minor in `Cargo.toml`
- deps: Update Rust crate clap to v4.5.22
- deps: Update Rust crate clap to v4.5.23
- deps: Update Rust crate libc to v0.2.168
- deps: Update wagoid/commitlint-github-action action to v6.2.0
- deps: Update Rust crate clap to v4.5.24
- deps: Update Rust crate clap to v4.5.25
- deps: Update Rust crate clap to v4.5.26
- deps: Update clechasseur/rs-clippy-check action to v4
- deps: Update wagoid/commitlint-github-action action to v6.2.1
- deps: Update Rust crate clap to v4.5.27
- deps: Update Rust crate libc to v0.2.169
- deps: Update Rust crate glob to v0.3.2
- deps: Update Rust crate clap to v4.5.28

## [v0.8.2](https://github.com/pando85/timer/tree/v0.8.2) - 2024-10-06

### Build

- Optimize release binary
- Remove debug symbols from build

## [v0.8.1](https://github.com/pando85/timer/tree/v0.8.1) - 2024-10-05

### Fixed

- ci: Publish was broken for MacOS

## [v0.8.0](https://github.com/pando85/timer/tree/v0.8.0) - 2024-10-05

### Fixed

- ci: Add groups to cliff template

### Documentation

- Minor change in features list

### Build

- deps: Update wagoid/commitlint-github-action action to v6.1.1
- deps: Update KSXGitHub/github-actions-deploy-aur action to v3.0.1
- deps: Update wagoid/commitlint-github-action action to v6.1.2
- deps: Change clippy to clechasseur/rs-clippy-check action to v3
- deps: Update Rust crate clap to v4.5.17
- deps: Remove pinned versions from `Cargo.toml`
- deps: Update Rust crate clap to v4.5.18
- deps: Update Rust crate regex to v1.11.0
- deps: Update Rust crate clap to v4.5.19
- Remove death code

### Refactor

- ci: Change comments in `release.sh`
- Simplify directory structure

## [v0.7.6](https://github.com/pando85/timer/tree/v0.7.6) - 2024-08-21

### Added

- ci: Add release.sh script

### Fixed

- ci: Remove `fetch-depth: 0` to get just last commit on commitlint
- ci: Avoid Commitlint running twice

### Documentation

- Order changelog groups

### Build

- **BREAKING**: Change required min glib version to 2.31
- deps: Update Rust crate rodio to 0.19.0
- deps: Update KSXGitHub/github-actions-deploy-aur action to v2.7.2
- deps: Update mindsers/changelog-reader-action action to v2.2.3
- deps: Update Rust crate clap to v4.5.10
- deps: Update softprops/action-gh-release action to v2
- deps: Update actions/checkout action to v4
- deps: Update Rust crate clap to v4.5.11
- deps: Update Rust crate clap to v4.5.12
- deps: Update Rust crate clap to v4.5.13
- deps: Update Rust crate regex to v1.10.6
- deps: Update wagoid/commitlint-github-action action to v6.0.2
- deps: Update Rust crate clap to v4.5.14
- deps: Update Rust crate clap to v4.5.15
- deps: Update Rust crate clap to v4.5.16
- deps: Update Rust crate libc to v0.2.156
- deps: Update Rust crate libc to v0.2.158
- deps: Update Rust crate crossterm to 0.28.0
- deps: Update KSXGitHub/github-actions-deploy-aur action to v3

### Refactor

- Replace lazy_static with std from 1.80

## [v0.7.5](https://github.com/pando85/timer/tree/v0.7.5) - 2024-06-24

### Build

- Update Rust crate lazy_static to v1.5.0

### CI

- Fix gpg key for aur publish
- Change AUR valid GPG

## [v0.7.4](https://github.com/pando85/timer/tree/v0.7.4) - 2024-06-21

### Build

- Update Rust crate rodio to 0.18.0
- Update Rust crate libc to v0.2.155
- Update Rust crate rodio to v0.18.1
- Update Rust crate nix to 0.29
- Update Rust crate clap to v4.5.6
- Update Rust crate regex to v1.10.5
- Update Rust crate clap to v4.5.7

### CI

- Add automerge on patch versions for renovate
- Add autotag workflow

## [v0.7.3](https://github.com/pando85/timer/tree/v0.7.3) - 2024-05-09

### Fixed

- Change action-gh-release to v1

## [v0.7.2](https://github.com/pando85/timer/tree/v0.7.2) - 2024-05-09

### Fixed

- Set changelog reader action to v2.2.2

## [v0.7.1](https://github.com/pando85/timer/tree/v0.7.1) - 2024-05-08

### Build

- Update KSXGitHub/github-actions-deploy-aur action to v2.7.1
- Update wagoid/commitlint-github-action action to v6.0.1
- Update Rust crate time to 0.3.35
- Update Rust crate clap to 4.5.4
- Update Rust crate time to 0.3.36
- Update Rust crate glob to 0.3.1
- Update Rust crate regex to 1.10.4
- Update Rust crate libc to 0.2.154
- Update Rust crate rodio to 0.17.3
- Update Rust crate signal-hook to 0.3.17

## [v0.7.0](https://github.com/pando85/timer/tree/v0.7.0) - 2024-03-30

### Added

- Support for non leading zero time formats

### Build

- Update Rust crate nix to 0.28
- Update softprops/action-gh-release action to v2
- Bump mio from 0.8.10 to 0.8.11
- Update wagoid/commitlint-github-action action to v5.4.6
- Update wagoid/commitlint-github-action action to v6

### CI

- Change renovate tag to build

## [v0.6.1](https://github.com/pando85/timer/tree/v0.6.1) - 2024-02-23

### Build

- Update cargo lock file

### CI

- Add renovate.json
- Change commit message format
- Update Rust crate time to 0.3.34
- Update KSXGitHub/github-actions-deploy-aur action to v2.7.0
- Update Rust crate clap to 4.5
- Update wagoid/commitlint-github-action action to v1.8.0
- Update Rust crate regex to 1.10
- Update actions/stale action to v9
- Update Rust crate tailcall to v1
- Update wagoid/commitlint-github-action action to v5

## [v0.6.0](https://github.com/pando85/timer/tree/v0.6.0) - 2023-09-22

### Build

- Upgrade dependencies
- Remove resolver warning

### CI

- Update to node16 github actions
- Change rust tools to dtolnay/rust-toolchain action
- Add stale bot for PRs and issues

### Documentation

- Add play example for HH:MM format less than 10 hours
- Fix build badge

### Fixed

- Cargo clippy errors
- Update MacOS workers
- Upgrade cache action version to v2

### Refactor

- Makefile project_version variable

## [v0.5.1](https://github.com/pando85/timer/tree/v0.5.1) - 2022-10-27

### Added

- Update clap to v4
- Add clap debug_assert
- Add `git-cliff` to update CHANGELOG automatically

### Fixed

- Change clippy params to lint all code

### Refactor

- Replace clap function with command or arg
- Add tailcall to enable recursion again in countdown

## [v0.5.0](https://github.com/pando85/timer/tree/v0.5.0) - 2022-09-30

### Added

- Add loop option to repeat counter infinitely(#68)

## [v0.4.0](https://github.com/pando85/timer/tree/v0.4.0) - 2022-09-29

### Added

- Add terminal-bell and silence options(#66)

## [v0.3.5](https://github.com/pando85/timer/tree/v0.3.5) - 2022-08-15

### Fixed

- Add `pkg-config` to AUR build dependencies(#62)

## [v0.3.4](https://github.com/pando85/timer/tree/v0.3.4) - 2022-08-15

### Fixed

- Zero delay when no output stream found in Rodio(#61)

## [v0.3.3](https://github.com/pando85/timer/tree/v0.3.3) - 2022-06-13

### Fixed

- ci: Fix rash install in AUR release

## [v0.3.2](https://github.com/pando85/timer/tree/v0.3.2) - 2022-06-10

### Fixed

- Bumps [regex](https://github.com/rust-lang/regex) from 1.5.4 to 1.5.5.
  - [Release notes](https://github.com/rust-lang/regex/releases)
  - [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)
  - [Commits](https://github.com/rust-lang/regex/compare/1.5.4...1.5.5)
- Update all dependencies

## [v0.3.1](https://github.com/pando85/timer/tree/v0.3.1) - 2022-01-05

### Fixed

- Replace recursive function with loop

## [v0.3.0](https://github.com/pando85/timer/tree/v0.3.0) - 2021-12-28

### Added

- Print info that fits on the terminal
- Automatically add body to GitHub release

## [v0.2.1](https://github.com/pando85/timer/tree/v0.2.1) - 2021-12-13

### Fixed

- Fix regex matching just counter times

## [v0.2.0](https://github.com/pando85/timer/tree/v0.2.0) - 2021-12-11

### Added

- Add seconds input support without tag
- Add parse for `min`

## [v0.1.4](https://github.com/pando85/timer/tree/v0.1.4) - 2021-12-09

### Fixed

- Fix terminal flickering

## [v0.1.3](https://github.com/pando85/timer/tree/v0.1.3) - 2021-12-07

### Added

- Add AUR packages automatic build and publish

## [v0.1.2](https://github.com/pando85/timer/tree/v0.1.2) - 2021-12-07

### Fixed

- Fix github-actions for publishing packages
- Add missing metadata to publish crates

## [v0.1.1](https://github.com/pando85/timer/tree/v0.1.1) - 2021-12-07

### Fixed

- Release with signed tags
- Add `Cargo.lock` file

## [v0.1.0](https://github.com/pando85/timer/tree/v0.1.0) - 2021-12-07

Initial release

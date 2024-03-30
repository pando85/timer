# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.7.0](https://github.com/pando85/timer/tree/v0.7.0) - 2024-03-30

### Added

* Support for non leading zero time formats

### Build

* Update Rust crate nix to 0.28
* Update softprops/action-gh-release action to v2
* Bump mio from 0.8.10 to 0.8.11
* Update wagoid/commitlint-github-action action to v5.4.6
* Update wagoid/commitlint-github-action action to v6

### CI

* Change renovate tag to build

## [v0.6.1](https://github.com/pando85/timer/tree/v0.6.1) - 2024-02-23

### Build

* Update cargo lock file

### CI

* Add renovate.json
* Change commit message format
* Update Rust crate time to 0.3.34
* Update KSXGitHub/github-actions-deploy-aur action to v2.7.0
* Update Rust crate clap to 4.5
* Update wagoid/commitlint-github-action action to v1.8.0
* Update Rust crate regex to 1.10
* Update actions/stale action to v9
* Update Rust crate tailcall to v1
* Update wagoid/commitlint-github-action action to v5

## [v0.6.0](https://github.com/pando85/timer/tree/v0.6.0) - 2023-09-22

### Build

* Upgrade dependencies
* Remove resolver warning

### CI

* Update to node16 github actions
* Change rust tools to dtolnay/rust-toolchain action
* Add stale bot for PRs and issues

### Documentation

* Add play example for HH:MM format less than 10 hours
* Fix build badge

### Fixed

* Cargo clippy errors
* Update MacOS workers
* Upgrade cache action version to v2

### Refactor

* Makefile project_version variable

## [v0.5.1](https://github.com/pando85/timer/tree/v0.5.1) - 2022-10-27

### Added

* Update clap to v4
* Add clap debug_assert
* Add `git-cliff` to update CHANGELOG automatically

### Fixed

* Change clippy params to lint all code

### Refactor

* Replace clap function with command or arg
* Add tailcall to enable recursion again in countdown

## [v0.5.0](https://github.com/pando85/timer/tree/v0.5.0) - 2022-09-30

### Added

* Add loop option to repeat counter infinitely(#68)

## [v0.4.0](https://github.com/pando85/timer/tree/v0.4.0) - 2022-09-29

### Added

* Add terminal-bell and silence options(#66)

## [v0.3.5](https://github.com/pando85/timer/tree/v0.3.5) - 2022-08-15

### Fixed

* Add `pkg-config` to AUR build dependencies(#62)

## [v0.3.4](https://github.com/pando85/timer/tree/v0.3.4) - 2022-08-15

### Fixed

* Zero delay when no output stream found in Rodio(#61)

## [v0.3.3](https://github.com/pando85/timer/tree/v0.3.3) - 2022-06-13

### Fixed

* ci: Fix rash install in AUR release

## [v0.3.2](https://github.com/pando85/timer/tree/v0.3.2) - 2022-06-10

### Fixed

* Bumps [regex](https://github.com/rust-lang/regex) from 1.5.4 to 1.5.5.
  * [Release notes](https://github.com/rust-lang/regex/releases)
  * [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)
  * [Commits](https://github.com/rust-lang/regex/compare/1.5.4...1.5.5)
* Update all dependencies

## [v0.3.1](https://github.com/pando85/timer/tree/v0.3.1) - 2022-01-05

### Fixed

* Replace recursive function with loop

## [v0.3.0](https://github.com/pando85/timer/tree/v0.3.0) - 2021-12-28

### Added

* Print info that fits on the terminal
* Automatically add body to GitHub release

## [v0.2.1](https://github.com/pando85/timer/tree/v0.2.1) - 2021-12-13

### Fixed

* Fix regex matching just counter times

## [v0.2.0](https://github.com/pando85/timer/tree/v0.2.0) - 2021-12-11

### Added

* Add seconds input support without tag
* Add parse for `min`

## [v0.1.4](https://github.com/pando85/timer/tree/v0.1.4) - 2021-12-09

### Fixed

* Fix terminal flickering

## [v0.1.3](https://github.com/pando85/timer/tree/v0.1.3) - 2021-12-07

### Added

* Add AUR packages automatic build and publish

## [v0.1.2](https://github.com/pando85/timer/tree/v0.1.2) - 2021-12-07

### Fixed

* Fix github-actions for publishing packages
* Add missing metadata to publish crates

## [v0.1.1](https://github.com/pando85/timer/tree/v0.1.1) - 2021-12-07

### Fixed

* Release with signed tags
* Add `Cargo.lock` file

## [v0.1.0](https://github.com/pando85/timer/tree/v0.1.0) - 2021-12-07

Initial release

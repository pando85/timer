# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.11.3](https://github.com/pando85/timer/tree/v0.11.3) - 2026-05-20

### Fixed

- Restore parallel beep and sound playback ([0d36421](https://github.com/pando85/timer/commit/0d36421e5b215ff2de19005717e0924fd91da0a5))

### Documentation

- Add git rules to prevent amend mistakes ([782aa97](https://github.com/pando85/timer/commit/782aa97843b54ab8bb8cc9fea064596f685a68c8))

### Build

- deps: Update mindsers/changelog-reader-action action to v2.4.0 ([d0cfcf3](https://github.com/pando85/timer/commit/d0cfcf3a152d77dcbe6609f745262ae8678c96e7))

## [v0.11.2](https://github.com/pando85/timer/tree/v0.11.2) - 2026-05-20

### Fixed

- Replace tailcall recursion with loop to fix runtime panic ([8b8a621](https://github.com/pando85/timer/commit/8b8a621cc20bedd3c4cfd1ba007aede76eba1422))

### Build

- deps: Update mindsers/changelog-reader-action action to v2.3.0 ([1d5793b](https://github.com/pando85/timer/commit/1d5793b76c081332205c458b739677be05566f11))

## [v0.11.1](https://github.com/pando85/timer/tree/v0.11.1) - 2026-05-18

### Added

- Add git cliff support for github squash commits ([1fcca27](https://github.com/pando85/timer/commit/1fcca27a2c27e617ff7ef6ab17cb8c24d9b3a8c9))

### Fixed

- ci: Handle shallow clones in release script ([4488932](https://github.com/pando85/timer/commit/4488932bd9d761a3e7ac06a80f8e6b42404a2a8e))
- deps: Update Cargo.lock for signal-hook 0.4.1 ([aabe8ce](https://github.com/pando85/timer/commit/aabe8cecbcb92e7c5fb8ab4d1867fd22de5301b8))
- Update rodio API for v0.22 compatibility ([435f916](https://github.com/pando85/timer/commit/435f9161c35146f8207661af43b385dd0edc271a))
- Update tailcall syntax for v2.0 API ([8361eb5](https://github.com/pando85/timer/commit/8361eb5c515280bef40b481490f1afb2d5e70b86))
- Use tailcall v2.0.1 (v2.0.0 was yanked) ([571d23b](https://github.com/pando85/timer/commit/571d23bf04d2ec8441b21c1b88bbe191c9b538ca))
- Update code for tailcall v2 compatibility ([3dc2c86](https://github.com/pando85/timer/commit/3dc2c864e3532d0587adbe2e4ac0ccb9b1e1ae83))
- Resolve clippy warnings for tailcall v2 ([e257ef3](https://github.com/pando85/timer/commit/e257ef35f40f600ea602b9bd2671c0b84bb8cc60))
- Improve reliability and remove dead code ([78dbfa3](https://github.com/pando85/timer/commit/78dbfa34d7a21d3f312a401489b956f25a246e38))

### Build

- ci: Automerge patch requests ([97ebfa5](https://github.com/pando85/timer/commit/97ebfa5c9f81686788c94ef720762ea527dc08f5))
- ci: Automerge minor requests ([a7e5fec](https://github.com/pando85/timer/commit/a7e5fec91b74c775b802c1ae0cddf98abd5b4ee9))
- deps: Update Rust crate signal-hook to 0.4 ([418baac](https://github.com/pando85/timer/commit/418baac0924106de5811415fa60b1c99202aa7ea))
- deps: Update Rust crate clap to v4.5.54 ([f98e745](https://github.com/pando85/timer/commit/f98e7458a8388398ccdd0dd9de51adb6dcdcfae0))
- deps: Update Rust crate libc to v0.2.180 ([29ee705](https://github.com/pando85/timer/commit/29ee705444e51d113a1edd50c92d046174c555b6))
- deps: Update Rust crate time to v0.3.45 ([9ced5c6](https://github.com/pando85/timer/commit/9ced5c62aa013608c239ba81e2f1b63fbbcfbbdb))
- deps: Update pre-commit hook adrienverge/yamllint to v1.38.0 ([2b33000](https://github.com/pando85/timer/commit/2b330004c2a1a75130d93e672d1f9c0ebae0e0f6))
- deps: Update pre-commit hook alessandrojcm/commitlint-pre-commit-hook to v9.24.0 ([206dee3](https://github.com/pando85/timer/commit/206dee34f0ca189695a0e64ae77d3624bafdbdbc))
- deps: Update Rust crate nix to 0.31 ([83c9f10](https://github.com/pando85/timer/commit/83c9f1076efee7e5f5dd3c26838cb787f20ceca0))
- deps: Update Rust crate time to v0.3.46 ([d84b612](https://github.com/pando85/timer/commit/d84b612463f7240fd680eb54de23f0a785be1a66))
- deps: Update Rust crate signal-hook to v0.4.2 ([46fa41f](https://github.com/pando85/timer/commit/46fa41fce77cc9cb8e163ee4a4953451e27ae35d))
- deps: Update Rust crate signal-hook to v0.4.3 ([1c690cd](https://github.com/pando85/timer/commit/1c690cd48e3b49cb2336efe8d9a5b58c84092fc4))
- deps: Update Rust crate clap to v4.5.55 ([a5d271f](https://github.com/pando85/timer/commit/a5d271f7e5ae2eaec61d034503218ce4699e9cd4))
- deps: Update Rust crate clap to v4.5.56 ([47888d1](https://github.com/pando85/timer/commit/47888d1f27ba2a8a747065aa2c4c889dd3ee4220))
- deps: Update Rust crate clap to v4.5.57 ([b2c95d7](https://github.com/pando85/timer/commit/b2c95d7e6dc9ef3da350feea758b8472966bbd17))
- deps: Update Rust crate regex to v1.12.3 ([d830629](https://github.com/pando85/timer/commit/d830629e5bad8d02ca5123ef581a36af7009eab0))
- deps: Bump bytes from 1.10.1 to 1.11.1 ([66b816c](https://github.com/pando85/timer/commit/66b816c56e6ba591b676675493a9b3649774064c))
- deps: Update Rust crate time to v0.3.47 ([107d9dd](https://github.com/pando85/timer/commit/107d9ddbb7e0f6ea9a44aa2e970b18851892c6c6))
- deps: Update Rust crate clap to v4.5.58 ([3738912](https://github.com/pando85/timer/commit/37389121cf2957074654316cda6845cfda504b0a))
- deps: Update Rust crate clap to v4.5.59 ([b414f80](https://github.com/pando85/timer/commit/b414f806a360d86907d6489043c95f911c0b3da1))
- deps: Update Rust crate clap to v4.5.60 ([bbffa9a](https://github.com/pando85/timer/commit/bbffa9a02083b1833989e8a29dd6eadeadd7d402))
- deps: Update Rust crate nix to v0.31.2 ([147a52a](https://github.com/pando85/timer/commit/147a52a59f76c7c1eb902f577d2554e0eacca1ad))
- deps: Update Rust crate rodio to 0.22 ([acef723](https://github.com/pando85/timer/commit/acef723082c3a9fa46a2cf5af02deae8f19d36ec))
- deps: Update Rust crate rodio to v0.22.2 ([ed51ca5](https://github.com/pando85/timer/commit/ed51ca54ac75f95e2fbb09f0685273e18cb629f8))
- deps: Update Rust crate libc to v0.2.183 ([ca1a6d0](https://github.com/pando85/timer/commit/ca1a6d051b7a291076b8de476b8c4061ce1adffe))
- deps: Update Rust crate clap to v4.6.0 ([fa9c8a3](https://github.com/pando85/timer/commit/fa9c8a37e3bb02d8ec186262478daa583cacd05f))
- deps: Update Rust crate libc to v0.2.184 ([101a43a](https://github.com/pando85/timer/commit/101a43a0bd246417e6e278d3a2ae7a02c2d85b1e))
- deps: Update KSXGitHub/github-actions-deploy-aur action to v4.1.2 ([f7f57a3](https://github.com/pando85/timer/commit/f7f57a362a51d08c355ea509a59b329078de6b40))
- deps: Update Rust crate signal-hook to v0.4.4 ([b7bee8e](https://github.com/pando85/timer/commit/b7bee8ea945b8cba0244b4b1362ec8f15fd28d6d))
- deps: Update Rust crate libc to v0.2.185 ([93de506](https://github.com/pando85/timer/commit/93de506dd81b0759e2d934caf1fffefde057b40e))
- deps: Update Rust crate clap to v4.6.1 ([e4436fe](https://github.com/pando85/timer/commit/e4436fefbff5c794d91f106067b24edf073d171d))
- deps: Update KSXGitHub/github-actions-deploy-aur action to v4.1.3 ([239b5fb](https://github.com/pando85/timer/commit/239b5fbcf2af8a9b93b44efe4204c576d44b3702))
- deps: Update Rust crate tailcall to v2 ([2e54c28](https://github.com/pando85/timer/commit/2e54c28e191901aa3545fdc49a819cdcc40a9728))
- deps: Update Rust crate tailcall to v2.0.4 ([e983c4e](https://github.com/pando85/timer/commit/e983c4e256730dd1f07d71772de4f4818cf44410))
- deps: Update Rust crate tailcall to v2.1.0 ([8441c6c](https://github.com/pando85/timer/commit/8441c6cdaa51fdd0bf1e90f9f400254f354abbbc))
- deps: Update Rust crate tailcall to v2.2.0 ([93209cf](https://github.com/pando85/timer/commit/93209cf20a390245286e000de4f7df807141b810))
- deps: Update softprops/action-gh-release action to v3 ([25ad187](https://github.com/pando85/timer/commit/25ad187f4f78d9f9c0a5fb6d60b14278cbbeb2ec))
- deps: Update clechasseur/rs-clippy-check action to v6 ([1ab3ffd](https://github.com/pando85/timer/commit/1ab3ffdfe7f6597822e41121ac6e959f654abee3))
- deps: Update Rust crate tailcall to v2.2.1 ([1c36893](https://github.com/pando85/timer/commit/1c36893048fcd4407efeea8b1d6605ae94663d5b))
- deps: Update Rust crate tailcall to v2.2.2 ([68dbecc](https://github.com/pando85/timer/commit/68dbecc860172a717c37456bddc78a8f0dc20c80))
- deps: Update Rust crate tailcall to v2.3.0 ([8f8da59](https://github.com/pando85/timer/commit/8f8da59debff69a4efd11621bbdee796dbc06c46))
- deps: Update Rust crate libc to v0.2.186 ([a7c84f9](https://github.com/pando85/timer/commit/a7c84f922ca8ce8c91c703f57fafa265cb853e92))
- deps: Update clechasseur/rs-clippy-check action to v6.0.3 ([4d728a6](https://github.com/pando85/timer/commit/4d728a64e70461e580bfc5116890c657afc3f3d4))
- deps: Update pre-commit hook alessandrojcm/commitlint-pre-commit-hook to v9.25.0 ([19c5542](https://github.com/pando85/timer/commit/19c5542c0cf07b287fde97d0331154be749d024c))
- deps: Update clechasseur/rs-clippy-check action to v6.0.4 ([191e6a5](https://github.com/pando85/timer/commit/191e6a5b67ce522a819a29ba45ba3eb8031f13dc))
- deps: Update Rust crate nix to v0.31.3 ([696aeab](https://github.com/pando85/timer/commit/696aeabe7086026af0eae71504ad3c2c467c87bd))

## [v0.11.0](https://github.com/pando85/timer/tree/v0.11.0) - 2025-12-25

### Added

- Add stopwatch function ([9f7eb34](https://github.com/pando85/timer/commit/9f7eb34ae7bf727e50878980b6abb8d0fd327b93))

### Fixed

- Ensure CHANGELOG commit IDs are correct on release process ([f22bb4b](https://github.com/pando85/timer/commit/f22bb4bd257bae1c3e7028bcab0d8383b73b6c62))

### Documentation

- Fix build status link ([703df54](https://github.com/pando85/timer/commit/703df54a65baf31ddde494d961184ef11c0f95a9))
- Add white background to the logo ([cbd895b](https://github.com/pando85/timer/commit/cbd895ba017fefa19fc35ae411473564b8bf4bf0))

### Build

- deps: Update Rust crate clap to v4.5.52 ([96a85ab](https://github.com/pando85/timer/commit/96a85ab77c4e0471c29e2189bf185039eda59210))
- deps: Update Rust crate clap to v4.5.53 ([c7b3d3b](https://github.com/pando85/timer/commit/c7b3d3bda108690c022cc999c2c23161f95bb62f))
- deps: Update actions/checkout action to v6 ([6552868](https://github.com/pando85/timer/commit/65528683206b960c5ba24ba61652624e05ee9871))
- deps: Update actions/cache action to v5 ([a11a1dc](https://github.com/pando85/timer/commit/a11a1dcd7d33050eb0f1071941501e122170f395))
- deps: Update Rust crate libc to v0.2.178 ([aa3a393](https://github.com/pando85/timer/commit/aa3a3933133e13ed1ffc661e2c6f86875daa2247))

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

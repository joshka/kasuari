# Changelog

## [0.4.0-alpha.2](https://github.com/joshka/kasuari/compare/v0.4.0-alpha.1...v0.4.0-alpha.2) - 2025-04-04

### Added

- make the crate `no_std` ([#2](https://github.com/joshka/kasuari/pull/2))

### Fixed

- clippy lints
- doc tests

### Other

- add release-plz automation
- rename ci workflow
- tweak cargo.toml, unignore cargo.lock
- add simple ci workflow

## Kasuari 0.4.0 (unreleased)

This release is a fork of the library under a new name, `Kasuari`. The name change is to avoid confusion
with the original `Cassowary-rs` library, which has been unmaintained since 2018. The name `Kasuari` is
the Indonesian name for the Cassowary bird.

- Initial Kasuari release
- Update to Rust 2021 edition
- Update docs
- Reformat code
- Cleanup lints
- Move code to appropriate modules
- Add `Debug` implementations for various types
- Implement Error for errors and provide better error messages
- Add error source to InternalSolverError for better debugging
- Add tests
- Spell optimise with US english (optimize)
- Make Strength a newtype instead of f64
- Pass constrains by value instead of reference

## Casssowary 0.3.0

- Various fixes (PR #4) from @christolliday.
  > Main breaking change is that variables no longer silently initialise to zero and will report
  their initial value in the first call to `fetch_changes`, also `has_edit_variable` now takes
  `&self` instead of `&mut self`.

## Casssowary 0.2.1

- Fixed crash under certain use cases. See PR #1 (Thanks @christolliday!).

## Casssowary 0.2.0

- Changed API to only report changes to the values of variables. This allows for more efficient use
  of the library in typical applications.

## Casssowary  0.1

Initial release

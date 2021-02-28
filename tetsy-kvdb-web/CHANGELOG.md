# Changelog

The format is based on [Keep a Changelog]. 

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

## [Unreleased]

## [0.3.2] - 2020-02-28
- Utilized tetsy-util-mem and upgrade rocksdb to 0.14

## [0.2.0] - 2019-12-19
### Changed
- Default column support removed from the API
  - Column argument type changed from `Option<u32>` to `u32`
  - Migration `None` -> unsupported, `Some(0)` -> `0`, `Some(1)` -> `1`, etc.

## [0.1.1] - 2019-10-24
### Dependencies
- Updated dependencies (https://github.com/paritytech/parity-common/pull/239)

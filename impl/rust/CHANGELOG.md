# Changelog

All notable changes to the `adp` crate. The format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/), and the
crate follows Semantic Versioning on its own line (ADP-A-008 C2.1).

Each release names the spec versions it implements. The specs have their
own history in the root `CHANGELOG.md`.

## [0.1.0] - unreleased (date set when the maintainer tags `adp-v0.1.0`)

**Implements:** ADP-1 1.1.1

### Added

- `v1::validate_dispatch` and `v1::validate_report`: every rule of
  ADP-1 §7.1 (V-01 to V-13) and §7.2 (R-01 to R-13).
- `Verdict`, `Violation` (rule, JSON Pointer, message) and `Rule`, all
  `#[non_exhaustive]`.
- `CLAIMS` and `v1::SPEC_VERSION`.
- `tests/conformance.rs`: all 68 ADP-1 vectors, verdict and exact rule.

# Releasing the `adp` crate

From ADP-A-008 C2.1. If this file and the proposal disagree, the
proposal wins.

## Identity

| Item | Setting |
|------|---------|
| Crate name | `adp`, `publish = false` until a later proposal (ADP-A-008 Q8) |
| Git tag | `adp-vX.Y.Z`, e.g. `adp-v0.1.0`, on `stable` |
| Crate changelog | `impl/rust/CHANGELOG.md`, with an **Implements** line per release |
| Root changelog | A row in the **Rust implementation** table of `CHANGELOG.md` |

## Spec → crate version map

Kept in `README.md` and the `lib.rs` crate docs, and as the constants
`adp::v1::SPEC_VERSION` (and `adp::v2::SPEC_VERSION` from 0.2.0).

| Crate | ADP-1 | ADP-2 | Stage |
|-------|-------|-------|-------|
| 0.1.0 | 1.1.1 | —     | 1     |
| 0.2.0 | 1.1.1 | 2.0.1 | 2     |

## Which bump

| Spec change | Crate bump while 0.x | Crate bump from 1.0 |
|-------------|----------------------|---------------------|
| Erratum or editorial Patch, no new rule, no verdict change | patch | patch |
| New rule, or a verdict that tightens | minor | minor |
| Spec Minor (e.g. a new optional field) | minor | minor |
| Spec Major (e.g. ADP-2 3.0.0) | minor | major |
| Crate API break, no spec change | minor | major |
| MSRV raised | minor | minor |

A new rule is a new `Rule` variant. `Rule` is `#[non_exhaustive]`, so
that breaks no caller's `match`. Rejecting more input is listed under
**Changed**.

A spec Major gets a new module (`v3`). The old module is `#[deprecated]`
for one minor release, then removed.

## Public API

- `Rule`, `Violation` and `Verdict` are `#[non_exhaustive]`, with
  private fields behind accessors.
- The only foreign type in the API is `serde_json::Value`. A
  `serde_json` major bump is a crate major bump.
- `cargo-semver-checks` joins CI with the first published release.

## MSRV

`rust-version = "1.85"`. CI tests 1.85.0 and the pinned toolchain
1.99.0. The MSRV is raised only in a minor release.

## Checklist

1. Bump `version` in `Cargo.toml`; update `Cargo.lock`.
2. Add the `CHANGELOG.md` entry with its **Implements** line, and the
   row in the root `CHANGELOG.md`.
3. Update the version map in `README.md` and `src/lib.rs`.
4. Open the PR into `main`; it merges under ADP-0 §8.1.
5. The maintainer merges `main` into `stable`, tags `adp-vX.Y.Z` on
   `stable`, pushes the tag by name, and publishes a GitHub pre-release
   with the changelog entry (ADP-0 §8.2).

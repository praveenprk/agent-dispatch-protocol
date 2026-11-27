# adp

Reference validator for the Agent Dispatch Protocol (ADP), in Rust.

It checks a dispatch or report envelope against the validation rules of
the spec and says which rules it breaks, and where.

```rust
let verdict = adp::v1::validate_dispatch(&envelope);
assert!(verdict.is_valid());

for v in verdict.violations() {
    println!("{} at {}: {}", v.rule(), v.pointer(), v.message());
}
```

`envelope` is a `serde_json::Value`. A caller holding YAML, TOML or
another serialization parses it into the JSON data model first.

## What it claims

| Crate | ADP-1 | ADP-2 |
|-------|-------|-------|
| 0.1.0 | 1.1.1 | —     |

For each claimed document, the crate gives every conformance vector in
`conformance/` its expected verdict and, on a `fail` vector, exactly
the vector's rule. `tests/conformance.rs` checks this, and CI runs it.

The crate is the reference implementation (ADP-0 §3.4). It is not
binding: where it disagrees with the spec text, the schemas or the
vectors, the crate has a defect.

## Properties

- Pure functions. No I/O, no network, no clock, no global state.
- `#![forbid(unsafe_code)]`.
- One runtime dependency: `serde_json`.
- SemVer (ADP-1 §9) is parsed by hand from the SemVer 2.0.0 grammar,
  with no upper bound on numbers.
- MSRV: Rust 1.85 (edition 2024).
- Not published to crates.io (`publish = false`).

## Versions

The crate has its own version line, separate from the specs. The
bump rules and the release checklist are in [RELEASING.md](RELEASING.md),
from ADP-A-008 C2.1. Changes are in [CHANGELOG.md](CHANGELOG.md).

## Develop

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

## License

Apache-2.0, as the rest of the repository.

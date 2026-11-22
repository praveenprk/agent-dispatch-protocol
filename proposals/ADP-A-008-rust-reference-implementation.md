# ADP-A-008: Rust reference implementation

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-008                              |
| **Target**  | ADP-0 (`spec/ADP-0.md`), new `impl/rust/`, `.github/workflows/adp.yml`, `README.md` |
| **Resolves**| No defect. Addresses the need below (see Anchor) |
| **Bump**    | ADP-0 Minor: 1.2.0 → 1.3.0. ADP-1 and ADP-2: none (Q7) |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-17                             |

---

## Outcome and impact

**In one line:** ADP gets one reference validator, written in Rust,
that CI runs against every conformance vector, and the README stops
promising a Python one.

| Area | Effect when applied |
|------|---------------------|
| ADP-0 | New artifact class **Implementation** (`impl/`). The reference implementation never takes precedence over spec, schema or vectors, and must pass every vector. §5.4 step 4 names it. §9 layout brought up to date. |
| New code | Rust crate `adp` at `impl/rust/`: pure functions, no I/O, no network, no `unsafe`. One runtime dependency (`serde_json`). |
| Conformance | No vector changes. The crate must give every vector its expected verdict **and** report exactly the vector's rule on a `fail` vector. |
| Checked for the first time | The 19 ADP-2 `authority` vectors (G-1 to G-7). Today no CI check runs them: there is no schema for a dispatch + grant chain. |
| CI | New job `implementation`: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --locked`. |
| README | `python/adp/` and the Python example replaced by the Rust crate and a Rust example. |
| Not changed | ADP-1, ADP-2, `schema/`, `conformance/`, `DEFECTS.md`. The Python CI scripts stay (Q5). |

Merging this proposal does not apply it. Application is a separate
change (see Application).

---

## Scenarios

One job throughout: a bank's orchestrator dispatches an agent to fix
a loan-interest rounding bug under change ticket CHG-4821. The bank's
platform team wants to check every envelope before it reaches the agent.

| Q | Situation | Today | With the recommendation | With the alternative |
|---|-----------|-------|-------------------------|----------------------|
| Q1 | The platform team wants a validator to embed in its Java and Go services | README promises `python/adp/`; it does not exist | One Rust core. Bindings for other languages come later from the same core (Q8). | A Python core: an interpreter in every service, and each later language port re-implements the rules. |
| Q2 | The agent's dispatch has `produce: [{action: create}]` (no `path`) | A schema rejects it, but says only "required property missing" | The crate rejects it as **V-13**, and the test proves it named V-13, not some other rule | Checking verdicts only: a validator that rejects for the wrong reason still passes |
| Q3 | The grant for CHG-4821 allows `src/fees/**`; the dispatch writes `ci/hook` | No CI check runs the 19 ADP-2 authority vectors | Stage 2 runs them: G-4 rejects the write | All of it in one PR: one very large review |
| Q4 | A contributor's PR breaks the validator | Nothing in CI builds Rust | The `implementation` job fails; once it is a required check, the PR cannot merge | A job that is not required: a red check can still be merged |
| Q5 | The schema files change | `check_conformance.py` checks schemas against vectors | Unchanged. Schemas are normative and still need their own check. | Removing it: nothing checks the published schemas any more |
| Q6 | A vector and the crate disagree | No implementation exists | The crate has a defect. The vector wins (ADP-0 §3.2). | Crate as Normative: two normative artifacts could disagree with no rule for which wins |
| Q7 | ADP-1 1.2.0 adds rule V-14; a contributor implements it, and the bank's platform team depends on `adp = "0.1"` | No rule for which crate number follows a spec change | C2.1 bump table: crate minor (0.2.0). `Rule` is `#[non_exhaustive]`, so the new variant breaks no `match`. The crate CHANGELOG says "Implements: ADP-1 1.2.0" and lists V-14 under Changed, because more envelopes are now rejected. | Crate version mirrors the spec: no number left for a crate-only fix, and one number cannot mirror ADP-1 1.x and ADP-2 2.x at once |
| Q8 | The bank asks for `pip install adp` or `npm install adp` | — | Out of scope. Later proposals, built on this crate. | In scope now: three build systems before the core is proven |

---

## Anchor

No defect. The need:

| Gap | Evidence at `0f5dbfd` |
|-----|------------------------|
| ADP-0 §5.4 step 4 and §5.6.4 describe a reference implementation; none exists | `grep -n "IMPLEMENTATION\|implementation" spec/ADP-0.md` → lines 228, 243, 281, 293 |
| README promises one that does not exist | `grep -n "python/adp" README.md` → line 86; `ls python` → no such directory |
| ADP-0 §3 has no class for implementation code, so code would default to Informative (§3.4) with no duty to pass the vectors | `sed -n 61,72p spec/ADP-0.md` |
| 19 ADP-2 `authority` vectors are checked by nothing | `check_conformance.py` `SCHEMAS["ADP-2"]` has `grant` and `record` only |
| Maintainer decision on the language (2026-11-17): Rust, not Python. A protocol validator should be a compiled, embeddable core. | Chat, recorded here |

---

## Change

### C1: `spec/ADP-0.md`

**C1.1: §3 table.** New row after **Informative**:

```
| **Implementation** | `impl/` (the reference implementation)     | No      |
```

and a new subsection after §3.3:

```
### 3.4 Implementation

The reference implementation lives in `impl/`. It is not binding: it
never takes precedence over spec text, schemas or vectors. It MUST
give every conformance vector of every document it claims its
expected verdict, except where §5.6.4 applies. A disagreement is a
defect in the implementation. The README states which document
versions it claims.
```

§3.4 "Unlisted files" becomes §3.5. The only other reference to
§3.4 is in the Revision Log, which is history and stays as written.

**C1.2: §3.2 Precedence**, rule 3 becomes:

```
3. Informative and implementation artifacts never take precedence
   over normative ones. One that contradicts normative text is a
   defect in that artifact.
```

**C1.3: §5.4 step 4:**

Before: `4. **IMPLEMENTATION** — any reference implementation updated until every conformance vector yields its expected verdict.`

After: `4. **IMPLEMENTATION** — the reference implementation (§3.4) updated until every conformance vector it claims yields its expected verdict.`

**C1.4: §9 Repository Layout.** Adds the files that exist today and
the new directory:

```
agent-dispatch-protocol/
├── spec/
│   ├── ADP-0.md          # this document (process)
│   ├── ADP-1.md          # protocol specification
│   └── ADP-2.md          # dispatch authority
├── proposals/            # ADP-A-NNN-<slug>.md
├── conformance/          # vectors (§5.6)
├── schema/               # normative JSON Schemas
├── impl/rust/            # reference implementation (§3.4)
├── examples/             # informative
├── experiments/          # records (§3.3)
├── scripts/              # hooks and CI checks
├── DEFECTS.md            # defect register
├── CHANGELOG.md          # informative (release notes)
├── README.md             # informative
├── AGENTS.md             # informative (repo rules)
└── LICENSE
```

**C1.5: header and Revision Log.** Version `1.2.0` → `1.3.0`. Append:

```
| <applied>  | 1.3.0. Implementation class (§3, §3.4);        |
|            | precedence rule 3 covers it (§3.2); §5.4 step 4 |
|            | names it; layout updated (§9). ADP-A-008.       |
```

### C2: `impl/rust/` (new)

| Item | Setting |
|------|---------|
| Crate | `adp`, library, `version = "0.1.0"`, `publish = false`, license Apache-2.0 |
| Rust | Edition 2024; `rust-version = "1.85"`; CI pinned to toolchain 1.99.0 |
| Dependencies | Runtime: `serde_json` only. Dev: none beyond it. `Cargo.lock` committed. |
| Safety | `#![forbid(unsafe_code)]`; no I/O, no network, no globals, no clock |
| Input | `&serde_json::Value`. Any serialization (YAML, TOML, …) is parsed by the caller into the JSON data model. |
| API, ADP-1 | `adp::v1::validate_dispatch(&Value) -> Verdict`, `adp::v1::validate_report(&Value) -> Verdict` |
| API, ADP-2 | `adp::v2::validate(dispatch: &Value, grant_chain: &[Value]) -> Verdict`, `adp::v2::validate_grant(&Value) -> Verdict`, `adp::v2::validate_record(&Value) -> Verdict` |
| Verdict | `Verdict` with `is_valid()` and `violations() -> &[Violation]`. `Violation` with `rule() -> Rule`, `pointer() -> &str` (JSON Pointer, RFC 6901) and `message() -> &str`. `Rule` is an enum: `V01`…`V13`, `R01`…`R13`, `G01`…`G10`, with `Display` giving the spec id (`V-13`). Stability rules: C2.1(d). |
| No regex, no glob crate | SemVer (ADP-1 §9) and paths and globs (ADP-2 §4.2) are hand-written from the spec text, so the behaviour is the spec's, not a library's |
| Conformance test | `impl/rust/tests/conformance.rs` reads every file under `conformance/`. Per vector: verdict equals `expect`; on `fail`, the set of reported rules equals the vector's `rules`; `known_defect` handled as ADP-0 §5.6.4. |
| Claims | A constant lists the documents the crate claims. The test runs only their vectors and fails if a claimed document has a vector the crate does not handle. |

### C2.1 Crate versioning and releases

The crate and the specs have separate version lines. The tables below
fix how they relate, so a contributor never has to guess a number and a
user can always tell which spec a crate release implements.

**(a) Identity**

| Item | Setting | Precedent |
|------|---------|-----------|
| crates.io name | `adp`; unclaimed when checked on 2026-11-18 | Publishing stays `publish = false` until Q8's later proposal |
| Git tag | `adp-vX.Y.Z` (`{crate}-v{version}`), e.g. `adp-v0.1.0` | Default tag form of release-plz and cargo-release in multi-crate repositories. Cannot be confused with the spec tags `adp-1/v1.1.1`. |
| Crate changelog | `impl/rust/CHANGELOG.md`, Keep a Changelog 1.1.0 sections (Added, Changed, Fixed, …), plus an **Implements** line per release | tokio, hyper and rustls each keep a changelog next to the crate |
| Root `CHANGELOG.md` | Gains a **Rust implementation** table: crate version, date, Implements, link to the crate changelog | One entry point for readers who come from the spec |

**(b) Spec → crate version map.** In the crate README and in the
`lib.rs` crate docs (so docs.rs shows it), and as constants
`adp::v1::SPEC_VERSION` and `adp::v2::SPEC_VERSION` (`&str`):

| Crate | ADP-1 | ADP-2 | Stage |
|-------|-------|-------|-------|
| 0.1.0 | 1.1.1 | — | 1 |
| 0.2.0 | 1.1.1 | 2.0.1 | 2 |

**(c) Bump rule: spec change → crate change**

| Spec change | Crate bump while 0.x | Crate bump from 1.0 | Why |
|-------------|----------------------|---------------------|-----|
| Erratum or editorial Patch, no new rule, no verdict change | patch | patch | Docs or behaviour fix only |
| New rule (as V-13, R-13) or a verdict that tightens | minor | minor | `Rule` is `#[non_exhaustive]`, so a new variant breaks no caller's `match`. Rejecting more input is listed under **Changed**. |
| Spec Minor (e.g. new optional field) | minor | minor | Additive |
| Spec Major (e.g. ADP-2 3.0.0) | minor | **major** | New module `v3`; the old module is `#[deprecated]` for one minor release, then removed |
| Crate API break, no spec change | minor | major | Plain SemVer for Rust APIs (Cargo treats 0.x minor as breaking) |
| MSRV raised | minor | minor | Never in a patch release |

**(d) Public API stability**

- `Rule`, `Violation` and `Verdict` are `#[non_exhaustive]`; fields are
  private behind accessors.
- The only foreign type in the public API is `serde_json::Value` (the
  input). A `serde_json` major bump is therefore a crate major bump.
- `cargo-semver-checks` joins CI with the first published release
  (Q8). Before that there is no public API to compare against.

**(e) MSRV**

- `rust-version = "1.85"`, the first release with edition 2024.
- CI tests MSRV 1.85.0 and the pinned toolchain 1.99.0 (C3).

**(f) Release checklist**, in `impl/rust/RELEASING.md`:

1. Bump `version` in `Cargo.toml`; update `Cargo.lock`.
2. Add the `impl/rust/CHANGELOG.md` entry with its **Implements** line,
   and the row in the root `CHANGELOG.md`.
3. Update the version map (b) in the README and `lib.rs`.
4. Open the PR into `main`; it merges under ADP-0 §8.1.
5. The maintainer merges `main` into `stable`, tags `adp-vX.Y.Z` on
   `stable`, pushes the tag by name, and publishes a GitHub pre-release
   with the changelog entry (ADP-0 §8.2).

### C3: `.github/workflows/adp.yml`

New job `implementation` on `ubuntu-24.04`:

```yaml
  implementation:
    runs-on: ubuntu-24.04
    defaults:
      run:
        working-directory: impl/rust
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
      - run: rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy
      - run: rustup override set 1.99.0
      - run: cargo fmt --check
      - run: cargo clippy --locked --all-targets -- -D warnings
      - run: cargo test --locked
      - run: cargo doc --locked --no-deps
        env:
          RUSTDOCFLAGS: -D warnings
      - run: rustup toolchain install 1.85.0 --profile minimal
      - run: cargo +1.85.0 test --locked   # MSRV, C2.1(e)
```

No third-party action: `rustup` is on the runner image.

### C4: `README.md`

- Project Structure: `python/adp/  # Python reference implementation`
  → `impl/rust/  # Rust reference implementation`.
- Validation example:

```rust
let verdict = adp::v1::validate_dispatch(&envelope);
assert!(verdict.is_valid());
```

- Status: names the document versions the crate claims.

---

## Version impact

| Document | Bump | Reasoning |
|----------|------|-----------|
| ADP-0 | **Minor**, 1.2.0 → 1.3.0 | A new artifact class and a new duty (§3.4) are additions to the process. Nothing existing is reclassified: no file is in `impl/` today. |
| ADP-1 | None | No text, schema or vector changes |
| ADP-2 | None | No text, schema or vector changes |

The crate has its own SemVer, starting at 0.1.0. It is unrelated to
the spec versions (Q7).

---

## Conformance impact

No vector is added, changed or flips. After application the crate
must pass:

| Document | Vectors | Kinds | Checked by CI today |
|----------|---------|-------|---------------------|
| ADP-1 1.1.1 | 68 | dispatch, report | Schemas |
| ADP-2 2.0.1 | 33 | authority (19), record (9), grant (5) | Schemas for record and grant only |

If the crate and a vector disagree and the vector is right, the crate
is fixed. If the vector is wrong, a defect is filed (ADP-0 §4); the
vector is never edited to fit the crate.

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | Python reference implementation, as the README says | Maintainer decision: a protocol validator should be compiled and embeddable. Every other language would need its own port. |
| A2 | Go, C or Zig | Rust gives memory safety without a runtime, builds to native libraries and WebAssembly, and has mature JSON (`serde_json`). |
| A3 | Validate with a JSON Schema crate against `schema/` | Schemas say "required property missing", not "V-13". They cannot express G-1 to G-7 (two documents at once). It would repeat the schemas, not test them. |
| A4 | Make the implementation Normative | Two normative artifacts could disagree with no rule for which wins. Spec, schema and vectors already define the contract. |
| A5 | Leave code in `impl/` Informative (§3.4 default) | Informative carries no duty to pass the vectors, and the README would keep promising more than it delivers. |
| A6 | Cargo workspace at the repository root | A root `Cargo.toml` and `target/` sit next to the spec. `impl/rust/` keeps the protocol repo readable. |
| A7 | Port the Python CI scripts in this proposal | They check schemas, the register and commits, not envelopes. Unrelated to the validator; one proposal must not bundle unrelated changes (ADP-0 §5.1). |

---

## Questions for the maintainer

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | Language and location? | **Rust, one crate `adp` at `impl/rust/`.** Compiled, embeddable, one core for later bindings; keeps Cargo files out of the repo root (A1, A2, A6 rejected). | Accepted as sensible default (2026-11-17). | Settled as drafted. |
| Q2 | What must the crate match on a vector? | **Verdict and rule.** On a `fail` vector the reported rules equal the vector's `rules`. A validator that rejects for the wrong reason is not conformant in any useful sense. | Accepted as sensible default (2026-11-17). | Settled as drafted. |
| Q3 | One application PR or staged? | **Two stages.** Stage 1: ADP-0 text, crate with ADP-1 (68 vectors), CI job, README. Stage 2: ADP-2 (33 vectors), README claim updated. Each stage reviewable on its own. | Accepted as sensible default (2026-11-17). | Settled as drafted. |
| Q4 | Make `implementation` a required status check? | **Yes, after stage 1 merges.** The maintainer adds it to `adp-ruleset` (a settings change, so the maintainer does it). Until then it runs but cannot block. | Accepted as sensible default (2026-11-17). | Settled as drafted. |
| Q5 | What happens to the Python CI scripts? | **Keep them.** `check_conformance.py` is the only check of the published schemas; the others check the register and commits. Porting them is a separate proposal if wanted (A7). | Accepted as sensible default (2026-11-17). | Settled as drafted. |
| Q6 | Class of `impl/`? | **New class Implementation: not binding, must pass every claimed vector.** (A4, A5 rejected.) | Accepted as sensible default (2026-11-17). | Settled as drafted. |
| Q7 | Versions? | **ADP-0 1.3.0 (Minor). Crate `adp` starts at 0.1.0 on its own SemVer line, tied to the specs by C2.1: tags `adp-vX.Y.Z`, a spec → crate version map in README, docs.rs and `SPEC_VERSION` constants, a bump table for every kind of spec change, `#[non_exhaustive]` public types, and a stated MSRV policy.** One crate implements two specs (1.1.1 and 2.0.1), so one number cannot mirror both, and crate-only fixes need their own number. | Accepted as sensible default (2026-11-17), with the crate versioning made explicit (C2.1) at the maintainer's request (2026-11-18). | C2.1 added; C3 gains doc and MSRV steps. |
| Q8 | Publishing and bindings (crates.io, Python, WASM, C)? | **Out of scope.** `publish = false`. Each later, as its own proposal, once the core passes every vector. | Accepted as sensible default (2026-11-17). | Settled as drafted. |

---

## Application (if ACCEPTED), per ADP-0 §5.4

**Stage 1 (one PR):**

1. **SPEC:** C1, one commit:
   `governance(adp-0): add the implementation class — ADP-A-008`.
2. **CONFORMANCE:** none.
3. **SCHEMA:** none.
4. **IMPLEMENTATION:** C2 with ADP-1 only and C3:
   `feat(impl): add the Rust reference validator for ADP-1 — ADP-A-008`,
   `feat(ci): run the reference implementation against the vectors — ADP-A-008`.
   The crate commit includes `impl/rust/CHANGELOG.md` (0.1.0,
   Implements: ADP-1 1.1.1) and `impl/rust/RELEASING.md` (C2.1).
5. **INFORMATIVE:** C4 and the root `CHANGELOG.md` rows:
   `docs(readme): point to the Rust reference implementation — ADP-A-008`.
6. **REGISTER:** none.

After merge, when the maintainer releases (ADP-0 §8.2): `main` into
`stable`, then tags `adp-0/v1.3.0` and `adp-v0.1.0` on `stable`, each
with its pre-release Release. Stage 1 changes `.github/workflows/adp.yml`,
so it is security-related and the maintainer merges it (ADP-0 §8.1
rule 4).

**Stage 2 (one PR):** C2 for ADP-2, crate 0.2.0 (Implements: ADP-1
1.1.1, ADP-2 2.0.1):
`feat(impl): add ADP-2 authority, grant and record validation — ADP-A-008`,
with the version map, both changelogs, and tag `adp-v0.2.0` on `stable`
when the maintainer releases.

---

## Reconciliation (2026-11-22)

At the maintainer's instruction of 2026-11-22 ("reconcile it as per
today's truth"), before merge, this proposal was brought in line with
what landed after its acceptance. The decision and every answer stand.

| What changed since 2026-11-18 | Effect here |
|-------------------------------|-------------|
| ADP-A-009 took ADP-0 1.2.0 (#25, #26) | ADP-0 bump is 1.2.0 → 1.3.0 (header, C1.5, Version impact, Q7) |
| ADP-0 §8.1: contributors merge into `main`; security-related changes are the maintainer's | Release checklist step 4; stage 1 (workflow change) is the maintainer's merge |
| ADP-0 §8.2: releases and tags on `stable`, by the maintainer | Release checklist step 5; tags after each stage |
| Repository renamed `agent-dispatch-protocol` | §9 layout root (C1.4) |
| eng-principles ENG-DEC-014: `0.y.z` until declared market-ready | No change: the crate starts at 0.1.0 |

---

## Decision
ACCEPTED 2026-11-18 by the maintainer (ADP-0 §5.3). Apply as written,
with the answers recorded under the questions for the maintainer.
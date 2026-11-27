# Changelog

Release notes for each ADP document version, then every change to the
repository since its first commit. Newest first.

- **Normative history:** the spec's own Revision Log. If this file and a
  spec disagree, the spec wins (ADP-0 §3).
- **Tag:** each version is tagged `adp-N/vX.Y.Z` on `stable`, after the
  maintainer merges `main` into `stable` (ADP-0 §8.2), and has a GitHub
  Release. Tags made before ADP-0 1.2.0 sit on the merge commit that put
  the version on `main`.
- **Status:** every document is a Draft. Releases are marked pre-release.

## Latest

| Document | Version | Tag |
|----------|---------|-----|
| ADP-0 Governance | 1.3.0 | `adp-0/v1.3.0` (when released) |
| ADP-1 Agent Dispatch Protocol | 1.1.1 | `adp-1/v1.1.1` |
| ADP-2 Dispatch Authority | 2.0.1 | `adp-2/v2.0.1` |
| Rust crate `adp` | 0.1.0 | `adp-v0.1.0` (when released) |

---

## ADP-1: Agent Dispatch Protocol

| Version | Date | Bump | Change | Upgrade impact | Defects closed | Proposal | PR |
|---------|------|------|--------|----------------|----------------|----------|----|
| 1.1.1 | 2026-11-17 | Patch | `SOURCE` is the `source` field, not a keyword. Field constraints written down (§4.5, §5.5) and checked by new V-13 and R-12. `COMPLETE` requires every exit result `true` (new R-13). Tier selection is informative, lowest tier wins on conflict. Deduplication is SHOULD NOT. | A `COMPLETE` report with a failed exit result is now rejected: report `BLOCKED` instead. Nothing else the published schemas accepted is rejected. | D-001, D-002, D-008, D-010, D-013, D-024, D-025 | ADP-A-007 | #21, #22 |
| 1.1.0 | 2026-11-16 | Minor | Extensions live in `metadata` on the work item, dispatch and report, with keys `x-{namespace}-{name}`. Unknown fields are rejected at every level for a known minor version, ignored for a higher one. New V-12, R-10, R-11. | Move extension keys into `metadata`. A misspelt field is now rejected. | D-003, D-006, D-007 | ADP-A-005 | #15, #16 |
| 1.0.2 | 2026-11-16 | Patch | `adp_version` is SemVer 2.0.0; major version 1 checked by new V-11 and R-09. | An envelope with a non-SemVer version, or a major version other than 1, is rejected. | D-004, D-009 | ADP-A-004 | #13, #14 |
| 1.0.1 | 2026-11-15 | Patch | Key words per BCP 14. The §7.1 and §7.2 rules bind validators. | None for envelopes. | D-012 | ADP-A-001 | #7, #11 |
| 1.0.0 | 2026-11-15 | — | Initial specification (written 2026-06-26): 10 keywords frozen, dispatch and report envelopes, compression tiers, validation contract. | — | — | — | #2 |

## ADP-2: Dispatch Authority

| Version | Date | Bump | Change | Upgrade impact | Defects closed | Proposal | PR |
|---------|------|------|--------|----------------|----------------|----------|----|
| 2.0.1 | 2026-11-17 | Patch | §3 note: `SOURCE` is not an ADP-1 keyword. Editorial. | None. | — | ADP-A-007 | #21, #22 |
| 2.0.0 | 2026-11-17 | Major | Grant id, paths, globs and overlap defined (§4.1, §4.2). Codebase event record and `validate_record` (§4.3, §6). AT-VERIFY and codebase events are never inferred (§3). Every limiting grant field required (G-10). | Grants use `adp_grant_version` 2.x, carry every limiting field, and give `expires_at` in UTC (`Z`). Records are full documents checked by `validate_record`. | D-018, D-019, D-020, D-021, D-022 | ADP-A-006 | #19, #20 |
| 1.0.0 | 2026-11-16 | — | Initial Draft: approval-rooted, session-bound grants; human signature for creating or deleting a codebase; approval and merge stay human. | — | — | ADP-A-002 | #8, #10, #17 |

## ADP-0: Governance

| Version | Date | Bump | Change | Upgrade impact | Defects closed | Proposal | PR |
|---------|------|------|--------|----------------|----------------|----------|----|
| 1.3.0 | 2026-11-26 | Minor | New artifact class Implementation (§3.4): `impl/` holds the reference implementation, which is not binding and must pass every vector of each document it claims. Precedence rule 3 covers it (§3.2); §5.4 step 4 names it; §9 layout brought up to date. | None for envelopes. The reference implementation now has a defined duty and place. | — | ADP-A-008 | #24, #29 |
| 1.2.0 | 2026-11-22 | Minor | §8.1 contributors merge into `main` when checks pass, nothing is asked of the maintainer, no decision is recorded and the change is not security-related. §8.2 releases: the maintainer merges `main` into `stable` and tags there. | Routine changes no longer wait for the maintainer's merge. Decisions, security-related changes and releases stay with the maintainer. | — | ADP-A-009 | #25, #26 |
| 1.1.0 | 2026-11-15 | Minor | §5.6 conformance suite format: layout, vector file, rules for vectors, known-defect markers. | Conformance vectors follow §5.6. | — | ADP-A-003 | #9, #12 |
| 1.0.0 | 2026-11-15 | — | Initial governance: artifact classes and precedence, defect register, proposal process, application order, errata, version impact, maintainer roles. | — | — | — | #3 |

## Rust implementation

The crate `adp` at `impl/rust/` has its own version line (ADP-A-008 C2.1).
Details per release: [impl/rust/CHANGELOG.md](impl/rust/CHANGELOG.md).

| Crate | Date | Implements | Change | PR |
|-------|------|------------|--------|----|
| 0.1.0 | 2026-11-26 | ADP-1 1.1.1 | First release: `v1::validate_dispatch` and `v1::validate_report`, every V and R rule, all 68 ADP-1 vectors with the exact rule. | #29 |

---

## Repository history

Every change merged to `main`, from the first commit. Dates are commit
dates on `main`.

| Date | PR | Kind | Change | Versions |
|------|----|------|--------|----------|
| 2026-11-27 | #39 | CI | The `adp` checks also run on pull requests into `stable`, so a release pull request gets the checks `stable-integrity` requires. | — |
| 2026-11-26 | #37 | Tooling | `scripts/hooks/commit-msg` matches the shared check's version (eng-principles `b7ff0ab`): a tool attribution line is refused, and config may come from `ENG_HOOKS_CONFIG`. | — |
| 2026-11-26 | #38 | CI | eng-principles' shared commit check adopted (`commit-check.yml`, pinned to `b7ff0ab`). It also checks the pull request title and description. The `commits` job skips Dependabot pull requests. Replaces #36. | — |
| 2026-11-26 | #35 | Proposal | ADP-A-010 drafted: contributors merge a pull request that only records a ruling the maintainer made on GitHub. Not yet decided. | — |
| 2026-11-26 | #34 | CI | The `adp` checks also run on pushes to `main` and `stable`. Replaces #31. | — |
| 2026-11-26 | #33 | CI | `check-commits`, `check-schema-gate` and `check_defects.py` refuse a commit range that does not resolve, instead of passing it. 10 self-test cases. | — |
| 2026-11-26 | #32 | Tooling | Dependabot for GitHub Actions and the Rust crate, weekly. | — |
| 2026-11-26 | #30 | Docs | `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1), pull request template and issue forms. | — |
| 2026-11-26 | #29 | Apply | ADP-A-008 stage 1: Implementation class in ADP-0; Rust crate `adp` at `impl/rust/` passes all 68 ADP-1 vectors with the exact rule; CI job `implementation`; README points to the crate. | ADP-0 1.3.0, crate 0.1.0 |
| 2026-11-22 | #24 | Proposal | ADP-A-008 drafted and ACCEPTED: Rust reference implementation at `impl/rust/`, new Implementation class in ADP-0 (1.3.0 when applied). Reconciled with ADP-0 1.2.0 before merge. | — |
| 2026-11-22 | #26 | Apply | ADP-A-009: contributors merge straightforward changes into `main` (ADP-0 §8.1); releases on `stable` (§8.2). AGENTS.md roles and enforcement updated. | ADP-0 1.2.0 |
| 2026-11-22 | #25 | Proposal | ADP-A-009 drafted and ACCEPTED. | — |
| 2026-11-17 | #23 | Docs | This changelog. Tags `adp-0/v1.0.0` … `adp-2/v2.0.1` and pre-release GitHub Releases created for every version below. | — |
| 2026-11-17 | #22 | Apply | ADP-A-007: ADP-1 text matches its validation contract. 15 vectors (68 ADP-1), new V-13, R-12, R-13 (26 rules), R-13 in `report.schema.json`. CI self-test fixed to not depend on one defect's status. 7 defects resolved, 6 open, none high. | ADP-1 1.1.1, ADP-2 2.0.1 |
| 2026-11-17 | #21 | Proposal | ADP-A-007 drafted and ACCEPTED, with a Scenarios table. | — |
| 2026-11-17 | #20 | Apply | ADP-A-006: decidable ADP-2 authority checks. `schema/grant.schema.json` updated, new `schema/record.schema.json`, CI checks ADP-2 grant and record vectors (33). 5 defects resolved. | ADP-2 2.0.0 |
| 2026-11-16 | #19 | Proposal | ADP-A-006 drafted and ACCEPTED. | — |
| 2026-11-16 | #18 | Defects | ADP-D-018 to ADP-D-025 filed: gaps found in ADP-2 and ADP-1. CI test defect IDs derived from the register. | — |
| 2026-11-16 | #17 | Apply | ADP-A-002: new document ADP-2 Dispatch Authority, `schema/grant.schema.json`, first ADP-2 vectors. | ADP-2 1.0.0 |
| 2026-11-16 | #16 | Apply | ADP-A-005: extensions in `metadata`, version-gated unknown fields, new V-12, R-10, R-11. 3 defects resolved. | ADP-1 1.1.0 |
| 2026-11-16 | #15 | Proposal | ADP-A-005 drafted and ACCEPTED. | — |
| 2026-11-16 | #14 | Apply | ADP-A-004: `adp_version` SemVer grammar, major-version check V-11, R-09. 2 defects resolved. | ADP-1 1.0.2 |
| 2026-11-15 | #13 | Proposal | ADP-A-004 drafted and ACCEPTED. | — |
| 2026-11-15 | #12 | Apply | ADP-A-003: conformance suite format (ADP-0 §5.6), first ADP-1 vectors, `check_conformance.py` in CI. | ADP-0 1.1.0 |
| 2026-11-15 | #11 | Apply | ADP-A-001: BCP 14 key words; §7 rules bind validators. 1 defect resolved. | ADP-1 1.0.1 |
| 2026-11-15 | #10 | Proposal | ADP-A-002 decision recorded: ACCEPTED. | — |
| 2026-11-15 | #9 | Proposal | ADP-A-003 drafted and ACCEPTED. | — |
| 2026-11-15 | #8 | Proposal | ADP-A-002 (authorised automation types) drafted. | — |
| 2026-11-15 | #7 | Proposal | ADP-A-001 drafted and ACCEPTED. | — |
| 2026-11-15 | #6 | CI | CI jobs: commit format, schema validity, schema-change gate (needs an ACCEPTED proposal), defect register rules, and a self-test of these checks. | — |
| 2026-11-15 | #5 | Tooling | `commit-msg` and `pre-push` hooks with ADP anchors. | — |
| 2026-11-15 | #4 | Governance | `AGENTS.md` repository rules and `CLAUDE.md` adapter. | — |
| 2026-11-15 | #3 | Governance | ADP-0 adopted; defect register seeded with ADP-D-001 to ADP-D-017. | ADP-0 1.0.0 |
| 2026-11-15 | #2 | Spec | Full ADP-1 normative text restored; `dispatch.schema.json`, `report.schema.json` and tier 1–3 examples added. | ADP-1 1.0.0 |
| 2026-11-15 | #1 | Legal | Apache 2.0 `LICENSE`. | — |
| 2026-06-26 | — | Spec | First commit (`07bae4c`): ADP-1 specification draft and README. | — |

---

## Adding a release

In every PR, add a row at the top of **Repository history**. In a PR
that also changes a document's version:

1. Add a row at the top of that document's table, and update **Latest**.
2. After the maintainer merges, tag the merge commit `adp-N/vX.Y.Z` and
   publish a GitHub Release (pre-release while the document is a Draft)
   whose notes are the row.

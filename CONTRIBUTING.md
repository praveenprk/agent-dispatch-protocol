# Contributing to ADP

Thank you for helping. This file is the way in. The binding rules are
elsewhere, and this file does not repeat them:

| Rules for | Where |
|-----------|-------|
| How ADP changes: artifact classes, defects, proposals, application order, versions, who merges | [`spec/ADP-0.md`](spec/ADP-0.md) |
| Branches, commits, history, evidence, and what enforces each rule | [`AGENTS.md`](AGENTS.md) |
| Behaviour | [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) |
| Vulnerabilities (report privately) | [`SECURITY.md`](SECURITY.md) |

`AGENTS.md` binds every contributor, human or agent. If this file and
`AGENTS.md` or ADP-0 disagree, they win.

## Ways to contribute

| You have | Do this |
|----------|---------|
| A spec, schema, vector or example that is wrong | File a defect with evidence at a named commit (ADP-0 §4.2, §4.3). Open an issue with the **Defect** form, or a pull request that appends to `DEFECTS.md`. |
| A change to the protocol or the process | Write a proposal, `proposals/ADP-A-NNN-<slug>.md` (ADP-0 §5.1). The maintainer decides (§5.3). |
| An accepted proposal to apply | Apply it as written, in ADP-0 §5.4 order. |
| A bug in the reference implementation | Open an issue or a pull request against `impl/rust/`. A vector always wins over the crate (ADP-0 §3.4). |
| A vulnerability | Do not open an issue. See [`SECURITY.md`](SECURITY.md). |

## First steps

1. Read `spec/ADP-0.md`, `DEFECTS.md` and `spec/ADP-1.md`, in that order.
2. Once per clone: `scripts/hooks/install`.
3. Branch as `type/ANCHOR-short-description` and write commits as
   `type(scope): description — ANCHOR` (`AGENTS.md`).
4. Open a pull request into `main`. CI must pass.

## Local checks

The same scripts CI runs (`.github/workflows/adp.yml`):

```sh
python -m pip install jsonschema==4.26.0 pyyaml==6.0.3
sh scripts/hooks/test
python scripts/ci/check_schema.py
python scripts/ci/check_conformance.py
cd impl/rust && cargo test --locked
```

## Who merges

A contributor may merge into `main` when ADP-0 §8.1 allows it. Anything
security-related (label `security`), any recorded decision, and every
release are the maintainer's.

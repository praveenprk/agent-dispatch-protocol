# ADP-A-014: Neutral names in examples

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-014                              |
| **Target**  | ADP-1 (`spec/ADP-1.md` §3.5 example); informative `README.md` and `examples/*.yaml` |
| **Resolves**| No defect                              |
| **Bump**    | ADP-1 Patch                            |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-28                             |

---

## Outcome and impact

**In one line:** the spec, the README and the three examples stop
naming a private project and describing its design, and use a neutral
example subject instead.

| Area | Effect when applied |
|------|---------------------|
| `spec/ADP-1.md` §3.5 | The extension example uses `x-example-seam` and a neutral `source` |
| `README.md` Quick Start | Neutral anchor, path and description |
| `examples/*.yaml` | Neutral identifiers, paths and descriptions. Same keys, same structure, same line count |
| Validators, schemas, vectors | No change. No vector uses these names |
| Open defects ADP-D-014, -015, -016 | Still describe the examples: structure and line numbers are kept. Their Evidence is pinned to `49bf19f`, so it stays reproducible |

---

## Scenarios

| # | Reader | Today | After |
|---|--------|-------|-------|
| S1 | A new contributor reads §3.5 | Sees `x-contextos-seam`, a namespace of a project they cannot see | Sees `x-example-seam`. The namespace is visibly an example |
| S2 | Someone reads `examples/tier1-design.yaml` | Reads the private project's spec numbers, design document names, kernel axioms and seven design decisions | Reads a neutral refund-flow example with the same structure |
| S3 | An implementer copies the Quick Start | Copies `SPEC-027` and `docs/specs/SPEC-027.md` | Copies `SPEC-001` and `docs/specs/SPEC-001.md` |
| S4 | A reviewer checks ADP-D-015 | The defect names `contextos/approve.py` at line 25 | The defect text is unchanged (append-only). The evidence command runs at `49bf19f`, where that line still exists |

---

## Anchor

The maintainer asked, through engineering-fa on 2026-11-28, for a
cleanup across repositories: no unnecessary files, content or mentions.
ContextOS is a private project. Spec text changes go through a proposal.

What the repository says today (`git grep -i contextos` and the
project-specific identifiers, at `450b2e5`):

| Where | What | Class |
|-------|------|-------|
| `spec/ADP-1.md` line 133, 135 | `source: "SPEC-027 §5"`, `x-contextos-seam: 4` | Normative document, example block |
| `README.md` lines 20, 24, 29 | `SPEC-027`, `docs/specs/SPEC-027.md`, "choose append-only superseding edge" | Informative |
| `examples/tier1-design.yaml` | `contextos/*.py`, `x-contextos-seam` ×7, SPEC-027, RFC-027, SPEC-013/020/026, KERNEL_SPEC axioms, four design document names, seven design decisions | Informative |
| `examples/tier2-test.yaml` | `contextos/*.py`, SPEC-027, INV-027-01 to -18, `test_spec_027_destructive_ops.py` | Informative |
| `examples/tier3-impl.yaml` | `contextos/approve.py`, SPEC-027, V-027-02, INV-027-16, VIOLATES and OVERRIDES edges | Informative |
| `DEFECTS.md` ADP-D-002, ADP-D-015 | Quote the old text in Does and Evidence | Register, append-only (ADP-0 §4.2) |
| `proposals/ADP-A-007` | Quotes the old §3.5 example | Decided proposal |

**Precedent.** RFC 2606 reserves `example` names so that documentation
never points at a real party. `x-example-seam` follows it within the
§3.5 key form.

---

## Change

### C1: `spec/ADP-1.md` §3.5

Before:

```yaml
work_items:
  - keyword: "BIND"
    description: "choose superseding structure"
    source: "SPEC-027 §5"
    metadata:
      x-contextos-seam: 4
```

After:

```yaml
work_items:
  - keyword: "BIND"
    description: "choose the refund record structure"
    source: "SPEC-001 §5"
    metadata:
      x-example-seam: 4
```

### C2: `README.md` Quick Start

`SPEC-027` → `SPEC-001` (twice, anchor and path).
`"choose append-only superseding edge"` → `"choose the refund record structure"`.

### C3: `examples/*.yaml`

The examples become a refund flow in an orders service. Each file keeps
its keys, structure and line count. The open defects ADP-D-014 to
ADP-D-016 therefore still apply line for line.

| Today | After |
|-------|-------|
| `x-contextos-seam` | `x-example-seam` |
| `contextos/*.py` | `src/*.py` |
| `contextos/approve.py` | `src/refund.py` |
| `SPEC-027`, `RFC-027`, `V-027-02` | `SPEC-001`, `RFC-001`, `V-001-02` |
| `INV-027-NN` | `INV-001-NN` |
| `SPEC-013`, `SPEC-020`, `SPEC-026` | `SPEC-002`, `SPEC-003`, `SPEC-004` |
| `rfc/RFC-027-destructive-operation-governance` | `rfc/RFC-001-refund-flow` |
| `docs/design/destructive-op-governance-contract.md` | `docs/design/refund-flow-contract.md` |
| `graph-abstraction-layer.md`, `ledger-store-architecture.md`, `agentnode-contract.md` | `storage-layer.md`, `payments-gateway.md`, `orders-api.md` |
| `docs/KERNEL_SPEC.md`, sections `Axiom I/II/III`, `G2`, `G6` | `docs/ARCHITECTURE.md`, sections `Principle 1/2/3`, `G2`, `G6` |
| `tests/spec/test_spec_027*.py`, `test_inv_027_16` | `tests/spec/test_spec_001*.py`, `test_inv_001_16` |
| Work item descriptions and exit conditions that state the private design (destructive operations, Claim-Rule, VIOLATES/OVERRIDES edges, key rotation, ledger markers) | One neutral sentence each about the refund flow, with the same keyword and the same role in the example (decision, test, change) |

With Q1 = (A), only the first three rows apply.

### C4: header and Revision Log

ADP-1 `Version` +1 Patch; `Date` → the application date. Revision Log
row: `1.x.y. §3.5 example uses neutral names. Editorial. ADP-A-014.`

---

## Version impact

| Document | Bump | Reason |
|----------|------|--------|
| ADP-1 | Patch | Text of an example, no schema change (ADP-1 §9) |
| ADP-0, ADP-2 | none | |

## Conformance impact

None. No vector contains any of these names (`git grep` in
`conformance/`: no match). The `schema` job validates the examples, and
they stay valid: the keys and types do not change.

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | Edit the examples directly, without a proposal | The §3.5 example is in a normative document and must change through a proposal. One proposal keeps the spec, README and examples consistent |
| A2 | Also edit `DEFECTS.md` | ADP-0 §4.2: entries are append-only, and only Status, Resolved by and Notes may change. `check_defects` refuses an edit to Does or Evidence |
| A3 | Also edit ADP-A-007 | A decided proposal records what was decided, quoting the text as it was. Editing it changes the record |
| A4 | Rewrite history to remove the name | The repository is public, and the name is in commits and in all 13 tags (every release tag from `adp-0/v1.0.0` to `adp-v0.1.0`, and `evidence/a9e1a4b`). A rewrite breaks every hash cited as evidence (ADP-0 §4.3, AGENTS.md "History") and invalidates the tags |
| A5 | Remove the examples | `check_schema.py` validates them, ADP-0 §9 lists `examples/`, and three open defects describe them |

---

## Questions for the maintainer

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | Scope: (A) the `contextos` name only, or (B) every project-specific identifier and description (C3, full table)? | **(B).** The examples name the private project's specs, kernel axioms, design documents and seven design decisions. Removing only the name leaves the design readable | | |
| Q2 | Neutral subject? | **A refund flow in an orders service, `SPEC-001`, `x-example-seam`.** Familiar to any reader, and `example` follows RFC 2606 | | |
| Q3 | `DEFECTS.md` and decided proposals (ADP-A-007): leave as they are? | **Leave them (A2, A3).** Both are records. The register's own rules forbid the edit. The quoted text is evidence of what the spec said at a pinned commit | | |
| Q4 | Git history, tags and Releases: leave? | **Leave them (A4).** This proposal changes current text only. A rewrite would break the evidence chain and the signed tags | | |

---

## Application (if ACCEPTED), per ADP-0 §5.4

1. **SPEC:** C1 and C4,
   `spec(adp-1): neutral names in the §3.5 example — ADP-A-014`.
2. **CONFORMANCE, SCHEMA, IMPLEMENTATION:** none.
3. **INFORMATIVE:** C2 and C3, `docs(examples): neutral example names —
   ADP-A-014`; the CHANGELOG row for the ADP-1 Patch version.
4. **REGISTER:** none.

---

## Decision


# ADP Defect Register

Governed by ADP-0 §4. Append-only. Entries are never deleted; only
`Status`, `Resolved by`, and `Notes` change after filing.

---

## Evidence harness

Entries marked `harness:` use this script. Save it OUTSIDE the
repository (e.g. `/tmp/adp_ev.py`) and run from the repo root inside
a throwaway virtualenv with `jsonschema` and `pyyaml`:

```
python3 /tmp/adp_ev.py <check-name>
```

```python
import sys, json, copy, yaml
from jsonschema import Draft202012Validator as V

D = V(json.load(open("schema/dispatch.schema.json")))
R = V(json.load(open("schema/report.schema.json")))
B = yaml.safe_load(open("examples/tier2-test.yaml"))

def disp(mut):
    e = copy.deepcopy(B); mut(e); return D.is_valid(e)

REPORT_COMPLETE_FAILED = {
    "adp_version": "1.0.0",
    "report": {"agent": "a", "anchor": ["X"], "status": "COMPLETE"},
    "exit_results": [{"condition": "c", "result": False}],
    "items": [{"key": "k", "value": "v"}],
}

def spec_keyword_rows():
    lines = open("spec/ADP-1.md").read().splitlines()
    start = next(i for i, l in enumerate(lines) if l.startswith("## 3. Keyword"))
    end = next(i for i, l in enumerate(lines) if l.startswith("### 3.5"))
    bt = chr(96)
    return [l.split(bt)[1] for l in lines[start:end] if l.startswith("| " + bt)]

CHECKS = {
    "keyword-enum-size": lambda: len(json.load(open(
        "schema/dispatch.schema.json"))["$defs"]["WorkItem"]
        ["properties"]["keyword"]["enum"]),
    "minor-unknown-field": lambda: disp(lambda e: e.update(
        adp_version="1.1.0", new_optional="x")),
    "major-2": lambda: disp(lambda e: e.update(adp_version="2.0.0")),
    "envelope-x-field": lambda: disp(lambda e: e.update(
        {"x-acme-trace": "1"})),
    "dispatch-metadata": lambda: disp(lambda e: e["dispatch"].update(
        metadata={"x-acme-trace": "1"})),
    "x-uppercase-ns": lambda: disp(lambda e: e["work_items"][0].update(
        metadata={"x-Acme-thing": 1})),
    "x-lowercase-ns": lambda: disp(lambda e: e["work_items"][0].update(
        metadata={"x-acme-thing": 1})),
    "semver-prerelease": lambda: disp(lambda e: e.update(
        adp_version="1.0.0-rc.1")),
    "semver-leading-zero": lambda: disp(lambda e: e.update(
        adp_version="01.0.0")),
    "complete-failed-exit": lambda: R.is_valid(REPORT_COMPLETE_FAILED),
    "spec-keyword-rows": lambda: spec_keyword_rows(),
}

print(CHECKS[sys.argv[1]]())
```

---

### ADP-D-001
Artifact    : Normative — spec/ADP-1.md §3; schema/dispatch.schema.json
Says        : §3 line 64: "10 keywords. Fixed at v1.0.0."
Does        : §3.1–§3.4 tables define 11 keywords (SOURCE in §3.4,
              line 103). Schema `WorkItem.keyword` enum lists 10,
              without SOURCE.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n "^10 keywords" spec/ADP-1.md` → line 64
              harness: `spec-keyword-rows` → 11 keywords, incl. SOURCE
              harness: `keyword-enum-size` → 10
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-007, 5c59a8c
Notes       : Internal contradiction (ADP-0 §3.2 rule 2). No
              artifact takes precedence. Shares root with ADP-D-002.

### ADP-D-002
Artifact    : Normative — spec/ADP-1.md §3.4, §3.5, §4.3
Says        : §3.4 defines `SOURCE` as a keyword ("Origin section in
              governing document that motivates a specific work
              item").
Does        : The same concept is also defined as the optional
              `WorkItem.source` field (§4.3, line 186), and the §3.5
              extension example carries it a third time as
              `x-contextos-source` (line 120).
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -nE 'SOURCE. +\||^source: string|x-contextos-source' spec/ADP-1.md`
              → lines 103, 120, 186
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-007, 5c59a8c
Notes       : Internal contradiction: one concept, three
              representations with no stated relationship.

### ADP-D-003
Artifact    : Normative — spec/ADP-1.md §7.1 V-09, §7.3;
              schema/dispatch.schema.json; schema/report.schema.json
Says        : §7.3 line 339: "Parsers MUST accept unknown minor
              versions (ignore unknown optional fields)."
Does        : §7.1 V-09 (line 320) requires "No unknown top-level
              fields outside schema". Both schemas set
              `additionalProperties: false` at every object level. An
              envelope declaring a newer minor version with a new
              optional field is rejected.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -nE '^\| V-09|accept unknown minor' spec/ADP-1.md` → lines 320, 339
              harness: `minor-unknown-field` → False
Severity    : CRITICAL
Status      : RESOLVED
Resolved by : ADP-A-005, 6e7be3b
Notes       : Conformant envelope rejected. Also an internal
              contradiction between V-09 and §7.3. Shares root
              (`additionalProperties: false`) with ADP-D-006.

### ADP-D-004
Artifact    : Normative — spec/ADP-1.md §7.1, §7.3;
              schema/dispatch.schema.json; schema/report.schema.json
Says        : §7.3: "Parsers MUST reject unknown major versions."
Does        : No V- or R- rule checks the major version. Schemas
              check only the `^[0-9]+\.[0-9]+\.[0-9]+$` pattern. An
              envelope with `adp_version: "2.0.0"` validates.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n "reject unknown major" spec/ADP-1.md`
              harness: `major-2` → True
Severity    : CRITICAL
Status      : RESOLVED
Resolved by : ADP-A-004, 37e0f8b
Notes       : Non-conformant envelope accepted.
              Vector `conformance/adp-1/7.3/fail-unknown-major.json`
              carries this marker (ADP-A-003, d15282b).
              That vector moved to `conformance/adp-1/V-11/fail-major-2.json`
              with the marker removed (ADP-A-004, 37e0f8b).

### ADP-D-005
Artifact    : Legal — LICENSE (absent); README.md; spec/ADP-1.md header
Says        : README.md "License: Apache 2.0"; ADP-1 header
              `License | Apache 2.0`.
Does        : No LICENSE file in the repository.
Evidence    : At `a9e1a4b`: `ls LICENSE` → No such file or directory
Severity    : HIGH
Status      : RESOLVED
Resolved by : Pre-ADP-0 — LICENSE added via GitHub license template,
              commit 6042e278cbb537382ceaf867d7303fe70abcf570. No proposal: ADP-0 did not
              exist at resolution time.
Notes       : The severity table (ADP-0 §4.4) has no Legal row.
              Graded HIGH because the declared license had no legal
              effect without its text. The missing Legal row is a
              gap in ADP-0, to be fixed by proposal.

### ADP-D-006
Artifact    : Normative — spec/ADP-1.md §3.5, §4.2, §5.1;
              schema/dispatch.schema.json; schema/report.schema.json
Says        : §3.5 line 111–112: "Extensions are carried as
              namespaced metadata on work items, envelope, or
              report." "Validators MUST ignore unknown `x-` metadata
              fields."
Does        : Schemas permit `metadata` only on `WorkItem`.
              Envelope-level and report-level `x-` fields are
              rejected. The spec defines no `metadata` field on the
              envelope or report (§4.2, §5.1), so where such
              extensions go is unspecified.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n "on work items," spec/ADP-1.md`
              harness: `envelope-x-field` → False
              harness: `dispatch-metadata` → False
Severity    : CRITICAL
Status      : RESOLVED
Resolved by : ADP-A-005, 6e7be3b
Notes       : Conformant envelope rejected. Also underspecified:
              the spec permits envelope/report extensions without
              defining a location. Shares root with ADP-D-003.

### ADP-D-007
Artifact    : Normative — spec/ADP-1.md §3.5;
              schema/dispatch.schema.json
Says        : §3.5 line 112: namespace format `x-{namespace}-{name}`.
              No character set is defined.
Does        : Schema restricts keys to `^x-[a-z]+-[a-z0-9-]+$` and
              rejects every other key (`additionalProperties: false`
              on `metadata`). `x-Acme-thing` is rejected; spec says
              unknown `x-` fields MUST be ignored.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n 'x-{namespace}' spec/ADP-1.md` → line 112
              harness: `x-uppercase-ns` → False
              harness: `x-lowercase-ns` → True
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-005, 6e7be3b
Notes       : Departs from the CRITICAL row: whether `x-Acme-thing`
              is a conformant key is undefined, because the spec sets
              no character set. The defect is the spec gap plus a
              schema rule with no normative basis.

### ADP-D-008
Artifact    : Normative — spec/ADP-1.md §5.3, §7.2;
              schema/report.schema.json
Says        : §5.3 line 237: `COMPLETE` = "All EXIT conditions
              passed. Work done."
Does        : No R- rule checks it. The schema accepts a COMPLETE
              report whose `exit_results` contains `result: false`.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n 'COMPLETE' spec/ADP-1.md` → lines 210, 237, 330
              harness: `complete-failed-exit` → True
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-007, 5c59a8c
Notes       : Departs from the CRITICAL row: §5.3 states the
              semantics declaratively, without an RFC 2119 keyword,
              and the binding strength of the validation tables is
              itself undefined (ADP-D-012).
              ADP-1 1.0.1 (ADP-A-001 C2, 7a55eb5): the §7 tables now
              bind validators. Grade stays as filed (ADP-0 §4.5).

### ADP-D-009
Artifact    : Normative — spec/ADP-1.md §7.1 V-01, §7.2 R-01;
              schema/dispatch.schema.json; schema/report.schema.json
Says        : V-01 line 312: "`adp_version` present and parseable as
              semver".
Does        : Schema pattern `^[0-9]+\.[0-9]+\.[0-9]+$` rejects
              valid semver with pre-release (`1.0.0-rc.1`) and
              accepts invalid semver with leading zeros (`01.0.0`).
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n 'V-01' spec/ADP-1.md` → line 312
              harness: `semver-prerelease` → False
              harness: `semver-leading-zero` → True
Severity    : CRITICAL
Status      : RESOLVED
Resolved by : ADP-A-004, 37e0f8b
Notes       : Conformant rejected and non-conformant accepted.
              The CRITICAL grade assumes the V-/R- tables are binding;
              see ADP-D-012. Whether pre-release versions are
              meaningful for ADP is a separate question for the
              resolving proposal.
              ADP-1 1.0.1 (ADP-A-001 C2, 7a55eb5): the §7 tables now
              bind validators. Grade stays as filed (ADP-0 §4.5).
              Vector `conformance/adp-1/V-01/fail-leading-zero.json`
              carries this marker (ADP-A-003, d15282b).
              Same rule, found while drafting ADP-A-004: the pattern
              accepted "1.0.0\n" in Python but not in ECMAScript.
              Fixed by the same change (ADP-A-004 Q4).

### ADP-D-010
Artifact    : Normative — spec/ADP-1.md §6 "Tier Selection Guidance"
Says        : Line 295: "Work items contain BIND → 1".
Does        : Line 297: "Work items mix BIND + CONFIRM → 2". Any
              envelope with BIND + CONFIRM matches both rows with
              different tiers.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -nE '^\| Work items (contain BIND|mix BIND)' spec/ADP-1.md` → lines 295, 297
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-007, 5c59a8c
Notes       : Internal contradiction. The section is normative by
              default (ADP-0 §3.1): "Guidance" in its title does not
              make it informative.

### ADP-D-011
Artifact    : Normative — spec/ADP-1.md §6
Says        : Tier headings assign compression ranges: T1 40–50%,
              T2 55–65%, T3 65–75%.
Does        : No baseline (compression relative to what), unit
              (tokens, bytes, words), or measurement method is
              defined. The ranges cannot be checked, so no envelope
              or orchestrator can be shown conformant or
              non-conformant to them.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -nE '^### Tier [123]' spec/ADP-1.md`
              `grep -niE 'baseline|measur' spec/ADP-1.md` → no match
Severity    : MEDIUM
Status      : OPEN
Resolved by : —
Notes       : No table row fits: the text is not a MUST or SHOULD,
              but it is normative by default and cannot be verified.
              The resolving proposal needs measured data. The factory
              API cost experiment (per-station input tokens) is a
              candidate source.

### ADP-D-012
Artifact    : Normative — spec/ADP-1.md (whole document)
Says        : ADP-1 uses MUST, MUST NOT, SHOULD throughout (§3.5,
              §4.4, §5.3, §5.4, §7.3, §8).
Does        : ADP-1 never declares that these key words are to be
              interpreted per RFC 2119. RFC 2119 appears only in the
              §10 comparison table. The V-/R- validation tables
              carry no key word at all, so their binding strength is
              undefined.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n "RFC 2119" spec/ADP-1.md` → lines 384, 387 only (§10 table)
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-001, 7a55eb5
Notes       : No table row fits. Graded HIGH because the grade of
              every other normative entry depends on it (ADP-D-008,
              ADP-D-009).

### ADP-D-013
Artifact    : Informative — examples/tier3-impl.yaml
Says        : ADP-1 §6: "Work items contain BIND → 1".
Does        : The tier 3 example (line 6 `tier: 3`) carries a BIND
              work item (line 29).
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -nE 'tier: 3|keyword: "BIND"' examples/tier3-impl.yaml` → lines 6, 29
Severity    : LOW
Status      : RESOLVED
Resolved by : ADP-A-007, 2d5f8b3
Notes       : Depends on ADP-D-010: which tier rule governs is
              itself contradictory.

### ADP-D-014
Artifact    : Informative — examples/tier2-test.yaml
Says        : ADP-1 §3.2: IGNORE sources are "treated as
              contamination".
Does        : `ignore: tests/spec/test_spec_0*.py` matches the
              example's own `produce` path
              `tests/spec/test_spec_027_destructive_ops.py`.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -nE 'test_spec_0\*|test_spec_027_destructive_ops' examples/tier2-test.yaml`
              `python3 -c "import fnmatch;print(fnmatch.fnmatch('tests/spec/test_spec_027_destructive_ops.py','tests/spec/test_spec_0*.py'))"` → True
Severity    : LOW
Status      : OPEN
Resolved by : —
Notes       : ADP-1 does not define the glob semantics of IGNORE
              patterns; `fnmatch` is used here as the evidence
              interpretation.

### ADP-D-015
Artifact    : Informative — examples/tier3-impl.yaml
Says        : ADP-1 §3.2 (IGNORE = isolation boundary); ADP-1 §4.2
              (`produce` lists output artifacts).
Does        : The example IGNOREs `docs/VIOLATIONS.md` (line 22), but
              its EXIT (line 36) requires that file to be updated.
              The file is not listed under `produce` (only
              `contextos/approve.py`, line 25).
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -nE 'VIOLATIONS.md|^produce:|path: "contextos' examples/tier3-impl.yaml` → lines 22, 24, 25, 36
Severity    : LOW
Status      : OPEN
Resolved by : —

### ADP-D-016
Artifact    : Informative — examples/tier2-test.yaml, examples/tier3-impl.yaml
Says        : ADP-1 §5.4: "The next agent's `context.read` includes
              the prior agent's report and/or produced artifacts."
Does        : tier2 produces `tests/spec/test_spec_027_destructive_ops.py`
              (line 21). tier3 READs `tests/spec/test_spec_027.py`
              (line 19) and exits on `test_spec_027::test_inv_027_16`
              (line 33). The example chain does not connect.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n 'test_spec_027' examples/tier2-test.yaml examples/tier3-impl.yaml`
Severity    : LOW
Status      : OPEN
Resolved by : —

### ADP-D-017
Artifact    : Informative — README.md "Project Structure"
Says        : README lists `python/adp/  # Python reference
              implementation` in the repository tree.
Does        : No `python/` directory exists.
Evidence    : At 49bf19f6297e3bbf3d694dc49e7758080e255ee9:
              `grep -n 'python/adp' README.md`
              `ls python` → No such file or directory
Severity    : LOW
Status      : RESOLVED
Resolved by : ADP-A-008, f6137f0
Notes       : README is informative (ADP-0 §3). The tree describes
              intended layout, not actual layout; it also omits
              `proposals/`, `conformance/`, `DEFECTS.md` defined by
              ADP-0 §9.

### ADP-D-018
Artifact    : Normative — spec/ADP-2.md §5 G-9, §6
Says        : §6 line 182–183: "`validate(dispatch, grant_chain)` is
              a pure function … It checks G-1 to G-7 and G-9 against
              the documents it is given."
Does        : G-9 (line 159–160) is a property of a human-signed
              codebase record, which is not one of the validator's
              inputs. The record also has no document format: G-9
              names `classes`, `boundary` and `span` but defines no
              keys for `boundary` and no signature fields.
Evidence    : At 12e644b:
              `grep -n "validate(dispatch, grant_chain)\|record MUST state\|boundary\`: the" spec/ADP-2.md`
              → lines 160, 162, 182
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-006, 1c435c8
Notes       : Internal contradiction in normative text: the
              validator must check a document it is never given.
              Vectors under conformance/adp-2/G-9 use input
              `{record}` with assumed keys (ADP-A-002 application,
              eee0003).

### ADP-D-019
Artifact    : Normative — spec/ADP-2.md §4, §5 G-4, G-7
Says        : G-4 line 144–145: paths "MUST match" `scope.read` /
              `scope.write`. G-7 line 152: "`scope.write` MUST NOT
              overlap `protected`." §4 lines 121–125 call these
              "globs".
Does        : No glob dialect is defined (`*`, `**`, character
              classes, case, leading `/`), and no meaning is given
              for "overlap" between two glob sets. Two validators can
              give different verdicts on the same grant.
Evidence    : At 12e644b:
              `grep -n "globs\|MUST match \`scope\|MUST NOT overlap" spec/ADP-2.md`
              → lines 87, 90, 121, 122, 125, 144, 152
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-006, 1c435c8
Notes       : Departs from the CRITICAL row: whether a given path
              matches is undefined, so no envelope can be shown to
              be wrongly accepted or rejected. The defect is a MUST
              with no defined outcome (same reasoning as ADP-D-007).
              ADP-2 vectors use only paths whose verdict holds in any
              common dialect.

### ADP-D-020
Artifact    : Normative — spec/ADP-2.md §4.1, §5 G-1
Says        : G-1 line 138: "A dispatch MUST list exactly one grant
              id in `dispatch.anchor`."
Does        : `dispatch.anchor` also carries other references (ADP-1
              §4.1, e.g. `SPEC-027`). ADP-2 does not say how a
              validator tells a grant id apart from them: by format,
              or by membership in the grant chain it is given.
Evidence    : At 12e644b:
              `grep -n "grant id\|grant \`id\`" spec/ADP-2.md` → lines 130, 138
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-006, 1c435c8
Notes       : Departs from the CRITICAL row for the same reason as
              ADP-D-019: the count G-1 checks is undefined.

### ADP-D-021
Artifact    : Normative — spec/ADP-2.md §3, §5 G-3, §6
Says        : §3 lines 89 and 93: `CONFIRM` and `EXIT` imply
              `AT-VERIFY` "if checking the condition requires running
              anything". G-3 requires the implied types to be a subset
              of the grant's types. §6: the validator is a pure
              function over the documents.
Does        : Whether a condition such as "tests pass" requires
              running anything is not visible in the envelope, so a
              pure validator cannot compute the implied types, and
              cannot decide G-3 for any dispatch with `exit`.
Evidence    : At 12e644b:
              `grep -n "requires running anything" spec/ADP-2.md` → lines 89, 93
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-006, 1c435c8
Notes       : Internal contradiction in normative text: G-3 must be
              checked by a pure function, and its input depends on
              facts outside the documents.

### ADP-D-022
Artifact    : Normative — spec/ADP-2.md §4; schema/grant.schema.json
Says        : §4 lines 106–125 list the grant fields. Only `root`
              carries a condition ("required unless parent is
              present", line 111).
Does        : ADP-2 does not say which other fields are required,
              what grammar `adp_grant_version` follows, or whether
              unknown fields are allowed. schema/grant.schema.json
              makes every unconditioned grant-level field required
              and allows unknown fields. Neither choice has a
              normative basis.
Evidence    : At 12e644b:
              `grep -n "adp_grant_version\|required unless parent" spec/ADP-2.md` → lines 106, 111
              `grep -n '"required"' schema/grant.schema.json`
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-006, 1c435c8
Notes       : Departs from the CRITICAL row: conformance of a grant
              with a missing field is undefined, so no verdict can be
              shown wrong. Same shape as ADP-D-007 (schema rule
              without normative basis).

### ADP-D-023
Artifact    : Process — spec/ADP-0.md §9
Says        : §9 is the repository layout. Line 373:
              "conformance/  # created by first validation proposal
              (§5.5)".
Does        : The tree lists `spec/ADP-1.md` only (line 371), not
              `spec/ADP-2.md`, and still describes `conformance/` as
              to be created, though ADP-A-003 created it.
Evidence    : At 12e644b:
              `grep -n "ADP-1.md \|created by first" spec/ADP-0.md` → lines 371, 373
              `ls spec conformance`
Severity    : LOW
Status      : RESOLVED
Resolved by : ADP-A-008, 8dcee38
Notes       : ADP-0 §4.4 has no row for process documents. Graded as
              the "Informative artifact contradicts normative text"
              row, because the layout describes files and binds no
              envelope.

### ADP-D-024
Artifact    : Normative — spec/ADP-1.md §7.1, §7.2;
              schema/dispatch.schema.json; schema/report.schema.json
Says        : §7: "A validator MUST reject a dispatch envelope that
              fails any rule in §7.1, and MUST reject a report
              envelope that fails any rule in §7.2." The 23 rules
              are the validation contract.
Does        : The schemas also reject envelopes that fail no rule:
              a produce or read entry without `path`, a work item
              without `description`, an exit result without
              `condition` or `result`, a report item without `key` or
              `value`. These follow the §4.3/§5.2 field definitions,
              which never say which fields are required. A validator
              that implements only §7 accepts them.
Evidence    : At 12e644b:
              `grep -n '"required": \["path"\]\|"required": \["keyword", "description"\]\|"required": \["condition", "result"\]\|"required": \["key", "value"\]' schema/*.json`
              → dispatch lines 99, 115, 131; report lines 96, 111
              `grep -cE '^\| [VR]-[0-9]{2} ' spec/ADP-1.md` → 23, none
              naming these fields.
              The minimal valid dispatch vector with
              `produce: [{"action": "create"}]` → schema verdict False.
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-007, 5c59a8c
Notes       : Departs from the CRITICAL row: ADP-1 does not say
              whether such envelopes are conformant, so neither
              verdict can be shown wrong. ADP-A-001 A5 and ADP-A-005
              ("Not in this proposal") deferred this.

### ADP-D-025
Artifact    : Normative — spec/ADP-1.md §8
Says        : Line 382: "A dispatch envelope MUST NOT carry content
              that is structurally available to the agent through its
              READ context."
Does        : Line 392: "Deduplication is a SHOULD for v1.0.0 (not
              mechanically enforced by the validator …)". The same
              requirement is stated as MUST and as SHOULD.
Evidence    : At 12e644b:
              `grep -n "MUST NOT carry\|Deduplication is a SHOULD" spec/ADP-1.md` → lines 382, 392
Severity    : HIGH
Status      : RESOLVED
Resolved by : ADP-A-007, 5c59a8c
Notes       : Internal contradiction in normative text.

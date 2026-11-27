# ADP-A-012: A deadline for each dispatch

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-012                              |
| **Target**  | ADP-1 (`spec/ADP-1.md` §4.2, §4.5, §5.3), ADP-2 (`spec/ADP-2.md` §5, §6) |
| **Resolves**| No defect. Candidate C6 of ADP-A-011 (see Anchor) |
| **Bump**    | ADP-1 Minor: 1.1.1 → 1.2.0. ADP-2 Minor: 2.0.1 → 2.1.0 |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-28                             |

---

## Outcome and impact

**In one line:** an orchestrator can say when a dispatch must be back,
the deadline can never outlast the grant, and a dispatch stopped at its
deadline still produces a report.

| Area | Effect when applied |
|------|---------------------|
| Dispatch envelope | New optional field `dispatch.deadline`, an RFC 3339 date-time in UTC (ADP-1 §4.2). Checked by V-13 (§4.5). |
| Report envelope | No new field. Work stopped at its deadline reports `BLOCKED` with a `blocker` item whose value is `deadline` (§5.3). |
| Authority | New invariant G-11: a deadline is never later than the listed grant's `expires_at` (ADP-2 §5). |
| Runtime | Stops the work at the deadline, and writes the `BLOCKED` report if the agent has not (ADP-2 §6). |
| Existing envelopes | Unchanged. A dispatch without `deadline` is valid exactly as before. |

---

## Scenarios

The bank example from ADP-A-008. The CHG-4821 grant expires at
`2026-11-27T18:00:00Z`.

| # | Dispatch | Today | After |
|---|----------|-------|-------|
| S1 | No `deadline` | Valid. The work may run until the grant expires (G-8) | Unchanged |
| S2 | `deadline: 2026-11-27T12:00:00Z` | Refused: unknown field (V-09) | Valid. The runtime stops the work at noon |
| S3 | `deadline: 2026-11-27T20:00:00Z`, after the grant expires | Refused: unknown field (V-09) | Refused: G-11. The grant would end first, so the deadline would claim time the agent has no authority for |
| S4 | `deadline: 2026-11-27T12:00:00+05:30` | Refused: unknown field (V-09) | Refused: V-13. Not UTC, the same rule as `expires_at` (G-10) |
| S5 | The fees agent loops on a flaky test past noon and never reports | Silence until the grant expires | The runtime stops it and writes `status: BLOCKED`, `blocker: deadline`. The change record shows why the work stopped |
| S6 | The agent finishes at 11:40 | `COMPLETE` | Unchanged: `COMPLETE` |

---

## Anchor

**The decision to draft.** ADP-A-011 assessed ADP against OSTEP
Part 1 scheduling. Candidate C6 (OSTEP ch. 6, limited direct
execution) found that ADP bounds authority by time (ADP-2 G-8) but has
no limit for each dispatch and no outcome when a limit fires. On
2026-11-28 the maintainer answered ADP-A-011 Q1: draft C6 and the C7
note. The answer is recorded in ADP-A-011's questions table.

**The gap.** OSTEP ch. 6 explains why an operating system cannot wait
for a process to give the CPU back: a process in an endless loop never
does. A timer interrupt returns control to the OS. ADP today is
cooperative in the same sense:

| What ADP has | Where | Why it is not enough |
|--------------|-------|----------------------|
| Grant expiry | ADP-2 G-8 | It covers the whole session. A session may serve many dispatches, and the orchestrator cannot bound one of them |
| `BLOCKED` with a `blocker` item | ADP-1 §5.3, R-07 | A report carries results from the agent (§5). No text says who writes one when the agent is stopped, and a stopped agent may never write one |
| The runtime enforces G-8 | ADP-2 §6 | Nothing tells it to stop a single dispatch |

**Precedents.**

| System | Mechanism | What this proposal takes from it |
|--------|-----------|----------------------------------|
| gRPC | An absolute deadline. A server calling further services passes on the same deadline or an earlier one | G-11: a deadline only narrows, like G-5 for delegated grants |
| GitHub Actions | `timeout-minutes` on a job: the runner stops the job and records it as cancelled | The runtime stops the work and records the outcome (ADP-2 §6) |
| Kubernetes Job | `activeDeadlineSeconds`: the Job fails with reason `DeadlineExceeded` | The outcome names the deadline as its cause (`blocker: deadline`) |

---

## Change

### C1: `spec/ADP-1.md` §4.2 Optional Fields

Before:

```yaml
dispatch:
  model: string          # model identifier
  tier: integer          # compression tier (1, 2, 3)
  gate: string           # gate identifier
  branch: string         # version control branch
```

After:

```yaml
dispatch:
  model: string          # model identifier
  tier: integer          # compression tier (1, 2, 3)
  gate: string           # gate identifier
  branch: string         # version control branch
  deadline: string       # RFC 3339 date-time in UTC (§5.3)
```

### C2: `spec/ADP-1.md` §4.5 Field Constraints

New row after the `dispatch.tier` row:

```
| `dispatch.deadline`            | RFC 3339 date-time in UTC, ending in `Z`     | V-13 |
```

### C3: `spec/ADP-1.md` §5.3 Status Values

New paragraph after the table:

```
When work stops because the dispatch's `deadline` (§4.2) has passed,
the report status is `BLOCKED` and its `blocker` item has the value
`deadline`. The report MAY be written by the runtime that stopped
the work (ADP-2 §6). Validators do not check this paragraph: a
report validator does not see the dispatch.
```

### C4: `spec/ADP-2.md` §5 Invariants

New invariant after G-10:

```
- **G-11** When a dispatch has a `dispatch.deadline` (ADP-1 §4.2), it
  MUST NOT be later than the `expires_at` of the grant it lists.
  Both are compared as instants.
```

### C5: `spec/ADP-2.md` §6 Validation and enforcement

Validators bullet, before:

```
  - `validate(dispatch, grant_chain)` checks G-1 to G-7 and G-10.
```

After:

```
  - `validate(dispatch, grant_chain)` checks G-1 to G-7, G-10 and G-11.
```

Runtime bullet: after "G-8," add "the dispatch's `deadline`: the runtime
stops the work when it passes and, if the agent has not reported, writes
the `BLOCKED` report on the agent's behalf (ADP-1 §5.3),".

### C6: headers and Revision Logs

- ADP-1: `Version` 1.1.1 → 1.2.0; `Date` → the application date.
  Revision Log row: `1.2.0. Optional dispatch.deadline (§4.2), UTC,
  checked by V-13 (§4.5); work stopped at its deadline reports BLOCKED
  with blocker: deadline (§5.3). ADP-A-012.`
- ADP-2: `Version` 2.0.1 → 2.1.0; `Date` → the application date.
  Revision Log row: `2.1.0. G-11: a dispatch deadline is no later than
  the grant's expires_at; the runtime enforces it (§6). ADP-A-012.`

### C7: `schema/dispatch.schema.json`

`dispatch.properties` gains `deadline` with the same `type`, `format`
and `pattern` as `grant.schema.json`'s `expires_at`. The closed
property list for `dispatch` (V-09) gains `deadline`. G-11 compares
two documents, so no schema checks it, like G-3 to G-7.

---

## Version impact

| Document | Bump | Reason |
|----------|------|--------|
| ADP-1 | Minor, 1.1.1 → 1.2.0 | New optional field (ADP-1 §9). Every envelope valid under 1.1.1 is still valid. A 1.1 validator ignores `deadline` in a 1.2.0 envelope, as §7.3 requires for a higher minor. |
| ADP-2 | Minor, 2.0.1 → 2.1.0 | New invariant. It applies only to a dispatch with `deadline`, which ADP-1 1.1.1 refused (V-09), so no input valid today becomes invalid. |
| ADP-0 | none | |

---

## Conformance impact

New vectors, each a passing vector with one change (ADP-0 §5.6.3):

| Path | Expect | Input |
|------|--------|-------|
| `conformance/adp-1/V-13/pass-deadline-utc.json` | pass | `deadline: 2026-11-27T12:00:00Z` |
| `conformance/adp-1/V-13/fail-deadline-not-utc.json` | fail V-13 | `deadline: 2026-11-27T12:00:00+05:30` |
| `conformance/adp-1/V-13/fail-deadline-not-string.json` | fail V-13 | `deadline: 1764244800` |
| `conformance/adp-2/G-11/pass-deadline-before-expiry.json` | pass | deadline `12:00:00Z`, grant expires `18:00:00Z` |
| `conformance/adp-2/G-11/pass-deadline-equals-expiry.json` | pass | both `18:00:00Z` |
| `conformance/adp-2/G-11/fail-deadline-after-expiry.json` | fail G-11 | deadline `20:00:00Z`, grant expires `18:00:00Z` |
| `conformance/adp-2/G-11/fail-deadline-after-expiry-fraction.json` | fail G-11 | deadline `18:00:00.5Z`, grant expires `18:00:00Z`: compared as instants, not as strings |

No existing vector's verdict changes: no vector uses `deadline` today.

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | A relative duration (`timeout: 3600`) | It cannot be compared with `expires_at` without knowing when the work started, so G-11 would need runtime state. A queueing delay also silently extends it. |
| A2 | Put the deadline in the grant, not the dispatch | A grant covers a session of many dispatches (G-8). The deadline is a property of one piece of work. |
| A3 | A new report status `TIMEOUT` | ADP-1 §5.3 fixes three statuses. A fourth is a breaking change (Major, §9). `BLOCKED` with `blocker: deadline` says the same within 1.x. |
| A4 | Leave the deadline to `metadata` (§3.5) | `metadata` is not checked against the grant. A deadline beyond the grant's expiry would pass, which is the S3 case. |
| A5 | Also limit tokens, tool calls or cost | Units differ by model and vendor, and ADP-1 §2 excludes model selection. Time is the one unit every runtime measures. |
| A6 | A new rule V-14 for the date-time format | In §4.5, a field constraint without a rule of its own is checked by V-13 (`dispatch.model`, `context.ignore`, `metadata`). One more V-13 row keeps "each constraint is checked by exactly one rule" without adding to §7.1. |

---

## Questions for the maintainer

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | Absolute deadline or relative duration? | **Absolute, RFC 3339 UTC (C1).** It compares directly with `expires_at` (G-11), and a queueing delay cannot stretch it (A1). Same format as `expires_at`, so one parser serves both. | | |
| Q2 | Who writes the report when the agent never does? | **The runtime, with `report.agent` unchanged and `blocker: deadline` (C3, C5).** Otherwise a stopped agent produces silence, which is the cooperative failure OSTEP ch. 6 describes. The report is still checked by R-01 to R-13. | | |
| Q3 | Limit other resources too? | **Not now: time only (A5).** A later proposal can add others in a `metadata` namespace if a runtime needs them. | | |
| Q4 | Which versions? | **ADP-1 1.2.0 and ADP-2 2.1.0 (Minor each).** See Version impact: nothing valid today becomes invalid. | | |

---

## Application (if ACCEPTED), per ADP-0 §5.4

1. **SPEC, CONFORMANCE, SCHEMA** in one commit (§5.4: steps 1–3 SHOULD
   land together): C1 to C7 and the vectors above,
   `schema(dispatch): add dispatch.deadline and G-11 — ADP-A-012`.
2. **IMPLEMENTATION:** `impl/rust` gains the `deadline` check under
   V-13, and the crate's claim moves to ADP-1 1.2.0. G-11 is checked
   when the crate implements ADP-2 (ADP-A-008 stage 2).
3. **INFORMATIVE:** an example with `deadline` in `examples/`; the
   CHANGELOG rows for ADP-1 1.2.0 and ADP-2 2.1.0.
4. **REGISTER:** none.

The schema change is in step 1, so the maintainer merges the
application (`schema` CI job, ADP-0 §5.4).

---

## Decision


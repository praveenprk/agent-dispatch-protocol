# ADP-A-013: Timing keys for turnaround and response time (informative)

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-013                              |
| **Target**  | ADP-1 (`spec/ADP-1.md`, new informative subsection at the end of §5) |
| **Resolves**| No defect. Candidate C7 of ADP-A-011 (see Anchor) |
| **Bump**    | ADP-1 Patch (see Version impact)       |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-28                             |

---

## Outcome and impact

**In one line:** ADP-1 names three `metadata` keys for when work was
issued, started and finished, so anyone can compute how long work waited
and how long it took from the envelopes alone.

| Area | Effect when applied |
|------|---------------------|
| Dispatch and report envelopes | No new field. Three suggested `metadata` keys (§3.5) |
| Validators | No change. The keys already have the §3.5 form, so validators ignore them |
| Schema and vectors | No change |
| Existing envelopes | Unchanged. The keys are optional, and nothing requires them |

---

## Scenarios

The bank example from ADP-A-008: change CHG-4821, one dispatch to the
fees agent.

| # | Question asked | Today | After |
|---|----------------|-------|-------|
| S1 | The auditor asks how long CHG-4821 waited before an agent started on it | No answer from the envelopes. Logs from each runtime, in each runtime's format | `x-adp-started-at` minus `x-adp-issued-at` |
| S2 | The auditor asks how long the work took in total | Same | `x-adp-finished-at` minus `x-adp-issued-at` |
| S3 | Two vendors' orchestrators record timing | Each picks its own key names. A cross-vendor report has to map them | Both use the same three keys |
| S4 | An envelope carries none of the keys | Valid | Valid. The keys are suggestions, not requirements |
| S5 | A key holds `2026-11-27 09:00` (not RFC 3339) | Valid: `metadata` values are not constrained (§3.5) | Still valid. The note describes the format but no validator checks it. A consumer that cannot parse the value has no timing for that envelope |

---

## Anchor

**The decision to draft.** ADP-A-011 assessed ADP against OSTEP
Part 1 scheduling. Candidate C7 (OSTEP ch. 7, scheduling basics) found
that neither envelope says when work was issued, started or finished,
so neither of OSTEP's two basic metrics can be computed. On 2026-11-28
the maintainer answered ADP-A-011 Q1: draft C6 and the C7 note. C6 is
ADP-A-012 (#47). The answer is recorded in ADP-A-011's questions table.

**The two metrics** (OSTEP ch. 7), in ADP terms:

| Metric | OSTEP | ADP |
|--------|-------|-----|
| Turnaround time | completion − arrival | report finished − dispatch issued |
| Response time | first run − arrival | agent started − dispatch issued |

**Why informative, not normative.** Timing is measured by the
orchestrator and the runtime. ADP-1 §11 places both outside ADP, and
ADP-1 §7 makes validation a pure function with no clock. `metadata`
(§3.5) already carries data that validators ignore. A note that names
the keys gives interoperability without a field or a rule.

**Precedents.**

| System | Mechanism | What this proposal takes from it |
|--------|-----------|----------------------------------|
| OpenTelemetry semantic conventions | Recommended attribute names, in a shared namespace, for data the specification does not validate | Suggested keys under one namespace. Interoperability by naming, not by validation |
| Kubernetes Job status | `startTime` and `completionTime` next to the object's `creationTimestamp` | Three instants: issued, started, finished |
| ADP-1 §6 | "Tier Selection Guidance (Informative)", an unnumbered informative subsection | The form of the new text, so no section is renumbered |

---

## Change

### C1: `spec/ADP-1.md`, new subsection at the end of §5

Inserted after §5.5 and its closing sentence ("Status-dependent rules
are R-07, R-08 and R-13 (§5.3)."), before the `---` that ends §5:

````
### Timing (Informative)

Two measures describe how a dispatch was served:

| Measure       | Computed as                     |
|---------------|---------------------------------|
| Response time | started at − issued at          |
| Turnaround    | finished at − issued at         |

ADP does not require timing data. Where an orchestrator or runtime
records it, these `metadata` keys (§3.5) give every implementation the
same names:

| Key                 | Envelope | Written by   | Instant                              |
|---------------------|----------|--------------|--------------------------------------|
| `x-adp-issued-at`   | dispatch | orchestrator | the dispatch was issued              |
| `x-adp-started-at`  | report   | runtime      | the agent started work               |
| `x-adp-finished-at` | report   | runtime      | the report was written               |

Each value is an RFC 3339 date-time in UTC, ending in `Z`, the format
of ADP-2's `expires_at`. Validators ignore these keys like any other
`metadata` entry (§3.5). They do not check that the keys are present,
that the values have this format, or that the instants are in order.

A dispatch does not carry an expected length for the work. A length
written before the work runs is an estimate, and a field would present
it as a measurement.
````

### C2: header and Revision Log

`Version` +1 Patch; `Date` → the application date. Revision Log row:
`1.x.y. Timing (Informative) at the end of §5: response time,
turnaround and three suggested metadata keys. ADP-A-013.`

---

## Version impact

| Document | Bump | Reason |
|----------|------|--------|
| ADP-1 | Patch | Spec text with no schema change (ADP-1 §9). 1.1.1 → 1.1.2 if applied first; 1.2.0 → 1.2.1 if ADP-A-012 is applied first |
| ADP-2, ADP-0 | none | |

**Order with ADP-A-012.** This proposal does not depend on ADP-A-012.
It states its own date-time format, which is the format ADP-A-012 would
give `dispatch.deadline`. ADP-A-011 C7-Q3 recommended applying it after
ADP-A-012, so that one date-time format is in place first. Either order
gives the same text.

---

## Conformance impact

None. No vector is added or changed. The keys have the §3.5 form, which
existing vectors already cover (V-12, R-11). A vector cannot test an
informative recommendation (ADP-0 §5.6.3 rule 2: vectors follow
normative text).

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | Normative optional fields (`dispatch.issued_at`, `report.started_at`, `report.finished_at`) | A validator could check only the format; it has no clock (§7). The fields would add a Minor bump and schema work for data the protocol never acts on. A later Minor can promote the keys if a validator ever needs them |
| A2 | Say nothing; leave timing to each runtime | Each vendor picks its own names, which is S3 |
| A3 | A dispatch field for expected length, so an orchestrator can run the shortest job first | OSTEP ch. 7: the length is usually unknown. OSTEP ch. 8 (MLFQ) is the answer to unknown lengths, and it lives in the orchestrator (ADP-1 §11). Recorded in the note's last paragraph |
| A4 | A new numbered section (§12 or a new §6) | A new section between existing ones renumbers §7 to §11, which rules, proposals and vectors cite. §6's informative subsection is the precedent for an unnumbered one |
| A5 | A queue-time key (`x-adp-queued-at`) | Response time already measures the wait. A fourth instant adds nothing an auditor asked for |

---

## Questions for the maintainer

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | Which namespace for the keys? | **`adp` (`x-adp-…`).** It tells readers the names come from the protocol. §3.5 reserves no namespace, so a vendor could already use `x-adp-`. The risk is low, and a reservation would be a normative change for another proposal | | |
| Q2 | Where does the text go? | **Unnumbered "Timing (Informative)" at the end of §5 (C1).** The started and finished keys are on the report. No renumbering (A4) | | |
| Q3 | Should a runtime that stops work at a deadline (ADP-A-012) write `x-adp-finished-at`? | **Not stated here.** ADP-A-012 is not decided. If both are accepted, the application of the later one can add one sentence | | |

---

## Application (if ACCEPTED), per ADP-0 §5.4

1. **SPEC:** C1 and C2 in one commit,
   `spec(adp-1): add the informative timing note — ADP-A-013`.
2. **CONFORMANCE, SCHEMA, IMPLEMENTATION:** none.
3. **INFORMATIVE:** the CHANGELOG row for the new ADP-1 Patch version.
   Optionally, the three keys in an example under `examples/`.
4. **REGISTER:** none.

No schema change. A contributor may merge the application when every
condition of ADP-0 §8.1 holds; otherwise the maintainer merges it.

---

## Decision


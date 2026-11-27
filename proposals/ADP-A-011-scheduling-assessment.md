# ADP-A-011: Assess the dispatch model against OSTEP Part 1 scheduling

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-011                              |
| **Target**  | None directly. Assesses ADP-1 (§2, §4–§6, §11) and ADP-2 (§4, §5, §6) |
| **Resolves**| No defect. Addresses the need below (see Anchor) |
| **Bump**    | None. Each candidate states its own bump if drafted |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-27                             |

---

## Outcome and impact

**In one line:** an orchestrator that hands work to agents is a
scheduler. This proposal checks what ADP already says about five
scheduling problems from OSTEP Part 1 (Arpaci-Dusseau and
Arpaci-Dusseau, *Operating Systems: Three Easy Pieces*, chapters 6–10).
It recommends one change, two informative notes, and two cases where
ADP should not change.

| Chapter | Problem | ADP today | Recommend |
|---------|---------|-----------|-----------|
| 6 Limited direct execution | Regaining control from a runaway agent | Grant expiry only (ADP-2 G-8). No limit per dispatch | **C6:** draft an optional `dispatch.deadline`, bounded by the grant (ADP-1 and ADP-2 Minor) |
| 7 Scheduling basics | Turnaround versus response time | No time data in either envelope | **C7:** informative note on measuring both through `metadata`. No field |
| 8 MLFQ | Unknown task lengths, starvation | No priority. Attempts already link through `context.read` (ADP-1 §5.4) | **C8:** no change. Priority is orchestrator policy (ADP-1 §2) |
| 9 Lottery and stride | Fair share across requesters | One issuer per grant. No quota | **C9:** defer until a runtime exists to enforce it (ADP-2 §6) |
| 10 Multiprocessor | Affinity: send a task back to the agent that holds its context | `dispatch.agent` names the agent. A dispatch must not depend on state outside it (ADP-1 §5.4) | **C10:** no field. Informative note: affinity may make a dispatch faster, never correct |

If ACCEPTED, this proposal changes no artifact. It authorises drafting
the candidates the maintainer picks, each as its own proposal, because
ADP-0 §5.1 does not allow a proposal to bundle unrelated changes.

---

## Scenarios

The bank example from ADP-A-008. The change orchestrator sends several
agents work for CHG-4821 (fix rounding in card fees) and for two other
teams' changes on the same day.

| # | Situation | Today | After the recommended candidates |
|---|-----------|-------|----------------------------------|
| S1 | The fees agent loops on a flaky test for four hours | Runs until the grant expires at end of day (G-8). Nothing in the dispatch says when the orchestrator should take the work back | **C6:** the dispatch says `deadline: 2026-11-27T12:00:00Z`. The runtime stops the work at noon and the outcome is `BLOCKED`, `blocker: deadline` |
| S2 | The auditor asks how long CHG-4821 waited before an agent started, and how long it took | Nobody can tell from the envelopes | **C7:** the agreed `metadata` keys record when the work was issued, started and finished. The two numbers come from the record |
| S3 | A large refactor keeps being re-dispatched, and the fees fix waits behind it | The orchestrator's policy. ADP is silent | Unchanged (**C8**). Policy stays with the orchestrator. No agent can raise its own priority, because the envelope has no priority field to forge |
| S4 | The payments team sends 40 dispatches and the cards team gets none all morning | The orchestrator's policy | Unchanged for now (**C9**, deferred). Once a runtime exists, a grant could cap dispatches, and a child grant could only lower the cap (G-5) |
| S5 | The agent that read `src/fees/**` this morning is busy, and the follow-up goes to a fresh agent | Works: the follow-up lists the morning report in `context.read` (ADP-1 §5.4) | Unchanged (**C10**). The informative note says a warm agent is an optimisation, and the fresh agent must reach the same result |

---

## Anchor

**The request.** On 2026-11-27 the maintainer asked, in the
engineering-fa session, that OSTEP be applied across the projects.
engineering-fa relayed the request to this session the same day, with
the five topics above. That relay is not a decision. The maintainer
decides this proposal first-hand (ADP-0 §5.3).

**Why scheduling applies.** ADP-1 §1 puts ADP at the boundary between
an orchestrator and an agent. An orchestrator that decides which agent
runs which dispatch, when, and for how long is a scheduler. OSTEP Part 1
is the standard account of what goes wrong in schedulers. Each chapter
gives a problem ADP may or may not need to address.

**The boundary this assessment keeps.** ADP-1 §2 excludes the
transport mechanism and agent internal reasoning, and ADP-1 §11 says ADP
is "Not an agent runtime or orchestrator". So a scheduling *policy* is
out of scope. Two things are in scope: what crosses the envelope that
the orchestrator or runtime needs (C6, C7), and what the envelope must
never let an agent do to the schedule (C8, C9). Each candidate is
judged against that boundary.

**Source.** OSTEP is cited by chapter and topic. The text was not
machine-read for this draft, so it quotes no line from the book.

---

## Assessment

### C6: a timer for runaway agents (OSTEP ch. 6)

**OSTEP.** Under limited direct execution, a process runs directly on
the CPU. The OS regains control by a timer interrupt, not by waiting for
the process to give it up. The cooperative approach, where a process
yields when it chooses, fails when a process loops forever.

**What ADP specifies.**

| Mechanism | Where | What it bounds |
|-----------|-------|----------------|
| Grant expiry: a grant ends at `expires_at` or at session end, and is never renewed | ADP-2 G-8 | Authority, for the whole session |
| A delegated grant never expires after its parent | ADP-2 G-5 | Authority, down a delegation chain |
| `BLOCKED` with a `blocker` item | ADP-1 §5.3, R-07 | Reporting that the agent stopped, if the agent reports it itself |
| G-8 is enforced by the runtime | ADP-2 §6 | Who stops the agent |

**What is missing.**

1. No limit per dispatch. A session's grant may cover many dispatches.
   An orchestrator cannot tell the runtime that this one dispatch must
   be back within an hour.
2. No defined outcome when the limit fires. Only the agent writes a
   report (ADP-1 §5.1, `report.agent`). An agent stopped by its runtime
   may never write one, and the orchestrator is left with silence.
   That is the cooperative model.

**Candidate (to be drafted as its own proposal).**

- ADP-1 §4.2: new optional field `dispatch.deadline`, an RFC 3339 UTC
  date-time, using the same format as ADP-2's `expires_at` (§4).
- ADP-1 §5.3: work stopped at its deadline reports `BLOCKED` with
  `blocker: deadline`.
- ADP-2 §5: new invariant G-11. A dispatch's `deadline`, when present,
  MUST NOT be later than the `expires_at` of the grant it lists. This
  applies G-5's narrowing rule to time.
- ADP-2 §6: the runtime enforces the deadline. The validator checks
  only G-11.
- Bump: ADP-1 Minor (new optional field, ADP-1 §9). ADP-2 Minor (new
  invariant, and no dispatch that is valid today becomes invalid).
  New vectors: a dispatch whose deadline is after the grant's expiry is
  refused, one before it passes, and one with a malformed date-time is
  refused.

**Precedents.** gRPC deadlines: a client sets an absolute deadline,
and a server calling further services passes on the same deadline or an
earlier one, never a later one. That is the rule G-11 proposes. GitHub
Actions `timeout-minutes` and Kubernetes Job `activeDeadlineSeconds`
stop a job from outside it and report the stop as its outcome.

| # | Question | Recommendation (setting and reason) |
|---|----------|-------------------------------------|
| C6-Q1 | Absolute deadline or relative duration? | **Absolute, RFC 3339 UTC.** It compares directly with `expires_at` (G-11) and stays correct when queueing delays the start. gRPC sends deadlines in this form for the same reason. |
| C6-Q2 | Who writes the report when the agent never does? | **The runtime writes it on the agent's behalf, with `report.agent` unchanged and the item `blocker: deadline`.** Otherwise a stopped agent produces silence, which is the cooperative failure. Needs a sentence in ADP-1 §5 that a runtime may write this one report. |
| C6-Q3 | Limit other resources too (tokens, tool calls, cost)? | **Not now. Time only.** Time has one unit every runtime can measure. Token and cost units differ by model, and ADP-1 §2 excludes model selection. Add them later in a `metadata` namespace if a runtime needs them. |
| C6-Q4 | Draft C6? | **Yes.** It is the one gap where ADP has the boundary (G-8) but not the mechanism a scheduler needs at dispatch level. |

### C7: turnaround versus response time (OSTEP ch. 7)

**OSTEP.** Turnaround time is when a job completes minus when it
arrived. Response time is when it first runs minus when it arrived.
Shortest-job-first policies give good turnaround but need to know each
job's length. Round robin gives good response time and worse turnaround.
A job that waits on I/O should give up the CPU so that other jobs run
in the meantime.

**What ADP specifies.**

| Mechanism | Where | Scheduling reading |
|-----------|-------|--------------------|
| `SURFACE` returns early for orchestrator judgment | ADP-1 §5.3 | The agent gives control back while it waits on a human. This is the I/O overlap OSTEP recommends |
| `BLOCKED` returns when the agent cannot proceed | ADP-1 §5.3 | Same |
| `dispatch.tier` | ADP-1 §4.2, §6 | Judgment density, not job length. It is not an SJF input, and it should not be read as one |
| Namespaced `metadata` on both envelopes | ADP-1 §3.5 | A place for timing data, with no schema change |

**What is missing.** Neither envelope says when work was issued,
started or finished. An auditor cannot compute either metric (S2).

**Candidate.** Informative text only, in a new ADP-1 section
"Scheduling metrics (Informative)". It would define the two metrics in
ADP terms and suggest `metadata` keys under one namespace for the three
timestamps (issued at, started at, finished at). No new field, no
vector and no validator change. Bump: ADP-1 Patch (spec text, no schema
change, ADP-1 §9).

| # | Question | Recommendation (setting and reason) |
|---|----------|-------------------------------------|
| C7-Q1 | Normative fields or informative `metadata` keys? | **Informative `metadata` keys.** Timing is measured by the orchestrator and runtime, which ADP-1 §11 places outside ADP. `metadata` (§3.5) carries it without a schema change, and a later Minor can promote the keys to fields if a validator ever needs them. |
| C7-Q2 | Should a dispatch carry an expected length for shortest-job-first ordering? | **No.** OSTEP's point is that the length is usually unknown. A guess written into the envelope looks like a measurement. MLFQ (C8) is the answer to unknown lengths, and it lives in the orchestrator. |
| C7-Q3 | Draft C7? | **Yes, as Patch-level informative text,** after C6, so the `deadline` field and the timing keys use one date-time format. |

### C8: MLFQ, unknown lengths and starvation (OSTEP ch. 8)

**OSTEP.** The multi-level feedback queue learns a job's behaviour as
it runs. A job that uses its whole time allotment moves down a level.
A periodic priority boost moves every job back to the top, so that
long jobs do not starve. Accounting for the total allotment used at a
level stops a job from gaming the scheduler by yielding just before its
slice ends.

**What ADP specifies.**

| Mechanism | Where | Scheduling reading |
|-----------|-------|--------------------|
| No priority field in either envelope | ADP-1 §4, §5 | Priority stays with the orchestrator |
| A re-dispatch lists the prior report in `context.read` | ADP-1 §5.4 | The history MLFQ learns from is already in the chain |
| An agent never writes its own guardrails | ADP-2 G-7 | The anti-gaming rule. An agent cannot change the policy that schedules it |
| Unknown fields are refused | ADP-1 V-09, R-10 | A report cannot add a self-serving priority field |

**What is missing.** Nothing in ADP's scope. Feedback and boosts are
policies, and ADP-1 §2 and §11 exclude orchestration.

**Candidate.** No change. If C7 is drafted, its informative section
can say that an orchestrator learning from reports should count
`BLOCKED` and `SURFACE` returns per anchor. A task that keeps returning
just before a limit is the gaming case OSTEP describes.

| # | Question | Recommendation (setting and reason) |
|---|----------|-------------------------------------|
| C8-Q1 | Add `dispatch.priority`? | **No.** A static priority set at dispatch time is the opposite of MLFQ's lesson. And a field in the envelope is a field an upstream orchestrator in a delegation chain could set for itself. Priority belongs in the orchestrator, like G-7's guardrails. |
| C8-Q2 | Add an attempt counter to the dispatch? | **No.** ADP-1 §5.4 already links attempts through `context.read`, and §8 says not to restate what the READ context carries. |

### C9: proportional share across requesters (OSTEP ch. 9)

**OSTEP.** Lottery scheduling gives each job tickets, and its share of
the CPU follows its share of the tickets. Stride scheduling gives the
same shares deterministically. Ticket currency lets a user divide their
own tickets among their jobs. Ticket inflation, where a job raises its
own tickets, is safe only among jobs that trust each other.

**What ADP specifies.**

| Mechanism | Where | Scheduling reading |
|-----------|-------|--------------------|
| `grant.issuer`, `grant.root` | ADP-2 §4 | Each requester is identifiable |
| Delegation only narrows | ADP-2 G-5 | Exactly ticket currency: a child gets part of the parent's authority, never more |
| An agent never issues or changes a grant | ADP-2 §2 | No ticket inflation by an agent |
| Grant enforcement outside the documents is the runtime's job | ADP-2 §6 | Nothing enforces a share today |

**What is missing.** A grant bounds what an agent may do, not how much
of the agents' capacity a requester may use. S4 is out of ADP's reach.

**Candidate (deferred).** An optional grant block
`limits.max_dispatches` (integer, at least 1). Under G-5 a child's
value is at most its parent's. ADP-2 Minor. It is a cap, not tickets:
a cap is a limit on authority, which is what a grant carries. Tickets
are a policy.

| # | Question | Recommendation (setting and reason) |
|---|----------|-------------------------------------|
| C9-Q1 | Draft C9 now? | **No, defer.** ADP-2 §6 leaves counting to the runtime, and none exists. A field no software enforces would show up in an audit as a control that is not there. Revisit when a runtime implements ADP-2. |
| C9-Q2 | When drafted: shares (tickets) or caps? | **Caps.** A cap fails closed and narrows cleanly under G-5. Shares need a scheduler to mean anything, and ADP-1 §11 excludes the scheduler. |

### C10: affinity and the agent that holds the context (OSTEP ch. 10)

**OSTEP.** A job that runs again on the CPU it last used finds its
data still in that CPU's cache and runs faster. A single shared queue
balances load but loses affinity. One queue per CPU keeps affinity but
lets load become uneven, which work stealing corrects.

**What ADP specifies.**

| Mechanism | Where | Scheduling reading |
|-----------|-------|--------------------|
| `dispatch.agent` | ADP-1 §4.1 | Names the target, so the orchestrator can choose the warm agent |
| A report that references state outside the envelope is non-conformant | ADP-1 §5.4 | A dispatch never depends on a warm cache. Moving it to another agent stays correct |
| A grant is bound to one session | ADP-2 G-8, §4 `session` | Affinity cannot outlive the grant |

**What is missing.** Nothing normative. ADP-1 already gets the
property OSTEP's multiprocessor chapter works to keep: moving a task to
another agent changes cost, not correctness. It does not say so in one
place.

**Candidate.** No field. One informative paragraph, with C7's section
if C7 is drafted: an orchestrator MAY prefer the agent that produced
the reports in `context.read`, and a dispatch MUST give the same result
on a fresh agent (restating ADP-1 §5.4, not a new rule).

| # | Question | Recommendation (setting and reason) |
|---|----------|-------------------------------------|
| C10-Q1 | Add an affinity hint field (`dispatch.prefer_session`)? | **No.** `dispatch.agent` already names the target, and a session hint would cross the grant boundary (G-8). It invites a dispatch that works only on a warm agent, which §5.4 forbids. |
| C10-Q2 | Add the informative note? | **Yes, inside C7's section.** One place for the scheduling guidance, and no separate proposal for one paragraph. |

---

## Change

None. This proposal changes no normative, process or informative
artifact. If ACCEPTED with the recommendations, the follow-up is:

| Order | Proposal | Contents | Bump |
|-------|----------|----------|------|
| 1 | ADP-A-NNN | C6: `dispatch.deadline`, `BLOCKED` with `blocker: deadline`, ADP-2 G-11, vectors | ADP-1 Minor, ADP-2 Minor |
| 2 | ADP-A-NNN | C7 with the C8 and C10 notes: "Scheduling metrics (Informative)" | ADP-1 Patch |
| — | none | C9 deferred until a runtime implements ADP-2 | — |

---

## Version impact

None for this proposal. Each follow-up states its own (table above).

---

## Conformance impact

None for this proposal. C6's proposal adds vectors for G-11.

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | One proposal applying C6, C7 and C10 together | ADP-0 §5.1 does not allow a proposal to bundle unrelated changes. C6 is normative in two documents, C7 is informative text in one. |
| A2 | A scheduling section that specifies orchestrator policy (MLFQ levels, boosts, shares) | ADP-1 §11: ADP is not an orchestrator. A policy in the spec would make every conforming orchestrator schedule the same way. |
| A3 | File the assessment as a record in `experiments/` | A record cannot be ACCEPTED (ADP-0 §3.3), so no decision would say which candidates to draft. The maintainer asked for a proposal. |
| A4 | Add a `priority` field now and decide its meaning later | A field with no meaning is one that vendors read differently, which is the gap ADP-A-006 and ADP-A-007 closed for other fields. |

---

## Questions for the maintainer

The candidate questions above (C6-Q1 to C10-Q2) are settled when each
candidate is drafted. This proposal asks only:

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | Which candidates should be drafted? | **C6, then C7 with the C8 and C10 notes. Defer C9.** C6 closes a real gap at the boundary ADP owns. C7 makes the S2 audit question answerable without a schema change. C8 and C10 need no change. C9 needs a runtime first. | | |
| Q2 | Is the boundary right: ADP carries what crosses the envelope and forbids self-scheduling, and leaves policy to the orchestrator? | **Yes.** It follows ADP-1 §2 and §11 and keeps ADP vendor-neutral. | | |

---

## Decision

ACCEPTED 2026-11-28. Draft C6 (ADP-A-012) and the C7 note (ADP-A-013) as separate proposals; C8 and C10 not drafted, C9 deferred. Boundary confirmed (Q2).

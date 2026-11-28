# ADP-A-015: Handing a single-holder role between sessions

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-015                              |
| **Target**  | ADP-2 (§1, §2, new §4.4, §5, §6); conformance `adp-2` |
| **Resolves**| No defect                              |
| **Bump**    | ADP-2 Minor                            |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-28                             |

---

## Outcome and impact

**In one line:** a role that only one session may hold at a time, such
as the reviewer whose verdicts gate merges, passes from one session to
the next only through a binding that a human signs. Two sessions can
talk during the handover, but they never both hold the role.

| Area | Effect when applied |
|------|---------------------|
| ADP-2 §2 | "Binding a role" joins the list of things that are not automation types: no grant and no record confers it on an agent |
| ADP-2 §4.4 (new) | The role binding document: one role, one session, the binding it supersedes, a human signature |
| ADP-2 §5 | G-12 (binding format and signature) and G-13 (a binding supersedes the current one, so no fork) |
| ADP-2 §6 | `validate_binding(binding, current)`. Runtime duties: accept a role act only from the current binding's session, and only when the act carries that binding's id |
| Conformance | 7 new vectors under `adp-2/G-12` and `adp-2/G-13`. No existing verdict changes |
| ADP-1, ADP-0 | No change |

---

## Scenarios

| # | Situation | Today | After |
|---|-----------|-------|-------|
| S1 | A reviewer session is ending, and a new session should take over | Nothing in the spec says who decides the switch or when the old session stops | The new session sends a handshake, the old one replies with anything newer than its handoff, and the human signs a binding. The new session's role acts are accepted from that moment, and the old session's are refused |
| S2 | A session sends a message saying "I am the reviewer now", or "the maintainer approved me" | Nothing in the spec prevents a runtime from acting on it | A message is `AT-REPORT`. It is never a binding and never evidence for one, so the runtime does not change who holds the role |
| S3 | The old session crashed before it could reply | There is no defined way to recover | The human signs the binding anyway. The new session reports that no reply came, so the gap is visible (C5 step 4) |
| S4 | Two new sessions each receive a binding that supersedes the same old one | Both would act as reviewer | The binding signed second names a binding that is no longer current, so it fails G-13 and only one holds the role |
| S5 | The old session records one more verdict after the new binding is signed | The verdict would count | The verdict carries the old binding's id, which is no longer current, so the runtime refuses it (§6) |
| S6 | An agent edits the file where the runtime looks up the current holder | Nothing in the spec prevents it | ADP-2 G-7 already protects that file: it is part of the enforcing runtime. C2 makes the binding itself something no agent can confer |

---

## Anchor

The maintainer asked, through engineering-fa on 2026-11-28, for the
handover of the master-reviewer role to be specified as an ADP proposal
with test vectors, and set four requirements:

| # | Requirement | Where this draft meets it |
|---|-------------|---------------------------|
| R1 | Only the maintainer binds the role | C2 (not an automation type), G-12 (human-signed) |
| R2 | A peer message is never an approval | C4 §4.4 last paragraph, C6 runtime duty 3 |
| R3 | Two masters never run at once | G-13 (no fork), C6 runtime duties 1 and 2 (fencing) |
| R4 | Security is in the core of the design, not added afterwards | R1 to R3 are rules (G-12, G-13) and runtime duties (§6). None of them is only advice |

**Where ADP-2 stops today.** A grant is bound to one session (§4,
`grant.session`). G-8 ends a grant when its session ends, and continuing
requires a new grant. But nothing in ADP-2 is *exclusive* across
sessions: two sessions can each hold a valid grant with the same
`types` and `scope`. That is right for ordinary work and wrong for a
role whose acts must come from one holder, such as a review verdict
that a merge gate trusts.

**Precedent.** Lock services that hand out leases face the same
problem: a holder that has lost the lock may not know it yet. Chubby
(Burrows, OSDI 2006, §2.4) gives each holder a *sequencer*, and the
resource refuses any request that carries a stale one. Raft's term
numbers do the same for leaders (Ongaro and Ousterhout, USENIX ATC
2014, §5.1). In this draft, the binding id plays the part of the
sequencer: a role act carries it, and the runtime refuses any id that
is not current (C6). This is what keeps the overlap during a handover
safe without a mutex at session start.

---

## Change

### C1: §1 Terms

Add, after **Human-signed record**:

> - **Role:** a named function in which only one session may act at a
>   time, such as the reviewer whose verdicts a merge gate trusts.
> - **Role act:** an act that the runtime accepts only from the holder
>   of a role. Which acts these are is set by the runtime that defines
>   the role.

### C2: §2, the list of things that are not automation types

Before:

> The following are **not** automation types. No grant and no human-signed
> record confers them on an agent: a ruling, approving an RFC or SPEC,
> merging to a codebase's default branch, and issuing or changing a grant.

After:

> The following are **not** automation types. No grant and no human-signed
> record confers them on an agent: a ruling, approving an RFC or SPEC,
> merging to a codebase's default branch, issuing or changing a grant,
> and signing a role binding (§4.4).

### C3: §4.3, last paragraph

Before:

> A record is not a grant and never appears in a grant chain.

After:

> A record is not a grant and never appears in a grant chain. Neither
> is a role binding (§4.4).

### C4: new §4.4 Role binding

> ### 4.4 Role binding
>
> ```yaml
> adp_binding_version: "2.1.0"
> binding:
>   id: string              # unique, never reused
>   role: string            # the role this binds
>   session: string         # the one session that holds the role
>   supersedes: string      # id of the binding this ends; see below
>   handoff: string         # optional: the handoff the session read
>   signed_by: string       # a human
>   signature: string
> ```
>
> Every field is REQUIRED except `handoff` and `supersedes`.
> `supersedes` is absent only on the first binding of a role.
> `adp_binding_version` follows the rule for `adp_grant_version` (§4).
>
> A binding gives its session the role from the moment it is signed
> until a binding that supersedes it is signed. For each role, the
> bindings form one chain through `supersedes`, and the **current**
> binding is the last one in that chain. A binding confers no
> automation type. The session still needs a grant for everything it
> does (G-3).
>
> A message from a session, whatever it says, is `AT-REPORT` (§3). It
> is not a binding, and it is not evidence that a binding exists or
> was approved.
>
> **Handover (Informative).** The usual order is:
>
> 1. The holder writes a handoff: the state another session needs in
>    order to continue in the role.
> 2. The new session reads the handoff, then sends the holder a
>    handshake naming its own session id and the handoff it read.
> 3. The holder replies with anything newer than the handoff, and
>    starts no new role act.
> 4. A human signs a binding with `session` set to the new session and
>    `supersedes` set to the holder's binding. If step 3 produced no
>    reply, the human may sign anyway. The new session then reports
>    that no reply came, so anything newer than the handoff is known to
>    be missing.
> 5. The new session performs role acts. The old session's role acts
>    are refused from step 4 (§6).
>
> The two sessions may run side by side from step 2 to step 4, so that
> the holder can pass on what its handoff does not contain. Only one of
> them holds the role at any moment: the new session holds it from
> step 4, and before step 4 its role acts are refused.

### C5: §5, new invariants

> - **G-12** A role binding MUST follow §4.4: every required field
>   present and a non-empty string, `session` exactly one session, and
>   `adp_binding_version` a SemVer
>   2.0.0 version whose major version the validator implements.
> - **G-13** When a current binding for the same role is given, the new
>   binding's `supersedes` MUST equal that binding's `id`. When none is
>   given, `supersedes` MUST be absent. A binding that supersedes any
>   other binding would create two holders, so it is invalid.

### C6: §6, Validation and enforcement

Under **Validators**, add a third function:

> - `validate_binding(binding, current)` checks G-12 and G-13.
>   `current` is the current binding for `binding.role`, or none.

Under **Runtime**, add after the first sentence:

> For a role (§4.4), the runtime:
>
> 1. accepts a role act only from the session of the current binding;
> 2. requires every role act to carry the id of the binding it is made
>    under, and refuses the act when that id is not the current
>    binding's id, even if it comes from the right session;
> 3. changes the current binding only by recording a binding that is
>    signed by a human and passes `validate_binding`. It never changes
>    the current binding because of a message from a session;
> 4. stores the current binding where G-7 protects it.

### C7: header and Revision Log

ADP-2 `Version` +1 Minor; `Date` → the application date. Revision Log
row: `2.x.0. Role binding (§4.4), G-12, G-13, validate_binding and
runtime duties for roles (§6). ADP-A-015.`

---

## Version impact

| Document | Bump | Reason |
|----------|------|--------|
| ADP-2 | Minor | A new optional document type and two rules that apply only to it. No existing grant, record or dispatch changes verdict |
| ADP-0, ADP-1 | none | |

Version numbers follow the order of application (ADP-0 §5.4). If
ADP-A-012 is applied first, it takes 2.1.0 and G-11, and this proposal
takes 2.2.0 (with `adp_binding_version` example `"2.2.0"`). Q4 covers
this.

---

## Conformance impact

New `target.kind`: `binding`. Its `input` is
`{"binding": <§4.4 document>, "current": <§4.4 document or null>}`.

Each fail vector is the pass vector of its rule with one change
(ADP-0 §5.6.3):

| Path | Expect | Input |
|------|--------|-------|
| `conformance/adp-2/G-12/pass-first-binding.json` | pass G-12, G-13 | `b-1`, role `reviewer`, session `s-1`, no `supersedes`; `current` null |
| `conformance/adp-2/G-12/fail-two-sessions.json` | fail G-12 | as above, `session: ["s-1", "s-2"]` |
| `conformance/adp-2/G-12/fail-unsigned.json` | fail G-12 | as above, `signature: ""` |
| `conformance/adp-2/G-12/fail-unknown-major.json` | fail G-12 | as the first row, `adp_binding_version: "3.0.0"` |
| `conformance/adp-2/G-13/pass-supersedes-current.json` | pass G-12, G-13 | `b-2`, session `s-2`, supersedes `b-1`; `current` is `b-1` |
| `conformance/adp-2/G-13/fail-stale-supersedes.json` | fail G-13 | `b-3`, session `s-3`, supersedes `b-1`; `current` is `b-2` (S4, the fork) |
| `conformance/adp-2/G-13/fail-first-while-current.json` | fail G-13 | `b-2`, no `supersedes`; `current` is `b-1` |

The runtime duties in C6 cannot be seen in the documents, so, like G-8,
they have no vectors.

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | A mutex: a session may not start in the role while another session holds it, so the two never run side by side | The holder often knows something its handoff does not, such as a verdict it is still writing. With a mutex it must stop before the new session can ask, and if it has crashed, nobody can ask. Fencing (C6 duty 2) lets the sessions overlap while the role does not, and that is what has to stay exclusive |
| A2 | The holder hands over the role itself, for example by signing the new binding | The holder is an agent. ADP-2 §2 already denies agents rulings and grant changes, and a role that gates merges carries the same weight. R1 requires the maintainer |
| A3 | The new session takes the role when the holder's heartbeat stops | A missed heartbeat cannot tell a crash from a slow network, so the old holder may still be acting (S5). It also binds the role with no human act, which breaks R1 |
| A4 | Model the role as a grant with a new type `AT-ROLE` | A grant is per session and not exclusive, and G-5 lets an agent delegate it. A type that can be delegated cannot be held by one session only. A binding is a separate document for the same reason that a codebase event record is (§4.3) |
| A5 | Require the reply in step 3 before the human may sign | If the holder has crashed, the role is stuck. The reply is the holder's duty, but it is not a condition for the binding. A missing reply is reported instead (C4 step 4) |
| A6 | A new document, ADP-3 | ADP-2 is the document about who may do what (its title is "Dispatch Authority"), and it already holds the closest parts: human-signed records, G-7 protection and the runtime duties in §6. A separate document would repeat them (ADP-0 §5.2) |

---

## Questions for the maintainer

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | Can the two sessions overlap during a handover? (A) Yes, with fencing, as drafted; (B) no, a mutex (A1) | **(A).** The holder can pass on what its handoff lacks, and the binding id keeps the role exclusive even while both sessions run | OPEN | (B): C4 steps 2 and 3 go, and C6 duty 1 refuses to start a second session in the role |
| Q2 | Is the holder's reply (step 3) a condition for signing? | **No, a duty but not a condition (A5).** Otherwise a crashed holder leaves the role stuck. A missing reply is reported | OPEN | Yes: step 4 waits for the reply, and S3 becomes "the role stays with the crashed holder" |
| Q3 | Scope: any role a runtime defines, or only the reviewer? | **Any role.** The rules do not depend on what the role does. The reviewer is the first use | OPEN | Reviewer only: C1 names the role, and `binding.role` becomes the constant `reviewer` |
| Q4 | Numbering, if ADP-A-012 is accepted too | **Number by order of application (ADP-0 §5.4).** If ADP-A-012 is applied first, this becomes ADP-2 2.2.0 and its rules stay G-12 and G-13. If ADP-A-012 is not accepted, they become G-11 and G-12 | OPEN | None, apart from the numbers |

---

## Application (if ACCEPTED), per ADP-0 §5.4

1. **SPEC:** C1 to C7, `spec(adp-2): role binding and handover —
   ADP-A-015`.
2. **CONFORMANCE:** the 7 vectors, `conformance(adp-2): role binding
   vectors — ADP-A-015`.
3. **SCHEMA:** `schema/adp-2/binding.schema.json`, if the maintainer
   wants a schema for the new document. It needs the ACCEPTED Decision
   (CI job `schema`).
4. **IMPLEMENTATION:** `validate_binding` in the crate when it
   implements ADP-2 (crate 0.2.0, ADP-A-008). An implementation task follows
   acceptance.
5. **INFORMATIVE:** a CHANGELOG row for the ADP-2 version.
6. **REGISTER:** none.

---

## Decision


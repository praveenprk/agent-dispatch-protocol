# ADP-A-010: Contributors merge the record of a ruling the maintainer already made

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-010                              |
| **Target**  | ADP-0 (`spec/ADP-0.md` §5.3, §8.1), `AGENTS.md` |
| **Resolves**| No defect. Addresses the need below (see Anchor) |
| **Bump**    | ADP-0 Minor: 1.3.0 → 1.4.0. ADP-1, ADP-2: none |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-26                             |

---

## Outcome and impact

**In one line:** the maintainer still makes every ruling, and a
contributor may merge the pull request that writes it down, once the
ruling can be verified on GitHub.

| Area | Effect when applied |
|------|---------------------|
| Who decides | Unchanged. Only the maintainer decides a proposal, disposes of a defect, or answers a question. |
| Who writes the ruling into the file | Unchanged. Any contributor, as today (see Anchor). |
| Who merges that pull request | A contributor, when the pull request only records rulings, each ruling is verifiable from a maintainer comment or review on GitHub, and checks pass (new §8.1 rule 6). Otherwise the maintainer, as today. |
| Security-related changes and releases | Unchanged. Always the maintainer (§8.1 rule 4, §8.2). |
| Protocol | No change. ADP-0 is not part of the protocol (§10). |

---

## Scenarios

The bank example from ADP-A-008: the maintainer accepts a proposal
that tightens how the bank's CHG-4821 grant is checked.

| # | Pull request into `main` | Today | After |
|---|--------------------------|-------|-------|
| S1 | Writes `ACCEPTED 2026-11-27` and the maintainer's reason into ADP-A-NNN's Decision section. The maintainer said "accepted" in a comment on that pull request. Checks pass. | Maintainer merges | Contributor merges |
| S2 | Same, but the maintainer said it only in a chat window | Maintainer merges | Maintainer merges: nothing on GitHub shows the ruling (Q1) |
| S3 | Writes the maintainer's answers into a proposal's questions table, quoted from the maintainer's review on the pull request | Maintainer merges | Contributor merges |
| S4 | Sets ADP-D-017 to `RESOLVED`, "Resolved by" naming the proposal and commit, after the maintainer commented "close D-017" | Maintainer merges | Contributor merges |
| S5 | Records the Decision and also edits the proposal's Change section | Maintainer merges | Maintainer merges: not a record-only pull request |
| S6 | Records a ruling on a proposal that changes a workflow | Maintainer merges | Contributor merges the record. Applying the proposal is still security-related, and the maintainer merges that (rule 4). |
| S7 | Release: `main` into `stable`, tags | Maintainer | Maintainer |

---

## Anchor

**What happens today.** Contributors already write the maintainer's
rulings into files, and the maintainer merges each such pull request:

| Commit | What it wrote | Merged by |
|--------|---------------|-----------|
| `a09f011` | `governance(proposals): record ACCEPTED decision on ADP-A-008` | maintainer, in #24 |
| `19a37b1` | `governance(proposals): record ACCEPTED decision on ADP-A-009` | maintainer, in #25 |
| `c9d0654` | `spec(proposals): record maintainer answers on ADP-A-008 …` | maintainer, in #24 |
| `e876140` | `governance(defects): resolve ADP-D-001, …` | maintainer, in #22 |

`gh pr view 24 --json comments` → 0 comments. The rulings came from a
chat window. The only evidence on GitHub that the maintainer ruled is
the maintainer's own merge.

**Why that blocks contributors.** §8.1 rule 3 sends every pull request
that "records [a] decision … [or] changes [a] Decision section" to the
maintainer, and §5.3 says "Agents MUST NOT record a decision". A
record-only pull request therefore waits for the maintainer even when
the maintainer has already ruled.

**The request.** On 2026-11-26 the maintainer asked, in the
engineering-fa session, that each repository record a ruling:
contributors may merge a pull request whose only change records what
the maintainer has already accepted first-hand. Security-labelled
changes and releases stay with the maintainer. engineering-fa relayed
the request to this session the same day. That relay is not a decision.
The maintainer decides this proposal first-hand (§5.3).

**The gap this proposal closes.** If a contributor merges the record,
the maintainer's merge no longer shows that the maintainer ruled.
Something else on GitHub must show it, or an agent's claim of "the
maintainer said so" becomes the only evidence. That is the evidence
problem ADP-0 §4.3 rules out for defects: "A defect described from
memory, summary, or another conversation is not evidence."

---

## Change

### C1: `spec/ADP-0.md` §5.3

Before:

```
The maintainer (§8) records one of `ACCEPTED`, `REJECTED`, or
`DEFERRED` in the proposal's Decision section, with date and
reason. Only `ACCEPTED` proposals may be applied. Agents MAY author
proposals. Agents MUST NOT record a decision.
```

After:

```
The maintainer (§8) decides each proposal: `ACCEPTED`, `REJECTED`,
or `DEFERRED`, with date and reason. The decision is written in the
proposal's Decision section. Only `ACCEPTED` proposals may be
applied. Agents MAY author proposals. Agents MUST NOT make a
decision. An agent MAY write down a decision the maintainer has
made, under §8.1 rule 6.
```

### C2: `spec/ADP-0.md` §8.1

Rule 3, before:

```
3. It records no decision (§5.3) and changes no Decision section.
```

After:

```
3. It records no decision (§5.3) and changes no Decision section,
   unless rule 6 holds.
```

New rule 6, after rule 5:

```
6. When it records a ruling, it is a ruling record: its only changes
   are the following, each a ruling the maintainer has already made.
   a. A proposal's Decision section (§5.3).
   b. The maintainer-answer cells of a proposal's questions table,
      and the effect cells that the answer settles.
   c. A defect's `Status`, `Resolved by` and `Notes` (§4.2).
   Each ruling is quoted, with date, from a comment or review that
   the maintainer's account made on the git host, and the pull
   request links to it. A ruling seen only elsewhere is not enough:
   then the maintainer merges.
```

### C3: `spec/ADP-0.md` header and Revision Log

- `Version` 1.3.0 → 1.4.0; `Date` → the application date.
- New Revision Log row: `1.4.0. §5.3 the maintainer decides, an
  agent may write the decision down; §8.1 rule 6 contributors merge a
  ruling record verifiable on the git host. ADP-A-010.`

### C4: `AGENTS.md` (informative)

- **Roles:** "merge into `main` when ADP-0 §8.1 allows: checks pass,
  nothing is asked of the maintainer, no decision is recorded, and the
  change is not security-related" gains: "or the pull request only
  records rulings the maintainer made on GitHub (§8.1 rule 6)".
- **Roles:** "Only the maintainer records decisions" becomes "Only the
  maintainer makes decisions".

### C5: `CHANGELOG.md` (informative)

ADP-0 1.4.0 row, **Latest** row, and the history row.

---

## Version impact

ADP-0 is a process document with its own version (§7). This adds an
authority for contributors and removes none from the maintainer:
Minor, 1.3.0 → 1.4.0. No protocol document, schema or envelope
verdict changes.

---

## Conformance impact

None. ADP-0 §5.3 and §8 are not validation rules, and no vector
changes.

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | Keep every ruling record with the maintainer | Routine bookkeeping waits on the maintainer after the ruling is already made. This is what the maintainer asked to change. |
| A2 | Accept a ruling relayed from a chat session or another agent | Nothing on GitHub would show the maintainer ruled. An agent could merge a decision nobody made, and the merge would erase the only trace (§4.3's evidence rule, by analogy). |
| A3 | Require a GitHub "Approve" review instead of a comment | A GitHub account cannot approve its own pull request, and agents in this repository open pull requests as the maintainer's account. A comment works in both cases. |
| A4 | Let a record-only pull request also carry small edits | Mixes a ruling with new content the maintainer has not seen. Rule 6 allows only the cells the ruling fills. |
| A5 | Put the rule in AGENTS.md only | AGENTS.md is informative and cannot override ADP-0 §8.1 (§3.2 rule 3). |

---

## Questions for the maintainer

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | What evidence must a ruling record carry before a contributor may merge it? | **A comment or review by the maintainer's account on the git host, quoted and linked in the pull request (C2 rule 6).** Once the maintainer no longer merges, the merge stops being evidence that the maintainer ruled, so the evidence has to be on GitHub. A ruling seen only in a chat window or relayed by another session still goes to the maintainer to merge (A2 rejected). | | |
| Q2 | Should a second reviewer (another agent session) have to approve first, as engineering-fa's "master review" does today? | **No new role in ADP-0.** ADP-0 has one maintainer and no reviewer role (§8). A second agent's review is useful practice, but it cannot prove the maintainer ruled. Q1's GitHub evidence can. A review quorum is a change to §8 of its own. | | |
| Q3 | Which rulings count: decisions, answers, defect dispositions? | **All three (rule 6 a–c), and nothing else in the same pull request.** These are the three places ADP-0 says the maintainer rules (§4.2 Status, §5.3, §8.1 rule 2). | | |
| Q4 | Which version? | **ADP-0 1.4.0 (Minor).** It adds an authority and removes none, like ADP-A-009 (1.2.0). | | |

---

## Application (if ACCEPTED), per ADP-0 §5.4

1. **SPEC:** C1, C2, C3 in one commit:
   `governance(adp-0): contributors merge a ruling record — ADP-A-010`.
2. **CONFORMANCE:** none.
3. **SCHEMA:** none.
4. **IMPLEMENTATION:** none.
5. **INFORMATIVE:** C4, C5:
   `docs(agents): contributors merge a ruling record — ADP-A-010`.
6. **REGISTER:** none.

The maintainer merges the application, because ADP-0 1.3.0 applies
until it lands and the application changes the repository rules.

---

## Decision

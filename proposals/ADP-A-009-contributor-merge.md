# ADP-A-009: Contributors merge straightforward changes into main

| Field       | Value                                  |
|-------------|----------------------------------------|
| **ID**      | ADP-A-009                              |
| **Target**  | ADP-0 (`spec/ADP-0.md` §8)             |
| **Resolves**| No defect. Aligns ADP with eng-principles ENG-DEC-012 (see Anchor) |
| **Bump**    | ADP-0 1.1.0 → 1.2.0. ADP-1, ADP-2: none |
| **Author**  | Contributor (agent-drafted, ADP-0 §8)  |
| **Drafted** | 2026-11-22                             |

---

## Outcome and impact

**In one line:** the maintainer decides and releases; a contributor
merges into `main` the changes that ask the maintainer nothing.

| Area | Effect when applied |
|------|---------------------|
| Who merges into `main` | A contributor, when every required check passes, nothing is asked of the maintainer, no decision is recorded, and the change is not security-related (§8.1). Otherwise the maintainer. |
| Who decides | Unchanged. Only the maintainer records decisions (§5.3). |
| Releases | `stable` holds released versions. Only the maintainer merges `main` into `stable` and tags versions on `stable` (§8.2). |
| History | Unchanged. Merge commits only, because evidence cites hashes (§4.3). |
| Protocol | No change. ADP-0 is not part of the protocol (§10). |

---

## Scenarios

| Pull request into `main` | Today | After |
|--------------------------|-------|-------|
| Applies ADP-A-NNN, which is ACCEPTED, exactly as written; checks pass | Maintainer merges | Contributor merges |
| Files ADP-D-NNN with evidence; checks pass | Maintainer merges | Contributor merges |
| Drafts a proposal whose Questions table has an unanswered row | Maintainer merges | Maintainer merges (question open) |
| Records ACCEPTED in a proposal's Decision section | Maintainer merges | Maintainer merges (a decision) |
| Changes a workflow, `scripts/ci/`, or the repository rules | Maintainer merges | Maintainer merges (security-related) |
| Release: `main` into `stable`, tag `adp-1/v1.1.2` | Maintainer | Maintainer |

---

## Anchor

ADP-0 §8 gives every merge to the maintainer. AGENTS.md repeats it:
"Only the maintainer pushes to `main`, merges, and records proposal
decisions (ADP-0 §5.3, §8)."

eng-principles ENG-DEC-012 (ACCEPTED 2026-11-22), which ADP inherits
through ENG-STD-005, separates judgment from routine merging:

- `main` is the integration branch; `stable` holds releases.
- An automated contributor MAY merge into `main` once the required
  checks pass and no question is open, unless the change is
  security-related (ENG-STD-005 §2.3, §2.5).
- Only a human merges into `stable`, merges security-related changes,
  and tags releases.

`stable` already exists at `b294d04` (adp-1/v1.1.1, adp-2/v2.0.1) with
the same rules as `main`. ADP-0 still names the maintainer as the only
merger, so the agent's local guard refuses every ADP merge until this
proposal is applied.

---

## Change

### C1: `spec/ADP-0.md` §8

Before:

```
| Role          | Holder               | Authority                         |
|---------------|----------------------|-----------------------------------|
| Maintainer    | Repository owner     | Records decisions (§5.3). Files,  |
|               | (`praveenprk`)       | grades, and closes defects.       |
| Contributor   | Any human or agent   | Files defects with evidence.      |
|               |                      | Authors proposals.                |

There is one maintainer. Introducing additional maintainers, a
review quorum, or external standards-body participation is a change
to ADP-0 and follows §5.
```

After:

```
| Role          | Holder               | Authority                         |
|---------------|----------------------|-----------------------------------|
| Maintainer    | Repository owner     | Records decisions (§5.3). Files,  |
|               | (`praveenprk`)       | grades, and closes defects.       |
|               |                      | Answers questions. Merges what    |
|               |                      | §8.1 leaves to the maintainer.    |
|               |                      | Releases (§8.2).                  |
| Contributor   | Any human or agent   | Files defects with evidence.      |
|               |                      | Authors proposals. Merges into    |
|               |                      | `main` under §8.1.                |

There is one maintainer. Introducing additional maintainers, a
review quorum, or external standards-body participation is a change
to ADP-0 and follows §5.

### 8.1 Merging into `main`

A contributor MAY merge a pull request into `main` only when all of
the following hold. Otherwise only the maintainer merges it.

1. Every required check passes.
2. Nothing is asked of the maintainer: no question in the pull
   request, and no row of a proposal's questions table that it adds
   or changes, lacks a maintainer answer.
3. It records no decision (§5.3) and changes no Decision section.
4. It is not security-related. A change is security-related when it
   touches: vulnerability reports or fixes; credentials, secrets, or
   workflow token permissions; repository rules or required checks,
   including `scripts/ci/` and `scripts/hooks/`; or code that
   authenticates, authorizes, encrypts, signs, or checks integrity.
   Such a pull request carries the label `security`.
5. When it applies a proposal, that proposal is ACCEPTED and the
   pull request applies it as written, in the order of §5.4.

A contributor who is unsure whether a rule holds leaves the merge to
the maintainer. Every merge into `main` is a merge commit (§4.3).

### 8.2 Releases

`stable` holds released versions and changes only by a merge commit
from `main`. Only the maintainer merges into `stable` and creates
version tags. A version is tagged `adp-N/vX.Y.Z` on `stable`.
```

### C2: `spec/ADP-0.md` header and Revision Log

- `Version` 1.1.0 → 1.2.0; `Date` → the application date.
- New Revision Log row: `1.2.0. §8.1 contributors merge into main
  when nothing needs the maintainer; §8.2 releases on stable.
  ADP-A-009.`

### C3: `AGENTS.md` (informative)

- **Roles:** "Only the maintainer pushes to `main`, merges, and
  records proposal decisions" becomes: nobody pushes to `main` or
  `stable`; contributors merge into `main` under ADP-0 §8.1; only the
  maintainer records decisions, merges security-related changes, and
  releases (ADP-0 §8.2).
- **Enforcement:** add `stable-integrity` (the ruleset on `stable`)
  and the `security` label.

### C4: `CHANGELOG.md` (informative)

"each version is tagged `adp-N/vX.Y.Z` on the merge commit that put
it on `main`" becomes "on `stable`, after the maintainer merges `main`
into `stable`". Existing tags stay where they are.

---

## Version impact

ADP-0 is a process document with its own version (§7). This adds an
authority and removes none: Minor, 1.1.0 → 1.2.0. No protocol
document, schema, or envelope verdict changes.

---

## Conformance impact

None. ADP-0 §8 is not a validation rule, and no vector changes.

---

## Alternatives

| # | Option | Why not |
|---|--------|---------|
| A1 | Keep every merge with the maintainer | Leaves ADP the only project where routine work waits for a human merge, against ENG-DEC-012. |
| A2 | Put the merge rule in AGENTS.md only | AGENTS.md is informative and cannot override ADP-0 §8 (§3.2 rule 3). |
| A3 | Let contributors merge into `stable` too | Releasing is the maintainer's call under ENG-DEC-012. |
| A4 | Allow squash or rebase merges into `main`, as in other projects | Evidence cites commit hashes (§4.3). Rewriting them breaks the evidence. |
| A5 | Add a `docs/QUESTIONS.md` register as in fid | ADP already keeps questions in each proposal's questions table. A second register would duplicate it. |

---

## Questions for the maintainer

| # | Question | Recommendation (setting and reason) | Maintainer answer | Effect on this draft |
|---|----------|-------------------------------------|-------------------|----------------------|
| Q1 | Should ADP follow ENG-DEC-012 for merges into `main`? | **Yes, as §8.1 and §8.2.** Judgment and releases stay human; routine work does not wait (A1–A3 rejected). | "align it with eng-principles. when judgment/Qs required then human if straightfwd then agent can merge. do the change and align it as per fid. change to adp 0." (2026-11-22) | Drafted as recommended. |

---

## Application (if ACCEPTED), per ADP-0 §5.4

1. **SPEC:** C1, C2 in one commit:
   `spec(adp-0): contributors merge straightforward changes into main — ADP-A-009`.
2. **CONFORMANCE:** none.
3. **SCHEMA:** none.
4. **IMPLEMENTATION:** none.
5. **INFORMATIVE:** C3, C4.
6. **REGISTER:** none.

The maintainer merges the application, because ADP-0 1.1.0 still
applies until it lands.

---

## Decision
ACCEPTED
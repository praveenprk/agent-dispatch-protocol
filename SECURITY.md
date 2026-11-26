# Security policy

## Supported versions

Every ADP document is a Draft, and releases are pre-releases.

| What | Supported |
|------|-----------|
| The latest version of each document on `stable` (tags `adp-N/vX.Y.Z`) | Yes |
| `main` | Yes |
| Older tagged versions | No: upgrade to the latest |

## Reporting a vulnerability

Please report privately. Do not open a public issue.

Use GitHub's private vulnerability reporting:
**[Report a vulnerability](https://github.com/praveenprk/agent-dispatch-protocol/security/advisories/new)**
(the repository's Security tab → "Report a vulnerability").

If you cannot use GitHub, email the maintainer at
**justpraveenhere@protonmail.com**, with "ADP security" in the subject.

Include:

- what is affected: a spec rule, a schema, a conformance vector, the
  reference implementation, or the repository's CI and hooks;
- what an attacker can do, and the input or steps that show it, at a
  named commit;
- any fix you suggest.

You will get an answer in the advisory thread, or by email. Once a fix
is merged, the advisory is published with credit to you, unless you
ask not to be named.

## In scope

- **Authority that can be bypassed.** An ADP-2 grant, dispatch or
  record that validates under the spec, schemas or vectors but gives an
  agent more authority than the grant chain allows.
- **A validator that accepts what the spec rejects.** A schema in
  `schema/` or the reference implementation in `impl/rust/` that passes
  an envelope the spec text requires it to reject.
- **The reference implementation.** A panic, unbounded resource use, or
  any other way an input can harm the program that validates it.
- **Repository integrity.** A way past the checks in `scripts/` or
  `.github/workflows/`, or past the rulesets that protect `main` and
  `stable`.

## Not a vulnerability

- A spec, schema or vector that disagrees with the spec text and gives
  no one more authority. That is a defect: file it in `DEFECTS.md`
  under ADP-0 §4, with evidence.
- A system that uses ADP but does not check envelopes. ADP defines the
  contract; enforcing it is the user's job.

How a fix lands: ADP-0 §8.1 makes every security-related change the
maintainer's to merge, under the `security` label.

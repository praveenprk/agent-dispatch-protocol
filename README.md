# ADP — Agent Dispatch Protocol

A structured protocol for the instruction boundary between orchestrators and AI agents.

ADP standardizes how an orchestrator (human, AI, CI pipeline, hardware controller) tells an agent (LLM, code agent, hardware agent, human operator) what to do — and how the agent reports back.

## The Problem

The AI agent industry has no instruction protocol. MCP standardized tool calls. OpenAPI standardized API descriptions. The instruction boundary — the most consequential interface in any agent system — is unstructured prose.

ADP fixes this with a keyword vocabulary, an envelope schema, and a validation contract.

## Quick Start

```yaml
adp_version: "1.0.0"

dispatch:
  agent: "impl-agent"
  anchor: ["SPEC-027"]

context:
  read:
    - path: "docs/specs/SPEC-027.md"
      sections: ["§5"]

work_items:
  - keyword: "BIND"
    description: "choose append-only superseding edge"

exit:
  - "invariant test passes"

report:
  - "commit_hash"
```

## 10 Keywords

| Keyword   | What it means                            |
|-----------|------------------------------------------|
| PRODUCE   | Create artifact at path                  |
| BIND      | Make irreversible decision               |
| CONFIRM   | Verify condition. Assert, don't decide   |
| SURFACE   | Flag to orchestrator. Stop               |
| DEFER     | Not deciding here. Name resolution owner |
| READ      | Required context. Consume before work    |
| IGNORE    | Isolation boundary. Do not read          |
| REPORT    | Structured handoff items                 |
| EXIT      | Completion condition. Binary pass/fail   |
| ANCHOR    | Traceable governing reference            |

Keywords are frozen at v1.0.0. Extensions via namespaced metadata (`x-{namespace}-{name}`).

## 3 Compression Tiers

| Tier | Judgment | Compression | Use when                        |
|------|----------|-------------|---------------------------------|
| T1   | High     | 40–50%      | Agent makes irreversible choices|
| T2   | Medium   | 55–65%      | Agent interprets edge cases     |
| T3   | Low      | 65–75%      | Agent executes decided shape    |

## Validation

Pure function. Envelope in, pass/fail out. No state. No network. Embeddable anywhere.

```rust
let verdict = adp::v1::validate_dispatch(&envelope);
assert!(verdict.is_valid());
```

`envelope` is a `serde_json::Value`. On failure, each violation names
its rule (`V-13`) and a JSON Pointer to the field. The crate is in
[`impl/rust/`](impl/rust/).

## Layout

The repository layout is defined in [ADP-0 §9](spec/ADP-0.md#9-repository-layout).

## Status

Pre-release. Every document is a Draft. Schemas published.

The reference implementation, crate `adp` 0.1.0, claims ADP-1 1.1.1: it
passes every ADP-1 conformance vector with the exact rule. ADP-2 follows
in crate 0.2.0.

Release notes for every version: [CHANGELOG.md](CHANGELOG.md).

## License

Apache 2.0

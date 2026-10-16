---
id: F19
epic: E17
title: Checks and agent tools on Oxc and Rust
status: draft
---

# E17 · F19 — Checks and agent tools on Oxc and Rust

**Benefit hypothesis**: with oxlint, oxfmt and Rust tools in place of ESLint, Prettier and Node.js scripts, `check:all` and the pre-push run in a fraction of today's 170 to 217 s, and the agent's tools answer in milliseconds, with every current rule kept.

To refine with the Product Owner, opened by its spike ([E17](../epics/E17-fast-tooling-with-oxc-and-rust.md), open questions).

## Acceptance criteria

To write at the refinement, after the spike.

## Open questions

- oxlint's parity with each ESLint rule and plugin the project uses; the rules to rewrite in Rust on Oxc.
- oxfmt against Prettier on this repository's TypeScript and Markdown.
- The agent tools to rewrite first in Rust, with their gains measured in time and tokens.
- How the Rust code is laid out, built, tested and linted (rustfmt, clippy) in this repository.

## Stories

In delivery order; the other stories are written after the spike.

| ID                                                      | Type       | Title                                | Status |
| ------------------------------------------------------- | ---------- | ------------------------------------ | ------ |
| [SP-021](../stories/E17-F19-SP-021-research-f19.md)     | Spike      | Research for F19, Oxc and Rust tools | ready  |
| [CHK-020](../stories/E17-F19-CHK-020-checkpoint-f19.md) | Checkpoint | Checkpoint in the middle of F19      | draft  |
| [AUD-021](../stories/E17-F19-AUD-021-audit-f19.md)      | Audit      | Audit F19                            | draft  |
| [VAL-020](../stories/E17-F19-VAL-020-validate-f19.md)   | Validation | Validate F19                         | draft  |

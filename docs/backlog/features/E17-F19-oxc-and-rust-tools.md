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

None yet: the spike is written at the refinement.

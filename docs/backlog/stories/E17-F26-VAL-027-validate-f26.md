---
id: VAL-027
epic: E17
feature: F26
title: Validate F26
status: ready
points: 1
---

# E17 · F26 · VAL-027 — Validate F26

F26 is a tooling fix: the agent validates it by #370's `Rust` check (ADR-0072).

## Acceptance criteria

- Given #370's CI after F26's merge into `develop`, when it ends, then its `Rust` check has ended, `stops_a_mutation_that_never_ends` passing.

## Tasks

- [ ] T1 — The check seen on #370 (main thread)

One task = one commit, referenced as `VAL-027.T1`.

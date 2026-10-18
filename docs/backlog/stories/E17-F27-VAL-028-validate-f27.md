---
id: VAL-028
epic: E17
feature: F27
title: Validate F27
status: ready
points: 1
---

# E17 · F27 · VAL-028 — Validate F27

F27 is a tooling fix: the agent validates it by #370's `Commit messages and branch name` check (ADR-0072).

## Acceptance criteria

- Given #370's CI after F27's merge into `develop`, when it ends, then its history check passes.

## Tasks

- [ ] T1 — The check seen on #370 (main thread)

One task = one commit, referenced as `VAL-028.T1`.

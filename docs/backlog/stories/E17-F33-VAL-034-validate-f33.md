---
id: VAL-034
epic: E17
feature: F33
title: Validate F33
status: done
points: 1
---

# E17 · F33 · VAL-034 — Validate F33

F33 is a tooling feature: the agent validates it on the checks (ADR-0072).

## Acceptance criteria

- Given F33's criteria, when the validation ends, then each is proven again and shown in a demo page.

## Tasks

- [x] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-034.T1`.

## Result (2026-10-19)

Validated by the agent (ADR-0072): F33's tagged tests pass (20 Rust, 10 TypeScript); the demo page `docs/guide/f33-the-checks-ret-004-listed.md`. Pending the Product Owner: whether `bench` runs without asking (criterion 3's permission).

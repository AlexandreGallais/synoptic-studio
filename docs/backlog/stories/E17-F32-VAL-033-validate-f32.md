---
id: VAL-033
epic: E17
feature: F32
title: Validate F32
status: done
points: 1
---

# E17 · F32 · VAL-033 — Validate F32

F32 is a tooling feature: the agent validates it on the reports of a run (ADR-0072).

## Acceptance criteria

- Given F32's criteria, when the validation ends, then each is proven again and shown in a demo page.

## Tasks

- [x] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-033.T1`.

## Result (2026-10-19)

Validated by the agent (ADR-0072): F32's tagged tests pass; `epic-check` read the CI of the last pull request into `main` live; the demo page `docs/guide/f32-reports-that-say-what-happened.md`.

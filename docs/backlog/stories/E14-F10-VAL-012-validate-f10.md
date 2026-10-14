---
id: VAL-012
epic: E14
feature: F10
title: Validate F10
status: done
points: 1
---

# E14 · F10 · VAL-012 — Validate F10

The Product Owner tries the round shape in the playground, guided by short steps, and says what they see (ADR-0020, ADR-0045).

## Acceptance criteria

- Given the playground, when the Product Owner follows the guided steps, then each option of the round shape is tried and the decisions taken by the agent at the refinement are confirmed or changed.

## Tasks

One task = one commit, referenced as `VAL-012.Tn`.

- [x] T1 — The guided test, one step per option, tested end to end (1 h)
- [x] T2 — The demo page as a list of the options (30 min)
- [x] T3 — The Product Owner's validation (1 h)

## Result (2026-10-14)

Validated. The agent's decisions are accepted as built: the cubic sides, point 1 at the top, the name **Convert to free shape**. **Convert to rounded rectangle** is removed: for an ellipse it gives a capsule, so it keeps the drawing only for a circle; F10.AC5 now states that no such conversion is offered.

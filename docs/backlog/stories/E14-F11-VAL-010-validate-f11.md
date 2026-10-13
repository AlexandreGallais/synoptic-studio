---
id: VAL-010
epic: E14
feature: F11
title: Validate F11
status: draft
points: 1
---

# E14 · F11 · VAL-010 — Validate F11

Show the Product Owner, on one demo page, the figures before and after F11: no guided test, since no drawing changes (decision of 2026-10-13).

## Acceptance criteria

- Given the conventions page of the folders, when the Product Owner reads it, then the layout and its reasons are clear to them.
- Given the demo page, when the Product Owner reads it, then it shows before and after: the largest folder (92 files in `playground/`), the functions without a property test (9), the repeated test helpers, the path writers (2), and `npm run check:all` green with the same tests and values.

## Tasks

One task = one commit, referenced as `VAL-010.Tn`. This story is not merged automatically (ADR-0028).

- [ ] T1 — Demo page of the figures, layout page (1 h)

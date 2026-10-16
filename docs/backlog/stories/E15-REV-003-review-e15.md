---
id: REV-003
epic: E15
title: Review E15 with the Product Owner
status: done
points: 3
---

# E15 · REV-003 — Review E15 with the Product Owner

Show everything E15 delivered, in plain language, and test it together (ADR-0023, `/review` skill).

## Acceptance criteria

- Given the validated features of E15, when the review is prepared, then an epic report in `docs/guide/` explains each feature, its terms and its limits, with pictures and a user test crossing all features.
- Given the feedback, when the review ends, then every change is a backlog item or a domain update, and the epic's closure criteria are checked.

## Tasks

One task = one commit, referenced as `REV-003.Tn`.

- [x] T1 — Re-run the guided tests of E15 (1 h)
- [x] T2 — Write the epic report and its user test (3 h)
- [x] T3 — Session with the Product Owner (1 h)

## Result (2026-10-16)

Held right after the third pass of VAL-018, as the Product Owner's tour of the whole Text tool in Chrome on `feature/f17-text-corrections`, which stands for the guided tests: every option worked. The guided tests of E15 pass in `check:all`. Report, remarks and closure: [E15 report](../../guide/e15-place-text.md).

**Backlog changes**

- The text direction (right to left) is kept for now and must be removed at the later UI/UX epic, the Latin script being the strict target: a to-do in [E02, ideas](../epics/E02-edit-shapes.md), noted in `docs/domain/shapes.md`.
- Italic leaving the box unchanged is kept as built.

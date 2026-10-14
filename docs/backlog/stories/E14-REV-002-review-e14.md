---
id: REV-002
epic: E14
title: Review E14 with the Product Owner
status: done
points: 3
---

# E14 · REV-002 — Review E14 with the Product Owner

Show everything E14 delivered, in plain language, and test it together (ADR-0023, `/review` skill).

## Acceptance criteria

- Given the validated features of E14, when the review is prepared, then an epic report in `docs/guide/` explains each feature, its terms and its limits, with pictures and a user test crossing all features.
- Given the user test, when the Product Owner runs it in the playground, then each step's result is recorded, with their remarks in neutral words.
- Given the feedback, when the review ends, then every misunderstanding or change is a backlog item or a domain update, and the epic's closure criteria are checked.

## Tasks

One task = one commit, referenced as `REV-002.Tn`.

- [x] T1 — Re-run the guided tests of every feature of E14 on `develop` (1 h)
- [x] T2 — Write the epic report and its user test (3 h)
- [x] T3 — Session with the Product Owner; record feedback and backlog changes (1 h)

## Result (2026-10-14)

Held right after the validation of F10 and F13, as a free walk-through of the playground on `develop` rather than a list of guided steps: every step worked as intended. Report, remarks and closure: [E14 report](../../guide/e14-draw-curves.md). The guided tests of E01, F12 and F10 pass on `develop` (`check:all`).

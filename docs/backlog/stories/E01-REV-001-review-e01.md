---
id: REV-001
epic: E01
title: Review E01 with the Product Owner
status: done
points: 3
---

# E01 · REV-001 — Review E01 with the Product Owner

Show everything E01 delivered, in plain language, and test it together (ADR-0023, `/review` skill).

## Acceptance criteria

- Given the validated features of E01, when the review is prepared, then an epic report in `docs/guide/` explains each feature, its terms and its limits, with pictures and a user test crossing all features.
- Given the user test, when the Product Owner runs it in the playground, then each step's result is recorded, with their remarks in neutral words.
- Given the feedback, when the review ends, then every misunderstanding or change is a backlog item or a domain update, and the epic's closure criteria are checked.

## Tasks

One task = one commit, referenced as `REV-001.Tn`.

- [x] T1 — Re-run the guided tests of every feature of E01 on `main` (1 h)
- [x] T2 — Write the epic report and its user test (3 h)
- [x] T3 — Session with the Product Owner; record feedback and backlog changes (1 h)

## Re-run on `main`

On 2026-10-12, `main` at release 0.14.0: the playground tests pass, 119 tests in 11 files, among them the walk through the 66 steps of the guided tests of F01, F02, F07, F03, F09, F08, F04 and F06, each step showing what it tells to look at. No regression.

Finding: the closure criteria of E01 point to `shapes.md` §2–3, whose §3 still holds text (moved to E15) and the layout box (E13); a question for the Product Owner at the session; they limited it to the rectangle and the regular polygon.

## Session

On 2026-10-12 the Product Owner ran steps 67 to 72; each showed what it meant to show. Their account and answers are in the [epic report](../../guide/e01-draw-shapes.md#review). Routed: the closure criterion of E01 limited to the rectangle and the regular polygon; the doubt of step 72 (a shape that seems to leave the canvas) and the "press Enter" wording to the improvement log. Closure criterion met: the cases of `shapes.md` §2 and of §3 Rectangle and Regular polygon are covered by the guided steps and tests of the features of E01, all green on `main`. E14 stays the next epic.

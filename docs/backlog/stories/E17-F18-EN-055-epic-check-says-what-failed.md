---
id: EN-055
epic: E17
feature: F18
title: "`epic:check` says what failed; the golden check at push"
status: done
points: 2
---

# E17 · F18 · EN-055 — `epic:check` says what failed; the golden check at push

Make the epic's full check name what failed, and catch a drift of results at the push rather than by hand (row 14 of RET-003; I34; F18.AC11).

## Acceptance criteria

- Given a `check:all` that fails, when `epic:check` reports it, then it names the failing lane and its failing tests.
- Given failed calls, when `epic:check` counts them, then refusals by the worktree isolation are counted apart, and calls left open by a force-quit are reported as lost.
- Given a golden file, when a branch is pushed, then the pre-push runs `golden:check`.
- Given `npm run metrics -- --since <day>`, then the report covers the calls from that day only.

## Tasks

One task = one commit, referenced as `EN-055.Tn`.

- [x] T1 — `epic:check`: failing tests, refusals apart, lost calls; tests (1 h)
- [x] T2 — `golden:check` in the pre-push; `metrics --since` (1 h)

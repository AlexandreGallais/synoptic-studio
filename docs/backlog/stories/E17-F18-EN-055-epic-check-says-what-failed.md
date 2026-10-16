---
id: EN-055
epic: E17
feature: F18
title: "`epic:check` says what failed; the golden check at push"
status: ready
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

## For the main thread

### Journal

- EN-055: `epic:check` names a failed `check:all` by its failing checks, their lanes and failing tests (Vitest, `cargo test`), counts the worktree isolation's refusals apart (549 in E15) and reports the calls started over an hour ago and never ended as lost, never as a failure; the pre-push runs `golden:check` after `check:all` whenever `.git/metrics/golden.json` exists (about 13 s, through the heavy lock); `npm run metrics -- --since <day>` covers the calls, requests and turns from 00:00 UTC that day on.

### Improvements

- `npm run metrics` read the transcripts of the folder named after the checkout it ran in: from a worktree it found none and reported 0 calls. It now reads the main checkout's folder (EN-055).
- A golden file left in `.git/metrics/` after a refactor now fails every push whose library results differ, including a feature that changes them on purpose: delete or rewrite it (`golden:write`, main checkout only) once the refactor is merged.

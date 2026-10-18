---
id: EN-128
epic: E17
feature: F27
title: The history check reads the scopes a commit was written with
status: done
points: 1
---

# E17 · F27 · EN-128 — The history check reads the scopes a commit was written with

`git-hooks commits`, the CI's history check, lints every commit since `main` against today's `SCOPES`: the six F19 commits with the scope `eslint`, removed by EN-085, refused #370 (run 39143909219).

## Acceptance criteria

- Given a commit whose only error is a scope of `RETIRED_SCOPES` (`eslint`), when the history check reads it, then it passes; given the same message, when the `commit-msg` hook reads it, then it is refused.

## Tasks

- [x] T1 — The interface and its tests, failing (main thread)
- [x] T2 — `without_retired_scopes` in the history check (main thread)

One task = one commit, referenced as `EN-128.T1` in the commit footer.

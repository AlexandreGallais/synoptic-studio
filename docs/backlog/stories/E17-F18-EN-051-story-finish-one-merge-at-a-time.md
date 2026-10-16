---
id: EN-051
epic: E17
feature: F18
title: "`story:finish` one merge at a time, from a worktree too"
status: draft
points: 2
---

# E17 · F18 · EN-051 — `story:finish` one merge at a time, from a worktree too

Merge parallel branches one after the other, and finish a story from its worktree without cleaning up by hand (ADR-0065, decision 5; I76, I55; F18.AC7).

## Acceptance criteria

- Given two stories finishing at once, when each runs `story:finish`, then the second waits for the merge lock, rebases on the updated base, then merges.
- Given `story:finish` run in a worktree whose base is checked out in the main checkout, when GitHub has merged, then it fetches, removes the worktree and deletes the branch here and on GitHub without switching branch.
- Given a pull request not yet `MERGED`, when the clean-up runs, then its branch is kept.

## Tasks

One task = one commit, referenced as `EN-051.Tn`.

- [ ] T1 — The merge lock and the rebase of the next branch; tests (1 h)
- [ ] T2 — The clean-up from a worktree, conditional on `MERGED`; tests (1 h)

---
id: EN-052
epic: E17
feature: F18
title: Worktrees named after their story, commands that fit them
status: ready
points: 2
---

# E17 · F18 · EN-052 — Worktrees named after their story, commands that fit them

Name each worktree after its story and answer at once a command that a worktree refuses, so that a worker loses no round trip (rows 4 and 10 of RET-003; 549 refused calls in E15; F18.AC8).

## Acceptance criteria

- Given `npm run worktree:new -- US-061`, when it runs, then `.claude/worktrees/us-061` exists on the story's branch, and `npm run activity` and the status line show `us-061`.
- Given an agent in a worktree, when it runs `cd <worktree> && git …` or a chained git command, then `guard-bash` denies it with a one-line hint of the form that works.
- Given `npm run golden:write` run in a worktree, then it refuses and says why.
- Given `branches:clean`, when a worktree's branch is merged or deleted, then the worktree is removed and pruned.

## Tasks

One task = one commit, referenced as `EN-052.Tn`.

- [x] T1 — `worktree:new` and its test; `branches:clean` prunes worktrees (1 h)
- [ ] T2 — `guard-bash` hints in worktrees; `golden:write` refused in a worktree; tests (1 h)

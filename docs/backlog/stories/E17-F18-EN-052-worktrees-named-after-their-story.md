---
id: EN-052
epic: E17
feature: F18
title: Worktrees named after their story, commands that fit them
status: done
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
- [x] T2 — `guard-bash` hints in worktrees; `golden:write` refused in a worktree; tests (1 h)

## For the main thread

- Technical choices made by the worker: the branch type follows the story's kind (`US` and `VAL` `feat`, `EN` `chore`, `NAM` `refactor`, `SP`, `CHK`, `AUD`, `REV` and `RET` `docs`), `--type` overrides it; an epic's own story (no feature in its file name) starts from `origin/develop`; the story file is read from the checkout the tool runs in, so a story added on a feature branch only needs that branch checked out or its file on `develop`.
- Husky writes the hooks path as `.husky/_`, relative, in the configuration shared by every worktree: the main checkout's was absolute (`<main>/.husky/_`), so every worktree ran the main checkout's hooks. The first `worktree:new` turns it relative and runs Husky in every worktree without `.husky/_` — those running at that moment included — so each runs the hooks of its own branch from then on.
- The guard answers only when the call's working directory is a worktree (Claude Code's worktree isolation). A worker started without isolation, whose Bash calls start in the main checkout, still needs `cd <worktree> && git …`: it is allowed there, since denying it would leave only `git -C`.

### Journal

- 2026-10-17 — EN-052: `npm run worktree:new -- <ID>` (Rust, `tools/worktree/`) makes `.claude/worktrees/<id>` on the story's branch from its feature branch, links `node_modules` and installs Husky's hooks folder so a push runs the branch's own `pre-push`; `branches:clean` removes the worktrees of merged or deleted branches; `guard-bash` answers a chained Git command in a worktree with the plain command, and refuses `golden:write` there, which the golden test refuses too.

### Improvements

| 2026-10-17 | EN-052 | Warn when a subagent working for a worktree writes in the main checkout: a hook cannot tell it today, since such a subagent's calls start in the main checkout and the hook input names no worktree; a worker's prompt naming its worktree, read by a `PreToolUse` hook on `Write` and `Edit`, could | agent |

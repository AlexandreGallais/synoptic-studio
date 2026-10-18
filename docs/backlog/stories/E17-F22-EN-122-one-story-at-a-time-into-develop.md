---
id: EN-122
epic: E17
feature: F22
title: One story at a time into develop
status: done
points: 2
---

# E17 · F22 · EN-122 — One story at a time into develop

ADR-0086, decided by the Product Owner on 2026-10-18 during F22's run and revised the same day: one agent at a time, one story at a time; a feature branch per feature from `develop`; every merge a merge commit, story → feature and feature → `develop`; a review at each return of the Product Owner, a `REV` from `develop`, after which `develop` → `main` is opened; the merge into `main` deploys the site and makes no release.

## Acceptance criteria

- Given a story's pull request into its feature branch labelled `autonomous`, when `story-automerge.yml` runs, then it merges it by a merge commit; a feature branch or a `REV` into `develop` is merged by a merge commit too; a branch that is neither a story's nor a feature's is still refused.
- Given a story of a feature, when `worktree-new` makes its worktree, then its branch starts from the feature branch on GitHub; a `REV` starts from `origin/develop`.
- Given a review asked by the Product Owner while a feature is under way, when the run skill is read, then it says the feature is finished first, then a `REV` from `develop` holds the review, after which `develop` → `main` is opened.
- Given the run skill, the worker brief, `project-rules`, `AGENTS.md` and `docs/tooling/git-workflow.md`, when read, then they describe ADR-0086's flow: one agent at a time, every merge a merge commit, a review at each return; nothing of the rebase merges, the parallel stories nor the two agents per story.

## Tasks

- [x] T1 — ADR-0086, the backlog of F22 rewritten on it (main thread, 2026-10-18; revised the same day)
- [x] T2 — `story-automerge.yml` by a merge commit into a feature branch and into `develop`, `worktree-new` from the feature branch, tests first (1 h)
- [x] T3 — The run skill, the worker brief, `project-rules`, `AGENTS.md`, `CLAUDE.md` and `docs/tooling/git-workflow.md` on ADR-0086 (1 h)

One task = one commit, referenced as `EN-122.T1` in the commit footer.

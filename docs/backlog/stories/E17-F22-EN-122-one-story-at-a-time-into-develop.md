---
id: EN-122
epic: E17
feature: F22
title: One story at a time into develop
status: ready
points: 2
---

# E17 · F22 · EN-122 — One story at a time into develop

ADR-0086, decided by the Product Owner on 2026-10-18 during F22's run: one agent at a time; each story from `develop` back into `develop` by a merge commit, no feature branch; `develop` → `main` opened at the Product Owner's review when `develop`'s head is a finished feature, a review asked mid-feature waiting for its end; the merge into `main` deploys the site and makes no release.

## Acceptance criteria

- Given a story's pull request into `develop` labelled `autonomous`, when `story-automerge.yml` runs, then it merges it by a merge commit; a branch that is not a story's is still refused.
- Given a story of a feature, when `worktree-new` makes its worktree, then its branch starts from `origin/develop`; `story:finish` with `develop` as its base merges it and moves its rows.
- Given a review asked by the Product Owner while a feature is under way, when the run skill is read, then it says the review waits for that feature's end and covers, in one pull request `develop` → `main`, every feature finished since the last merge into `main`.
- Given the run skill, the worker brief, `project-rules`, `AGENTS.md` and `docs/tooling/git-workflow.md`, when read, then they describe ADR-0086's flow and nothing of the feature branch, the parallel stories nor the two agents per story.

## Tasks

- [x] T1 — ADR-0086, the backlog of F22 rewritten on it (main thread, 2026-10-18)
- [ ] T2 — `story-automerge.yml`, `worktree-new`, `git-sequences sync` and `check-handback` on `develop`, tests first (1 h)
- [ ] T3 — The run skill, the worker brief, `project-rules`, `AGENTS.md`, `CLAUDE.md` and `docs/tooling/git-workflow.md` on ADR-0086 (1 h)

One task = one commit, referenced as `EN-122.T1` in the commit footer.

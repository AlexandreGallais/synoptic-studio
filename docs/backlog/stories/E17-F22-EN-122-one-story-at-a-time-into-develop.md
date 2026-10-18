---
id: EN-122
epic: E17
feature: F22
title: One story at a time into develop
status: ready
points: 2
---

# E17 · F22 · EN-122 — One story at a time into develop

ADR-0086, decided by the Product Owner on 2026-10-18 during F22's run: one agent at a time; each story from `develop` back into `develop` by a merge commit, no feature branch; a finished feature marked `review/f<nn>`, merged into `main` at the Product Owner's review, which deploys the site and makes no release.

## Acceptance criteria

- Given a story's pull request into `develop` labelled `autonomous`, when `story-automerge.yml` runs, then it merges it by a merge commit; a branch that is not a story's is still refused.
- Given a story of a feature, when `worktree-new` makes its worktree, then its branch starts from `origin/develop`; `story:finish` with `develop` as its base merges it and moves its rows.
- Given a feature's `VAL` merged, when the feature ends, then `review/f<nn>` points at its merge commit and the branch pattern accepts it.
- Given the run skill, the worker brief, `project-rules`, `AGENTS.md` and `docs/tooling/git-workflow.md`, when read, then they describe ADR-0086's flow and nothing of the feature branch, the parallel stories nor the two agents per story.

## Tasks

- [x] T1 — ADR-0086, the backlog of F22 rewritten on it (main thread, 2026-10-18)
- [ ] T2 — `story-automerge.yml`, `worktree-new` and the branch pattern, tests first (1 h)
- [ ] T3 — The run skill, the worker brief, `project-rules`, `AGENTS.md`, `CLAUDE.md` and `docs/tooling/git-workflow.md` on ADR-0086 (1 h)

One task = one commit, referenced as `EN-122.T1` in the commit footer.

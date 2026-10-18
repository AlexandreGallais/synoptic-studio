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
- [ ] T2 — `story-automerge.yml` by a merge commit into a feature branch and into `develop`, `worktree-new` from the feature branch, tests first (1 h)
- [ ] T3 — The run skill, the worker brief, `project-rules`, `AGENTS.md`, `CLAUDE.md` and `docs/tooling/git-workflow.md` on ADR-0086 (1 h)

One task = one commit, referenced as `EN-122.T1` in the commit footer.

## For the main thread

### Journal

- `#run` The Product Owner started an autonomous run on F22 (`/run F22`) on 2026-10-18: F22 set `in-progress`, its stories `ready`; F22 is a tooling feature, validated by the agent (ADR-0072).
- `#decision` ADR-0086, decided by the Product Owner during the run, superseding ADR-0083 and parts of ADR-0084: the parallel work spent the week's usage too fast, in rebases and merges done again; so one agent at a time, in the background so that the chat stays free; a feature branch per feature from `develop`, every merge a merge commit (story → feature, feature → `develop`); a review at each return of the Product Owner, a feature under way finished first, then a `REV` from `develop` (naming, tool fixes, method, docs) and `develop` → `main`, which deploys the site only, epics losing their closing (EN-123); also superseding parts of ADR-0023, ADR-0049 and ADR-0085; the release made by a button of GitHub's Actions tab, its version from the tags, its notes the features' texts, `CHANGELOG.md` deleted. F22 rewritten: EN-122 first, then EN-116, EN-117 on the button, AUD-024, VAL-023, on `feature/f22-release-by-a-button`.

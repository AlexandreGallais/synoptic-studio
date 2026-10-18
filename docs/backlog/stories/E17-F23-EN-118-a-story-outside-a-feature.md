---
id: EN-118
epic: E17
feature: F23
title: A story outside a feature, by the same tools
status: ready
points: 3
---

# E17 · F23 · EN-118 — A story outside a feature, by the same tools

From the cards C1, C3 and C4, the row I50 (RET-004), the cards of 2026-10-18 on `worktree-new` from the main checkout and on `story-automerge.yml` for an epic's `SP` ([tool incidents](../../process/tool-incidents.md)), and ADR-0085's consequences. A story attached to `develop` is a `REV` or an epic's refinement `SP` (ADR-0086); the `SP` is merged by the Product Owner, kept so at REV-006. Its rows reach the logs by EN-137's `done` commit, as any story's (C6).

## Acceptance criteria

- Given a story attached to `develop`, when `worktree-new`, the backlog test and `story-finish develop` run, then the worktree starts from `origin/develop`, the story is accepted, and `story-finish` opens its pull request: a `REV` labelled `autonomous`, waited for until merged; an epic's `SP` without the label, `story-finish` saying at once that it waits for the Product Owner's merge, never waiting three minutes (C1, C3, C4).
- Given the main checkout on `develop` and a story whose file is only on its feature branch, when `worktree-new <ID>` runs there, then it finds the file on `origin`'s feature branches (`git ls-tree`), starts the worktree from that branch, and refuses naming them when two branches hold the id.
- Given a fix without a story id, when its branch is named, then the auto-merge's pattern, the pre-push's and the guidance of `docs/tooling/git-workflow.md` accept the same one form, and refuse the others alike (I50).

## Tasks

- [x] T1 — Each criterion's tests in `tools/worktree`, `story-finish`, `tests/backlog.test.ts` and `tests/story-automerge.test.ts`, failing (1 h)
- [x] T2 — `worktree-new`, `story-finish` and the branch form, the run skill and `docs/tooling/git-workflow.md` (2 h)

One task = one commit, referenced as `EN-118.T1` in the commit footer.

## For the main thread

### Journal

- `#tooling` EN-118: `story-finish develop` labels a `REV` and awaits it, opens an epic's `SP` without the label and says at once that it waits for the Product Owner's merge; `worktree-new` in the main checkout finds a story only on its feature branch (`git ls-tree`) and refuses one two branches hold; the pre-push refuses a work branch without the id of a story the auto-merge takes, six names of the history now refused (I50, cards C1, C3, C4).

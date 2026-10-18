---
id: EN-118
epic: E02
feature: F23
title: Stories outside a feature, and the cards as a log
status: draft
points: 3
---

# E02 · F23 · EN-118 — Stories outside a feature, and the cards as a log

From the cards C1, C3, C4 and C6 and the row I50 (RET-004), and ADR-0085's consequences. The story auto-merge into `develop` stays parked until the next review (ADR-0086).

## Acceptance criteria

- Given a story attached to `develop` (an epic's story), when `worktree-new`, the backlog test and `story-finish develop` run, then the worktree starts from `origin/develop`, the story is accepted, and `story-finish` opens its pull request and says it waits for the Product Owner's merge rather than waiting three minutes (C1, C3, C4).
- Given such a story merged, when its rows are moved, then they reach the logs on `develop` (C6).
- Given `docs/process/tool-incidents.md`, when a story branch of a worker changes it, then the pre-push refuses it; a worker's `### Incidents` is moved by `move-rows`; a card without its date, use, failure or workaround fails `tests/logs.test.ts`; the file merges by union (ADR-0085).
- Given a fix without a story id, when its branch is named, then the auto-merge's pattern and the guidance agree on one form (I50).

## Tasks

- [ ] T1 — Each criterion's tests first, failing; then the tools (3 h)

One task = one commit, referenced as `EN-118.T1` in the commit footer.

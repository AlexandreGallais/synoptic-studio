---
id: EN-123
epic: E17
feature: F22
title: Epics without a closing, a review at each return
status: ready
points: 3
---

# E17 · F22 · EN-123 — Epics without a closing, a review at each return

ADR-0086, decided by the Product Owner on 2026-10-18 during F22's run: an epic is a list of features grouped by theme, with no naming pass, review nor retrospective at its end; the review and the improvement of the method and the tools happen at each return of the Product Owner, in a `REV` story from `develop`; a refinement opens an epic as a bubble on `develop`, and a feature when questions remain, as the first bubble of its branch.

## Acceptance criteria

- Given the backlog's templates, the `/review`, `/retro` and `/spike` skills, `docs/process/agile.md`, `AGENTS.md`, `project-rules` and the worker brief, when read, then no epic has a closing: a `REV` at each return covers the features finished since the last merge into `main`, the naming pass over the code changed since then, the tool incident cards fixed, the method's improvements decided with the Product Owner and the docs; a tool that fails is fixed at the next review, no longer at the epic's end.
- Given a new epic, when it starts, then its refinement is an `SP` story from `develop` (the Product Owner's account, the documentary research, the features cut); given a feature with questions left, then its first story is a short `SP` on its feature branch.
- Given a feature refined with more than 4 stories, or a story with more than 3 tasks, when the agent writes it, then it proposes a split to the Product Owner instead, with an example (ADR-0084).
- Given E02's `NAM-003`, `REV-005` and `RET-005`, when the story ends, then they are removed from the backlog, and `tests/backlog.test.ts` neither asks an epic for a closing nor accepts one; `docs/tooling/ci.md` describes the `feature branches` ruleset as merge commits only.

## Tasks

- [ ] T1 — `tests/backlog.test.ts` on the new shape, failing: no closing per epic, a feature of at most 4 stories, a story of at most 3 tasks (1 h)
- [ ] T2 — The templates, E02's closing stories removed, the skills, `docs/process/agile.md`, `AGENTS.md`, `project-rules`, the worker brief and `docs/tooling/ci.md` (2 h)

One task = one commit, referenced as `EN-123.T1` in the commit footer.

## For the main thread

### Journal

- `#decision` The Product Owner, during F22's run (2026-10-18): a refinement opens each epic as a bubble on `develop`, with the documentary research on what they describe, and a feature when questions remain; the agent refuses a feature over 4 stories and proposes its split; the epics' division by theme is to be reworked with the Product Owner at the next review.

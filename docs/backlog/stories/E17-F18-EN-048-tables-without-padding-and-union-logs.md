---
id: EN-048
epic: E17
feature: F18
title: Tables without padding, append-only logs merged by union
status: ready
points: 2
---

# E17 · F18 · EN-048 — Tables without padding, append-only logs merged by union

Stop Markdown tables from being rewritten whole when one cell grows, and let parallel branches add log rows without conflict (ADR-0065, decisions 1 and 2; F18.AC4).

SP-020 ([research note 0022](../../research/notes/0022-f18-parallel-run.md)): Prettier prints a table without padding only with `proseWrap: "never"` **and** when its padded form is wider than `printWidth` (`REF-PRETTIER-TABLE-CODE`); tried on the repository, about 61 of 607 tables stay padded and every paragraph written on several lines is joined. ADR-0065 decision 1 and its source disagree: the Product Owner chooses before T1 (options in the note). Whether GitHub's merge applies `merge=union` is not documented: tried once before relying on it (`REF-GIT-UNION`).

## Acceptance criteria

- Given Prettier's `proseWrap: "never"`, when a cell wider than the others is added to a table, then only that row changes.
- Given `merge=union` on `journal.md`, `improvements.md` and `feedback.md`, when two branches each add a row, then the merge keeps both without conflict.
- Given a log, when a row appears twice after a union merge, then a test fails and names it.

## Tasks

One task = one commit, referenced as `EN-048.Tn`.

- [ ] T1 — `proseWrap: "never"` and the docs reformatted once, in a commit of its own (1 h)
- [ ] T2 — `.gitattributes` union lines and the uniqueness test, mutation-checked (1 h)

## For the main thread

### Improvements

| 2026-10-17 | EN-048 | Whether GitHub's merge of a pull request applies `merge=union` is not documented (SP-020): try it once at a later merge that brings log rows from two sides, and see whether both are kept, before relying on it beyond local merges | tooling |

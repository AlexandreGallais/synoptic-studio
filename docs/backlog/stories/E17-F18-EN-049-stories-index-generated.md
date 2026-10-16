---
id: EN-049
epic: E17
feature: F18
title: The stories index generated from the story files
status: ready
points: 2
---

# E17 · F18 · EN-049 — The stories index generated from the story files

Generate `docs/backlog/stories/README.md` from each story's front matter, so that no branch writes it by hand (ADR-0065, decision 3; F18.AC5).

## Acceptance criteria

- Given the story files, when `npm run backlog:status` (or a `backlog:index` step) runs, then the index is written from their front matter in the order of their features.
- Given an index edited by hand, when `backlog.test.ts` runs, then it fails and says to run the generator.

## Tasks

One task = one commit, referenced as `EN-049.Tn`.

- [ ] T1 — The generator and its test; `backlog:status` calls it (2 h)

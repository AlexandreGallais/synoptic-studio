---
id: EN-050
epic: E17
feature: F18
title: Shared files written only at merge
status: ready
points: 2
---

# E17 · F18 · EN-050 — Shared files written only at merge

Keep the shared files — the indexes, the logs, the feature file, the checklist — to the main thread at merge: a worker writes its rows in its story file (ADR-0065, decision 4; F18.AC6).

## Acceptance criteria

- Given a story branch that edits a shared file, when it is pushed, then the pre-push check refuses it and names the file.
- Given a story file holding rows for a log under a heading the tool knows, when the story is merged, then `story:finish` moves them into the log.

## Tasks

One task = one commit, referenced as `EN-050.Tn`.

- [ ] T1 — The list of shared files and the pre-push check; tests (1 h)
- [ ] T2 — The rows moved at merge by `story:finish`; the worker brief updated (1 h)

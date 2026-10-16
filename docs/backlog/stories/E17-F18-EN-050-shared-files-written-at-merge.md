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

## For the main thread

- Technical choices made by the worker: the check covers `US` and `EN` branches only, the stories a worker implements; the others (`SP`, `CHK`, `AUD`, `VAL`, `NAM`, `REV`, `RET`) are the main thread's and may write shared files. The story's `done` commit (`Closes: <ID>`) may still set its status in the indexes and the feature's table, since `backlog.test.ts` wants the three to agree on the story branch; it may not write a log nor a checklist.
- Once rows were moved, `story:finish` pushes the feature branch: its pre-push runs `check:all` under the heavy lock, one more full check per story that had rows. EN-051 reworks `story:finish` for worktrees and keeps this step.
- The known headings are `### Journal`, `### Improvements` and `### Feedback`; another heading under `## For the main thread`, such as EN-047's `### Rows for the logs`, stays in the story file.

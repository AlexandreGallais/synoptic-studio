---
id: EN-137
epic: E17
feature: F23
title: The log rows and the cards in the done commit
status: ready
points: 3
---

# E17 · F23 · EN-137 — The log rows and the cards in the done commit

A commit `docs(docs): move the rows of … into the logs` sits between the story bubbles of a feature: `story:finish` moves a story's rows after its merge, on the feature branch, because ADR-0065 kept workers off the shared files while stories ran in parallel (REV-006). With one story at a time (ADR-0086) the main thread writes the `done` commit (`Closes: <ID>`), which the pre-push already lets change the story's status in the indexes and its feature's table (`tools/shared-files/src/shared.rs`). The cards of `docs/process/tool-incidents.md` are written by hand, outside the logs' tools (ADR-0085's consequences, C6).

## Acceptance criteria

- Given a story whose file holds rows under `## For the main thread`, when the main thread makes its `done` commit, then `move-rows` writes those rows into the logs in that commit and removes them from the story file, and `story:finish` pushes no commit after the merge; the graph shows the story's bubble and nothing between it and the next.
- Given a worker's story branch, when a commit other than its `done` commit changes a log, then the pre-push refuses it naming the file; when its `done` commit adds the story's own rows to the logs, then it passes.
- Given `docs/process/tool-incidents.md`, when it is read by the tools, then it is a log of `tools/shared-files`, a worker's `### Incidents` is moved by `move-rows` as a card, a card without its date, its use, its failure or its workaround fails `tests/logs.test.ts`, and the file merges by union (ADR-0085).

## Tasks

- [ ] T1 — The tests of `move-rows`, `story-finish`, `shared-files` and `tests/logs.test.ts` for each criterion, failing (1 h)
- [ ] T2 — The rows and cards moved in the `done` commit, the pre-push and the docs of the run (2 h)

One task = one commit, referenced as `EN-137.T1` in the commit footer.

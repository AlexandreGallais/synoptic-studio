---
id: EN-137
epic: E17
feature: F23
title: The log rows and the cards in the done commit
status: done
points: 3
---

# E17 · F23 · EN-137 — The log rows and the cards in the done commit

A commit `docs(docs): move the rows of … into the logs` sits between the story bubbles of a feature: `story:finish` moves a story's rows after its merge, on the feature branch, because ADR-0065 kept workers off the shared files while stories ran in parallel (REV-006). With one story at a time (ADR-0086) the main thread writes the `done` commit (`Closes: <ID>`), which the pre-push already lets change the story's status in the indexes and its feature's table (`tools/shared-files/src/shared.rs`). The cards of `docs/process/tool-incidents.md` are written by hand, outside the logs' tools (ADR-0085's consequences, C6).

## Acceptance criteria

- Given a story whose file holds rows under `## For the main thread`, when the main thread makes its `done` commit, then `move-rows` writes those rows into the logs in that commit and removes them from the story file, and `story:finish` pushes no commit after the merge; the graph shows the story's bubble and nothing between it and the next.
- Given a worker's story branch, when a commit other than its `done` commit changes a log, then the pre-push refuses it naming the file; when its `done` commit adds the story's own rows to the logs, then it passes.
- Given `docs/process/tool-incidents.md`, when it is read by the tools, then it is a log of `tools/shared-files`, a worker's `### Incidents` is moved by `move-rows` as a card, a card without its date, its use, its failure or its workaround fails `tests/logs.test.ts`, and the file merges by union (ADR-0085).

## Tasks

- [x] T1 — The tests of `move-rows`, `story-finish`, `shared-files` and `tests/logs.test.ts` for each criterion, failing (1 h)
- [x] T2 — The rows and cards moved in the `done` commit, the pre-push and the docs of the run (2 h)

One task = one commit, referenced as `EN-137.T1` in the commit footer.

## For the main thread

- This story's own `done` commit is the first made by the new step: run `target/release/move-rows chore/en-137-the-log-rows-in-the-done-commit` before `backlog-status` (it prints nothing for this story, which holds no rows), then commit the files it prints with `docs/backlog`.
- `docs/process/tool-incidents.md`, a shared file, still says in its opening paragraph "Written by the main thread at merge; a worker's card goes into its story file under `### Incidents`": to change in the `done` commit into "Moved by the main thread in the story's `done` commit (ADR-0091); a worker's card goes into its story file under `### Incidents`, its heading `#### <date> — <title>`".
- `.claude/settings.json` allows no `target/release/move-rows`: each call of the new step asks for permission until a review adds it beside `backlog-status`, and `docs/tooling/agent-tools.md`'s list of the binaries the agent calls with it (left out here, since a worker does not change the permission settings).

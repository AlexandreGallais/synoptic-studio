# ADR-0091 — A story's log rows and cards moved in its `done` commit

**Status**: Accepted (the agent, 2026-10-19, EN-137, from the Product Owner's decision at REV-006 on 2026-10-18 that no commit sits between the bubbles of a feature's stories; to be told to the Product Owner). Supersedes, in ADR-0065's decision 4, "at merge" for the logs: the main thread writes a story's rows into the logs in the story's `done` commit, on its branch, before its merge; the rest of decision 4 stands. Supersedes, in ADR-0085's decision 2, "written by the main thread at merge like the other logs": the cards are moved the same way. Decision 2 superseded by ADR-0102 (2026-10-25): no pre-push check reads the logs, and `move-rows` is no longer required.

## Context

- ADR-0065 kept the shared files off the story branches while stories ran in parallel: a row written on two branches at once conflicted at their merges. `story:finish` therefore moved a story's rows into the logs after its merge, in a commit `docs(docs): move the rows of <ID> into the logs` on the feature branch.
- Since ADR-0086 a run works one story at a time, and each story enters its feature branch by a merge commit, one bubble per story in the graph. The rows' commit sat on the feature's line between two bubbles, outside the story it belonged to (REV-006).
- The `done` commit (`Closes: <ID>`) is the main thread's, made in the story's worktree once its worker has ended (ADR-0086, decision 1); the pre-push check already let it set the story's status in the indexes and its feature's table (`tools/shared-files`).
- With one story at a time, no other branch writes the logs while a story is open, so its `done` commit writing them meets no conflict; the logs keep their union merge (ADR-0065, decision 2) for the main thread's own commits beside it.

## Decision

1. **The `done` commit holds the story's rows and cards.** Before it, the main thread runs `target/release/move-rows <branch>` in the story's worktree, which moves the rows under `## For the main thread` into the logs and the cards under `### Incidents` into `docs/process/tool-incidents.md`, and commits the files it prints with the statuses.
2. **The pre-push lets the `done` commit change the logs.** Any other commit of a worker's branch that changes a log is still refused, the file named; a checklist stays refused in every commit of a worker's branch.
3. **`story:finish` commits nothing after the merge.** It brings the base up to date and removes the worktree and the branch; a story file still holding rows for the logs stops it before its pull request, naming the command that moves them.

## Reasons

- The graph the Product Owner reads holds one bubble per story and nothing between two bubbles: the rows are inside the story they describe.
- One commit fewer per story, and no push on the feature branch outside a merge, so no `check:all` run for it.
- The rows stay written by the main thread, as ADR-0065 wanted: only the moment moves, from after the merge to the `done` commit just before it.

| Option | For | Against |
| --- | --- | --- |
| Rows in the `done` commit (chosen) | one bubble per story, nothing between bubbles; one commit and one push fewer | the main thread runs `move-rows` before its commit |
| Rows moved after the merge (ADR-0065) | no step before the `done` commit | a commit between the bubbles of every story with rows |
| `story:finish` folds the rows into the `done` commit | nothing to remember | an amend and a second push with lease of the story branch, its `check:all` run twice |

## Consequences

- `tools/shared-files` lists `tool-incidents.md` among the logs, lets the `done` commit change the logs, and `move-rows` takes the cards of `### Incidents`; `story-finish` no longer runs `move-rows` and refuses a story file still holding rows; `tests/logs.test.ts` checks the cards' form and their union merge (EN-137, [F23](../backlog/features/E17-F23-the-path-of-a-story.md)).
- The run skill's `done` commit step, the worker brief's shared files, `AGENTS.md` and `docs/tooling/agent-tools.md` say so.
- If stories run in parallel again, this decision is reviewed with ADR-0086: two `done` commits writing the logs would meet at their merges, which the union merge resolves for appended lines only.

## References

ADR-0065, ADR-0085, ADR-0086, `REF-GIT-UNION`

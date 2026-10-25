# ADR-0102 — A worker may write the shared files, no push refused for them

**Status**: Accepted (2026-10-25, AUD-050): asked by the Product Owner on 2026-10-25 with the faster pushes. Supersedes what stood of ADR-0065's decision 4 (the shared files written by the main thread only, a pre-push check refusing a story branch that edits one) and ADR-0091's decision 2 (the pre-push letting only the `done` commit change the logs).

## Context

- ADR-0065, decision 4, let only the main thread write the shared files — the backlog's indexes, the logs of `docs/process/`, the feature files and the checklists — and a pre-push check, `refuse-shared-files` (EN-050), refused a worker's story branch whose commits changed one. ADR-0091 moved a story's rows into the logs in its `done` commit, through `move-rows`.
- The refusal cost pushes: a worker that wrote a log row, or a table the formatter padded, was refused after its commits, and had to undo the change and push again, each push then running its checks. The logs merge by union (ADR-0065, decision 2) and the stories index is generated (decision 3), so most edits of a shared file no longer conflict.

## Decision

1. **A worker may write the shared files**, the logs included, in any commit of its story branch.
2. **No pre-push refusal**: `refuse-shared-files` and the padding check it alone used are removed; the `pre-push` hook no longer reads the pushed refs for it.
3. **`move-rows` stays and is no longer required**: rows a worker writes under `## For the main thread` in its story file are still moved into the logs by the `done` commit, and `story:finish` still stops on a story file holding rows; a worker may instead write the row in the log itself.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| No refusal (chosen) | no push refused and made again; a worker writes a log row where it belongs | a conflict on a feature file or an index between two stories stays possible, resolved at the rebase of `story:finish` |
| The refusal as it was | no worker's change on a shared file | pushes refused after their commits, each retry a push and its checks |

## Consequences

- The `pre-push` hook runs the branch name, `check:all` and `golden:check`; its tests and `tests/check-targets.test.ts` no longer name the refusal.
- The worker's brief and the run skill say a worker may write the shared files; `docs/tooling/agent-tools.md`, `git-workflow.md` and the hooks' table of `docs/conventions/rules.md` drop the refusal, and the workspace counts one binary fewer (`tools/tool-use/tests/binaries.rs`).
- If stories run in parallel again (ADR-0086), two branches writing the same row of an index or a feature file meet at their merges: this decision is reviewed then.

## References

ADR-0065, ADR-0091, EN-050, AUD-025
